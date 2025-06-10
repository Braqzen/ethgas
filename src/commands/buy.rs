use crate::{
    commands::{accounts::AccountsCommand, markets::MarketsCommand, slot::SlotCommand},
    types::preconf::OrderResponse,
};
use anyhow::Result;
use clap::Args;
use reqwest::Client;
use url::Url;

#[derive(Args)]
pub struct BuyCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    jwt: String,
}

impl BuyCommand {
    pub async fn run(&self) -> Result<()> {
        let current_slot = SlotCommand {
            base_url: self.base_url.clone(),
            jwt: self.jwt.clone(),
        }
        .execute()
        .await?;

        let markets = MarketsCommand {
            base_url: self.base_url.clone(),
            jwt: self.jwt.clone(),
        }
        .execute()
        .await?;

        let market = markets
            .iter()
            .find(|m| m.slot == current_slot)
            .ok_or_else(|| anyhow::anyhow!("Market not found"))?;

        let (_account_1, account_2, _token) = AccountsCommand {
            base_url: self.base_url.clone(),
            jwt: self.jwt.clone(),
        }
        .execute()
        .await?;

        let mut url = Url::parse(&format!("{}/v1/inclusion-preconf/order", self.base_url))?;
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("instrumentId", &market.instrument_id);
            pairs.append_pair("accountId", &account_2.account_id.to_string());
            pairs.append_pair("side", "1");
            pairs.append_pair("orderType", "2"); // 2 for limit order
            pairs.append_pair("quantity", &market.min_quantity);
            pairs.append_pair("passive", "false");
            pairs.append_pair("price", &market.best_bid);
        }

        let response: OrderResponse = Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", self.jwt))
            .send()
            .await?
            .json()
            .await?;

        if !response.success {
            return Err(anyhow::anyhow!(
                "Failed to place preconf order: {:?}",
                response
            ));
        }

        println!("Done");
        Ok(())
    }
}
