use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::crypto::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/config", get(handle_crypto_config))
        .route("/public-key", get(handle_crypto_public_key))
        .route("/decrypt", post(handle_crypto_decrypt))
}
