use crate::api::client::error::BinanceError;
use crate::api::client::response::Response;
use crate::api::spot::BinanceSpot;

use crate::schemes::error::Error;
use crate::schemes::common::{Pong, ServerTime};
use crate::schemes::spot::general::ExchangeInfo;
use crate::schemes::spot::market::*;
use crate::types::spot::limits::{DepthLimit, KLinesInterval};

pub mod prelude {
    pub use crate::types::spot::limits::{DepthLimit, KLinesInterval};
}

#[rustfmt::skip]
pub mod endpoints {
    pub static API_V3_PING: &str               = "/api/v3/ping";
    pub static API_V3_TIME: &str               = "/api/v3/time";
    pub static API_V3_EXCHANGE_INFO: &str      = "/api/v3/exchangeInfo";
    pub static API_V3_DEPTH: &str              = "/api/v3/depth";
    pub static API_V3_TRADES: &str             = "/api/v3/trades";
    pub static API_V3_HISTORICAL_TRADES: &str  = "/api/v3/historicalTrades";
    pub static API_V3_AGG_TRADES: &str         = "/api/v3/aggTrades";
    pub static API_V3_KLINES: &str             = "/api/v3/klines";
    pub static API_V3_UI_KLINES: &str          = "/api/v3/uiKlines";
    pub static API_V3_AVG_PRICE: &str          = "/api/v3/avgPrice";
    pub static API_V3_TICKER_24H: &str         = "/api/v3/ticker/24hr";
    pub static API_V3_TICKER_PRICE: &str       = "/api/v3/ticker/price";
    pub static API_V3_TICKER_BOOK_TICKER: &str = "/api/v3/ticker/bookTicker";
}

impl BinanceSpot {
    /// Test connectivity to the Rest API.
    ///
    /// Weight(IP): 1
    pub async fn ping(&self) -> Result<Response<Pong>, BinanceError<Error>> {
        self
            .client
            .get::<_, _>(endpoints::API_V3_PING)
            .await
    }

    /// Test connectivity to the Rest API and get the current server time.
    ///
    /// Weight(IP): 1
    pub async fn get_server_time(&self) -> Result<Response<ServerTime>, BinanceError<Error>> {
        self
            .client
            .get::<_, _>(endpoints::API_V3_TIME)
            .await
    }

    /// Current exchange trading rules and symbol information
    ///
    /// Weight(IP): 10
    pub async fn get_exchange_info(&self) -> Result<Response<ExchangeInfo>, BinanceError<Error>> {
        self
            .client
            .get::<_, _>(endpoints::API_V3_EXCHANGE_INFO)
            .await
    }

