pub use crate::common::market::PriceLevel;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub asks: Vec<PriceLevel>,
    pub bids: Vec<PriceLevel>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub time: u64, // Trade executed timestamp, as same as `T` in the stream
    pub is_buyer_maker: bool,
    pub is_best_match: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AggregateTrade {
    #[serde(rename = "a")]
    pub aggregate_trade_id: u64,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "l")]
    pub last_trade_id: u64,

    #[serde(rename = "T")]
    pub timestamp: u64,

    #[serde(rename = "m")]
    pub is_maker: bool,

    #[serde(rename = "M")]
    pub is_best_price_match: bool,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "q")]
    pub qty: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineSummary {
    pub open_time: u64,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub close_price: Decimal,
    pub volume: Decimal,
    pub close_time: u64,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: u64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
    pub unused: Option<Decimal>, // Unused field, ignore.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AveragePrice {
    pub mins: u64,
    pub price: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerStatsFull {
    pub symbol: String,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub weighted_avg_price: Decimal,
    pub prev_close_price: Decimal,
    pub last_price: Decimal,
    pub last_qty: Decimal,
    pub bid_price: Decimal,
    pub bid_qty: Decimal,
    pub ask_price: Decimal,
    pub ask_qty: Decimal,
    pub open_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub volume: Decimal,
    pub quote_volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64, // First tradeId
    pub last_id: i64,  // Last tradeId
    pub count: u64,    // Trade count
}

#[rustfmt::skip]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerStatsMini {
    pub symbol: String,        // Symbol Name
    pub open_price: Decimal,   // Opening price of the Interval
    pub high_price: Decimal,   // Highest price in the interval
    pub low_price: Decimal,    // Lowest  price in the interval
    pub last_price: Decimal,   // Closing price of the interval
    pub volume: Decimal,       // Total trade volume (in base asset)
    pub quote_volume: Decimal, // Total trade volume (in quote asset)
    pub open_time: u64,        // Start of the ticker interval
    pub close_time: u64,       // End of the ticker interval
    pub first_id: i64,         // First tradeId considered
    pub last_id: i64,          // Last tradeId considered
    pub count: u64,            // Total trade count
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SymbolPrice {
    pub symbol: String,
    pub price: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTicker {
    pub symbol: String,
    pub bid_price: Decimal,
    pub bid_qty: Decimal,
    pub ask_price: Decimal,
    pub ask_qty: Decimal,
}
