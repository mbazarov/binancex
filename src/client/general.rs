use super::{API1_HOST, Binancex};
use crate::{
    error::{BinancexError, BinanceResponse},
    schemes::general::ExchangeInfo,
};


impl Binancex {
    pub async fn get_exchange_info(&self) -> Result<ExchangeInfo, BinancexError> {
        let url = format!("{}/api/v3/exchangeInfo", API1_HOST);

        Ok(self.client
            .get(&url)
            .send()
            .await?
            .json::<BinanceResponse<ExchangeInfo>>()
            .await?
            .to_result()?)
    }
}
