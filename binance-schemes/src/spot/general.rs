pub use crate::common::general::*;

use binance_types::spot::{general::*, permissions::Permission};

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
    PercentPrice {
        multiplier_up: Decimal,
        multiplier_down: Decimal,
        avg_price_mins: u16,
    },

    #[serde(rename_all = "camelCase")]
    PercentPriceBySide {
        bid_multiplier_up: Decimal,
        bid_multiplier_down: Decimal,
        ask_multiplier_up: Decimal,
        ask_multiplier_down: Decimal,
        avg_price_mins: u16,
    },

    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    MinNotional {
        min_notional: Decimal,
        apply_to_market: bool,
        avg_price_mins: u16,
    },

    #[serde(rename_all = "camelCase")]
    Notional {
        min_notional: Decimal,
        apply_min_to_market: bool,
        max_notional: Decimal,
        apply_max_to_market: bool,
        avg_price_mins: u32,
    },

    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u16 },

    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: Decimal,
        max_qty: Decimal,
        step_size: Decimal,
    },

    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: u16 },

    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u16 },

    #[serde(rename_all = "camelCase")]
    MaxNumIcebergOrders { max_num_iceberg_orders: u16 },

    #[serde(rename_all = "camelCase")]
    MaxPosition { max_position: Decimal },

    #[serde(rename_all = "camelCase")]
    TrailingDelta {
        min_trailing_above_delta: u16,
        max_trailing_above_delta: u16,
        min_trailing_below_delta: u16,
        max_trailing_below_delta: u16,
    },

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_deserializing)]
    Unknown,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: SymbolStatus,
    pub base_asset: String,
    pub base_asset_precision: u8,
    pub quote_asset: String,
    pub quote_precision: u8,
    pub quote_asset_precision: u8,
    pub base_commission_precision: u8,
    pub quote_commission_precision: u8,
    pub order_types: Vec<OrderType>,
    pub iceberg_allowed: bool,
    pub oco_allowed: bool,
    pub quote_order_qty_market_allowed: bool,
    pub is_spot_trading_allowed: bool,
    pub is_margin_trading_allowed: bool,
    pub filters: Vec<SymbolFilter>,
    pub permissions: Vec<Permission>,
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
