use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DecisionQuery {
    pub trader_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StatisticsQuery {
    pub trader_id: Option<String>,
    pub days: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecisionPayload {
    pub id: String,
    pub symbol: String,
    pub timeframe: String,
    pub decision: String,
    pub confidence: f64,
    pub reason: String,
    pub payload_json: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecisionListPayload {
    pub trader_id: String,
    pub items: Vec<DecisionPayload>,
    pub count: usize,
    pub limit: i64,
    pub offset: i64,
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LatestDecisionsPayload {
    pub trader_id: String,
    pub items: Vec<DecisionPayload>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradePayload {
    pub id: String,
    pub symbol: String,
    pub side: String,
    pub entry_price: f64,
    pub exit_price: f64,
    pub quantity: f64,
    pub realized_pnl: f64,
    pub fees: f64,
    pub roi_pct: f64,
    pub opened_at: i64,
    pub closed_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TradeListPayload {
    pub trader_id: String,
    pub items: Vec<TradePayload>,
    pub count: usize,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraderStatisticsPayload {
    pub trader_id: String,
    pub period_days: i64,
    pub total_trades: i64,
    pub winning_trades: i64,
    pub win_rate_pct: f64,
    pub total_realized_pnl: f64,
    pub total_fees: f64,
    pub net_pnl: f64,
    pub avg_roi_pct: f64,
    pub open_positions: i64,
}
