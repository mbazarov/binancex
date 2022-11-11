#[cfg(test)]
mod tests {
    use binancex::{spot::market::endpoints::*, utils::vec_strings_to_string, BinanceSpot};
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_spot_general_ping() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET).path(API_V3_PING);

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body("{}");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.ping().await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_general_get_server_time() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET).path(API_V3_TIME);

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .json_body(serde_json::json!({
                        "serverTime": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    }));
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_server_time().await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_general_get_exchange_info() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET).path(API_V3_EXCHANGE_INFO);

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "10")
                    .header("x-mbx-used-weight-1m", "10")
                    .body_from_file("tests/api_dataset/spot/market/exchangeInfo_2022-09-28.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_exchange_info().await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_depth() {
        use binancex::spot::market::prelude::DepthLimit;

        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_DEPTH)
                    .query_param_exists("symbol")
                    .query_param_exists("limit")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("limit", "5");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/depth_2022-03-25.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client
            .get_depth("BTCUSDT", DepthLimit::Limit(5))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_recent_trades() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TRADES)
                    .query_param_exists("symbol")
                    .query_param_exists("limit")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("limit", "2");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/trades_2022-10-06.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_recent_trades("BTCUSDT", Some(2)).await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_historical_trades() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_HISTORICAL_TRADES)
                    .header_exists("x-mbx-apikey")
                    .header(
                        "x-mbx-apikey",
                        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                    )
                    .query_param_exists("symbol")
                    .query_param_exists("limit")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("limit", "2");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "5")
                    .header("x-mbx-used-weight-1m", "5")
                    .body_from_file(
                        "tests/api_dataset/spot/market/historicalTrades_2022-11-11.json",
                    );
            })
            .await;

        let client = BinanceSpot::signed_with_host(
            &server.url(""),
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        )
        .unwrap();
        let _response = client
            .get_historical_trades("BTCUSDT", None, Some(2))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_aggregate_trades() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_AGG_TRADES)
                    .query_param_exists("symbol")
                    .query_param_exists("limit")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("limit", "2");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/aggTrades_2022-10-06.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client
            .get_aggregate_trades("BTCUSDT", None, None, None, Some(2))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_klines() {
        use binancex::spot::market::prelude::KLinesInterval;

        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_KLINES)
                    .query_param_exists("interval")
                    .query_param_exists("interval")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("interval", "1m");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/klines_2022-10-06.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client
            .get_klines("BTCUSDT", KLinesInterval::Minute, None, None, Some(2))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_klines_ui() {
        use binancex::types::spot::limits::KLinesInterval;

        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_UI_KLINES)
                    .query_param_exists("symbol")
                    .query_param_exists("interval")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("interval", "1m");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/klines_2022-10-06.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client
            .get_klines_ui("BTCUSDT", KLinesInterval::Minute, None, None, Some(2))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_average_price() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_AVG_PRICE)
                    .query_param_exists("symbol")
                    .query_param("symbol", "BTCUSDT");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/avgPrice_2022-10-06.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_average_price("BTCUSDT").await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_24h_ticker_full() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_24H)
                    .query_param_exists("symbol")
                    .query_param_exists("type")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("type", "FULL");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_24hr_full_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_24h_ticker_full("BTCUSDT").await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_24h_ticker_mini() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_24H)
                    .query_param_exists("symbol")
                    .query_param_exists("type")
                    .query_param("symbol", "BTCUSDT")
                    .query_param("type", "MINI");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_24hr_mini_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_24h_ticker_mini("BTCUSDT").await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_24h_tickers_full() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_24H)
                    .query_param_exists("symbols")
                    .query_param_exists("type")
                    .query_param("symbols", format!("[\"BTCUSDT\",\"BNBBTC\"]"))
                    .query_param("type", "FULL");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_24hr_full_symbols_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let symbols = vec!["BTCUSDT".to_owned(), "BNBBTC".to_owned()];
        let symbols_param = vec_strings_to_string(&symbols);
        let _response = client
            .get_24h_tickers_full(Some(&symbols_param))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_24h_tickers_mini() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_24H)
                    .query_param_exists("symbols")
                    .query_param_exists("type")
                    .query_param("symbols", format!("[\"BTCUSDT\",\"BNBBTC\"]"))
                    .query_param("type", "MINI");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_24hr_mini_symbols_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let symbols = vec!["BTCUSDT".to_owned(), "BNBBTC".to_owned()];
        let symbols_param = vec_strings_to_string(&symbols);
        let _response = client
            .get_24h_tickers_mini(Some(&symbols_param))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_latest_price() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_PRICE)
                    .query_param_exists("symbol")
                    .query_param("symbol", "BTCUSDT");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file("tests/api_dataset/spot/market/ticker_price_2022-10-06.json");
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_latest_price("BTCUSDT").await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_latest_prices() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_PRICE)
                    .query_param_exists("symbols")
                    .query_param("symbols", format!("[\"BTCUSDT\",\"BNBBTC\"]"));

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "2")
                    .header("x-mbx-used-weight-1m", "2")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_price_symbols_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let symbols = vec!["BTCUSDT".to_owned(), "BNBBTC".to_owned()];
        let symbols_param = vec_strings_to_string(&symbols);
        let _response = client
            .get_latest_prices(Some(&symbols_param))
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_book_ticker() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_BOOK_TICKER)
                    .query_param_exists("symbol")
                    .query_param("symbol", "BTCUSDT");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_bookTicker_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let _response = client.get_book_ticker("BTCUSDT").await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_spot_market_get_book_tickers() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(API_V3_TICKER_BOOK_TICKER)
                    .query_param_exists("symbols")
                    .query_param("symbols", format!("[\"BTCUSDT\",\"BNBBTC\"]"));

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "2")
                    .header("x-mbx-used-weight-1m", "2")
                    .body_from_file(
                        "tests/api_dataset/spot/market/ticker_bookTicker_symbols_2022-10-06.json",
                    );
            })
            .await;

        let client = BinanceSpot::with_host(&server.url("")).unwrap();
        let symbols = vec!["BTCUSDT".to_owned(), "BNBBTC".to_owned()];
        let symbols_param = vec_strings_to_string(&symbols);
        let _response = client.get_book_tickers(Some(&symbols_param)).await.unwrap();

        binance_mock.assert_async().await;
    }
}
