use super::{API1_HOST, Binancex};
use crate::{
    error::{BinancexError, BinanceResponse},
    schemes::market::{DepthLimit, OrderBook},
};


impl Binancex {
    pub async fn get_depth(&self, symbol: &str, limit: DepthLimit) -> Result<OrderBook, BinancexError> {
        let url = format!("{}/api/v3/depth?symbol={}&limit={}",
                          API1_HOST,
                          symbol,
                          limit as u16);

        Ok(self.client
            .get(&url)
            .send()
            .await?
            .json::<BinanceResponse<OrderBook>>()
            .await?
            .to_result()?)
    }
}
