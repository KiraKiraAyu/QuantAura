use crate::{entity, time::dt_to_ts};

use super::super::records::runtime_observability::RuntimeEventRecord;

pub(in crate::repositories::trading) fn map_runtime_event(
    row: entity::runtime_events::Model,
) -> RuntimeEventRecord {
    RuntimeEventRecord {
        id: row.id,
        event_type: row.event_type,
        symbol: row.symbol,
        side: row.side,
        risk_level: row.risk_level,
        trigger_source: row.trigger_source,
        action_taken: row.action_taken,
        correlation_id: row.correlation_id,
        payload_json: row.payload_json,
        created_at: dt_to_ts(row.created_at),
    }
}
