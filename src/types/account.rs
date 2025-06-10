use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountsResponse {
    pub success: bool,
    pub data: AccountsData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountsData {
    pub accounts: Vec<Account>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: u64,
    pub user_id: u64,
    #[serde(rename = "type")]
    pub account_type: u8,
    pub name: String,
    pub status: u8,
    pub update_date: u64,
    pub tokens: Option<Vec<Token>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub account_id: u64,
    pub token_id: u64,
    pub quantity: String,
    pub locked_quantity: String,
    pub code: String,
    pub available_quantity: String,
}
