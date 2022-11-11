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
    CurrentMonth,
    NextMonth,
    CurrentQuarter,
    NextQuarter,

    #[serde(rename = "")]
    Empty,

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
pub enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
    PendingTrading,
    Settling,

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_serializing)]
    Unknown,
}

#[rustfmt::skip]
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
