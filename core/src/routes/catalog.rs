use axum::{Router, routing::get};

use crate::{http::handlers::catalog::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/supported-provider-types", get(supported_provider_types))
        .route("/supported-exchanges", get(supported_exchanges))
}
