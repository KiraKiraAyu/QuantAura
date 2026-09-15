use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CreateDebateRequest {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub max_rounds: Option<i64>,
    pub participants: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateSummaryPayload {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub status: String,
    pub max_rounds: i32,
    pub current_round: i32,
    pub final_decision: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebatePayload {
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub status: String,
    pub max_rounds: i32,
    pub current_round: i32,
    pub participants: Vec<String>,
    pub final_decision: String,
    pub final_reasoning: String,
    pub error_message: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateMessageItem {
    pub id: String,
    pub round: i32,
    pub personality: String,
    pub role: String,
    pub content: String,
    pub vote: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateListPayload {
    pub debates: Vec<DebateSummaryPayload>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateDetailPayload {
    pub debate: DebatePayload,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateActionPayload {
    pub id: String,
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateMessagePayload {
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebatePersonalityPayload {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebatePersonalitiesPayload {
    pub personalities: Vec<DebatePersonalityPayload>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateExecutionPayload {
    pub id: String,
    pub final_decision: String,
    pub final_reasoning: String,
    pub message: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateMessagesPayload {
    pub messages: Vec<DebateMessageItem>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct DebateVotesPayload {
    pub votes: Value,
}
