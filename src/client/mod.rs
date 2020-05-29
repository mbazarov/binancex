mod general;
mod market;

use reqwest::{
    Client,
    Error,
    Proxy,
};
use std::time::Duration;

static API1_HOST: &'static str = "https://www.binance.com";

static HTTP_CONNECTION_TIMEOUT: u64 = 5000;
static HTTP_REQUEST_TIMEOUT: u64 = 5000;

pub struct Binancex {
    client: Client,
}

pub struct BinancexBuilder {
    connect_timeout: Duration,
    request_timeout: Duration,
    tcp_nodelay: bool,
    proxy: Option<Proxy>,
}

impl Binancex {
    pub fn new() -> Result<Self, Error> {
        Ok(Self { client: Client::builder()
            .connect_timeout(Duration::from_millis(HTTP_CONNECTION_TIMEOUT))
            .timeout(Duration::from_millis(HTTP_REQUEST_TIMEOUT))
            .build()?
        })
    }

    pub fn builder() -> BinancexBuilder {
        BinancexBuilder::new()
    }
}

impl BinancexBuilder {
    pub fn new() -> Self {
        BinancexBuilder {
            connect_timeout: Duration::from_millis(HTTP_CONNECTION_TIMEOUT),
            request_timeout: Duration::from_millis(HTTP_REQUEST_TIMEOUT),
            tcp_nodelay: false,
            proxy: None,
        }
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    pub fn tcp_nodelay(mut self, flag: bool) -> Self {
        self.tcp_nodelay = flag;
        self
    }

    pub fn proxy(mut self, proxy: Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }

    pub fn build(self) -> Result<Binancex, Error> {
        let mut client = Client::builder()
            .connect_timeout(self.connect_timeout.clone())
            .timeout(self.request_timeout.clone());

        if self.tcp_nodelay {
            client = client.tcp_nodelay()
        }

        if let Some(proxy) = self.proxy {
            client = client.proxy(proxy);
        }

        Ok(Binancex {
            client: client.build()?,
        })
    }
}
