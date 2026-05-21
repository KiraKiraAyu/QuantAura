pub use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};

pub use crate::clients::binance::*;
pub use futures_util::StreamExt;
pub use serde_json::{Value, json};
pub use tokio::{
    sync::{Mutex, mpsc, watch},
    task::JoinHandle,
    time::{self, MissedTickBehavior},
};
pub use tracing::{error, info, warn};
pub use uuid::Uuid;

pub use crate::{
    clients::exchanges::{
        ExchangeCredentials, ExchangeOrderType, ExchangeSide, ExchangeSymbolConstraints,
        LiveExchangeAdapter, PlaceOrderRequest, PositionSide, TimeInForce, create_exchange_adapter,
    },
    error::AppError,
    realtime::RealtimeHub,
    repositories::{ExchangeRepo, TradingRepo, trading::records::positions::TraderPositionRecord},
    runtime_events::{
        EVENT_CANCEL_REPLACE_SUCCEEDED, EVENT_CANCEL_REPLACE_THROTTLED,
        EVENT_CANCEL_REPLACE_USED_MARKET_FALLBACK, EVENT_LIVE_OPEN_SKIPPED_MEDIUM_RISK,
        EVENT_LIVE_OPEN_USED_MARKET_FALLBACK, EVENT_LIVE_ORDER_SUBMITTED, EVENT_LIVE_RISK_SNAPSHOT,
        EVENT_STALE_INTENT_RECONCILE_PENDING, EVENT_STALE_INTENT_RECONCILE_TERMINAL,
    },
    services::llm::LlmMessage,
    state::{RuntimeEngineManager, RuntimeEngineState},
};

pub use super::{
    account_sim::*, ai_decision::*, binance_events::*, config_loaders::*, db_utils::*, engine::*,
    events::*, execution_live::*, execution_live_limit::*, execution_sim::*, market::*,
    market_seed::*, models::*,
};

#[derive(Debug)]
pub struct EngineWorker {
    stop_tx: watch::Sender<bool>,
    handle: JoinHandle<()>,
}

#[derive(Debug)]
pub struct EngineInner {
    pub state: SharedState,
    pub(crate) workers: Mutex<HashMap<String, EngineWorker>>,
}

#[derive(Clone, Debug)]
pub struct TradingRuntimeService {
    pub inner: Arc<EngineInner>,
}

impl TradingRuntimeService {
    pub fn new(
        trading_repo: Arc<TradingRepo>,
        exchange_repo: Arc<ExchangeRepo>,
        live: crate::config::LiveRuntimeConfig,
        runtime_engine_manager: Arc<RwLock<RuntimeEngineManager>>,
        llm_service: Arc<crate::services::llm::LlmService>,
        realtime_hub: RealtimeHub,
    ) -> Self {
        Self {
            inner: Arc::new(EngineInner {
                state: RuntimeState::new(
                    trading_repo,
                    exchange_repo,
                    live,
                    runtime_engine_manager,
                    llm_service,
                    realtime_hub,
                ),
                workers: Mutex::new(HashMap::new()),
            }),
        }
    }

    fn state(&self) -> SharedState {
        self.inner.state.clone()
    }

    pub async fn recover_running_traders_from_db(&self) -> Result<Vec<String>, AppError> {
        let state = self.state();
        let rows = state.trading_repo.running_traders().await?;

        let mut recovered = Vec::new();

        for (user_id, trader_id) in rows {
            match self.start_trader(&user_id, &trader_id).await {
                Ok(_) => {
                    info!(
                        "startup recovery resumed trader={} user={}",
                        trader_id, user_id
                    );
                    recovered.push(trader_id);
                }
                Err(AppError::AlreadyRunning(_)) => {
                    recovered.push(trader_id);
                }
                Err(err) => {
                    warn!(
                        "startup recovery failed trader={} user={} err={}",
                        trader_id, user_id, err
                    );

                    if let Err(db_err) =
                        set_trader_running(&state, &trader_id, &user_id, false).await
                    {
                        warn!(
                            "startup recovery failed to reset is_running trader={} user={} err={}",
                            trader_id, user_id, db_err
                        );
                    }

                    let _ = state.set_runtime_engine_running(
                        &trader_id,
                        false,
                        Some(format!("startup recovery failed: {}", err)),
                    );
                }
            }
        }

        Ok(recovered)
    }

