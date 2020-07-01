use super::{
    market::{OrderBook, Ask, Bid}
};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    UserData(String),            // listen key
    AggregateTrade(String),      // symbol
    Trade(String),               // symbol
    Candlestick(String, String), // (symbol, interval)
    MiniTicker(String),          // symbol
    MiniTickerAll,
    Ticker(String),              // symbol
    TickerAll,
    OrderBook(String, i64),      // (symbol, depth)
    Depth(String, Option<u16>),  // (symbol, interval)
}

#[derive(Debug, Clone, Serialize)]
pub enum BinanceWebsocketMessage {
    UserOrderUpdate(UserOrderUpdate),
    UserAccountUpdate(AccountUpdate),
    AggregateTrade(AggregateTrade),
    Trade(TradeMessage),
    Candlestick(CandelStickMessage),
    MiniTicker(MiniTicker),
    MiniTickerAll(Vec<MiniTicker>),
    Ticker(Ticker),
    TickerAll(Vec<Ticker>),
    OrderBook(OrderBook),
    Depth(Depth),
    Ping,
    Pong,
    Close,
    Binary(Vec<u8>), // Unexpected, unparsed
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderExecType {
    New,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRejectReason {
    None,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,

    #[serde(rename = "T")]
    pub end_time: i64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: i32,

    #[serde(rename = "L")]
    pub last_trade_id: i32,

    #[serde(rename = "o")]
    pub open: String,

    #[serde(rename = "c")]
    pub close: String,

    #[serde(rename = "h")]
    pub high: String,

    #[serde(rename = "l")]
    pub low: String,

    #[serde(rename = "v")]
    pub volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: i32,

    #[serde(rename = "x")]
    pub is_final_bar: bool,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "V")]
    pub active_buy_volume: String,

    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,

    #[serde(skip_serializing, rename = "B")]
    pub ignore_me: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeMessage {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "t")]
    pub trade_id: u64,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "q")]
    pub qty: Decimal,

    #[serde(rename = "b")]
    pub buyer_order_id: u64,

    #[serde(rename = "a")]
    pub seller_order_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregateTrade {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "a")]
    pub aggregated_trade_id: u64,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "q")]
    pub qty: Decimal,

    #[serde(rename = "f")]
    pub first_break_trade_id: u64,

    #[serde(rename = "l")]
    pub last_break_trade_id: u64,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOrderUpdate {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub new_client_order_id: String,

    #[serde(rename = "S")]
    pub side: Side,

    #[serde(rename = "o")]
    pub order_type: OrderType,

    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,

    #[serde(rename = "q")]
    pub qty: Decimal,

    #[serde(rename = "p")]
    pub price: Decimal,

    #[serde(rename = "P")]
    pub stop_price: Decimal,

    #[serde(rename = "F")]
    pub iceberg_qty: Decimal,

    #[serde(skip_serializing)]
    pub g: i32,

    #[serde(skip_serializing, rename = "C")]
    pub c_ignore: Option<String>,

    #[serde(rename = "x")]
    pub execution_type: OrderExecType,

    #[serde(rename = "X")]
    pub order_status: OrderStatus,

    #[serde(rename = "r")]
    pub order_reject_reason: OrderRejectReason,

    #[serde(rename = "i")]
    pub order_id: u64,

    #[serde(rename = "l")]
    pub qty_last_filled_trade: Decimal,

    #[serde(rename = "z")]
    pub accumulated_qty_filled_trades: Decimal,

    #[serde(rename = "L")]
    pub price_last_filled_trade: Decimal,

    #[serde(rename = "n")]
    pub commission: Decimal,

    #[serde(skip_serializing, rename = "N")]
    pub asset_commisioned: Option<String>,

    #[serde(rename = "T")]
    pub trade_order_time: u64,

    #[serde(rename = "t")]
    pub trade_id: i64,

    #[serde(skip_serializing, rename = "I")]
    pub i_ignore: u64,

    #[serde(skip_serializing)]
    pub w: bool,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,

    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,

    #[serde(skip_serializing, rename = "O")]
    pub order_creation_time: u64,

    #[serde(skip_serializing, rename = "Z")]
    pub cumulative_quote_asset_transacted_qty: Decimal,
}

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
    pub bids: Vec<Bid>,

    #[serde(rename = "a")]
    pub asks: Vec<Ask>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "p")]
    pub price_change: Decimal,

    #[serde(rename = "P")]
    pub price_change_percent: Decimal,

    #[serde(rename = "w")]
    pub average_price: Decimal,

    #[serde(rename = "x")]
    pub prev_close: Decimal,

    #[serde(rename = "c")]
    pub current_close: Decimal,

    #[serde(rename = "Q")]
    pub current_close_qty: Decimal,

    #[serde(rename = "b")]
    pub best_bid: Decimal,

    #[serde(rename = "B")]
    pub best_bid_qty: Decimal,

    #[serde(rename = "a")]
    pub best_ask: Decimal,

    #[serde(rename = "A")]
    pub best_ask_qty: Decimal,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "v")]
    pub volume: Decimal,

    #[serde(rename = "q")]
    pub quote_volume: Decimal,

    #[serde(rename = "O")]
    pub open_time: u64,

    #[serde(rename = "C")]
    pub close_time: u64,

    #[serde(rename = "F")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "n")]
    pub num_trades: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CandelStickMessage {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "m")]
    pub maker_commision_rate: u64,

    #[serde(rename = "t")]
    pub taker_commision_rate: u64,

    #[serde(rename = "b")]
    pub buyer_commision_rate: u64,

    #[serde(rename = "s")]
    pub seller_commision_rate: u64,

    #[serde(rename = "T")]
    pub can_trade: bool,

    #[serde(rename = "W")]
    pub can_withdraw: bool,

    #[serde(rename = "D")]
    pub can_deposit: bool,

    #[serde(rename = "u")]
    pub last_account_update: u64,

    #[serde(rename = "B")]
    pub balance: Vec<AccountUpdateBalance>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateBalance {
    #[serde(rename = "a")]
    pub asset: String,

    #[serde(rename = "f")]
    pub free: Decimal,

    #[serde(rename = "l")]
    pub locked: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "c")]
    pub close: Decimal,

    #[serde(rename = "o")]
    pub open: Decimal,

    #[serde(rename = "l")]
    pub low: Decimal,

    #[serde(rename = "h")]
    pub high: Decimal,

    #[serde(rename = "v")]
    pub volume: Decimal,

    #[serde(rename = "q")]
    pub quote_volume: Decimal,
}
