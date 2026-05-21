use axum::{Router, routing::get};

use crate::{http::handlers::trading::runtime_observability, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/runtime-metrics",
            get(runtime_observability::runtime_metrics),
        )
        .route(
            "/runtime-metrics-series",
            get(runtime_observability::runtime_metrics_series),
        )
        .route(
            "/runtime-events",
            get(runtime_observability::runtime_events),
        )
        .route(
            "/runtime-event-types",
            get(runtime_observability::runtime_event_types),
        )
}
