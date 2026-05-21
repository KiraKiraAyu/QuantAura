pub mod backtest;
pub mod catalog;
pub mod competition;
pub mod crypto;
pub mod debates;
pub mod exchanges;
pub mod market;
pub mod models;
pub mod strategies;
pub mod trading;

use std::time::Duration;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{http::handlers, realtime, state::AppState};

pub fn build_app(state: AppState, timeout_secs: u64) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let public_api = Router::new()
        .route("/health", get(handlers::system::health))
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login));

    let protected_api = Router::new()
        .route("/config", get(handlers::system::config))
        .route("/logout", post(handlers::auth::logout))
        .route(
            "/auth/change-password",
            post(handlers::auth::change_password),
        )
        .route("/me", get(handlers::auth::me))
        .merge(catalog::router())
        .merge(competition::router())
        .nest("/crypto", crypto::router())
        .nest("/market", market::router())
        .nest("/models", models::router())
        .nest("/exchanges", exchanges::router())
        .nest("/strategies", strategies::router())
        .nest("/trading", trading::router())
        .nest("/backtest", backtest::router())
        .nest("/debates", debates::router());

    let timed_api =
        Router::new()
            .merge(public_api)
            .merge(protected_api)
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(timeout_secs),
            ));

    let stream_api = Router::new().route("/events", get(realtime::events_handler));

    let api = Router::new().merge(timed_api).merge(stream_api);

    Router::new()
        .nest("/api", api)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
