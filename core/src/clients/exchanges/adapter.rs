use async_trait::async_trait;

use crate::{
    clients::{
        binance::BinanceFuturesAdapter, bitget::BitgetFuturesAdapter,
        hyperliquid::HyperliquidAdapter, okx::OkxFuturesAdapter,
    },
    error::AppError,
};

use super::types::{
    CancelOrderResponse, ExchangeBalance, ExchangeCredentials, ExchangeOpenOrder,
    ExchangeOrderDetail, ExchangePosition, ExchangeSymbolConstraints, ExchangeTradeFill,
    PlaceOrderRequest, PlaceOrderResponse,
};

#[async_trait]
pub trait LiveExchangeAdapter: Send + Sync {
    #[allow(dead_code)]
    fn exchange_type(&self) -> &'static str;
    async fn ping(&self) -> Result<(), AppError>;
    async fn get_price(&self, symbol: &str) -> Result<f64, AppError>;
    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, AppError>;
    async fn cancel_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<CancelOrderResponse, AppError>;
    async fn get_balances(&self) -> Result<Vec<ExchangeBalance>, AppError>;
    async fn get_positions(&self) -> Result<Vec<ExchangePosition>, AppError>;
    async fn get_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<ExchangeOpenOrder>, AppError>;
    async fn get_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<ExchangeOrderDetail, AppError>;
    async fn get_order_fills(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<Vec<ExchangeTradeFill>, AppError>;
    async fn get_symbol_constraints(
        &self,
        symbol: &str,
    ) -> Result<ExchangeSymbolConstraints, AppError>;
    async fn start_user_stream(&self) -> Result<String, AppError>;
    async fn keepalive_user_stream(&self, listen_key: &str) -> Result<(), AppError>;
    async fn close_user_stream(&self, listen_key: &str) -> Result<(), AppError>;
    fn user_stream_ws_url(&self, listen_key: &str) -> Result<String, AppError>;
}

pub fn create_exchange_adapter(
    exchange_type: &str,
    credentials: ExchangeCredentials,
) -> Result<Box<dyn LiveExchangeAdapter>, AppError> {
    match exchange_type.to_ascii_lowercase().as_str() {
        "binance" => Ok(Box::new(BinanceFuturesAdapter::new(credentials)?)),
        "aster" => Ok(Box::new(BinanceFuturesAdapter::new_aster(credentials)?)),
        "okx" => Ok(Box::new(OkxFuturesAdapter::new(credentials)?)),
        "bitget" => Ok(Box::new(BitgetFuturesAdapter::new(credentials)?)),
        "hyperliquid" => Ok(Box::new(HyperliquidAdapter::new(credentials)?)),
        other => Err(AppError::UnsupportedExchange(other.to_string())),
    }
}
