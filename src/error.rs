use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum BinanceResponse<T> {
    Success(T),
    Error(BinanceResponseError),
}

#[derive(Debug)]
pub enum BinancexError {
    Binance(BinanceResponseError),
    Parse(serde_json::Error),
    Reqwest(reqwest::Error),
}

impl From<BinanceResponseError> for BinancexError {
    fn from(err: BinanceResponseError) -> Self { BinancexError::Binance(err) }
}

impl From<serde_json::error::Error> for BinancexError {
    fn from(err: serde_json::Error) -> Self { BinancexError::Parse(err) }
}

impl From<reqwest::Error> for BinancexError {
    fn from(err: reqwest::Error) -> Self { BinancexError::Reqwest(err) }
}


impl<T: for<'de> serde::de::Deserialize<'de>> BinanceResponse<T> {
    pub fn to_result(self) -> Result<T, BinanceResponseError> {
        match self {
            BinanceResponse::Success(result) => Ok(result),
            BinanceResponse::Error(e) => Err(e),
        }
    }
}
