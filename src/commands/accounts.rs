use crate::types::account::{Account, AccountsResponse, Token};
use anyhow::Result;
use clap::Args;
use reqwest::Client;
use url::Url;

#[derive(Args)]
pub struct AccountsCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    pub base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    pub jwt: String,
}

impl AccountsCommand {
    pub async fn run(&self) -> Result<()> {
        let (account_1, account_2, token) = self.execute().await?;
        println!("\nAccount 1: {:?}", account_1);
        println!("\nAccount 2: {:?}", account_2);
        println!("\nToken: {:?}", token);
        Ok(())
    }

    pub async fn execute(&self) -> Result<(Account, Account, Token)> {
        let url = Url::parse(&format!("{}/v1/user/accounts", self.base_url))?;
        let response: AccountsResponse = Client::new()
            .get(url)
            .header("Authorization", format!("Bearer {}", self.jwt))
            .send()
            .await?
            .json()
            .await?;

        if response.data.accounts.len() != 2 {
            return Err(anyhow::anyhow!(
                "Expected exactly 2 accounts, but found {}",
                response.data.accounts.len()
            ));
        }

        let account_1 = &response.data.accounts[0];
        let account_2 = &response.data.accounts[1];

        let token = account_1
            .tokens
            .as_ref()
            .and_then(|tokens| tokens.iter().find(|token| token.code == "ETH"))
            .ok_or_else(|| anyhow::anyhow!("No ETH token found in account 1."))?;

        Ok((account_1.to_owned(), account_2.to_owned(), token.to_owned()))
    }
}
