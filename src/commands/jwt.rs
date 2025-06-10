use crate::types::{login::LoginResponse, verify::VerifyResponse};
use alloy::{
    hex,
    primitives::Address,
    signers::{Signer, local::PrivateKeySigner},
};
use anyhow::{Context, Result};
use clap::Args;
use reqwest::Client;
use url::Url;

#[derive(Args)]
pub struct JwtCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    base_url: Url,

    #[arg(long, env = "CHAIN_ID", hide_env = true)]
    chain_id: u32,

    #[arg(long, env = "PRIVATE_KEY", hide_env = true)]
    private_key: PrivateKeySigner,
}

impl JwtCommand {
    pub async fn run(&self) -> Result<()> {
        let jwt = self.execute().await?;
        println!("Token:\n{jwt}");
        Ok(())
    }

    pub async fn execute(&self) -> Result<String> {
        let address = self.private_key.address();

        let login = self
            .login(&self.base_url, address)
            .await
            .context("Failed to login")?;

        let signature = self
            .sign_message(&login, &self.private_key)
            .await
            .context("Failed to sign message")?;

        let jwt = self
            .verify(&self.base_url, address, signature, &login.data.nonce_hash)
            .await
            .context("Failed to verify")?;

        Ok(jwt)
    }

    async fn login(&self, base_url: &Url, address: Address) -> Result<LoginResponse> {
        let mut url = Url::parse(&format!("{base_url}/v1/user/login"))?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("addr", &address.to_string());
            pairs.append_pair("chainId", &self.chain_id.to_string());
        }
        let response: LoginResponse = Client::new().post(url).send().await?.json().await?;
        Ok(response)
    }

    async fn sign_message(
        &self,
        login: &LoginResponse,
        private_key: &PrivateKeySigner,
    ) -> Result<String> {
        let digest = login.data.eip712_message.eip712_signing_hash()?;
        let signature = private_key.sign_hash(&digest).await?;
        let signature = format!("0x{}", hex::encode(signature.as_bytes()));
        Ok(signature)
    }

    async fn verify(
        &self,
        base_url: &Url,
        address: Address,
        signature: String,
        nonce_hash: &str,
    ) -> Result<String> {
        let mut url = Url::parse(&format!("{base_url}/v1/user/login/verify"))?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("addr", &address.to_string());
            pairs.append_pair("nonceHash", &nonce_hash);
            pairs.append_pair("signature", &signature);
        }

        let response: VerifyResponse = Client::new()
            .post(url)
            .header("User-Agent", "Rust Binary")
            .send()
            .await?
            .json()
            .await?;

        Ok(response.data.access_token.token)
    }
}
