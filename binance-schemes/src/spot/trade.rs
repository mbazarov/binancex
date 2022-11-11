use crate::common::general::{RateLimitInterval, RateLimitType};
use binance_types::spot::general::{OrderStatus, OrderType};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate Or Cancel
    IOC,
    /// Fill or Kill
    FOK,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TestNewOrderRes {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOrderReq {
    pub symbol: String,
    pub side: OrderSide,

    #[serde(flatten)]
    #[serde(rename = "type")]
    pub order_type: OrderTypeReq,

    // A unique id among open orders. Automatically generated if not sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    // The value cannot be less than 1000000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

    // Set the response JSON. ACK, RESULT, or FULL;
    // MARKET and LIMIT order types default to FULL,
    // all other orders default to ACK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderTypeReq {
    #[serde(rename_all = "camelCase")]
    Limit {
        time_in_force: TimeInForce,
        quantity: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        price: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        iceberg_qty: Option<Decimal>,
    },

    #[serde(rename_all = "camelCase")]
    Market {
        #[serde(skip_serializing_if = "Option::is_none")]
        time_in_force: Option<TimeInForce>,

        #[serde(skip_serializing_if = "Option::is_none")]
        quantity: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,
    },

    #[serde(rename_all = "camelCase")]
    StopLoss {
        #[serde(skip_serializing_if = "Option::is_none")]
        time_in_force: Option<TimeInForce>,

        quantity: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<u64>,
    },

    #[serde(rename_all = "camelCase")]
    StopLossLimit {
        time_in_force: TimeInForce,
        quantity: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        price: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<u64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        iceberg_qty: Option<Decimal>,
    },

    #[serde(rename_all = "camelCase")]
    TakeProfit {
        #[serde(skip_serializing_if = "Option::is_none")]
        time_in_force: Option<TimeInForce>,

        quantity: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        price: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<u64>,
    },

    #[serde(rename_all = "camelCase")]
    TakeProfitLimit {
        time_in_force: TimeInForce,
        quantity: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        price: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        stop_price: Option<Decimal>,

        #[serde(skip_serializing_if = "Option::is_none")]
        trailing_delta: Option<u64>,

        #[serde(skip_serializing_if = "Option::is_none")]
        iceberg_qty: Option<Decimal>,
    },

    #[serde(rename_all = "camelCase")]
    LimitMarket {
        #[serde(skip_serializing_if = "Option::is_none")]
        time_in_force: Option<TimeInForce>,

        quantity: Decimal,

        #[serde(skip_serializing_if = "Option::is_none")]
        quote_order_qty: Option<Decimal>,

        price: Decimal,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CancelReplaceMode {
    // If the cancel request fails, the new order placement will not be attempted.
    StopOnFailure,
    // new order placement will be attempted even if cancel request fails.
    AllowFailure,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderResponseType {
    Ack,
    Result,
    Full,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderReq {
    pub symbol: String,
    pub side: OrderSide,
    pub cancel_replace_mode: CancelReplaceMode,

    #[serde(flatten)]
    #[serde(rename = "type")]
    pub order_type: OrderTypeReq,

    // Used to uniquely identify this cancel. Automatically generated by default.
    pub cancel_new_client_order_id: Option<String>,

    // Either the cancelOrigClientOrderId or cancelOrderId must be provided.
    // If both are provided, cancelOrderId takes precedence.
    pub cancel_orig_client_order_id: Option<String>,
    pub cancel_order_id: Option<u64>,

    // A unique id among open orders. Automatically generated if not sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_client_order_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<u32>,

    // The value cannot be less than 1000000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<u32>,

    // Set the response JSON. ACK, RESULT, or FULL;
    // MARKET and LIMIT order types default to FULL,
    // all other orders default to ACK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_order_resp_type: Option<OrderResponseType>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelReplaceOrderRes {
    cancel_result: String,
    new_order_result: String,
    cancel_response: CancelOrderRes,
    new_order_response: NewOrderRes,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NewOrderRes {
    Full(TransactionFull),
    Result(TransactionResult),
    Ack(TransactionAck),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub maker_commission: u64,
    pub taker_commission: u64,
    pub buyer_commission: u64,
    pub seller_commission: u64,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub update_time: u64,
    pub account_type: String,
    pub balances: Vec<Asset>,
    pub permissions: Vec<String>,
}

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize, Serialize,
)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    BUY,
    SELL,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionAck {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64, // Unless OCO, value will be -1
    pub client_order_id: String,
    pub transact_time: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionResult {
    pub symbol: String,
    pub order_id: i64,
    pub order_list_id: i64, // Unless OCO, value will be -1
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,
    // This is only visible if the field was populated on order placement.
    pub strategy_id: Option<u32>,
    // This is only visible if the field was populated on order placement.
    pub strategy_type: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFull {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64, // Unless OCO, value will be -1
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,
    // This is only visible if the field was populated on order placement.
    pub strategy_id: Option<u32>,
    // This is only visible if the field was populated on order placement.
    pub strategy_type: Option<u32>,
    pub fills: Vec<TransactionFill>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionFill {
    pub price: Decimal,
    pub qty: Decimal,
    pub commission: Decimal,
    pub commission_asset: String,
    pub trade_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderReq {
    pub symbol: String,
    pub order_id: Option<u64>,
    pub orig_client_order_id: Option<String>,

    // Used to uniquely identify this cancel. Automatically generated by default.
    pub new_client_order_id: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRes {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u64,
    pub order_list_id: i64, // Unless part of an OCO, the value will always be -1.
    pub client_order_id: String,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: i64, // Unless OCO, value will be -1
    pub client_order_id: String,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,
    pub stop_price: Decimal,
    pub iceberg_qty: Decimal,
    pub time: u64,
    pub update_time: u64,
    pub is_working: bool,
    pub orig_quote_order_qty: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewOcoOrderReq {
    pub symbol: String,
    pub list_client_order_id: Option<String>, // A unique Id for the entire orderList
    pub side: OrderSide,
    pub quantity: Decimal,
    pub limit_client_order_id: Option<String>, // A unique Id for the limit order
    pub limit_strategy_id: Option<u32>,
    pub limit_strategy_type: Option<u32>, // The value cannot be less than 1000000.
    pub price: Decimal,
    pub limit_iceberg_qty: Option<Decimal>,
    pub trailing_delta: Option<u64>,
    pub stop_client_order_id: Option<String>, // A unique Id for the stop loss/stop loss limit leg
    pub stop_price: Decimal,
    pub stop_strategy_id: Option<u32>,
    pub stop_strategy_type: Option<u32>, // The value cannot be less than 1000000.
    pub stop_limit_price: Option<Decimal>, // If provided, stopLimitTimeInForce is required.
    pub stop_iceberg_qty: Option<Decimal>,
    pub stop_limit_time_in_force: TimeInForce, // Valid values are GTC/FOK/IOC
    pub new_order_resp_type: OrderResponseType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContingencyType {
    Oco,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OcoOrderStatusType {
    // This is used when the ListStatus is responding to a failed action.
    // (E.g. Orderlist placement or cancellation)
    Response,
    // The order list has been placed or there is an update to the order list status.
    ExecStarted,
    // The order list has finished executing and thus no longer active.
    AllDone,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OcoOrderStatus {
    // Either an order list has been placed or there is an update to the status of the list.
    Executing,
    // An order list has completed execution and thus no longer active.
    AllDone,
    // The List Status is responding to a failed action either during order placement or order canceled.)
    Reject,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrderRes {
    pub order_list_id: u64,
    pub contingency_type: ContingencyType,
    pub list_status_type: OcoOrderStatusType,
    pub list_order_status: OcoOrderStatus,
    pub list_client_order_id: String,
    pub transaction_time: u64,
    pub symbol: String,
    pub orders: Vec<OcoOrder>,
    pub order_reports: Vec<OrderReport>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OcoOrder {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderReport {
    pub symbol: String,
    pub order_id: u64,
    pub order_list_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
    pub price: Decimal,
    pub orig_qty: Decimal,
    pub executed_qty: Decimal,
    pub cummulative_quote_qty: Decimal,
    pub status: String,
    pub time_in_force: TimeInForce,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    pub side: OrderSide,
    pub stop_price: Option<Decimal>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderIdOrClientOrderId {
    OrderListId(u64),
    OrigClientOrderId(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradesReq {
    pub symbol: String,
    pub order_id: Option<u64>, // This can only be used in combination with symbol.
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub from_id: Option<u64>, // TradeId to fetch from. Default gets most recent trades.
    pub limit: u16,           // Default 500; max 1000.
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountTrade {
    pub symbol: String,
    pub id: u64,
    pub order_id: u64,
    pub order_list_id: i64, // Unless OCO, the value will always be -1
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub commission: Decimal,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
    pub count: u32,
}
