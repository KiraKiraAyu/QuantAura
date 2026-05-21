use crate::{entity, time::dt_to_ts};

use super::super::{
    records::history::{TraderDecisionRecord, TraderTradeRecord},
    values::decimal_to_f64,
};

pub(in crate::repositories::trading) fn map_decision(
    row: entity::trader_decisions::Model,
) -> TraderDecisionRecord {
    TraderDecisionRecord {
        id: row.id,
        symbol: row.symbol,
        timeframe: row.timeframe,
        decision: row.decision,
        confidence: decimal_to_f64(&row.confidence),
        reason: row.reason,
        payload_json: row.payload_json,
        created_at: dt_to_ts(row.created_at),
    }
}

pub(in crate::repositories::trading) fn map_trade(
    row: entity::trader_trades::Model,
) -> TraderTradeRecord {
    TraderTradeRecord {
        id: row.id,
        symbol: row.symbol,
        side: row.side,
        entry_price: decimal_to_f64(&row.entry_price),
        exit_price: decimal_to_f64(&row.exit_price),
        quantity: decimal_to_f64(&row.quantity),
        realized_pnl: decimal_to_f64(&row.realized_pnl),
        fees: decimal_to_f64(&row.fees),
        roi_pct: decimal_to_f64(&row.roi_pct),
        opened_at: dt_to_ts(row.opened_at),
        closed_at: dt_to_ts(row.closed_at),
    }
}
