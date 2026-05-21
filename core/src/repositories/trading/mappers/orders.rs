use crate::{entity, time::dt_to_ts};

use super::super::{
    records::orders::{OrderFillRecord, TraderOrderRecord},
    values::decimal_to_f64,
};

pub(in crate::repositories::trading) fn map_order(
    row: entity::trader_orders::Model,
) -> TraderOrderRecord {
    TraderOrderRecord {
        id: row.id,
        exchange_order_id: row.exchange_order_id,
        client_order_id: row.client_order_id,
        symbol: row.symbol,
        side: row.side,
        position_side: row.position_side,
        order_type: row.order_type,
        status: row.status,
        price: decimal_to_f64(&row.price),
        quantity: decimal_to_f64(&row.quantity),
        filled_quantity: decimal_to_f64(&row.filled_quantity),
        avg_fill_price: decimal_to_f64(&row.avg_fill_price),
        reduce_only: row.reduce_only != 0,
        time_in_force: row.time_in_force,
        placed_at: dt_to_ts(row.placed_at),
        updated_at: dt_to_ts(row.updated_at),
        closed_at: row.closed_at.map(dt_to_ts),
    }
}

pub(in crate::repositories::trading) fn map_fill(
    row: entity::order_fills::Model,
) -> OrderFillRecord {
    OrderFillRecord {
        id: row.id,
        exchange_trade_id: row.exchange_trade_id,
        symbol: row.symbol,
        side: row.side,
        price: decimal_to_f64(&row.price),
        quantity: decimal_to_f64(&row.quantity),
        fee: decimal_to_f64(&row.fee),
        fee_asset: row.fee_asset,
        realized_pnl: decimal_to_f64(&row.realized_pnl),
        executed_at: dt_to_ts(row.executed_at),
    }
}
