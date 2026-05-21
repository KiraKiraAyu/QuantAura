use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct TradingRepo {
    db: DatabaseConnection,
}

mod execution_intents;
mod mappers;
mod order_fills;
pub mod records;
mod runtime_alert_controls;
mod runtime_alert_delivery_log;
mod runtime_alert_history;
mod runtime_events;
mod trader_accounts;
mod trader_decisions;
mod trader_orders;
mod trader_positions;
mod trader_trades;
mod traders;
mod values;

impl TradingRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}
