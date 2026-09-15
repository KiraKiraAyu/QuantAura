use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct BacktestStartRequest {
    pub trader_id: String,
    pub run_id: Option<String>,
    pub start_ts: Option<i64>,
    pub end_ts: Option<i64>,
    pub initial_balance: Option<f64>,
    pub interval: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BacktestQueryParams {
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestRunRecord {
    pub run_id: String,
    pub state: String,
    pub last_error: String,
    pub summary: Value,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BacktestRunActionPayload {
    pub run_id: String,
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct BacktestMessagePayload {
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct BacktestRunsPayload {
    pub runs: Vec<BacktestRunRecord>,
    pub count: usize,
}
