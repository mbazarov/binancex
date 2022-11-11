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
pub enum OrderType {
    Limit,
    Market,
    Stop,
    StopMarket,
    TakeProfit,
    TakeProfitMarket,
    TrailingStopMarket,

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_order_type() {
        let order_types = vec![
            OrderType::Limit,
            OrderType::Market,
            OrderType::Stop,
            OrderType::StopMarket,
            OrderType::TakeProfit,
            OrderType::TakeProfitMarket,
            OrderType::TrailingStopMarket,
        ];

        for order_type in order_types.into_iter() {
            let order_type_static_str = order_type.as_str();
            assert_eq!(
                order_type,
                OrderType::from_str(order_type_static_str).unwrap()
            );
            assert_eq!(order_type, order_type_static_str.try_into().unwrap());

            let order_type_owned_str = order_type_static_str.to_string();
            assert_eq!(
                order_type,
                OrderType::from_str(&order_type_owned_str).unwrap()
            );
        }
    }
}
