#[allow(unused_imports)]
use std::str::FromStr;

use serde_repr::{Deserialize_repr, Serialize_repr};
use strum_macros::{EnumString, IntoStaticStr};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DepthLimit {
    Limit(u16), // If limit > 5000, then the response will truncate to 5000.
    Limit100,
    Limit500,
    Limit1000,
    Limit5000,
}

impl Default for DepthLimit {
    fn default() -> Self {
        DepthLimit::Limit100
    }
}

impl DepthLimit {
    pub fn request_weight(&self) -> RequestWeight {
        RequestWeight::from(*self)
    }
}

#[rustfmt::skip]
impl From<DepthLimit> for u16 {
    fn from(depth_limit: DepthLimit) -> Self {
        match depth_limit {
            DepthLimit::Limit(limit) => limit,
            DepthLimit::Limit100     => 100,
            DepthLimit::Limit500     => 500,
            DepthLimit::Limit1000    => 1000,
            DepthLimit::Limit5000    => 5000,
        }
    }
}

#[rustfmt::skip]
impl<'de> serde::Deserialize<'de> for DepthLimit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let limit: u16 = serde::Deserialize::deserialize(deserializer)?;
        match limit {
            100  => Ok(DepthLimit::Limit100),
            500  => Ok(DepthLimit::Limit500),
            1000 => Ok(DepthLimit::Limit1000),
            5000 => Ok(DepthLimit::Limit5000),
            0..=u16::MAX => Ok(DepthLimit::Limit(limit)),
        }
    }
}

#[rustfmt::skip]
impl serde::Serialize for DepthLimit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value: u16 = match *self {
            DepthLimit::Limit100     => 100,
            DepthLimit::Limit500     => 500,
            DepthLimit::Limit1000    => 1000,
            DepthLimit::Limit5000    => 5000,
            DepthLimit::Limit(limit) => limit,
        };
        serde::Serialize::serialize(&value, serializer)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RequestWeight {
    Weight0,
    Weight1,
    Weight5,
    Weight10,
    Weight50,
}

#[rustfmt::skip]
impl From<RequestWeight> for u8 {
    fn from(weight: RequestWeight) -> Self {
        match weight {
            RequestWeight::Weight0  => 0,
            RequestWeight::Weight1  => 1,
            RequestWeight::Weight5  => 5,
            RequestWeight::Weight10 => 10,
            RequestWeight::Weight50 => 50,
        }
    }
}

#[rustfmt::skip]
impl From<u16> for RequestWeight {
    fn from(limit: u16) -> Self {
        match limit {
            1..=100     => RequestWeight::Weight1,
            101..=500   => RequestWeight::Weight5,
            501..=1000  => RequestWeight::Weight10,
            1001..=5000 => RequestWeight::Weight50,
            0           => RequestWeight::Weight0,
            _           => RequestWeight::Weight50,
        }
    }
}

#[rustfmt::skip]
impl From<DepthLimit> for RequestWeight {
    fn from(limit: DepthLimit) -> Self {
        match limit {
            DepthLimit::Limit100  => RequestWeight::Weight1,
            DepthLimit::Limit500  => RequestWeight::Weight5,
            DepthLimit::Limit1000 => RequestWeight::Weight10,
            DepthLimit::Limit5000 => RequestWeight::Weight50,
            DepthLimit::Limit(limit) => limit.into(),
        }
    }
}

#[rustfmt::skip]
#[derive(
    Copy, Clone, Debug, Eq, PartialEq,
    Deserialize_repr, Serialize_repr,
)]
#[repr(u16)]
pub enum DepthUpdateSpeedMs {
    Speed100  = 100,
    Speed1000 = 1000,
}

#[rustfmt::skip]
impl From<DepthUpdateSpeedMs> for u16 {
    fn from(update_speed: DepthUpdateSpeedMs) -> Self {
        update_speed as u16
    }
}

#[rustfmt::skip]
#[derive(
Copy, Clone, Debug, PartialEq, Eq,
EnumString, IntoStaticStr,
)]
pub enum KLinesInterval {
    #[strum(serialize = "1s")]
    Second,
    #[strum(serialize = "1m")]
    Minute,
    #[strum(serialize = "3m")]
    Minutes3,
    #[strum(serialize = "5m")]
    Minutes5,
    #[strum(serialize = "15m")]
    Minutes15,
    #[strum(serialize = "30m")]
    Minutes30,
    #[strum(serialize = "1h")]
    Hour,
    #[strum(serialize = "2h")]
    Hours2,
    #[strum(serialize = "4h")]
    Hours4,
    #[strum(serialize = "6h")]
    Hours6,
    #[strum(serialize = "8h")]
    Hours8,
    #[strum(serialize = "12h")]
    Hours12,
    #[strum(serialize = "1d")]
    Day,
    #[strum(serialize = "3d")]
    Days3,
    #[strum(serialize = "1w")]
    Weak,
    #[strum(serialize = "1M")]
    Month,
}

impl KLinesInterval {
    pub fn as_str(&self) -> &'static str {
        self.into()
    }
}

impl std::fmt::Display for KLinesInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.pad(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_serialize_depth_limit() {
        use serde::{Deserialize, Serialize};

        #[derive(Deserialize, Serialize)]
        struct SomeData {
            pub depth_limit: DepthLimit,
        }

        let json100 = r#"{"depth_limit":100}"#;
        let data100: SomeData = serde_json::from_str(json100).unwrap();
        assert_eq!(data100.depth_limit, DepthLimit::Limit100);

        let json100_from_data = serde_json::to_string(&data100).unwrap();
        assert_eq!(json100, json100_from_data);

        let json_user_limit_50 = r#"{"depth_limit":50}"#;
        let data_user_limit_50: SomeData = serde_json::from_str(json_user_limit_50).unwrap();
        assert_eq!(data_user_limit_50.depth_limit, DepthLimit::Limit(50));

        let json_user_limit_50_from_data = serde_json::to_string(&data_user_limit_50).unwrap();
        assert_eq!(json_user_limit_50, json_user_limit_50_from_data);
    }

    #[test]
    fn test_depth_limit_over_5000_to_weight() {
        let limit = DepthLimit::Limit(6000);
        assert_eq!(limit.request_weight(), RequestWeight::Weight50);
    }
}
