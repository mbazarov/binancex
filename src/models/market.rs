use rust_decimal::Decimal;
use serde::Deserialize;


#[derive(Copy, Clone, Debug)]
pub enum DepthLimit {
    Limit5 = 5,
    Limit10 = 10,
    Limit20 = 20,
    Limit50 = 50,
    Limit100 = 100,
    Limit500 = 500,
    Limit1000 = 1000,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ask {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub asks: Vec<Ask>,
    pub bids: Vec<Bid>,
}
