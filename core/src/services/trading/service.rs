pub use hmac::{Hmac, Mac};
pub use reqwest::Client;
pub use serde_json::{Value, json};
pub use sha2::Sha256;
pub use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};
pub use tokio::time::sleep;
pub use tracing::warn;
pub use uuid::Uuid;

pub type HmacSha256 = Hmac<Sha256>;

pub use crate::{
    clients::{
        binance::normalize_order_quantity_by_constraints,
        exchanges::{
            ExchangeCredentials, ExchangeOrderType, ExchangeSide, LiveExchangeAdapter,
            PlaceOrderRequest, PositionSide, create_exchange_adapter,
        },
    },
    contracts::trading::{
        accounts::{
            ClosePositionPayload, ClosePositionRequest, GridRiskInfoPayload,
            SymbolConcentrationPayload, TraderAccountPayload, TraderBalanceSyncPayload,
        },
        alerts::{
            RuntimeAlertAckPayload, RuntimeAlertAckRequest, RuntimeAlertControlTargetRequest,
            RuntimeAlertControlsPayload, RuntimeAlertControlsQuery,
            RuntimeAlertDeliveriesFiltersPayload, RuntimeAlertDeliveriesPayload,
            RuntimeAlertDeliveriesQuery, RuntimeAlertDeliveryLogPayload,
            RuntimeAlertHistoryFiltersPayload, RuntimeAlertHistoryItemPayload,
            RuntimeAlertHistoryPayload, RuntimeAlertHistoryQuery, RuntimeAlertItemPayload,
            RuntimeAlertMutePayload, RuntimeAlertMuteRequest, RuntimeAlertNotificationPayload,
            RuntimeAlertRatesPayload, RuntimeAlertStatePayload, RuntimeAlertThresholdsPayload,
            RuntimeAlertTotalsPayload, RuntimeAlertsPayload, RuntimeAlertsQuery,
        },
        common::{PaginationQuery, TraderQuery},
        history::{
            DecisionListPayload, DecisionPayload, DecisionQuery, LatestDecisionsPayload,
            StatisticsQuery, TradeListPayload, TradePayload, TraderStatisticsPayload,
        },
        orders::{FillListPayload, FillPayload, OrderListPayload, OrderPayload},
        positions::{PositionListPayload, PositionPayload, PositionQuery},
        runtime_observability::{
            RiskLevelCountPayload, RuntimeEventPayload, RuntimeEventTypePayload,
            RuntimeEventTypesPayload, RuntimeEventTypesQuery, RuntimeEventsFilterPayload,
            RuntimeEventsPayload, RuntimeEventsQuery, RuntimeMetricRatesPayload,
            RuntimeMetricTotalsPayload, RuntimeMetricsPayload, RuntimeMetricsQuery,
            RuntimeMetricsSeriesBucketPayload, RuntimeMetricsSeriesPayload,
            RuntimeMetricsSeriesQuery,
        },
        traders::{
            CreateTraderRequest, RuntimeEnginePayload, ToggleCompetitionRequest,
            TraderCreatedPayload, TraderListPayload, TraderMessagePayload, TraderPayload,
            TraderStatusPayload, UpdatePromptRequest, UpdateTraderRequest,
        },
    },
    error::{AppError, AppErrorKind, Result as AppResult},
    repositories::trading::records::{
        accounts::TraderAccountRecord,
        alerts::{
            InsertRuntimeAlertDeliveryRecord, InsertRuntimeAlertHistoryRecord,
            RuntimeAlertControlsRecord, RuntimeAlertDeliveryRecord, RuntimeAlertHistoryRecord,
        },
        history::{TraderDecisionRecord, TraderTradeRecord},
        orders::{OrderFillRecord, TraderOrderRecord},
        positions::TraderPositionRecord,
        runtime_observability::RuntimeEventRecord,
        traders::{CreateTraderRecord, TraderRecord, UpdateTraderRecord},
    },
    runtime_events::{
        EVENT_CANCEL_REPLACE_SUCCEEDED, EVENT_CANCEL_REPLACE_THROTTLED,
        EVENT_CANCEL_REPLACE_USED_MARKET_FALLBACK, EVENT_LIVE_OPEN_SKIPPED_MEDIUM_RISK,
        EVENT_LIVE_OPEN_USED_MARKET_FALLBACK, EVENT_LIVE_ORDER_SUBMITTED, EVENT_LIVE_RISK_SNAPSHOT,
        EVENT_STALE_INTENT_RECONCILE_PENDING, EVENT_STALE_INTENT_RECONCILE_TERMINAL,
        canonical_runtime_event_types,
    },
    services::trading_runtime::{
        config_loaders::exchange_credentials_missing, execution_live::persist_live_order_record,
        models::TraderRuntimeConfig, service::TradingRuntimeService,
    },
    state::{RuntimeEngineManager, RuntimeEngineState},
};

