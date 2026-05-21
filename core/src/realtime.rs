use std::{convert::Infallible, sync::Arc, time::Duration};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use futures_util::{Stream, stream};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tracing::{debug, warn};

use crate::state::AppState;

pub const REALTIME_CHANNEL_CAPACITY: usize = 512;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeEvent {
    PositionUpdate {
        user_id: String,
        trader_id: String,
        positions: serde_json::Value,
    },
    TradeExecution {
        user_id: String,
        trader_id: String,
        trade: serde_json::Value,
    },
    AiDecision {
        user_id: String,
        trader_id: String,
        decision: serde_json::Value,
    },
    EngineStatus {
        user_id: String,
        trader_id: String,
        status: String,
        message: String,
    },
    EquitySnapshot {
        user_id: String,
        trader_id: String,
        equity: f64,
        available_cash: f64,
        unrealized_pnl: f64,
        ts: i64,
    },
    BacktestProgress {
        user_id: String,
        run_id: String,
        state: String,
        bar_index: usize,
        total_bars: usize,
        equity: f64,
        ts: i64,
    },
    DebateMessage {
        user_id: String,
        debate_id: String,
        round: i64,
        personality: String,
        content: String,
        vote: String,
    },
    DebateFinished {
        user_id: String,
        debate_id: String,
        status: String,
        final_decision: String,
        final_reasoning: String,
    },
    Error {
        user_id: String,
        code: String,
        message: String,
    },
}

impl RealtimeEvent {
    pub fn user_id(&self) -> &str {
        match self {
            RealtimeEvent::PositionUpdate { user_id, .. }
            | RealtimeEvent::TradeExecution { user_id, .. }
            | RealtimeEvent::AiDecision { user_id, .. }
            | RealtimeEvent::EngineStatus { user_id, .. }
            | RealtimeEvent::EquitySnapshot { user_id, .. }
            | RealtimeEvent::BacktestProgress { user_id, .. }
            | RealtimeEvent::DebateMessage { user_id, .. }
            | RealtimeEvent::DebateFinished { user_id, .. }
            | RealtimeEvent::Error { user_id, .. } => user_id,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RealtimeHub {
    tx: broadcast::Sender<Arc<RealtimeEvent>>,
}

impl RealtimeHub {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(REALTIME_CHANNEL_CAPACITY);
        Self { tx }
    }

    pub fn publish(&self, event: RealtimeEvent) -> usize {
        self.tx.send(Arc::new(event)).unwrap_or(0)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Arc<RealtimeEvent>> {
        self.tx.subscribe()
    }
}

impl Default for RealtimeHub {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
pub struct EventStreamQuery {
    pub token: Option<String>,
}

pub async fn events_handler(
    State(app): State<AppState>,
    Query(query): Query<EventStreamQuery>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, StatusCode> {
    let token = query.token.unwrap_or_default();
    let user_id = validate_stream_token(&app, &token).ok_or(StatusCode::UNAUTHORIZED)?;
    let rx = app.realtime_hub.subscribe();

    debug!("sse: client connected user_id={}", user_id);

    let stream = stream::unfold((rx, user_id), |(mut rx, user_id)| async move {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    if event.user_id() != user_id {
                        continue;
                    }

                    let json = match serde_json::to_string(event.as_ref()) {
                        Ok(json) => json,
                        Err(err) => {
                            warn!("sse: serialize error: {err}");
                            continue;
                        }
                    };

                    return Some((Ok(Event::default().data(json)), (rx, user_id)));
                }
                Err(broadcast::error::RecvError::Lagged(count)) => {
                    warn!("sse: client {} lagged by {count} events", user_id);
                }
                Err(broadcast::error::RecvError::Closed) => return None,
            }
        }
    });

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(20))
            .text("keep-alive"),
    ))
}

fn validate_stream_token(app: &AppState, token: &str) -> Option<String> {
    if token.is_empty() {
        return None;
    }

    match app.services.auth_service.authenticate_token(token) {
        Ok(claims) => Some(claims.sub),
        Err(err) => {
            debug!("sse: token auth failed: {err}");
            None
        }
    }
}
