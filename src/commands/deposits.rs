use anyhow::Result;
use clap::Args;
use reqwest::Client;
use url::Url;

#[derive(Args)]
pub struct DepositsCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    jwt: String,
}

impl DepositsCommand {
    pub async fn run(&self) -> Result<()> {
        let url = Url::parse(&format!("{}/v1/user/funding/deposits", self.base_url))?;

        let response = Client::new()
            .get(url)
            .header("Authorization", format!("Bearer {}", self.jwt))
            .send()
            .await?
            .text()
            .await?;

        let parsed_response: serde_json::Value =
            serde_json::from_str(&response).expect("Failed to parse raw JSON string");

        let pretty_json_string = serde_json::to_string_pretty(&parsed_response)
            .expect("Failed to pretty print parsed JSON Value");

        println!("\nDeposits:\n{pretty_json_string}");
        Ok(())
    }
}
