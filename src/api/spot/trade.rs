use crate::api::client::error::BinanceError;
use crate::api::client::response::Response;
use crate::api::spot::BinanceSpot;

use crate::schemes::error::{CancelReplaceOrderError, Error};
use crate::schemes::spot::trade::*;

pub mod prelude {
    pub use crate::schemes::spot::trade::{
        AccountTradesReq, CancelOrderReq, NewOrderReq, OrderSide, OrderTypeReq,
    };
}

#[rustfmt::skip]
pub mod endpoints {
    pub static API_V3_ORDER_TEST: &str           = "/api/v3/order/test";
    pub static API_V3_ORDER: &str                = "/api/v3/order";
    pub static API_V3_OPEN_ORDERS: &str          = "/api/v3/openOrders";
    pub static API_V3_ORDER_CANCEL_REPLACE: &str = "/api/v3/order/cancelReplace";
    pub static API_V3_ALL_ORDERS: &str           = "/api/v3/allOrders";
    pub static API_V3_ORDER_OCO: &str            = "/api/v3/order/oco";
    pub static API_V3_ORDER_LIST: &str           = "/api/v3/orderList";
    pub static API_V3_ALL_ORDER_LIST: &str       = "/api/v3/allOrderList";
    pub static API_V3_OPEN_ORDER_LIST: &str      = "/api/v3/openOrderList";
    pub static API_V3_ACCOUNT: &str              = "/api/v3/account";
    pub static API_V3_MY_TRADES: &str            = "/api/v3/myTrades";
    pub static API_V3_RATE_LIMIT_ORDER: &str     = "/api/v3/rateLimit/order";
}

impl BinanceSpot {
    /// Test new order creation and signature/recvWindow long.
    /// Creates and validates a new order but does not send it into the matching engine.
    ///
    /// Weight: 1
    ///
    pub async fn test_new_order(
        &self,
        req: &NewOrderReq,
    ) -> Result<Response<TestNewOrderRes>, BinanceError<Error>> {
        self.client
            .post_signed_with_query::<_, _>(endpoints::API_V3_ORDER_TEST, 128, |url| {
                url.add_params_from_data(req)?;
                Ok(())
            })
            .await
    }

    /// Send in a new order.
    ///
    /// Weight(UID): 1
    /// Weight(IP): 1
    ///
    pub async fn new_order(
        &self,
        req: &NewOrderReq,
    ) -> Result<Response<NewOrderRes>, BinanceError<Error>> {
        self.client
            .post_signed_with_query::<_, _>(endpoints::API_V3_ORDER, 128, |url| {
                url.add_params_from_data(req)?;
                Ok(())
            })
            .await
    }

    /// Cancel an active order.
    ///
    /// Weight(IP): 1
    ///
    /// Either orderId or origClientOrderId must be sent.
    /// If both orderId and origClientOrderId are provided, orderId takes precedence.
    pub async fn cancel_order(
        &self,
        req: &CancelOrderReq,
    ) -> Result<Response<CancelOrderRes>, BinanceError<Error>> {
        self.client
            .delete_signed_with_query::<_, _>(endpoints::API_V3_ORDER, 128, |url| {
                url.add_params_from_data(req)?;
                Ok(())
            })
            .await
    }

    /// Cancels all active orders on a symbol.
    /// This includes OCO orders.
    ///
    /// Weight(IP): 1
    ///
    pub async fn cancel_all_orders(
        &self,
        symbol: &str,
    ) -> Result<Response<Vec<CancelOrderRes>>, BinanceError<Error>> {
        self.client
            .delete_signed_with_query::<_, _>(endpoints::API_V3_OPEN_ORDERS, 32, |url| {
                url.add_param_str("symbol", symbol);
                Ok(())
            })
            .await
    }

