#[cfg(test)]
mod tests {
    use binancex::delivery_futures::market::endpoints::*;
    use binancex::BinanceDeliveryFutures;
    use httpmock::prelude::*;

    #[tokio::test]
    async fn test_futures_dapi_general_ping() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET).path(DAPI_V1_PING);

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body("{}");
            })
            .await;

        let client = BinanceDeliveryFutures::with_host(&server.url("")).unwrap();
        let _response = client.ping().await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_futures_dapi_general_get_server_time() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET).path(DAPI_V1_TIME);

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .json_body(serde_json::json!({
                        "serverTime": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                    }));
            })
            .await;

        let client = BinanceDeliveryFutures::with_host(&server.url("")).unwrap();
        let _response = client.get_server_time().await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_futures_dapi_general_get_exchange_info() {
        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET).path(DAPI_V1_EXCHANGE_INFO);

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/delivery_futures/market/exchangeInfo_2021-08-15.json",
                    );
            })
            .await;

        let client = BinanceDeliveryFutures::with_host(&server.url("")).unwrap();
        let _response = client.get_exchange_info().await.unwrap();

        binance_mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_futures_dapi_market_get_depth() {
        use binancex::delivery_futures::market::prelude::DepthLimit;

        let server = MockServer::start_async().await;

        let binance_mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(DAPI_V1_DEPTH)
                    .query_param_exists("symbol")
                    .query_param_exists("limit")
                    .query_param("symbol", "BTCUSDT_PERP")
                    .query_param("limit", "5");

                then.status(200)
                    .header("content-type", "application/json; charset=UTF-8")
                    .header("x-mbx-used-weight", "1")
                    .header("x-mbx-used-weight-1m", "1")
                    .body_from_file(
                        "tests/api_dataset/delivery_futures/market/depth_2021-08-15.json",
                    );
            })
            .await;

        let client = BinanceDeliveryFutures::with_host(&server.url("")).unwrap();
        let _response = client
            .get_depth("BTCUSDT_PERP", DepthLimit::Limit5)
            .await
            .unwrap();

        binance_mock.assert_async().await;
    }
}
