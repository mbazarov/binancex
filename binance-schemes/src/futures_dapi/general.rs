pub use crate::common::general::*;
use crate::futures_common::trade::TimeInForce;

use binance_types::futures_dapi::general::*;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: Decimal,
        max_price: Decimal,
        tick_size: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    MaxNumOrders { limit: u16 },

    // Missing from the documentation:
    // https://binance-docs.github.io/apidocs/delivery/en/#filters
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { limit: u16 },

    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        multiplier_decimal: Decimal,
    },

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub pair: String,
    pub contract_type: ContractType,
    pub delivery_date: u64,
    pub onboard_date: u64,
    pub contract_status: ContractStatus,

    // TODO: max_value?
    pub contract_size: u64,
    pub margin_asset: String,
    pub maint_margin_percent: Decimal,
    pub required_margin_percent: Decimal,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u16,
    pub quantity_precision: u16,
    pub base_asset_precision: u8,
    pub quote_precision: u8,
    pub equal_qty_precision: u16,
    pub max_move_order_limit: u64,
    pub trigger_protect: Decimal,

    // TODO: add type?
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub filters: Vec<SymbolFilter>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
    pub liquidation_fee: Decimal,
    pub market_take_bound: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<Symbol>,
}
