#![allow(non_snake_case)]

mod cli;
mod commands;
mod sol;
mod types;

use crate::cli::Commands;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let command = Commands::parse();

    match command {
        Commands::Jwt(cmd) => cmd.run().await?,
        Commands::Deposits(cmd) => cmd.run().await?,
        Commands::Deposit(cmd) => cmd.run().await?,
        Commands::Accounts(cmd) => cmd.run().await?,
        Commands::Transfer(cmd) => cmd.run().await?,
        Commands::Markets(cmd) => cmd.run().await?,
        Commands::Buy(cmd) => cmd.run().await?,
        Commands::Slot(cmd) => cmd.run().await?,
        Commands::SubmitBundle(cmd) => cmd.run().await?,
    }

    Ok(())
}
