use crate::{entity, time::dt_to_ts};

use super::super::{records::positions::TraderPositionRecord, values::decimal_to_f64};

pub(in crate::repositories::trading) fn map_position(
    row: entity::trader_positions::Model,
) -> TraderPositionRecord {
    TraderPositionRecord {
        id: row.id,
        trader_id: row.trader_id,
        symbol: row.symbol,
        side: row.side,
        quantity: decimal_to_f64(&row.quantity),
        entry_price: decimal_to_f64(&row.entry_price),
        mark_price: decimal_to_f64(&row.mark_price),
        liquidation_price: decimal_to_f64(&row.liquidation_price),
        leverage: row.leverage,
        margin_mode: row.margin_mode,
        unrealized_pnl: decimal_to_f64(&row.unrealized_pnl),
        realized_pnl: decimal_to_f64(&row.realized_pnl),
        status: row.status,
        opened_at: dt_to_ts(row.opened_at),
        closed_at: row.closed_at.map(dt_to_ts),
        updated_at: dt_to_ts(row.updated_at),
    }
}