    /// Check an order's status.
    ///
    /// Weight(IP): 2
    ///
    /// Either orderId or origClientOrderId must be sent.
    /// For some historical orders cummulativeQuoteQty will be < 0, meaning the data is not available at this time.
    ///
    pub async fn get_order_info(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        orig_client_order_id: Option<String>,
    ) -> Result<Response<OrderInfo>, BinanceError<Error>> {
        self.client
            .get_signed_with_query::<_, _>(endpoints::API_V3_ORDER, 128, |url| {
                url.add_param_str("symbol", symbol);
                if let Some(id) = order_id {
                    url.add_param_integer("orderId", id);
                }
                if let Some(id) = orig_client_order_id {
                    url.add_param_str("origClientOrderId", &id);
                }
                Ok(())
            })
            .await
    }

    /// Cancels an existing order and places a new order on the same symbol.
    ///
    /// Filters and Order Count are evaluated before the processing of the cancellation and order placement occurs.
    /// A new order that was not attempted (i.e. when newOrderResult: NOT_ATTEMPTED), will still increase the order count by 1.
    ///
    /// Weight(IP): 1
    ///
    pub async fn cancel_replace_order(
        &self,
        req: &CancelReplaceOrderReq,
    ) -> Result<Response<CancelReplaceOrderRes>, BinanceError<CancelReplaceOrderError>> {
        self.client
            .post_signed_with_query::<_, _>(endpoints::API_V3_ORDER_CANCEL_REPLACE, 128, |url| {
                url.add_params_from_data(&req)?;
                Ok(())
            })
            .await
    }

    /// Get all open orders on a symbol.
    ///
    /// Weight(IP): 3
    ///
    pub async fn get_open_orders(
        &self,
        symbol: &str,
    ) -> Result<Response<Vec<OrderInfo>>, BinanceError<Error>> {
        self.client
            .get_signed_with_query::<_, _>(endpoints::API_V3_OPEN_ORDERS, 32, |url| {
                url.add_param_str("symbol", symbol);
                Ok(())
            })
            .await
    }

    /// Get all open orders on a symbols.
    ///
    /// Weight(IP): 40
    ///
    pub async fn get_all_open_orders(
        &self,
    ) -> Result<Response<Vec<OrderInfo>>, BinanceError<Error>> {
        self.client
            .get_signed::<_, _>(endpoints::API_V3_OPEN_ORDERS)
            .await
    }

    /// Send in a new OCO
    ///
    /// Weight(UID): 2
    /// Weight(IP): 1
    pub async fn new_oco_order(
        &self,
        req: &NewOcoOrderReq,
    ) -> Result<Response<OcoOrderRes>, BinanceError<Error>> {
        self.client
            .post_signed_with_query::<_, _>(endpoints::API_V3_ORDER_OCO, 256, |url| {
                url.add_params_from_data(req)?;
                Ok(())
            })
            .await
    }

    /// Cancel an entire Order List.
    ///
    /// Weight(IP): 1
    ///
    /// Additional notes:
    /// - Canceling an individual leg will cancel the entire OCO
    /// - If both orderListId and listClientOrderID are provided, orderId takes precedence.
    pub async fn cancel_oco_order(
        &self,
        symbol: &str,
        order_list_id: Option<u64>,
        list_client_order_id: Option<String>,
        new_client_order_id: Option<String>,
    ) -> Result<Response<OcoOrderRes>, BinanceError<Error>> {
        self.client
            .delete_signed_with_query::<_, _>(endpoints::API_V3_ORDER_LIST, 256, |url| {
                url.add_param_str("symbol", symbol);
                if let Some(id) = order_list_id {
                    url.add_param_integer("orderListId", id);
                }
                if let Some(ref id) = list_client_order_id {
                    url.add_param_str("listClientOrderId", id);
                }
                if let Some(ref id) = new_client_order_id {
                    url.add_param_str("newClientOrderId", id);
                }
                Ok(())
            })
            .await
    }

