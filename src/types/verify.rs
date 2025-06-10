use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct VerifyResponse {
    pub success: bool,
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Data {
    pub user: User,
    #[serde(rename = "accessToken")]
    pub access_token: AccessToken,
}

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub address: String,
    pub status: u8,
    #[serde(rename = "userType")]
    pub user_type: u8,
    #[serde(rename = "userClass")]
    pub user_class: u8,
    #[serde(rename = "delegatedBuilderId")]
    pub delegated_builder_id: u64,
    pub accounts: Vec<Account>,
}

#[derive(Debug, Deserialize)]
pub struct Account {
    #[serde(rename = "accountId")]
    pub account_id: u64,
    #[serde(rename = "userId")]
    pub user_id: u64,
    #[serde(rename = "type")]
    pub account_type: u8,
    pub name: String,
    pub status: u8,
}

#[derive(Debug, Deserialize)]
pub struct AccessToken {
    pub data: AccessTokenData,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenData {
    pub header: Header,
    pub payload: Payload,
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub alg: String,
    pub typ: String,
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub user: TokenUser,
    pub access_type: String,
    pub iat: u64,
    pub exp: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenUser {
    #[serde(rename = "userId")]
    pub user_id: u64,
    pub address: String,
    pub roles: Vec<String>,
}
