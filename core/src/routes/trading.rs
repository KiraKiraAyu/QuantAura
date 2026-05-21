use axum::Router;

use crate::state::AppState;

mod accounts;
mod alerts;
mod history;
mod orders;
mod runtime_observability;
mod traders;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(traders::router())
        .merge(accounts::router())
        .merge(history::router())
        .merge(orders::router())
        .merge(runtime_observability::router())
        .merge(alerts::router())
}
