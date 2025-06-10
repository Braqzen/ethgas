use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddressResponse {
    pub success: bool,
    pub data: Data,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Data {
    #[serde(rename = "contractAddress")]
    pub address: Address,
}
