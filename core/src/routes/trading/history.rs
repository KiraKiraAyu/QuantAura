use axum::{Router, routing::get};

use crate::{http::handlers::trading::history, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/trades", get(history::trades))
        .route("/decisions", get(history::decisions))
        .route("/decisions/latest", get(history::latest_decisions))
        .route("/statistics", get(history::statistics))
}
