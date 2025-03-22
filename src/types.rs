use alloy::primitives::{B256, keccak256};
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
    #[serde(deserialize_with = "deserialize_eip712_message")]
    pub eip712Message: EIP712Message,
    pub nonceHash: String,
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
    pub primaryType: String,
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
