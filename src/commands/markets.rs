use crate::types::preconf::{Market, MarketsResponse};
use anyhow::Result;
use clap::Args;
use reqwest::Client;
use url::Url;

#[derive(Args)]
pub struct MarketsCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    pub base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    pub jwt: String,
}

impl MarketsCommand {
    pub async fn run(&self) -> Result<()> {
        let markets = self.execute().await?;
        println!("\nMarkets\n{:?}", markets);
        Ok(())
    }

    pub async fn execute(&self) -> Result<Vec<Market>> {
        let url = Url::parse(&format!("{}/v1/p/inclusion-preconf/markets", self.base_url))?;
        let response: MarketsResponse = Client::new()
            .get(url)
            .header("Authorization", format!("Bearer {}", self.jwt))
            .send()
            .await?
            .json()
            .await?;

        if !response.success {
            return Err(anyhow::anyhow!("Failed to get preconf markets"));
        }

        Ok(response.data.markets)
    }
}
