#[allow(unused_imports)]
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, IntoStaticStr};

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize, Serialize,
    EnumString, IntoStaticStr,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

impl OrderStatus {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize, Serialize,
    EnumString, IntoStaticStr,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

impl OrderType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize, Serialize,
    EnumString, IntoStaticStr,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

impl SymbolStatus {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for SymbolStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_order_status() {
        let order_statuses = vec![
            OrderStatus::New,
            OrderStatus::PartiallyFilled,
            OrderStatus::Filled,
            OrderStatus::Canceled,
            OrderStatus::PendingCancel,
            OrderStatus::Rejected,
            OrderStatus::Expired,
        ];

        for status in order_statuses.into_iter() {
            let status_static_str = status.as_str();
            assert_eq!(status, OrderStatus::from_str(status_static_str).unwrap());
            assert_eq!(status, status_static_str.try_into().unwrap());

            let status_owned_str = status_static_str.to_string();
            assert_eq!(status, OrderStatus::from_str(&status_owned_str).unwrap());
        }
    }

    #[test]
    fn test_convert_order_types() {
        let order_types = vec![
            OrderType::Limit,
            OrderType::Market,
            OrderType::StopLoss,
            OrderType::StopLossLimit,
            OrderType::TakeProfit,
            OrderType::TakeProfitLimit,
            OrderType::LimitMaker,
        ];

        for order_type in order_types.into_iter() {
            let type_static_str = order_type.as_str();
            assert_eq!(order_type, OrderType::from_str(type_static_str).unwrap());
            assert_eq!(order_type, type_static_str.try_into().unwrap());

            let type_owned_str = type_static_str.to_string();
            assert_eq!(order_type, OrderType::from_str(&type_owned_str).unwrap());
        }
    }

    #[test]
    fn test_convert_symbol_status() {
        let symbol_statuses = vec![
            SymbolStatus::PreTrading,
            SymbolStatus::Trading,
            SymbolStatus::PostTrading,
            SymbolStatus::EndOfDay,
            SymbolStatus::Halt,
            SymbolStatus::AuctionMatch,
            SymbolStatus::Break,
        ];

        for status in symbol_statuses.into_iter() {
            let status_static_str = status.as_str();
            assert_eq!(status, SymbolStatus::from_str(status_static_str).unwrap());
            assert_eq!(status, status_static_str.try_into().unwrap());

            let status_owned_str = status_static_str.to_string();
            assert_eq!(status, SymbolStatus::from_str(&status_owned_str).unwrap());
        }
    }
}
