use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    Orders,
    RawRequests,
    RequestWeight,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitInterval {
    Second,
    Minute,
    Day,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeFilter {
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumOrders {
        max_num_orders: u32,
    },

    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumAlgoOrders {
        max_num_algo_orders: u32,
    },

    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumIcebergOrders {
        max_num_iceberg_orders: u32,
    },

    #[cfg(not(feature = "strict-enums"))]
    #[serde(other)]
    #[serde(skip_deserializing)]
    Unknown,
}
