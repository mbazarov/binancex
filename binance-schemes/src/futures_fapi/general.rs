pub use crate::common::general::*;
use crate::futures_common::trade::TimeInForce;

use binance_types::futures_fapi::general::*;

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

    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { limit: u16 },

    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        multiplier_decimal: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    MinNotional { notional: Decimal },

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
    pub status: SymbolStatus,
    pub maint_margin_percent: Decimal,
    pub required_margin_percent: Decimal,
    pub base_asset: String,
    pub quote_asset: String,
    pub price_precision: u8,
    pub quantity_precision: u16,
    pub base_asset_precision: u8,
    pub quote_precision: u8,
    pub underlying_type: String,
    pub underlying_sub_type: Vec<String>,
    pub settle_plan: u16,
    pub trigger_protect: Decimal,
    pub liquidation_fee: Decimal,
    pub market_take_bound: Decimal,
    pub filters: Vec<SymbolFilter>,
    pub order_types: Vec<OrderType>,
    pub time_in_force: Vec<TimeInForce>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset: String,
    pub margin_available: bool,
    pub auto_asset_exchange: Decimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInfo {
    pub timezone: String,
    pub server_time: u64,
    pub futures_type: String,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub assets: Vec<Asset>,
    pub symbols: Vec<Symbol>,
}