    /// Get order book snapshot.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use binancex::{
    ///     BinanceSpot,
    ///     spot::market::prelude::DepthLimit,
    /// };
    ///
    /// let client = BinanceSpot::new().unwrap();
    /// let reps = client.get_depth("BTCUSDT", DepthLimit::Limit100).await.unwrap();
    /// println!("{:?}", reps.payload);
    /// ```
    pub async fn get_depth(
        &self,
        symbol: &str,
        limit: DepthLimit,
    ) -> Result<Response<OrderBook>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_DEPTH, 40, |url| {
                url.add_param_str("symbol", symbol);
                url.add_param_integer("limit", u16::from(limit));
            })
            .await
    }

    /// Get recent trades.
    ///
    /// Weight(IP): 1
    pub async fn get_recent_trades(
        &self,
        symbol: &str,
        limit: Option<u16>,
    ) -> Result<Response<Vec<Trade>>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_TRADES, 40, |url| {
                url.add_param_str("symbol", symbol);
                if let Some(l) = limit {
                    url.add_param_integer("limit", l);
                }
            })
            .await
    }

    /// Get older market trades.
    ///
    /// Weight(IP): 5
    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        from_id: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<Trade>>, BinanceError<Error>> {
        self.client
            .get_with_query_api_key::<_, _>(
                endpoints::API_V3_HISTORICAL_TRADES,
                70,
                |url| {
                    url.add_param_str("symbol", symbol);
                    if let Some(id) = from_id {
                        url.add_param_integer("fromId", id);
                    }
                    if let Some(val) = limit {
                        url.add_param_integer("limit", val);
                    }
                },
            )
            .await
    }

    /// Get compressed, aggregate trades. Trades that fill at the time, from the same order,
    /// with the same price will have the quantity aggregated.
    ///
    /// Weight(IP): 1
    pub async fn get_aggregate_trades(
        &self,
        symbol: &str,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<AggregateTrade>>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_AGG_TRADES, 128, |url| {
                url.add_param_str("symbol", symbol);

                if let Some(id) = from_id {
                    url.add_param_integer("fromId", id);
                }
                if let Some(val) = start_time {
                    url.add_param_integer("startTime", val);
                }
                if let Some(val) = end_time {
                    url.add_param_integer("endTime", val);
                }
                if let Some(l) = limit {
                    url.add_param_integer("limit", l);
                }
            })
            .await
    }

    /// Kline/candlestick bars for a symbol.
    ///
    /// Weight(IP): 1
    ///
    /// Klines are uniquely identified by their open time.
    /// If start_time and end_time are not sent, the most recent klines are returned.
    pub async fn get_klines(
        &self,
        symbol: &str,
        interval: KLinesInterval,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<KlineSummary>>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_KLINES, 128, |url| {
                url.add_param_str("symbol", symbol);
                url.add_param_str("interval", interval.as_str());

                if let Some(val) = start_time {
                    url.add_param_integer("startTime", val);
                }
                if let Some(val) = end_time {
                    url.add_param_integer("endTime", val);
                }
                if let Some(val) = limit {
                    url.add_param_integer("limit", val);
                }
            })
            .await
    }

    /// Kline/candlestick bars for a symbol.
    ///
    /// Weight: 1
    ///
    /// uiKlines return modified kline data, optimized for presentation of candlestick charts.
    /// If startTime and endTime are not sent, the most recent klines are returned.
    pub async fn get_klines_ui(
        &self,
        symbol: &str,
        interval: KLinesInterval,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<KlineSummary>>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_UI_KLINES, 128, |url| {
                url.add_param_str("symbol", symbol);
                url.add_param_str("interval", interval.as_str());

                if let Some(val) = start_time {
                    url.add_param_integer("startTime", val);
                }
                if let Some(val) = end_time {
                    url.add_param_integer("endTime", val);
                }
                if let Some(val) = limit {
                    url.add_param_integer("limit", val);
                }
            })
            .await
    }

    /// Current average price for a symbol.
    ///
    /// Weight(IP): 1
    pub async fn get_average_price(
        &self,
        symbol: &str,
    ) -> Result<Response<AveragePrice>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_AVG_PRICE, 32, |url| {
                url.add_param_str("symbol", symbol);
            })
            .await
    }

    /// 24 hour rolling window price change statistics.
    ///
    /// Weight(IP): 1
    pub async fn get_24h_ticker_full(
        &self,
        symbol: &str,
    ) -> Result<Response<TickerStatsFull>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_TICKER_24H, 40, |url| {
                url.add_param_str("symbol", symbol);
                url.add_param_str("type", "FULL");
            })
            .await
    }

    /// 24 hour rolling window price change statistics.
    ///
    /// Weight(IP): 1
    pub async fn get_24h_ticker_mini(
        &self,
        symbol: &str,
    ) -> Result<Response<TickerStatsMini>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_TICKER_24H, 40, |url| {
                url.add_param_str("symbol", symbol);
                url.add_param_str("type", "MINI");
            })
            .await
    }

    /// 24 hour rolling window price change statistics.
    /// Careful when accessing this when symbols are None.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use binancex::{
    ///     BinanceSpot,
    ///     utils::vec_strings_to_string,
    /// };
    ///
    /// let spot = BinanceSpot::new().unwrap();
    /// let symbols = vec![
    ///     "BTCUSDT".to_owned(),
    ///     "ETHUSDT".to_owned(),
    ///     "SOLETH".to_owned(),
    /// ];
    /// // return '["BTCUSDT","ETHUSDT","SOLETH"]'
    /// let symbols_param = vec_strings_to_string(&symbols);
    ///
    /// let reps = spot.get_24h_tickers_full(Some(&symbols_param)).await.unwrap();
    /// println!("{:?}", reps.payload);
    /// // or you can request statistics by all symbols
    /// let reps = spot.get_24h_tickers_full(None).await.unwrap();
    /// println!("{:?}", reps.payload);
    /// ```
    pub async fn get_24h_tickers_full(
        &self,
        symbols: Option<&str>,
    ) -> Result<Response<Vec<TickerStatsFull>>, BinanceError<Error>> {
        if let Some(val) = symbols {
            let query_len = val.len() + 32;
            return self
                .client
                .get_with_query::<_, _>(endpoints::API_V3_TICKER_24H, query_len, |url| {
                    url.add_param_str("type", "FULL");
                    url.add_param_str("symbols", val);
                })
                .await;
        }

        self.client
            .get_with_query::<_, _>(endpoints::API_V3_TICKER_24H, 10, |url| {
                url.add_param_str("type", "FULL");
            })
            .await
    }

    /// 24 hour rolling window price change statistics.
    /// Careful when accessing this when symbols are None.
    pub async fn get_24h_tickers_mini(
        &self,
        symbols: Option<&str>,
    ) -> Result<Response<Vec<TickerStatsMini>>, BinanceError<Error>> {
        if let Some(val) = symbols {
            let query_len = val.len() + 32;
            return self
                .client
                .get_with_query::<_, _>(endpoints::API_V3_TICKER_24H, query_len, |url| {
                    url.add_param_str("type", "MINI");
                    url.add_param_str("symbols", val);
                }).await;
        }

        self
            .client
            .get_with_query::<_, _>(endpoints::API_V3_TICKER_24H, 10, |url| {
                url.add_param_str("type", "MINI");
            })
            .await
    }

    /// Get latest price for a symbol.
    ///
    /// Weight(IP): 1
    pub async fn get_latest_price(
        &self,
        symbol: &str,
    ) -> Result<Response<SymbolPrice>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_TICKER_PRICE, 32, |url| {
                url.add_param_str("symbol", symbol);
            })
            .await
    }

    /// Get latest price for a symbols.
    ///
    /// Weight(IP): 2
    pub async fn get_latest_prices(
        &self,
        symbols: Option<&str>,
    ) -> Result<Response<Vec<SymbolPrice>>, BinanceError<Error>> {
        if let Some(val) = symbols {
            let query_len = val.len() + 16;
            return self
                .client
                .get_with_query::<_, _>(endpoints::API_V3_TICKER_PRICE, query_len, |url| {
                    url.add_param_str("symbols", val);
                })
                .await;
        }

        self.client
            .get::<_, _>(endpoints::API_V3_TICKER_PRICE)
            .await
    }

    /// Get best price/qty on the order book for a symbol.
    ///
    /// Weight(IP): 1
    pub async fn get_book_ticker(
        &self,
        symbol: &str,
    ) -> Result<Response<BookTicker>, BinanceError<Error>> {
        self.client
            .get_with_query::<_, _>(endpoints::API_V3_TICKER_BOOK_TICKER, 32, |url| {
                url.add_param_str("symbol", symbol);
            })
            .await
    }

    /// Get best price/qty on the order book for a symbols.
    ///
    /// Weight(IP): 2
    pub async fn get_book_tickers(
        &self,
        symbols: Option<&str>,
    ) -> Result<Response<Vec<BookTicker>>, BinanceError<Error>> {
        if let Some(val) = symbols {
            let query_len = val.len() + 16;
            return self
                .client
                .get_with_query::<_, _>(endpoints::API_V3_TICKER_BOOK_TICKER, query_len, |url| {
                    url.add_param_str("symbols", val);
                })
                .await;
        }

        self.client
            .get::<_, _>(endpoints::API_V3_TICKER_BOOK_TICKER)
            .await
    }
}
