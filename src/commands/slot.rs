use anyhow::Result;
use clap::Args;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

#[derive(Args)]
pub struct SlotCommand {
    #[arg(long, env = "BASE_URL", hide_env = true)]
    pub base_url: Url,

    #[arg(long, env = "JWT", hide_env = true)]
    pub jwt: String,
}

impl SlotCommand {
    pub async fn run(&self) -> Result<()> {
        let slot = self.execute().await?;
        println!("Current slot: {slot}");
        Ok(())
    }

    pub async fn execute(&self) -> Result<u64> {
        let url = format!(
            "wss://{}/ws",
            self.base_url.host_str().unwrap().replace("/api", "")
        );
        println!("\nConnecting to WebSocket: {url}");
        let (mut ws_stream, _) = connect_async(url).await?;

        let query = serde_json::json!({
            "op": "query",
            "args": [
                {
                    "queryType": "currentSlot"
                }
            ]
        });

        println!("Querying slot...");
        ws_stream.send(Message::Text(query.to_string())).await?;

        println!("Waiting for response...");
        while let Some(msg) = ws_stream.next().await {
            match msg? {
                Message::Text(text) => {
                    #[derive(serde::Deserialize)]
                    struct SlotResponse {
                        q: String,
                        P: SlotPayload,
                    }
                    #[derive(serde::Deserialize)]
                    struct SlotPayload {
                        r: u64,
                        s: u64,
                        t: u64,
                    }

                    if let Ok(response) = serde_json::from_str::<SlotResponse>(&text) {
                        if response.q == "currentSlot" {
                            println!(
                                "Received slot update: round={}, slot={}, timestamp={}",
                                response.P.r, response.P.s, response.P.t
                            );
                            return Ok(response.P.s);
                        }
                    } else {
                        println!("Failed to parse message");
                    }
                }
                Message::Close(_) => {
                    println!("WebSocket connection closed");
                    break;
                }
                Message::Binary(_) => {
                    println!("Received binary message (ignoring)");
                }
                Message::Ping(_) => {
                    println!("Received ping");
                }
                Message::Pong(_) => {
                    println!("Received pong");
                }
                Message::Frame(_) => {
                    println!("Received raw frame (ignoring)");
                }
            }
        }

        Err(anyhow::anyhow!("No slot response received"))
    }
}
