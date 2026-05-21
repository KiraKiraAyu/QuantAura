use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock},
};

pub use rust_decimal::Decimal;
pub use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
pub use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Set,
};

pub use crate::entity::{
    execution_intents, order_fills, trader_decisions, trader_orders, trader_positions,
    trader_trades, traders,
};
use crate::{
    realtime::RealtimeHub,
    repositories::{ExchangeRepo, TradingRepo},
    services::{
        llm::LlmService,
        trading_runtime::{
            models::RuntimeState,
            service::{EngineInner, TradingRuntimeService},
        },
    },
    state::RuntimeEngineManager,
};

pub struct TestRuntimeState {
    pub state: RuntimeState,
    pub db: DatabaseConnection,
}

impl TestRuntimeState {
    pub fn new(
        db: DatabaseConnection,
        live: crate::config::LiveRuntimeConfig,
        runtime_engine_manager: Arc<RwLock<RuntimeEngineManager>>,
        llm_service: Arc<LlmService>,
        realtime_hub: RealtimeHub,
    ) -> Self {
        Self {
            state: RuntimeState::new(
                Arc::new(TradingRepo::new(db.clone())),
                Arc::new(ExchangeRepo::new(db.clone())),
                live,
                runtime_engine_manager,
                llm_service,
                realtime_hub,
            ),
            db,
        }
    }
}

impl Deref for TestRuntimeState {
    type Target = RuntimeState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl TradingRuntimeService {
    pub fn new_for_test(
        db: DatabaseConnection,
        live: crate::config::LiveRuntimeConfig,
        runtime_engine_manager: Arc<RwLock<RuntimeEngineManager>>,
        llm_service: Arc<LlmService>,
        realtime_hub: RealtimeHub,
    ) -> Self {
        Self {
            inner: Arc::new(EngineInner {
                state: TestRuntimeState::new(
                    db,
                    live,
                    runtime_engine_manager,
                    llm_service,
                    realtime_hub,
                )
                .state,
                workers: tokio::sync::Mutex::new(HashMap::new()),
            }),
        }
    }
}

pub fn decimal_from_f64(value: f64) -> Decimal {
    Decimal::from_f64(value).unwrap_or(Decimal::ZERO)
}

pub fn decimal_to_f64(value: &Decimal) -> f64 {
    value.to_f64().unwrap_or(0.0)
}

pub fn ts_to_i32(ts: i64) -> sea_orm::entity::prelude::DateTimeWithTimeZone {
    crate::time::ts_to_dt(ts)
}

pub fn int_flag(value: bool) -> i32 {
    if value { 1 } else { 0 }
}
