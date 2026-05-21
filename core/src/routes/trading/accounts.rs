use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::trading::accounts, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/traders/{id}/sync-balance", post(accounts::sync_balance))
        .route(
            "/traders/{id}/close-position",
            post(accounts::close_position),
        )
        .route("/traders/{id}/grid-risk", get(accounts::grid_risk))
        .route("/account", get(accounts::account))
        .route("/positions", get(accounts::positions))
        .route("/positions/history", get(accounts::positions_history))
}
