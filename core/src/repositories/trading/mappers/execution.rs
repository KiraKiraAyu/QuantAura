use crate::{entity, time::dt_to_ts};

use super::super::records::execution::ExecutionIntentRecord;

pub(in crate::repositories::trading) fn map_execution_intent(
    row: entity::execution_intents::Model,
) -> ExecutionIntentRecord {
    ExecutionIntentRecord {
        id: row.id,
        intent_key: row.intent_key,
        symbol: row.symbol,
        side: row.side,
        decision: row.decision,
        status: row.status,
        exchange_order_id: row.exchange_order_id,
        payload_json: row.payload_json,
        updated_at: dt_to_ts(row.updated_at),
    }
}
