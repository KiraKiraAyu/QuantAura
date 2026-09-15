use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    contracts::trading::alerts::{
        RuntimeAlertAckPayload, RuntimeAlertAckRequest, RuntimeAlertControlsPayload,
        RuntimeAlertControlsQuery, RuntimeAlertDeliveriesPayload, RuntimeAlertDeliveriesQuery,
        RuntimeAlertHistoryPayload, RuntimeAlertHistoryQuery, RuntimeAlertMutePayload,
        RuntimeAlertMuteRequest, RuntimeAlertsPayload, RuntimeAlertsQuery,
    },
    error::{AppError, Result},
    http::response::{ApiResponse, AppJson},
    state,
};

use super::trading_service;

pub async fn runtime_alerts(
    State(app): State<state::AppState>,
    Query(q): Query<RuntimeAlertsQuery>,
) -> Result<Json<ApiResponse<RuntimeAlertsPayload>>> {
    let payload = trading_service(&app)
        .runtime_alerts(
            q.trader_id,
            q.window_hours,
            q.open_market_fallback_rate_max_pct,
            q.replace_throttle_rate_max_pct,
            q.stale_reconcile_terminal_rate_max_pct,
            q.persist_min_interval_secs,
        )
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn runtime_alert_history(
    State(app): State<state::AppState>,
    Query(q): Query<RuntimeAlertHistoryQuery>,
) -> Result<Json<ApiResponse<RuntimeAlertHistoryPayload>>> {
    let payload = trading_service(&app)
        .runtime_alert_history(
            q.trader_id,
            q.window_hours,
            q.limit.map(|l| l.clamp(1, 500)),
            q.offset.map(|o| o.max(0)),
            q.breached_only,
            q.severity,
        )
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn runtime_alert_deliveries(
    State(app): State<state::AppState>,
    Query(q): Query<RuntimeAlertDeliveriesQuery>,
) -> Result<Json<ApiResponse<RuntimeAlertDeliveriesPayload>>> {
    let payload = trading_service(&app)
        .runtime_alert_deliveries(
            q.trader_id,
            q.window_hours,
            q.limit.map(|l| l.clamp(1, 500)),
            q.offset.map(|o| o.max(0)),
            q.success,
            q.destination,
        )
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn runtime_alert_controls(
    State(app): State<state::AppState>,
    Query(q): Query<RuntimeAlertControlsQuery>,
) -> Result<Json<ApiResponse<RuntimeAlertControlsPayload>>> {
    let payload = trading_service(&app)
        .runtime_alert_controls(q.trader_id)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn mute_runtime_alerts(
    State(app): State<state::AppState>,
    Path(trader_id): Path<String>,
    AppJson(request): AppJson<RuntimeAlertMuteRequest>,
) -> Result<Json<ApiResponse<RuntimeAlertMutePayload>>> {
    let trader_id = trader_id.trim();
    if trader_id.is_empty() {
        return Err(AppError::BadRequest("trader_id must not be empty".into()));
    }
    let payload = trading_service(&app)
        .mute_runtime_alerts(
            Some(trader_id.to_string()),
            request.mute_minutes,
            request.mute_until,
            request.reason,
        )
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn unmute_runtime_alerts(
    State(app): State<state::AppState>,
    Path(trader_id): Path<String>,
) -> Result<Json<ApiResponse<RuntimeAlertMutePayload>>> {
    let trader_id = trader_id.trim();
    if trader_id.is_empty() {
        return Err(AppError::BadRequest("trader_id must not be empty".into()));
    }
    let payload = trading_service(&app)
        .unmute_runtime_alerts(Some(trader_id.to_string()))
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn ack_runtime_alerts(
    State(app): State<state::AppState>,
    Path(trader_id): Path<String>,
    AppJson(request): AppJson<RuntimeAlertAckRequest>,
) -> Result<Json<ApiResponse<RuntimeAlertAckPayload>>> {
    let trader_id = trader_id.trim();
    if trader_id.is_empty() {
        return Err(AppError::BadRequest("trader_id must not be empty".into()));
    }
    let payload = trading_service(&app)
        .ack_runtime_alerts(Some(trader_id.to_string()), request.note)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
