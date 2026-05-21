use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::models::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_model_configs).put(update_model_configs))
        .route("/list", post(list_available_models))
        .route("/check-provider", post(check_provider_availability))
}
