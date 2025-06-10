use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketsResponse {
    pub success: bool,
    pub data: MarketsData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketsData {
    pub markets: Vec<Market>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub market_id: u64,
    pub slot: u64,
    pub instrument_id: String,
    pub name: String,
    pub quantity_step: String,
    pub min_quantity: String,
    pub max_quantity: String,
    pub price_step: String,
    pub min_price: String,
    pub max_price: String,
    pub collateral_per_slot: String,
    pub best_bid: String,
    pub available_preconf: u64,
    pub price: String,
    pub mid_price: String,
    pub direction: bool,
    pub status: u8,
    pub maturity_time: u64,
    pub trx_submit_time: u64,
    pub block_time: u64,
    pub finality_time: u64,
    pub total_gas: u64,
    pub update_date: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMarketUpdate {
    pub market_id: u64,
    pub slot: u64,
    pub instrument_id: String,
    pub name: String,
    pub quantity_step: String,
    pub min_quantity: String,
    pub max_quantity: String,
    pub price_step: String,
    pub min_price: String,
    pub max_price: String,
    pub collateral_per_slot: String,
    pub best_bid: String,
    pub available_preconf: u64,
    pub price: String,
    pub mid_price: String,
    pub direction: bool,
    pub status: u8,
    pub maturity_time: u64,
    pub trx_submit_time: u64,
    pub block_time: u64,
    pub finality_time: u64,
    pub total_gas: u64,
    pub update_date: u64,
}

impl From<WsMarketUpdate> for Market {
    fn from(ws: WsMarketUpdate) -> Self {
        Market {
            market_id: ws.market_id,
            slot: ws.slot,
            instrument_id: ws.instrument_id,
            name: ws.name,
            quantity_step: ws.quantity_step,
            min_quantity: ws.min_quantity,
            max_quantity: ws.max_quantity,
            price_step: ws.price_step,
            min_price: ws.min_price,
            max_price: ws.max_price,
            collateral_per_slot: ws.collateral_per_slot,
            best_bid: ws.best_bid,
            available_preconf: ws.available_preconf,
            price: ws.price,
            mid_price: ws.mid_price,
            direction: ws.direction,
            status: ws.status,
            maturity_time: ws.maturity_time,
            trx_submit_time: ws.trx_submit_time,
            block_time: ws.block_time,
            finality_time: ws.finality_time,
            total_gas: ws.total_gas,
            update_date: ws.update_date,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsMessage {
    pub r#type: String,
    pub data: WsMarketUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub success: bool,
    pub data: OrderData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    pub order_id: String,
    pub client_order_id: Option<String>,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BundleResponse {
    pub success: bool,
    pub data: BundleData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BundleData {
    pub bundle_id: String,
    pub status: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PositionsResponse {
    pub success: bool,
    pub data: PositionsData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PositionsData {
    pub positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub instrument_id: String,
    pub quantity: String,
    pub side: bool,
    pub status: u8,
}
