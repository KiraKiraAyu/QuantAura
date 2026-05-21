use axum::{Router, routing::get};

use crate::{http::handlers::market::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/klines", get(handle_klines))
        .route("/symbols", get(handle_symbols))
}
