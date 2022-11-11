use std::convert::TryFrom;

use super::error::HeaderParseIntError;

use crate::api::client::error::BinanceError;
use reqwest::{header::HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SuccessOrError<T, E> {
    Success(T),
    Error(E),
}

impl<T, E> SuccessOrError<T, E>
where
    T: for<'de> serde::de::Deserialize<'de>,
{
    #[inline(always)]
    pub fn to_result(
        self,
        status: StatusCode,
        headers: BinanceHttpHeader,
    ) -> Result<Response<T>, BinanceError<E>> {
        match self {
            SuccessOrError::Success(result) => Ok(Response {
                headers,
                status,
                payload: result,
            }),
            SuccessOrError::Error(err) => Err(BinanceError::Binance(Response {
                headers,
                status,
                payload: err,
            })),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BinanceHttpHeader {
    pub x_mbx_used_weight: Option<u16>,
    pub x_mbx_used_weight_1m: Option<u16>,
    pub retry_after: Option<u16>,
}

#[derive(Debug)]
pub struct Response<T> {
    pub headers: BinanceHttpHeader,
    pub status: StatusCode,
    pub payload: T,
}

impl TryFrom<&HeaderMap> for BinanceHttpHeader {
    type Error = HeaderParseIntError;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        let x_mbx_used_weight = match headers.get("x-mbx-used-weight") {
            Some(val) => Some(val.to_str().unwrap().parse()?),
            None => None,
        };

        let x_mbx_used_weight_1m = match headers.get("x-mbx-used-weight-1m") {
            Some(val) => Some(val.to_str().unwrap().parse()?),
            None => None,
        };

        let retry_after = match headers.get("retry-after") {
            Some(val) => Some(val.to_str().unwrap().parse()?),
            None => None,
        };

        Ok(BinanceHttpHeader {
            x_mbx_used_weight,
            x_mbx_used_weight_1m,
            retry_after,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SuccessOrError;

    #[test]
    fn test_parse_binance_response_error_or_empty_json() {
        use crate::schemes::common::Pong;
        use binance_schemes::error::Error;

        let json = r#"
        {
            "code": -1003,
            "msg": "Too much request weight used; current limit is 1200 request weight per 1 MINUTE. Please use the websocket for live updates to avoid polling the API."
        }"#;

        match serde_json::from_str::<SuccessOrError<Pong, Error>>(json).unwrap() {
            SuccessOrError::Error(err) => {
                assert_eq!(err.code, -1003);
            }
            _ => unreachable!(),
        }
    }
}
