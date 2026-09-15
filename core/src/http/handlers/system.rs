use axum::{Json, extract::State};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    contracts::system::{
        HealthResponse, SettingsResponse, SystemConfigResponse, UpdateSettingsRequest,
    },
    error::Result,
    http::response::{ApiResponse, AppJson},
    state,
};

pub async fn health() -> Result<Json<ApiResponse<HealthResponse>>> {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);

    let payload = HealthResponse {
        status: "ok",
        time_ms: now_ms,
    };

    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn config(
    State(shared_state): State<state::AppState>,
) -> Result<Json<ApiResponse<SystemConfigResponse>>> {
    let runtime_alerts = &shared_state.config.runtime_alerts;

    let payload = SystemConfigResponse {
        runtime_alert_webhook_enabled: runtime_alerts.enabled(),
        runtime_alert_webhook_auth_header_set: runtime_alerts.auth_header_set(),
        runtime_alert_webhook_timeout_secs: runtime_alerts.timeout_secs,
        runtime_alert_webhook_max_retries: runtime_alerts.max_retries,
        runtime_alert_webhook_retry_backoff_ms: runtime_alerts.retry_backoff_ms,
        runtime_alert_webhook_signing_enabled: runtime_alerts.signing_enabled(),
        runtime_alert_webhook_signing_header_set: runtime_alerts.signing_header_set(),
        runtime_alert_webhook_signing_timestamp_header_set: runtime_alerts
            .signing_timestamp_header_set(),
        runtime_alert_webhook_signing_max_age_secs: runtime_alerts.signing_max_age_secs,
    };

    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn get_settings(
    State(shared_state): State<state::AppState>,
) -> Result<Json<ApiResponse<SettingsResponse>>> {
    let timezone = shared_state.services.system_service.get_timezone().await?;

    let payload = SettingsResponse { timezone };

    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn update_settings(
    State(shared_state): State<state::AppState>,
    AppJson(body): AppJson<UpdateSettingsRequest>,
) -> Result<Json<ApiResponse<SettingsResponse>>> {
    let timezone = shared_state
        .services
        .system_service
        .update_timezone(body.timezone.as_deref())
        .await?;

    let payload = SettingsResponse { timezone };

    Ok(Json(ApiResponse::success(Some(payload), None)))
}
