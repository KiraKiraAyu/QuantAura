#[derive(Debug, Clone)]
pub struct TraderAccountRecord {
    pub trader_id: String,
    pub total_balance: f64,
    pub available_balance: f64,
    pub used_margin: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub currency: String,
    pub snapshot_at: i64,
}
