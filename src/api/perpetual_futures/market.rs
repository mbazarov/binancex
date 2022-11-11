use crate::api::client::error::BinanceError;
use crate::api::client::response::Response;
use crate::api::perpetual_futures::BinancePerpFutures;

use crate::schemes::error::Error;
use crate::schemes::common::{Pong, ServerTime};
use crate::schemes::futures_fapi::general::ExchangeInfo;
use crate::schemes::futures_fapi::market::*;
use crate::types::futures_fapi::limits::DepthLimit;

pub mod prelude {
    pub use crate::types::futures_fapi::limits::DepthLimit;
}

#[rustfmt::skip]
pub mod endpoints {
    pub static FAPI_V1_PING: &str          = "/fapi/v1/ping";
    pub static FAPI_V1_TIME: &str          = "/fapi/v1/time";
    pub static FAPI_V1_EXCHANGE_INFO: &str = "/fapi/v1/exchangeInfo";
    pub static FAPI_V1_DEPTH: &str         = "/fapi/v1/depth";
}

impl BinancePerpFutures {
    /// Test connectivity to the Rest API.
    ///
    /// Weight(IP): 1
    pub async fn ping(&self) -> Result<Response<Pong>, BinanceError<Error>> {
        self
            .client
            .get::<_, _>(endpoints::FAPI_V1_PING)
            .await
    }

    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// Weight(IP): 1
    pub async fn get_server_time(&self) -> Result<Response<ServerTime>, BinanceError<Error>> {
        self
            .client
            .get::<_, _>(endpoints::FAPI_V1_TIME)
            .await
    }

    /// Current exchange trading rules and symbol information
    ///
    /// Weight(IP): 10
    pub async fn get_exchange_info(&self) -> Result<Response<ExchangeInfo>, BinanceError<Error>> {
        self
            .client
            .get::<_, _>(endpoints::FAPI_V1_EXCHANGE_INFO)
            .await
    }

    /// Get order book snapshot.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use binancex::{
    ///     BinancePerpFutures,
    ///     perpetual_futures::market::prelude::DepthLimit,
    /// };
    ///
    /// let client = BinancePerpFutures::new().unwrap();
    /// let reps = client.get_depth("BTCUSDT", DepthLimit::Limit100).await.unwrap();
    /// println!("{:?}", reps.payload);
    /// ```
    pub async fn get_depth(
        &self,
        symbol: &str,
        limit: DepthLimit,
    ) -> Result<Response<OrderBook>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::FAPI_V1_DEPTH, 34, |url| {
                url.add_param_str("symbol", symbol);
                url.add_param_integer("limit", u16::from(limit));
            })
            .await
    }
}
