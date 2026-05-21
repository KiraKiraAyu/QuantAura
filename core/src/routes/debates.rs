use axum::{
    Router,
    routing::{get, post},
};

use crate::{http::handlers::debates::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handle_get_debates).post(handle_create_debate))
        .route("/personalities", get(handle_get_debate_personalities))
        .route("/{id}", get(handle_get_debate).delete(handle_delete_debate))
        .route("/{id}/start", post(handle_start_debate))
        .route("/{id}/cancel", post(handle_cancel_debate))
        .route("/{id}/execute", post(handle_execute_debate))
        .route("/{id}/messages", get(handle_get_debate_messages))
        .route("/{id}/votes", get(handle_get_debate_votes))
}
