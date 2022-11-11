pub use crate::common::market::PriceLevel;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    pub symbol: String,
    pub pair: String,
    pub asks: Vec<PriceLevel>,
    pub bids: Vec<PriceLevel>,
}
