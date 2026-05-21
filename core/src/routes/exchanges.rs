use axum::{
    Router,
    routing::{delete, get},
};

use crate::{http::handlers::exchanges::*, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_exchange_configs)
                .post(create_exchange)
                .put(update_exchange_configs),
        )
        .route("/{id}", delete(delete_exchange))
}
