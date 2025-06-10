use alloy::dyn_abi::TypedData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginResponse {
    pub success: bool,
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    pub status: String,
    #[serde(rename = "eip712Message")]
    pub eip712_message: TypedData,
    #[serde(rename = "nonceHash")]
    pub nonce_hash: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EIP712Message {
    pub types: EIP712Types,
    pub primary_type: String,
    pub message: EIP712MessageData,
    pub domain: EIP712Domain,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EIP712Types {
    pub EIP712Domain: Vec<EIP712Field>,
    pub data: Vec<EIP712Field>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EIP712Field {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EIP712MessageData {
    pub hash: String,
    pub message: String,
    pub domain: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EIP712Domain {
    pub name: String,
    pub version: String,
    pub chainId: u64,
    pub verifyingContract: String,
}
