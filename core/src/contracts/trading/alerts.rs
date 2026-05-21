use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertsQuery {
    pub trader_id: Option<String>,
    pub window_hours: Option<i64>,
    pub open_market_fallback_rate_max_pct: Option<f64>,
    pub replace_throttle_rate_max_pct: Option<f64>,
    pub stale_reconcile_terminal_rate_max_pct: Option<f64>,
    pub persist_min_interval_secs: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertHistoryQuery {
    pub trader_id: Option<String>,
    pub window_hours: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub breached_only: Option<bool>,
    pub severity: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertDeliveriesQuery {
    pub trader_id: Option<String>,
    pub window_hours: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub success: Option<bool>,
    pub destination: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertControlsQuery {
    pub trader_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertControlTargetRequest {
    pub trader_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertMuteRequest {
    pub trader_id: Option<String>,
    pub mute_minutes: Option<i64>,
    pub mute_until: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuntimeAlertAckRequest {
    pub trader_id: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAlertThresholdsPayload {
    pub open_market_fallback_rate_max: f64,
    pub replace_throttle_rate_max: f64,
    pub stale_reconcile_terminal_rate_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAlertRatesPayload {
    pub open_market_fallback_rate: f64,
    pub replace_throttle_rate: f64,
    pub stale_reconcile_terminal_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAlertItemPayload {
    pub key: String,
    pub label: String,
    pub rate_pct: f64,
    pub max_pct: f64,
    pub breached: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertTotalsPayload {
    pub replace_succeeded: i64,
    pub replace_throttled: i64,
    pub open_market_fallback: i64,
    pub open_submitted: i64,
    pub stale_reconcile_terminal: i64,
    pub stale_reconcile_pending: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertStatePayload {
    pub breached: bool,
    pub severity: String,
    pub muted: bool,
    pub acked_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertControlsPayload {
    pub trader_id: String,
    pub is_muted: bool,
    pub muted_until: i64,
    pub mute_reason: String,
    pub acked_at: i64,
    pub acked_by: String,
    pub ack_note: String,
    pub updated_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertNotificationPayload {
    pub channel: String,
    pub suppressed: bool,
    pub reason: String,
    pub attempts: i64,
    pub max_attempts: i64,
    pub success: bool,
    pub status: i64,
    pub error: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertsPayload {
    pub trader_id: String,
    pub window_hours: i64,
    pub from_ts: i64,
    pub persist_min_interval_secs: i64,
    pub thresholds_pct: RuntimeAlertThresholdsPayload,
    pub rates_pct: RuntimeAlertRatesPayload,
    pub totals: RuntimeAlertTotalsPayload,
    pub alerts: Vec<RuntimeAlertItemPayload>,
    pub alert_state: RuntimeAlertStatePayload,
    pub controls: RuntimeAlertControlsPayload,
    pub alert_history_id: String,
    pub notification: RuntimeAlertNotificationPayload,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertHistoryItemPayload {
    pub id: String,
    pub window_hours: i32,
    pub thresholds_pct: RuntimeAlertThresholdsPayload,
    pub rates_pct: RuntimeAlertRatesPayload,
    pub alerts: Vec<RuntimeAlertItemPayload>,
    pub breached: bool,
    pub severity: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertHistoryFiltersPayload {
    pub breached_only: Option<bool>,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertHistoryPayload {
    pub trader_id: String,
    pub window_hours: i64,
    pub from_ts: i64,
    pub limit: i64,
    pub offset: i64,
    pub filters: RuntimeAlertHistoryFiltersPayload,
    pub total: i64,
    pub items: Vec<RuntimeAlertHistoryItemPayload>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertDeliveryLogPayload {
    pub id: String,
    pub alert_history_id: String,
    pub destination: String,
    pub endpoint: String,
    pub response_status: i32,
    pub response_body: String,
    pub attempt: i32,
    pub max_attempts: i32,
    pub success: bool,
    pub error_message: String,
    pub latency_ms: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertDeliveriesFiltersPayload {
    pub success: Option<bool>,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertDeliveriesPayload {
    pub trader_id: String,
    pub window_hours: i64,
    pub from_ts: i64,
    pub limit: i64,
    pub offset: i64,
    pub filters: RuntimeAlertDeliveriesFiltersPayload,
    pub total: i64,
    pub items: Vec<RuntimeAlertDeliveryLogPayload>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertMutePayload {
    pub message: &'static str,
    pub trader_id: String,
    pub is_muted: bool,
    pub muted_until: i64,
    pub mute_reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeAlertAckPayload {
    pub message: &'static str,
    pub trader_id: String,
    pub acked_at: i64,
    pub acked_by: String,
    pub ack_note: String,
}
