use crate::futures_fapi::market::PriceLevel;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "T")]
    pub transaction_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub final_update_id: u64,

    #[serde(rename = "pu")]
    pub last_stream_final_update_id: u64,

    #[serde(rename = "a")]
    pub asks: Vec<PriceLevel>,

    #[serde(rename = "b")]
    pub bids: Vec<PriceLevel>,
}
