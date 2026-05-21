use axum::{
    Router,
    routing::{get, put},
};

use crate::{http::handlers::trading::alerts, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/runtime-alerts", get(alerts::runtime_alerts))
        .route("/runtime-alert-history", get(alerts::runtime_alert_history))
        .route(
            "/runtime-alert-deliveries",
            get(alerts::runtime_alert_deliveries),
        )
        .route(
            "/runtime-alert-controls",
            get(alerts::runtime_alert_controls),
        )
        .route(
            "/runtime-alert-controls/mute",
            put(alerts::mute_runtime_alerts),
        )
        .route(
            "/runtime-alert-controls/unmute",
            put(alerts::unmute_runtime_alerts),
        )
        .route(
            "/runtime-alert-controls/ack",
            put(alerts::ack_runtime_alerts),
        )
}
