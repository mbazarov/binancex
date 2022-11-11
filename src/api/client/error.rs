use super::response::Response;

pub type HeaderParseIntError = std::num::ParseIntError;

#[derive(Debug)]
pub enum BinanceError<E> {
    Binance(Response<E>),
    HeaderParseInt(HeaderParseIntError),
    HttpClient(reqwest::Error),
    ParseQueryString(serde_qs::Error),

    #[cfg(all(feature = "serde_json", not(feature = "simd_json")))]
    ParseJson(serde_json::Error),

    #[cfg(all(feature = "simd_json", not(feature = "serde_json")))]
    ParseJson(simd_json::Error),
}

impl<E> From<Response<E>> for BinanceError<E> {
    fn from(err: Response<E>) -> Self {
        BinanceError::Binance(err)
    }
}

impl<E> From<HeaderParseIntError> for BinanceError<E> {
    fn from(err: HeaderParseIntError) -> Self {
        BinanceError::HeaderParseInt(err)
    }
}

impl<E> From<reqwest::Error> for BinanceError<E> {
    fn from(err: reqwest::Error) -> Self {
        BinanceError::HttpClient(err)
    }
}

impl<E> From<serde_qs::Error> for BinanceError<E> {
    fn from(err: serde_qs::Error) -> Self {
        BinanceError::ParseQueryString(err)
    }
}

#[cfg(all(feature = "serde_json", not(feature = "simd_json")))]
impl<E> From<serde_json::Error> for BinanceError<E> {
    fn from(err: serde_json::Error) -> Self {
        BinanceError::ParseJson(err)
    }
}

#[cfg(all(feature = "simd_json", not(feature = "serde_json")))]
impl<E> From<simd_json::Error> for BinanceError<E> {
    fn from(err: simd_json::Error) -> Self {
        BinanceError::ParseJson(err)
    }
}
