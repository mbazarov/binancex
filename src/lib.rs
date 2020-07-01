mod client;
pub mod error;
pub mod scheme;

pub use client::{Binancex, websocket::BinancexWebsocket};
pub use error::BinancexError;
