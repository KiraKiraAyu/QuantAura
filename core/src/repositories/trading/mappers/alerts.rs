use crate::{entity, time::dt_to_ts};

use super::super::records::alerts::{
    RuntimeAlertControlsRecord, RuntimeAlertDeliveryRecord, RuntimeAlertHistoryRecord,
};

pub(in crate::repositories::trading) fn map_runtime_alert_controls(
    row: entity::runtime_alert_controls::Model,
) -> RuntimeAlertControlsRecord {
    RuntimeAlertControlsRecord {
        trader_id: row.trader_id,
        is_muted: row.is_muted != 0,
        muted_until: row.muted_until.map(dt_to_ts).unwrap_or(0),
        mute_reason: row.mute_reason,
        acked_at: row.acked_at.map(dt_to_ts).unwrap_or(0),
        acked_by: row.acked_by,
        ack_note: row.ack_note,
        updated_at: dt_to_ts(row.updated_at),
        created_at: dt_to_ts(row.created_at),
    }
}

pub(in crate::repositories::trading) fn map_runtime_alert_history(
    row: entity::runtime_alert_history::Model,
) -> RuntimeAlertHistoryRecord {
    RuntimeAlertHistoryRecord {
        id: row.id,
        window_hours: row.window_hours,
        thresholds_json: row.thresholds_json,
        rates_json: row.rates_json,
        alerts_json: row.alerts_json,
        breached: row.breached != 0,
        severity: row.severity,
        created_at: dt_to_ts(row.created_at),
    }
}

pub(in crate::repositories::trading) fn map_runtime_alert_delivery(
    row: entity::runtime_alert_delivery_log::Model,
) -> RuntimeAlertDeliveryRecord {
    RuntimeAlertDeliveryRecord {
        id: row.id,
        alert_history_id: row.alert_history_id,
        destination: row.destination,
        endpoint: row.endpoint,
        response_status: row.response_status,
        response_body: row.response_body,
        attempt: row.attempt,
        max_attempts: row.max_attempts,
        success: row.success != 0,
        error_message: row.error_message,
        latency_ms: row.latency_ms,
        created_at: dt_to_ts(row.created_at),
    }
}
