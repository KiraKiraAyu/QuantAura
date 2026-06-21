use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::competition::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_public_competition))
        .route("/top-traders", get(handle_top_traders))
        .route("/equity-history", get(handle_equity_history))
        .route("/equity-history-batch", post(handle_equity_history_batch))
        .route(
            "/traders/{id}/public-config",
            get(handle_public_trader_config),
        )
}