    /// Retrieves a specific OCO based on provided optional parameters.
    ///
    /// Weight(IP): 2
    pub async fn get_oco_order(
        &self,
        id: &OrderIdOrClientOrderId,
    ) -> Result<Response<OcoOrderRes>, BinanceError<Error>> {
        self.client
            .get_signed_with_query::<_, _>(endpoints::API_V3_ORDER_LIST, 256, |url| {
                match id {
                    OrderIdOrClientOrderId::OrderListId(id) => {
                        url.add_param_integer("orderListId", *id)
                    }
                    OrderIdOrClientOrderId::OrigClientOrderId(id) => {
                        url.add_param_str("origClientOrderId", id);
                    }
                }
                Ok(())
            })
            .await
    }

    /// Retrieves all OCO based on provided optional parameters
    ///
    /// Weight(IP): 10
    pub async fn get_all_oco_orders(
        &self,
        from_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<OcoOrderRes>>, BinanceError<Error>> {
        self.client
            .get_signed_with_query::<_, _>(endpoints::API_V3_ALL_ORDER_LIST, 256, |url| {
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
                Ok(())
            })
            .await
    }

    /// Retrieves all OPEN OCO.
    ///
    /// Weight(IP): 3
    pub async fn get_all_open_oco_orders(
        &self,
    ) -> Result<Response<Vec<OcoOrderRes>>, BinanceError<Error>> {
        self.client
            .get_signed::<_, _>(endpoints::API_V3_OPEN_ORDER_LIST)
            .await
    }

    /// Get all account orders; active, canceled, or filled.
    ///
    /// Weight(IP): 10
    ///
    /// - If orderId is set, it will get orders >= that orderId. Otherwise most recent orders are returned.
    /// - For some historical orders cummulativeQuoteQty will be < 0, meaning the data is not available at this time.
    /// - If startTime and/or endTime provided, orderId is not required.
    pub async fn get_all_account_orders(
        &self,
        symbol: &str,
        order_id: Option<u64>,
        start_time: Option<u64>,
        end_time: Option<u64>,
        limit: Option<u16>,
    ) -> Result<Response<Vec<OrderInfo>>, BinanceError<Error>> {
        self.client
            .get_signed_with_query::<_, _>(endpoints::API_V3_ALL_ORDERS, 128, |url| {
                url.add_param_str("symbol", symbol);
                if let Some(id) = order_id {
                    url.add_param_integer("orderId", id);
                }
                if let Some(val) = start_time {
                    url.add_param_integer("startTime", val);
                }
                if let Some(val) = end_time {
                    url.add_param_integer("endTime", val);
                }
                if let Some(val) = limit {
                    url.add_param_integer("limit", val);
                }
                Ok(())
            })
            .await
    }

    /// Get current account information.
    ///
    /// Weight(IP): 10
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use binancex::BinanceSpot;
    ///
    /// let spot = BinanceSpot::signed("api_key", "secret_key").unwrap();
    /// let reps = spot.get_account_info().await.unwrap();
    /// println!("{:?}", reps.payload);
    /// ```
    pub async fn get_account_info(&self) -> Result<Response<AccountInfo>, BinanceError<Error>> {
        self.client
            .get_signed::<_, _>(endpoints::API_V3_ACCOUNT)
            .await
    }

    /// Get trades for a specific account and symbol.
    ///
    /// Weight(IP): 10
    ///
    /// If fromId is set, it will get id >= that fromId. Otherwise most recent trades are returned.
    /// The time between startTime and endTime can't be longer than 24 hours.
    ///
    pub async fn get_account_trades(
        &self,
        req: &AccountTradesReq,
    ) -> Result<Response<Vec<AccountTrade>>, BinanceError<Error>> {
        self.client
            .get_signed_with_query::<_, _>(endpoints::API_V3_MY_TRADES, 128, |url| {
                url.add_params_from_data(&req)?;
                Ok(())
            })
            .await
    }

    /// Displays the user's current order count usage for all intervals.
    ///
    /// Weight(IP): 20
    ///
    pub async fn get_order_rate_limit(
        &self,
    ) -> Result<Response<Vec<OrderRateLimit>>, BinanceError<Error>> {
        self.client
            .get::<_, _>(endpoints::API_V3_RATE_LIMIT_ORDER)
            .await
    }
}
