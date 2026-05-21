use std::sync::Arc;

use crate::{services::trading::service::TradingService, state};

pub mod accounts;
pub mod alerts;
pub mod history;
pub mod orders;
pub mod runtime_observability;
pub mod traders;

pub(super) fn trading_service(app: &state::AppState) -> Arc<TradingService> {
    app.services.trading_service.clone()
}
