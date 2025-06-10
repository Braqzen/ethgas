use crate::commands::{
    accounts::AccountsCommand, buy::BuyCommand, deposit::DepositCommand, deposits::DepositsCommand,
    jwt::JwtCommand, markets::MarketsCommand, slot::SlotCommand,
    submit_bundle::SubmitBundleCommand, transfer::TransferCommand,
};
use clap::Parser;

#[derive(Parser)]
pub enum Commands {
    /// Check account info
    Accounts(AccountsCommand),

    /// Buy gas for a market
    Buy(BuyCommand),

    /// Make a deposit
    Deposit(DepositCommand),

    /// Check deposits
    Deposits(DepositsCommand),

    /// Get JWT token
    Jwt(JwtCommand),

    /// Check markets
    Markets(MarketsCommand),

    /// Check slot
    Slot(SlotCommand),

    /// Submit transaction bundle
    SubmitBundle(SubmitBundleCommand),

    /// Transfer between accounts
    Transfer(TransferCommand),
}
