pub mod error;
mod hmac;
mod json;
pub(crate) mod query_string;
pub mod response;
pub(crate) mod url;

use std::net::IpAddr;
use std::time::Duration;

use crate::api::client::error::BinanceError;
use crate::api::client::response::{BinanceHttpHeader, Response};
use url::Url;

use http::Method;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Client, ClientBuilder,
};
use serde::de::Deserialize;

pub(crate) static HTTP_CONNECTION_TIMEOUT_MS_DEFAULT: u64 = 5000;
pub(crate) static HTTP_REQUEST_TIMEOUT_MS_DEFAULT: u64 = 5000;
pub(crate) static RECV_WINDOW_MS_DEFAULT: u16 = 5000;

const RECV_WINDOW_MAX_LEN: usize = 17;
const TIMESTAMP_MAX_LEN: usize = 30;
const SIGNATURE_MAX_LEN: usize = 64 + 11;
const SIGNED_PARAMS_LEN: usize =
    RECV_WINDOW_MAX_LEN + TIMESTAMP_MAX_LEN + SIGNATURE_MAX_LEN;

pub struct BinanceClientBuilder {
    pub host: String,
    pub api_key: Option<String>,
    pub secret_key: Option<String>,
    pub recv_window: Option<u16>,
    pub http_connect_timeout_ms: Option<Duration>,
    pub http_request_timeout_ms: Option<Duration>,
    pub tcp_nodelay: Option<bool>,
    pub local_addr: Option<IpAddr>,
}

impl BinanceClientBuilder {
    pub fn new(host: String) -> Self {
        BinanceClientBuilder {
            host,
            api_key: None,
            secret_key: None,
            recv_window: None,
            http_connect_timeout_ms: None,
            http_request_timeout_ms: None,
            tcp_nodelay: None,
            local_addr: None,
        }
    }

    pub fn build(self) -> Result<BinanceClient, BinanceError<()>> {
        let mut client = Client::builder();

        let http_connect_timeout = self
            .http_connect_timeout_ms
            .unwrap_or(Duration::from_millis(HTTP_CONNECTION_TIMEOUT_MS_DEFAULT));
        client = client.connect_timeout(http_connect_timeout);

        let http_request_timeout = self
            .http_request_timeout_ms
            .unwrap_or(Duration::from_millis(HTTP_REQUEST_TIMEOUT_MS_DEFAULT));
        client = client.timeout(http_request_timeout);

        client = client.tcp_nodelay(self.tcp_nodelay.unwrap_or(true));

        if let Some(addr) = self.local_addr {
            client = client.local_address(addr);
        }

        Ok(BinanceClient {
            api_key: self.api_key.unwrap_or(String::new()),
            secret_key: {
                let key = self.secret_key.unwrap_or_default();
                ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key.as_bytes())
            },
            recv_window: self.recv_window.unwrap_or(RECV_WINDOW_MS_DEFAULT),
            host: self.host,
            client: client.build()?,
        })
    }
}

fn http_client_default() -> ClientBuilder {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    Client::builder()
        // .http2_prior_knowledge()
        // .https_only(true)
        .tcp_nodelay(true)
        .connect_timeout(Duration::from_millis(HTTP_CONNECTION_TIMEOUT_MS_DEFAULT))
        .timeout(Duration::from_millis(HTTP_REQUEST_TIMEOUT_MS_DEFAULT))
        .default_headers(headers)
}

#[derive(Clone)]
pub struct BinanceClient {
    pub(crate) api_key: String,
    pub(crate) secret_key: ring::hmac::Key,
    pub(crate) recv_window: u16,
    pub(crate) host: String,
    pub(crate) client: Client,
}

impl BinanceClient {
    pub fn builder(host: String) -> BinanceClientBuilder {
        BinanceClientBuilder::new(host)
    }

    pub fn new(host: String) -> Result<Self, BinanceError<()>> {
        Ok(Self {
            api_key: String::new(),
            secret_key: ring::hmac::Key::new(ring::hmac::HMAC_SHA256, String::new().as_bytes()),
            recv_window: RECV_WINDOW_MS_DEFAULT,
            host,
            client: http_client_default().build()?,
        })
    }

    pub fn signed(
        host: String,
        api_key: String,
        secret_key: String,
    ) -> Result<Self, BinanceError<()>> {
        Ok(Self {
            api_key,
            secret_key: ring::hmac::Key::new(ring::hmac::HMAC_SHA256, secret_key.as_bytes()),
            recv_window: RECV_WINDOW_MS_DEFAULT,
            host,
            client: http_client_default().build()?,
        })
    }

    // pub async fn get<D, E>(
    //     &self,
    //     path: &str,
    //     add_params: Option<(usize, impl FnOnce(Url) -> Url)>,
    // ) -> Result<Response<D>, BinanceError<E>>
    // where
    //     D: for<'de> Deserialize<'de>,
    //     E: for<'de> Deserialize<'de>,
    // {
    //     let mut url_capacity = self.host.len() + path.len();
    //     let url = if let Some((capacity, query)) = query {
    //         url_capacity += capacity;
    //         query(Url::with_capacity(&self.host, path, url_capacity))
    //     } else {
    //         Url::with_capacity(&self.host, path, url_capacity)
    //     };
    //
    //     #[cfg(debug_assertions)]
    //     debug_url("final", &url);
    //
    //     self.request(Method::GET, url).await
    // }

