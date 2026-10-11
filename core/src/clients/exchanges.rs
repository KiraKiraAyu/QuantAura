pub mod adapter;
pub mod types;
pub mod user_stream;

pub use adapter::{LiveExchangeAdapter, create_exchange_adapter};

// 显式重导出：既保留 `clients::exchanges::X` 的既有调用方式，
// 又让依赖图可静态分析（glob 重导出会把真实依赖边抹掉）。
pub use types::{
    CancelOrderResponse, ExchangeBalance, ExchangeConditionalOrderType, ExchangeCredentials,
    ExchangeMarginMode, ExchangeOpenOrder, ExchangeOrderDetail, ExchangeOrderType,
    ExchangePosition, ExchangeSide, ExchangeSymbolConstraints, ExchangeTradeFill,
    PlaceConditionalOrderRequest, PlaceOrderRequest, PlaceOrderResponse, PositionSide, TimeInForce,
};
pub use user_stream::{
    ExchangeAccountBalanceUpdate, ExchangeAccountPositionUpdate, ExchangeAccountStreamUpdate,
    ExchangeOrderStreamUpdate, ExchangeUserStreamEvent, ExchangeUserStreamSession,
    spawn_exchange_user_stream_reader,
};