    pub async fn start_trader(&self, user_id: &str, trader_id: &str) -> Result<(), AppError> {
        let state = self.state();
        let cfg = load_trader_runtime_config(&state, user_id, trader_id)
            .await?
            .ok_or_else(|| AppError::TraderNotFound(trader_id.to_string()))?;

        if cfg.scan_interval_minutes <= 0 {
            return Err(AppError::InvalidConfig(
                "scan_interval_minutes must be > 0".to_string(),
            ));
        }

        {
            let workers = self.inner.workers.lock().await;
            if workers.contains_key(trader_id) {
                return Err(AppError::AlreadyRunning(trader_id.to_string()));
            }
        }

        let now = now_u64();
        state.upsert_runtime_engine(RuntimeEngineState {
            trader_id: cfg.trader_id.clone(),
            user_id: cfg.user_id.clone(),
            exchange_id: cfg.exchange_id.clone(),
            ai_model_id: cfg.ai_model_id.clone(),
            started_at: now,
            updated_at: now,
            is_running: true,
            last_error: None,
        })?;

        set_trader_running(&state, &cfg.trader_id, &cfg.user_id, true).await?;

        let (stop_tx, stop_rx) = watch::channel(false);
        let engine = self.clone();
        let cfg_for_task = cfg.clone();
        let handle = tokio::spawn(async move {
            if let Err(err) = run_trader_loop(engine, cfg_for_task, stop_rx).await {
                error!("runtime loop failed: {err}");
            }
        });

        let mut workers = self.inner.workers.lock().await;
        workers.insert(cfg.trader_id.clone(), EngineWorker { stop_tx, handle });

        info!(
            "runtime engine started for trader={} user={} exchange={} model={}",
            cfg.trader_id, cfg.user_id, cfg.exchange_id, cfg.ai_model_id
        );

        // Notify connected realtime clients that this trader started
        state
            .realtime_hub
            .publish(crate::realtime::RealtimeEvent::EngineStatus {
                user_id: cfg.user_id.clone(),
                trader_id: cfg.trader_id.clone(),
                status: "running".to_string(),
                message: format!(
                    "engine started (exchange={}, model={})",
                    cfg.exchange_id, cfg.ai_model_id
                ),
            });

        Ok(())
    }

    pub async fn stop_trader_for_user(
        &self,
        user_id: &str,
        trader_id: &str,
    ) -> Result<(), AppError> {
        let state = self.state();
        if state
            .trading_repo
            .get_trader(user_id, trader_id)
            .await?
            .is_none()
        {
            return Err(AppError::TraderNotFound(trader_id.to_string()));
        }

        self.stop_trader(trader_id).await
    }

    pub async fn stop_trader(&self, trader_id: &str) -> Result<(), AppError> {
        let state = self.state();
        let worker = {
            let mut workers = self.inner.workers.lock().await;
            workers.remove(trader_id)
        };

        let Some(worker) = worker else {
            return Err(AppError::NotRunning(trader_id.to_string()));
        };

        let _ = worker.stop_tx.send(true);
        worker.handle.await?;

        if let Err(err) = state.set_runtime_engine_running(trader_id, false, None) {
            warn!("set_runtime_engine_running(false) failed: {err}");
        }

        // Notify realtime clients that the trader stopped
        // (We don't have user_id here, so we look it up from the runtime manager)
        let user_id = state
            .runtime_engine(trader_id)
            .ok()
            .flatten()
            .map(|s| s.user_id)
            .unwrap_or_default();
        if !user_id.is_empty() {
            state
                .realtime_hub
                .publish(crate::realtime::RealtimeEvent::EngineStatus {
                    user_id,
                    trader_id: trader_id.to_string(),
                    status: "stopped".to_string(),
                    message: "engine stopped".to_string(),
                });
        }

        Ok(())
    }

    pub async fn shutdown_all(&self) -> Result<(), AppError> {
        let ids = {
            let workers = self.inner.workers.lock().await;
            workers.keys().cloned().collect::<Vec<_>>()
        };

        for id in ids {
            if let Err(err) = self.stop_trader(&id).await {
                warn!("failed to stop trader={} during shutdown: {}", id, err);
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn is_running(&self, trader_id: &str) -> bool {
        let workers = self.inner.workers.lock().await;
        workers.contains_key(trader_id)
    }

    #[allow(dead_code)]
    pub async fn running_traders(&self) -> Vec<String> {
        let workers = self.inner.workers.lock().await;
        workers.keys().cloned().collect()
    }
}

pub fn patch_json_payload(raw: &str, updates: &[(&str, Value)]) -> String {
    let mut payload = match serde_json::from_str::<Value>(raw) {
        Ok(Value::Object(map)) => map,
        _ => serde_json::Map::new(),
    };

    for (key, value) in updates {
        payload.insert((*key).to_string(), value.clone());
    }

    Value::Object(payload).to_string()
}

pub fn position_view_from_record(record: TraderPositionRecord) -> PositionView {
    PositionView {
        id: record.id,
        symbol: record.symbol,
        side: record.side,
        quantity: record.quantity,
        entry_price: record.entry_price,
        mark_price: record.mark_price,
        leverage: record.leverage as i64,
        opened_at: record.opened_at,
    }
}
