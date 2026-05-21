use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::backtest::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/start", post(handle_backtest_start))
        .route("/pause", post(handle_backtest_pause))
        .route("/resume", post(handle_backtest_resume))
        .route("/stop", post(handle_backtest_stop))
        .route("/label", post(handle_backtest_label))
        .route("/delete", post(handle_backtest_delete))
        .route("/status", get(handle_backtest_status))
        .route("/runs", get(handle_backtest_runs))
        .route("/equity", get(handle_backtest_equity))
        .route("/trades", get(handle_backtest_trades))
        .route("/metrics", get(handle_backtest_metrics))
        .route("/trace", get(handle_backtest_trace))
        .route("/decisions", get(handle_backtest_decisions))
        .route("/export", get(handle_backtest_export))
        .route("/klines", get(handle_backtest_klines))
}
