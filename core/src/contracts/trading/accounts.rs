use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ClosePositionRequest {
    pub symbol: String,
    pub side: String,
    #[serde(default)]
    pub local_only: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraderAccountPayload {
    pub trader_id: String,
    pub total_balance: f64,
    pub available_balance: f64,
    pub used_margin: f64,
    pub unrealized_pnl: f64,
    pub realized_pnl: f64,
    pub currency: String,
    pub snapshot_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraderBalanceSyncPayload {
    pub message: &'static str,
    pub mode: String,
    pub account: TraderAccountPayload,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClosePositionPayload {
    pub message: &'static str,
    pub mode: String,
    pub order_id: String,
    pub symbol: String,
    pub side: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SymbolConcentrationPayload {
    pub symbol: String,
    pub notional: f64,
    pub weight_pct: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GridRiskInfoPayload {
    pub trader_id: String,
    pub total_notional: f64,
    pub symbol_concentration: Vec<SymbolConcentrationPayload>,
}
