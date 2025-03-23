use alloy::dyn_abi::TypedData;
use alloy::primitives::{B256};
use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize, de};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EthGasResponse {
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

fn deserialize_eip712_message<'de, D>(deserializer: D) -> Result<EIP712Message, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(de::Error::custom)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EIP712Message {
    pub types: EIP712Types,
    pub primary_type: String,
    pub message: EIP712MessageData,
    pub domain: EIP712Domain,
}

impl EIP712Message {
    pub fn digest(&self) -> Result<B256> {
        let json = serde_json::to_string(self)?;
        // TODO: this errors
        let typed_data: ssi_eip712::TypedData = serde_json::from_str(&json)?;
        let digest = typed_data.hash()?;

        Ok(alloy::primitives::FixedBytes(digest))
    }
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
