use crate::common::market::PriceLevel;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub final_update_id: u64,

    #[serde(rename = "b")]
    pub bids: Vec<PriceLevel>,

    #[serde(rename = "a")]
    pub asks: Vec<PriceLevel>,
}