    pub async fn get<D, E>(&self, path: &str) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len();
        let url = Url::with_capacity(&self.host, path, url_capacity);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.request(Method::GET, url).await
    }

    pub async fn get_with_query<D, E>(
        &self,
        path: &str,
        query_capacity: usize,
        add_params: impl FnOnce(&mut Url),
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len() + query_capacity;
        #[cfg(debug_assertions)]
        println!("start url capacity: {}", url_capacity);

        let mut url = Url::with_capacity(&self.host, path, url_capacity);
        add_params(&mut url);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.request(Method::GET, url).await
    }

    pub async fn get_with_query_api_key<D, E>(
        &self,
        path: &str,
        query_capacity: usize,
        add_params: impl FnOnce(&mut Url),
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len() + query_capacity;
        #[cfg(debug_assertions)]
        println!("start url capacity: {}", url_capacity);

        let mut url = Url::with_capacity(&self.host, path, url_capacity);
        add_params(&mut url);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.signed_request(Method::GET, url).await
    }

    pub async fn get_signed<D, E>(&self, path: &str) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len() + SIGNED_PARAMS_LEN;
        let mut url = Url::with_capacity(&self.host, path, url_capacity);
        self.add_signed_params(&mut url);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.signed_request(Method::GET, url).await
    }

    pub async fn get_signed_with_query<D, E>(
        &self,
        path: &str,
        query_capacity: usize,
        add_params: impl FnOnce(&mut Url) -> Result<(), serde_qs::Error>,
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len() + query_capacity + SIGNED_PARAMS_LEN;
        #[cfg(debug_assertions)]
        println!("start url capacity: {}", url_capacity);

        let mut url = Url::with_capacity(&self.host, path, url_capacity);
        add_params(&mut url)?;
        self.add_signed_params(&mut url);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.signed_request(Method::GET, url).await
    }

    pub async fn post_signed_with_query<D, E>(
        &self,
        path: &str,
        query_capacity: usize,
        add_params: impl FnOnce(&mut Url) -> Result<(), serde_qs::Error>,
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len() + query_capacity + SIGNED_PARAMS_LEN;
        #[cfg(debug_assertions)]
        println!("start url capacity: {}", url_capacity);

        let mut url = Url::with_capacity(&self.host, path, url_capacity);
        add_params(&mut url)?;
        self.add_signed_params(&mut url);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.signed_request(Method::POST, url).await
    }

    pub async fn delete_signed_with_query<D, E>(
        &self,
        path: &str,
        query_capacity: usize,
        add_params: impl FnOnce(&mut Url) -> Result<(), serde_qs::Error>,
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let url_capacity = self.host.len() + path.len() + query_capacity + SIGNED_PARAMS_LEN;
        #[cfg(debug_assertions)]
        println!("start url capacity: {}", url_capacity);

        let mut url = Url::with_capacity(&self.host, path, url_capacity);
        add_params(&mut url)?;
        self.add_signed_params(&mut url);

        #[cfg(debug_assertions)]
        debug_url("final", &url);

        self.signed_request(Method::DELETE, url).await
    }

    pub fn add_signed_params(&self, url: &mut Url) {
        if self.recv_window != RECV_WINDOW_MS_DEFAULT {
            url.add_recv_window(self.recv_window);
        }
        url.add_timestamp();
        url.gen_and_add_signature(&self.secret_key);
    }

    #[inline(always)]
    async fn request<D, E>(
        &self,
        method: Method,
        url: Url,
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let resp = self.client.request(method, url.as_str()).send().await?;
        BinanceClient::handle_response(resp).await
    }

    #[inline(always)]
    async fn signed_request<D, E>(
        &self,
        method: Method,
        url: Url,
    ) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let resp = self
            .client
            .request(method, url.as_str())
            .header(
                HeaderName::from_static("x-mbx-apikey"),
                HeaderValue::from_str(&self.api_key).unwrap(),
            )
            .send()
            .await?;
        BinanceClient::handle_response(resp).await
    }

    async fn handle_response<D, E>(resp: reqwest::Response) -> Result<Response<D>, BinanceError<E>>
    where
        D: for<'de> Deserialize<'de>,
        E: for<'de> Deserialize<'de>,
    {
        let status = resp.status();
        let headers: BinanceHttpHeader = resp.headers().try_into()?;
        let json_bytes = resp.bytes().await?;
        let json: response::SuccessOrError<D, E> = json::from_slice(json_bytes.as_ref()).await?;

        json.to_result(status, headers)
    }
}

fn debug_url(marker: &str, url: &Url) {
    println!(
        "{} url: {}\nlen: {}\ncapacity: {}",
        marker,
        url.as_str(),
        url.len(),
        url.capacity(),
    );
}