pub use super::{
    account::*, alerts::*, events_util::*, history::*, lifecycle::*, metrics::*, models::*,
    statistics::*, utils::*,
};

#[derive(Debug, Clone)]
pub struct TradingService {
    state: SharedState,
    trading_runtime_service: Arc<TradingRuntimeService>,
}

impl TradingService {
    pub fn new(
        trading_repo: Arc<crate::repositories::TradingRepo>,
        exchange_repo: Arc<crate::repositories::ExchangeRepo>,
        runtime_alerts: crate::config::RuntimeAlertConfig,
        runtime_engine_manager: Arc<RwLock<RuntimeEngineManager>>,
        llm_service: Arc<crate::services::llm::LlmService>,
        trading_runtime_service: Arc<TradingRuntimeService>,
    ) -> Self {
        Self {
            state: TradingState::new(
                trading_repo,
                exchange_repo,
                runtime_alerts,
                runtime_engine_manager,
                llm_service,
            ),
            trading_runtime_service,
        }
    }

    fn state(&self) -> SharedState {
        self.state.clone()
    }

    pub async fn list_traders(&self, user_id: &str) -> AppResult<TraderListPayload> {
        let state = self.state();
        list_traders(&state, user_id).await
    }

    pub async fn get_trader(&self, user_id: &str, id: &str) -> AppResult<TraderPayload> {
        let state = self.state();
        get_trader(&state, user_id, id).await
    }

    pub async fn get_trader_config(&self, user_id: &str, id: &str) -> AppResult<TraderPayload> {
        let state = self.state();
        get_trader_config(&state, user_id, id).await
    }

    pub async fn create_trader(
        &self,
        user_id: &str,
        req: CreateTraderRequest,
    ) -> AppResult<TraderCreatedPayload> {
        let state = self.state();
        create_trader(&state, user_id, req).await
    }

    pub async fn update_trader(
        &self,
        user_id: &str,
        id: &str,
        req: UpdateTraderRequest,
    ) -> AppResult<TraderMessagePayload> {
        let state = self.state();
        update_trader(&state, user_id, id, req).await
    }

    pub async fn delete_trader(&self, user_id: &str, id: &str) -> AppResult<TraderMessagePayload> {
        let state = self.state();
        delete_trader(&state, self.trading_runtime_service.as_ref(), user_id, id).await
    }

    pub async fn start_trader(&self, user_id: &str, id: &str) -> AppResult<TraderMessagePayload> {
        start_trader(self.trading_runtime_service.as_ref(), user_id, id).await
    }

    pub async fn stop_trader(&self, user_id: &str, id: &str) -> AppResult<TraderMessagePayload> {
        stop_trader(self.trading_runtime_service.as_ref(), user_id, id).await
    }

    pub async fn update_trader_prompt(
        &self,
        user_id: &str,
        id: &str,
        req: UpdatePromptRequest,
    ) -> AppResult<TraderMessagePayload> {
        let state = self.state();
        update_trader_prompt(&state, user_id, id, req).await
    }

    pub async fn toggle_competition(
        &self,
        user_id: &str,
        id: &str,
        req: ToggleCompetitionRequest,
    ) -> AppResult<TraderMessagePayload> {
        let state = self.state();
        toggle_competition(&state, user_id, id, req).await
    }

    pub async fn sync_balance(
        &self,
        user_id: &str,
        id: &str,
    ) -> AppResult<TraderBalanceSyncPayload> {
        let state = self.state();
        sync_balance(&state, user_id, id).await
    }

    pub async fn close_position(
        &self,
        user_id: &str,
        id: &str,
        req: ClosePositionRequest,
    ) -> AppResult<ClosePositionPayload> {
        let state = self.state();
        close_position(
            &state,
            self.trading_runtime_service.as_ref(),
            user_id,
            id,
            req,
        )
        .await
    }

    pub async fn grid_risk_info(&self, user_id: &str, id: &str) -> AppResult<GridRiskInfoPayload> {
        let state = self.state();
        grid_risk_info(&state, user_id, id).await
    }

    pub async fn status(
        &self,
        user_id: &str,
        query: TraderQuery,
    ) -> AppResult<TraderStatusPayload> {
        let state = self.state();
        status(&state, user_id, query).await
    }

    pub async fn account(
        &self,
        user_id: &str,
        query: TraderQuery,
    ) -> AppResult<TraderAccountPayload> {
        let state = self.state();
        account(&state, user_id, query).await
    }

    pub async fn positions(
        &self,
        user_id: &str,
        query: PositionQuery,
    ) -> AppResult<PositionListPayload> {
        let state = self.state();
        positions(&state, user_id, query).await
    }

