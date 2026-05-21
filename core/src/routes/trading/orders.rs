use axum::{Router, routing::get};

use crate::{http::handlers::trading::orders, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/orders", get(orders::orders))
        .route("/orders/{id}/fills", get(orders::order_fills))
        .route("/open-orders", get(orders::open_orders))
}
