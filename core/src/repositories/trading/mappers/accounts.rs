use crate::{entity, time::dt_to_ts};

use super::super::{records::accounts::TraderAccountRecord, values::decimal_to_f64};

pub(in crate::repositories::trading) fn map_account(
    row: entity::trader_accounts::Model,
) -> TraderAccountRecord {
    TraderAccountRecord {
        trader_id: row.trader_id,
        total_balance: decimal_to_f64(&row.total_balance),
        available_balance: decimal_to_f64(&row.available_balance),
        used_margin: decimal_to_f64(&row.used_margin),
        unrealized_pnl: decimal_to_f64(&row.unrealized_pnl),
        realized_pnl: decimal_to_f64(&row.realized_pnl),
        currency: row.currency,
        snapshot_at: dt_to_ts(row.snapshot_at),
    }
}
