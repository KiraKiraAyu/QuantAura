use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::competition::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/competition", get(handle_public_competition))
        .route("/competition/top-traders", get(handle_top_traders))
        .route("/competition/equity-history", get(handle_equity_history))
        .route(
            "/competition/equity-history-batch",
            post(handle_equity_history_batch),
        )
        .route(
            "/competition/traders/{id}/public-config",
            get(handle_public_trader_config),
        )
}
