#[allow(unused_imports)]
use std::str::FromStr;

pub use crate::futures_common::general::OrderType;

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
pub enum ContractType {
    Perpetual,
    CurrentQuarter,
    NextQuarter,
    CurrentQuarterDelivering, // Invalid type, only used for DELIVERING status
    NextQuarterDelivering,    // Invalid type, only used for DELIVERING status

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

impl ContractType {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for ContractType {
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
pub enum ContractStatus {
    PendingTrading,
    Trading,
    PreDelivering,
    Delivering,
    Delivered,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

impl ContractStatus {
    #[inline]
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for ContractStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_contract_status() {
        let contract_statuses = vec![
            ContractStatus::PendingTrading,
            ContractStatus::Trading,
            ContractStatus::PreDelivering,
            ContractStatus::Delivering,
            ContractStatus::Delivered,
        ];

        for contract_status in contract_statuses.into_iter() {
            let status_static_str = contract_status.as_str();
            assert_eq!(
                contract_status,
                ContractStatus::from_str(status_static_str).unwrap()
            );
            assert_eq!(contract_status, status_static_str.try_into().unwrap());

            let status_owned_str = status_static_str.to_owned();
            assert_eq!(
                contract_status,
                ContractStatus::from_str(&status_owned_str).unwrap()
            );
        }
    }
}
