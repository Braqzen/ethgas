use anyhow::Result;
use clap::Args;
use reqwest::Client;
use url::Url;

#[derive(Args)]
pub struct TransferCommand {
    #[arg(long, env = "TRANSFER_AMOUNT", hide_env = true)]
    amount: u64,

    #[arg(long, env = "BASE_URL", hide_env = true)]
    base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    jwt: String,

    #[arg(long, env = "FROM_ACCOUNT_ID", hide_env = true)]
    from_account_id: String,

    #[arg(long, env = "TO_ACCOUNT_ID", hide_env = true)]
    to_account_id: String,

    #[arg(long, env = "TOKEN_ID", hide_env = true)]
    token_id: String,
}

impl TransferCommand {
    pub async fn run(&self) -> Result<()> {
        let mut url = Url::parse(&format!("{}/v1/user/account/transfer/token", self.base_url))?;
        {
            let eth = self.amount as f64 / 1_000_000_000_000_000_000.0; // Convert to ETH (10^18 wei)
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("fromAccountId", &self.from_account_id);
            pairs.append_pair("toAccountId", &self.to_account_id);
            pairs.append_pair("tokenId", &self.token_id);
            pairs.append_pair("quantity", &eth.to_string());
        }

        let response = Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", self.jwt))
            .send()
            .await?
            .text()
            .await?;

        println!("Response: {}", response);
        Ok(())
    }
}
