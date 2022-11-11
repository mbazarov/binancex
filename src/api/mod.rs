mod client;
pub mod delivery_futures;
pub mod perpetual_futures;
pub mod spot;

use std::marker::PhantomData;
use std::net::IpAddr;
use std::time::Duration;

use client::{BinanceClientBuilder, BinanceClient};

pub use client::error::BinanceError;
pub use delivery_futures::BinanceDeliveryFutures;
pub use perpetual_futures::BinancePerpFutures;
pub use spot::BinanceSpot;

pub trait FromBinanceClient {
    const HOST: &'static str;

    fn from_client(client: BinanceClient) -> Self;
}

pub struct BinanceBuilder<C: FromBinanceClient> {
    api_key: Option<String>,
    secret_key: Option<String>,
    recv_window: Option<u16>,
    http_connect_timeout_ms: Option<Duration>,
    http_request_timeout_ms: Option<Duration>,
    tcp_nodelay: Option<bool>,
    local_addr: Option<IpAddr>,
    phantom: PhantomData<C>,
}

impl<C> Default for BinanceBuilder<C>
where
    C: FromBinanceClient,
{
    fn default() -> Self {
        BinanceBuilder {
            api_key: None,
            secret_key: None,
            recv_window: None,
            http_connect_timeout_ms: None,
            http_request_timeout_ms: None,
            tcp_nodelay: None,
            local_addr: None,
            phantom: PhantomData,
        }
    }
}

impl<C> BinanceBuilder<C>
where
    C: FromBinanceClient,
{
    pub fn new() -> Self {
        BinanceBuilder::default()
    }

    pub fn signed<K1, K2>(mut self, api_key: K1, secret_key: K2) -> Self
    where
        K1: Into<String>,
        K2: Into<String>,
    {
        self.api_key = Some(api_key.into());
        self.secret_key = Some(secret_key.into());
        self
    }

    pub fn recv_window_ms(mut self, recv_window: u16) -> Self {
        self.recv_window = Some(recv_window);
        self
    }

    pub fn http_connect_timeout_ms(mut self, timeout: Duration) -> Self {
        self.http_connect_timeout_ms = Some(timeout);
        self
    }

    pub fn http_request_timeout_ms(mut self, timeout: Duration) -> Self {
        self.http_request_timeout_ms = Some(timeout);
        self
    }

    pub fn tcp_nodelay(mut self, enabled: bool) -> Self {
        self.tcp_nodelay = Some(enabled);
        self
    }

    pub fn bind_local_address(mut self, addr: IpAddr) -> Self {
        self.local_addr = Some(addr);
        self
    }

    fn to_client_builder(self, host: String) -> BinanceClientBuilder {
        BinanceClientBuilder {
            host,
            api_key: self.api_key,
            secret_key: self.secret_key,
            recv_window: self.recv_window,
            http_connect_timeout_ms: self.http_connect_timeout_ms,
            http_request_timeout_ms: self.http_request_timeout_ms,
            tcp_nodelay: self.tcp_nodelay,
            local_addr: self.local_addr,
        }
    }

    pub fn build(self) -> Result<C, BinanceError<()>> {
        let client = self.to_client_builder(C::HOST.to_owned());
        Ok(C::from_client(client.build()?))
    }
}
