mod types;

use crate::types::EthGasResponse;
use alloy::signers::{Signer, local::PrivateKeySigner};
use anyhow::Result;
use reqwest::{Client, Url};
use serde_json::Value;
use std::str::FromStr;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = Url::parse("https://testchain.app.ethgas.com/api")?;
    let chain_id = 17000;

    // let private_key = PrivateKeySigner::from_str("")?;
    let private_key = PrivateKeySigner::random();
    let address = private_key.address();
    let client = Client::new();

    // Step 1: Login to EthGas to get sign-in message
    let mut url = Url::parse(&format!("{base_url}/v1/user/login"))?;
    {
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("addr", &address.to_string());
        pairs.append_pair("chainId", &chain_id.to_string());
    }
    let response = client.post(url).send().await?;

    let response: EthGasResponse = response.json().await?;
    // println!("{:?}", response);

    // Step 2: Sign the EIP-712 login message with our private key
    dbg!(&response.data.eip712Message);

    let digest = response.data.eip712Message.digest()?;
    let signature = private_key.sign_hash(&digest.into()).await?;
    let sig_bytes = signature.as_bytes();
    let signature = format!("0x{}", alloy::hex::encode(sig_bytes));

    println!("Signature: {}", signature.to_string());
    println!("addr: {}", &address.to_string());
    println!("nonceHash: {}", &response.data.nonceHash);

    // Step 3: Verify login with signature to obtain JWT token
    let mut url = Url::parse(&format!("{base_url}/v1/user/login/verify"))?;

    {
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("addr", &address.to_string());
        pairs.append_pair("nonceHash", &response.data.nonceHash);
        pairs.append_pair("signature", &signature.to_string());
    }
    let response = client
        .post(url)
        .header("User-Agent", "Rust Binary")
        .send()
        .await?;

    let verify_data = response.text().await?;
    dbg!(verify_data);
    return Ok(());
}
