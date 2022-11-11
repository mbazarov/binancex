pub mod market;

use crate::api::client::error::BinanceError;
use crate::api::client::BinanceClient;
use crate::api::{BinanceBuilder, FromBinanceClient};

pub const DELIVERY_FUTURES_API: &'static str = "https://dapi.binance.com";

#[derive(Clone)]
pub struct BinanceDeliveryFutures {
    client: BinanceClient,
}

impl BinanceDeliveryFutures {
    pub fn builder() -> BinanceBuilder<Self> {
        BinanceBuilder::new()
    }

    pub fn new() -> Result<Self, BinanceError<()>> {
        Ok(Self {
            client: BinanceClient::new(DELIVERY_FUTURES_API.to_owned())?,
        })
    }

    pub fn signed<K1, K2>(api_key: K1, secret_key: K2) -> Result<Self, BinanceError<()>>
    where
        K1: Into<String>,
        K2: Into<String>,
    {
        Ok(Self {
            client: BinanceClient::signed(DELIVERY_FUTURES_API.to_owned(), api_key.into(), secret_key.into())?,
        })
    }

    pub fn with_host<S: Into<String>>(host: S) -> Result<Self, BinanceError<()>> {
        Ok(Self {
            client: BinanceClient::new(host.into())?,
        })
    }

    pub fn signed_with_host<S, K1, K2>(
        host: S,
        api_key: K1,
        secret_key: K2,
    ) -> Result<Self, BinanceError<()>>
    where
        S: Into<String>,
        K1: Into<String>,
        K2: Into<String>,
    {
        Ok(Self {
            client: BinanceClient::signed(host.into(), api_key.into(), secret_key.into())?,
        })
    }
}

impl FromBinanceClient for BinanceDeliveryFutures {
    const HOST: &'static str = DELIVERY_FUTURES_API;

    fn from_client(client: BinanceClient) -> Self {
        Self { client }
    }
}
