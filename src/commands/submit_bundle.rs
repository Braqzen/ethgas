use crate::{commands::slot::SlotCommand, types::preconf::BundleResponse};
use alloy::{
    eips::eip1559::Eip1559Estimation,
    hex,
    network::{
        Ethereum, EthereumWallet, NetworkWallet, TransactionBuilder, eip2718::Encodable2718,
    },
    primitives::U256,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use anyhow::Result;
use clap::Args;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use reqwest::Client;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

#[derive(Args)]
pub struct SubmitBundleCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    jwt: String,

    #[arg(long, env = "PRIVATE_KEY", hide_env = true)]
    private_key: PrivateKeySigner,
}

impl SubmitBundleCommand {
    pub async fn run(&self) -> Result<()> {
        let slot = SlotCommand {
            base_url: self.base_url.clone(),
            jwt: self.jwt.clone(),
        }
        .execute()
        .await?;

        let raw_tx = generate_test_transaction(&self.private_key).await?;

        let mut url = Url::parse(&format!("{}/v1/user/bundle/send", self.base_url))?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("slot", &slot.to_string());
            pairs.append_pair("replacementUuid", &Uuid::new_v4().to_string());
            pairs.append_pair("txs", &format!("[{{tx={}&canRevert=false}}]", raw_tx));
        }

        let response: BundleResponse = Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", self.jwt))
            .send()
            .await?
            .json()
            .await?;

        if !response.success {
            return Err(anyhow::anyhow!("Failed to submit bundle"));
        }

        println!("Bundle: {:?}", response);

        Ok(())
    }
}

async fn generate_test_transaction(private_key: &PrivateKeySigner) -> Result<String> {
    println!("[8] Generate test transaction");
    let provider = ProviderBuilder::new()
        .wallet(EthereumWallet::new(private_key.clone()))
        .on_http(Url::from_str("http://k8s-ethnode-hoodilb-51b8943c62-008bc2ba8cea2857.elb.ap-east-1.amazonaws.com:8545/")?);

    let signer = EthereumWallet::new(private_key.clone());
    let sender = <EthereumWallet as NetworkWallet<Ethereum>>::default_signer_address(&signer);

    // Generate random input
    let input: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    // Build initial transaction
    let mut tx = TransactionRequest::default();
    tx = tx
        .transaction_type(2) // EIP-1559
        .with_chain_id(560048) // Hoodi Chain ID
        .with_from(private_key.address())
        .with_to(private_key.address())
        .with_value(U256::from(1))
        .with_input(input);

    // Estimate gas parameters
    let (eip1559_estimate, gas_estimate) = setup_gas_params(&provider, &tx).await?;
    tx = tx
        .with_gas_limit(gas_estimate)
        .with_max_fee_per_gas(eip1559_estimate.max_fee_per_gas)
        .with_max_priority_fee_per_gas(eip1559_estimate.max_priority_fee_per_gas);

    // Build and sign typed transaction
    let typed_tx = tx
        .transaction_type(2)
        .build_typed_tx()
        .map_err(|err| anyhow::anyhow!("Error building typed transaction: {:?}", err))?;

    let signature = <EthereumWallet as NetworkWallet<Ethereum>>::sign_transaction_from(
        &signer, sender, typed_tx,
    )
    .await
    .map_err(|err| anyhow::anyhow!("Error signing transaction: {:?}", err))?;

    let mut buf = Vec::new();
    signature.encode_2718(&mut buf);
    Ok(format!("0x{}", hex::encode(buf)))
}

async fn setup_gas_params(
    provider: &impl Provider<Ethereum>,
    tx: &TransactionRequest,
) -> anyhow::Result<(Eip1559Estimation, u64)> {
    let max_fee = provider
        .estimate_eip1559_fees()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to estimate EIP-1559 fees: {:?}", e))?;

    let estimated_gas = provider
        .estimate_gas(tx.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to estimate gas: {:?}", e))?;

    Ok((max_fee, estimated_gas))
}