    pub async fn positions_history(
        &self,
        user_id: &str,
        query: PaginationQuery,
    ) -> AppResult<PositionListPayload> {
        let state = self.state();
        positions_history(&state, user_id, query).await
    }

    pub async fn decisions(
        &self,
        user_id: &str,
        query: DecisionQuery,
    ) -> AppResult<DecisionListPayload> {
        let state = self.state();
        decisions(&state, user_id, query).await
    }

    pub async fn latest_decisions(
        &self,
        user_id: &str,
        query: TraderQuery,
    ) -> AppResult<LatestDecisionsPayload> {
        let state = self.state();
        latest_decisions(&state, user_id, query).await
    }

    pub async fn trades(
        &self,
        user_id: &str,
        query: PaginationQuery,
    ) -> AppResult<TradeListPayload> {
        let state = self.state();
        trades(&state, user_id, query).await
    }

    pub async fn orders(
        &self,
        user_id: &str,
        query: PaginationQuery,
    ) -> AppResult<OrderListPayload> {
        let state = self.state();
        orders(&state, user_id, query).await
    }

    pub async fn order_fills(
        &self,
        user_id: &str,
        order_id: &str,
        query: TraderQuery,
    ) -> AppResult<FillListPayload> {
        let state = self.state();
        order_fills(&state, user_id, order_id, query).await
    }

    pub async fn open_orders(
        &self,
        user_id: &str,
        query: PaginationQuery,
    ) -> AppResult<OrderListPayload> {
        let state = self.state();
        open_orders(&state, user_id, query).await
    }

    pub async fn runtime_events(
        &self,
        user_id: &str,
        query: RuntimeEventsQuery,
    ) -> AppResult<RuntimeEventsPayload> {
        let state = self.state();
        runtime_events(&state, user_id, query).await
    }

    pub async fn runtime_event_types(
        &self,
        user_id: &str,
        query: RuntimeEventTypesQuery,
    ) -> AppResult<RuntimeEventTypesPayload> {
        let state = self.state();
        runtime_event_types(&state, user_id, query).await
    }

    pub async fn runtime_metrics(
        &self,
        user_id: &str,
        query: RuntimeMetricsQuery,
    ) -> AppResult<RuntimeMetricsPayload> {
        let state = self.state();
        runtime_metrics(&state, user_id, query).await
    }

    pub async fn runtime_metrics_series(
        &self,
        user_id: &str,
        query: RuntimeMetricsSeriesQuery,
    ) -> AppResult<RuntimeMetricsSeriesPayload> {
        let state = self.state();
        runtime_metrics_series(&state, user_id, query).await
    }

    pub async fn runtime_alerts(
        &self,
        user_id: &str,
        query: RuntimeAlertsQuery,
    ) -> AppResult<RuntimeAlertsPayload> {
        let state = self.state();
        runtime_alerts(&state, user_id, query).await
    }

    pub async fn runtime_alert_history(
        &self,
        user_id: &str,
        query: RuntimeAlertHistoryQuery,
    ) -> AppResult<RuntimeAlertHistoryPayload> {
        let state = self.state();
        runtime_alert_history(&state, user_id, query).await
    }

    pub async fn runtime_alert_deliveries(
        &self,
        user_id: &str,
        query: RuntimeAlertDeliveriesQuery,
    ) -> AppResult<RuntimeAlertDeliveriesPayload> {
        let state = self.state();
        runtime_alert_deliveries(&state, user_id, query).await
    }

    pub async fn runtime_alert_controls(
        &self,
        user_id: &str,
        query: RuntimeAlertControlsQuery,
    ) -> AppResult<RuntimeAlertControlsPayload> {
        let state = self.state();
        runtime_alert_controls(&state, user_id, query).await
    }

    pub async fn mute_runtime_alerts(
        &self,
        user_id: &str,
        req: RuntimeAlertMuteRequest,
    ) -> AppResult<RuntimeAlertMutePayload> {
        let state = self.state();
        mute_runtime_alerts(&state, user_id, req).await
    }

    pub async fn unmute_runtime_alerts(
        &self,
        user_id: &str,
        req: RuntimeAlertControlTargetRequest,
    ) -> AppResult<RuntimeAlertMutePayload> {
        let state = self.state();
        unmute_runtime_alerts(&state, user_id, req).await
    }

    pub async fn ack_runtime_alerts(
        &self,
        user_id: &str,
        req: RuntimeAlertAckRequest,
    ) -> AppResult<RuntimeAlertAckPayload> {
        let state = self.state();
        ack_runtime_alerts(&state, user_id, req).await
    }

    pub async fn statistics(
        &self,
        user_id: &str,
        query: StatisticsQuery,
    ) -> AppResult<TraderStatisticsPayload> {
        let state = self.state();
        statistics(&state, user_id, query).await
    }
}

// ====== trader lifecycle ======
