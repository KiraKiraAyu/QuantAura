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

use crate::contracts::trading::positions::PositionPayload;
use crate::state::AppState;

pub const REALTIME_CHANNEL_CAPACITY: usize = 512;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RealtimeEvent {
    PositionUpdate {
        user_id: String,
        trader_id: String,
        positions: Vec<PositionPayload>,
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn position_update_serializes_full_position_payloads() {
        let event = RealtimeEvent::PositionUpdate {
            user_id: "user_1".to_string(),
            trader_id: "trader_1".to_string(),
            positions: vec![sample_position_payload()],
        };

        let value = serde_json::to_value(event).expect("serialize position update event");

        assert_eq!(
            value,
            json!({
                "type": "position_update",
                "user_id": "user_1",
                "trader_id": "trader_1",
                "positions": [
                    {
                        "id": "position_1",
                        "trader_id": "trader_1",
                        "symbol": "BTCUSDT",
                        "side": "LONG",
                        "quantity": 1.25,
                        "entry_price": 256.5,
                        "mark_price": 260.75,
                        "liquidation_price": 128.0,
                        "leverage": 5,
                        "margin_mode": "cross",
                        "unrealized_pnl": 12.5,
                        "realized_pnl": -2.5,
                        "status": "open",
                        "opened_at": 1_700_000_000,
                        "closed_at": null,
                        "updated_at": 1_700_000_900
                    }
                ]
            })
        );
    }

    #[test]
    fn position_update_deserializes_typed_position_payloads() {
        let event: RealtimeEvent = serde_json::from_value(json!({
            "type": "position_update",
            "user_id": "user_1",
            "trader_id": "trader_1",
            "positions": [
                {
                    "id": "position_1",
                    "trader_id": "trader_1",
                    "symbol": "BTCUSDT",
                    "side": "LONG",
                    "quantity": 1.25,
                    "entry_price": 256.5,
                    "mark_price": 260.75,
                    "liquidation_price": 128.0,
                    "leverage": 5,
                    "margin_mode": "cross",
                    "unrealized_pnl": 12.5,
                    "realized_pnl": -2.5,
                    "status": "open",
                    "opened_at": 1_700_000_000,
                    "closed_at": null,
                    "updated_at": 1_700_000_900
                }
            ]
        }))
        .expect("deserialize position update event");

        let RealtimeEvent::PositionUpdate {
            user_id,
            trader_id,
            positions,
        } = event
        else {
            panic!("expected position update event");
        };

        assert_eq!(user_id, "user_1");
        assert_eq!(trader_id, "trader_1");
        assert_eq!(positions.len(), 1);

        let position = &positions[0];
        assert_eq!(position.id, "position_1");
        assert_eq!(position.trader_id, "trader_1");
        assert_eq!(position.symbol, "BTCUSDT");
        assert_eq!(position.side, "LONG");
        assert_eq!(position.quantity, 1.25);
        assert_eq!(position.entry_price, 256.5);
        assert_eq!(position.mark_price, 260.75);
        assert_eq!(position.liquidation_price, 128.0);
        assert_eq!(position.leverage, 5);
        assert_eq!(position.margin_mode, "cross");
        assert_eq!(position.unrealized_pnl, 12.5);
        assert_eq!(position.realized_pnl, -2.5);
        assert_eq!(position.status, "open");
        assert_eq!(position.opened_at, 1_700_000_000);
        assert_eq!(position.closed_at, None);
        assert_eq!(position.updated_at, 1_700_000_900);
    }

    fn sample_position_payload() -> PositionPayload {
        PositionPayload {
            id: "position_1".to_string(),
            trader_id: "trader_1".to_string(),
            symbol: "BTCUSDT".to_string(),
            side: "LONG".to_string(),
            quantity: 1.25,
            entry_price: 256.5,
            mark_price: 260.75,
            liquidation_price: 128.0,
            leverage: 5,
            margin_mode: "cross".to_string(),
            unrealized_pnl: 12.5,
            realized_pnl: -2.5,
            status: "open".to_string(),
            opened_at: 1_700_000_000,
            closed_at: None,
            updated_at: 1_700_000_900,
        }
    }
}
