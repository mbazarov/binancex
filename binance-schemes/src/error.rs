use crate::spot::trade::CancelOrderRes;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: i64,
    pub msg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CancelReplaceOrderError {
    pub code: i64,
    pub msg: String,
    pub data: Data,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub cancel_result: String,
    pub new_order_result: String,
    pub cancel_response: CancelResponse,
    pub new_order_response: NewOrderResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CancelResponse {
    Success(CancelOrderRes),
    Failure(Error),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum NewOrderResponse {
    #[serde(rename_all = "camelCase")]
    Success {
        symbol: String,
        order_id: u64,
        order_list_id: i64,
        client_order_id: String,
        transact_time: u64,
    },
    Failure(Error),
    None,
}
