pub mod backtest;
pub mod catalog;
pub mod crypto;
pub mod debates;
pub mod exchanges;
pub mod market;
pub mod models;
pub mod strategies;
pub mod trading;

use std::time::Duration;

use axum::{
    Json, Router,
    http::{HeaderValue, StatusCode},
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::warn;

use crate::{
    http::{
        handlers,
        middleware::{require_auth, require_auth_streaming},
        response::ApiResponse,
    },
    realtime,
    state::AppState,
};

async fn fallback_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(ApiResponse::<()>::failure("Route not found")),
    )
}

/// Build the CORS layer from the configured allow-list.
///
/// A bare `*` keeps the permissive default. Anything else is parsed as a
/// comma-separated origin list; entries that cannot be parsed are reported
/// rather than dropped silently, since a typo would otherwise lock out every
/// cross-origin caller with no diagnostic.
fn cors_layer(configured_origin: &str) -> CorsLayer {
    let layer = CorsLayer::new().allow_methods(Any).allow_headers(Any);

    if configured_origin.trim() == "*" {
        return layer.allow_origin(Any);
    }

    let mut origins: Vec<HeaderValue> = Vec::new();
    for entry in configured_origin.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }

        match entry.parse::<HeaderValue>() {
            Ok(origin) => origins.push(origin),
            Err(_) => warn!("ignoring invalid CORS_ALLOW_ORIGIN entry: {entry:?}"),
        }
    }

    if origins.is_empty() {
        warn!(
            "CORS_ALLOW_ORIGIN={configured_origin:?} contains no valid origin; cross-origin requests will be rejected"
        );
    }

    layer.allow_origin(origins)
}

pub fn build_app(state: AppState, timeout_secs: u64) -> Router {
    let cors = cors_layer(&state.config.server.cors_allow_origin);

    let public_api = Router::new()
        .route("/health", get(handlers::system::health))
        .route("/auth/status", get(handlers::auth::status))
        .route("/auth/verify", post(handlers::auth::verify))
        .route("/auth/setup/start", post(handlers::auth::setup_start))
        .route("/auth/setup/confirm", post(handlers::auth::setup_confirm));

    let standard_protected = Router::new()
        .route("/config", get(handlers::system::config))
        .route(
            "/settings",
            get(handlers::system::get_settings).put(handlers::system::update_settings),
        )
        .route("/auth/reset/start", post(handlers::auth::reset_start))
        .route("/auth/reset/confirm", post(handlers::auth::reset_confirm))
        .nest("/catalog", catalog::router())
        .nest("/crypto", crypto::router())
        .nest("/market", market::router())
        .nest("/exchanges", exchanges::router())
        .nest("/trading", trading::router());

    // Compute timeouts: standard CRUD uses configured timeout (e.g. 15s);
    // computationally heavy or LLM-driven endpoints receive a generous timeout (minimum 180s)
    let long_running_timeout_secs = timeout_secs.max(180);

    let long_running_protected = Router::new()
        .nest("/models", models::router())
        .nest("/strategies", strategies::router())
        .nest("/backtest", backtest::router())
        .nest("/debates", debates::router())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(long_running_timeout_secs),
        ));

    let protected_api = Router::new()
        .merge(standard_protected)
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(timeout_secs),
        ))
        .merge(long_running_protected)
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth));

    let stream_api = Router::new()
        .route("/events", get(realtime::events_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_auth_streaming,
        ));

    let api = Router::new()
        .merge(public_api)
        .merge(protected_api)
        .merge(stream_api)
        .fallback(fallback_404);

    Router::new()
        .nest("/api", api)
        .fallback(fallback_404)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
