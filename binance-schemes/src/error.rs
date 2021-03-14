use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct BinanceResponseError {
    pub code: i64,
    pub msg: String,
}
