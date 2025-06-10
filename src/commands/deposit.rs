use crate::{sol::DepositContract, types::deposit::AddressResponse};
use alloy::{
    network::EthereumWallet, primitives::U256, providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
};
use anyhow::Result;
use clap::Args;
use reqwest::Client;
use std::str::FromStr;
use url::Url;

#[derive(Args)]
pub struct DepositCommand {
    #[arg(long, env = "DEPOSIT_AMOUNT", hide_env = true)]
    amount: u64,

    #[arg(long, env = "BASE_URL", hide_env = true)]
    base_url: Url,

    #[arg(long, env = "PRIVATE_KEY", hide_env = true)]
    private_key: PrivateKeySigner,
}

impl DepositCommand {
    pub async fn run(&self) -> Result<()> {
        let url = Url::parse(&format!("{}/v1/p/funding/contractAddress", self.base_url))?;
        let response: AddressResponse = Client::new().get(url).send().await?.json().await?;

        let provider = ProviderBuilder::new()
            .wallet(EthereumWallet::new(self.private_key.clone()))
            .on_http(Url::from_str("http://k8s-ethnode-hoodilb-51b8943c62-008bc2ba8cea2857.elb.ap-east-1.amazonaws.com:8545/")?);

        println!("\nDepositing...");
        let contract = DepositContract::new(response.data.address, provider);
        let _ = contract
            .deposit(vec![])
            .value(U256::from(self.amount))
            .send()
            .await?;

        println!("Done");
        Ok(())
    }
}
