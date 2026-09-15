use axum::{
    Router,
    routing::{delete, post},
};

use crate::{http::handlers::backtest::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/runs",
            post(handle_backtest_start).get(handle_backtest_runs),
        )
        .route("/runs/{run_id}/stop", post(handle_backtest_stop))
        .route("/runs/{run_id}", delete(handle_backtest_delete))
}
