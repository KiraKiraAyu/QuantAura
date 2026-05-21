use chrono::{TimeZone, Utc};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    clients::outbound_http::{OutboundRequestLog, send_text},
    error::AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MarketKline {
    #[serde(rename = "openTime")]
    pub open_time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    #[serde(rename = "quoteVolume")]
    pub quote_volume: f64,
    #[serde(rename = "closeTime")]
    pub close_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MarketSymbol {
    pub symbol: String,
    pub name: String,
    pub category: String,
}

pub(crate) fn now_ts() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub(crate) fn ts_to_rfc3339(ts: i64) -> String {
    Utc.timestamp_opt(ts, 0)
        .single()
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_else(|| "1970-01-01T00:00:00+00:00".to_string())
}

pub(crate) fn parse_json_value(raw: &str) -> Value {
    serde_json::from_str(raw).unwrap_or_else(|_| json!({}))
}

pub(crate) fn normalize_crypto_symbol(symbol: &str) -> String {
    let upper = symbol.trim().to_uppercase();
    if upper.ends_with("USDT") {
        upper
    } else {
        format!("{}USDT", upper)
    }
}

pub(crate) async fn fetch_binance_klines(
    symbol: &str,
    interval: &str,
    limit: usize,
) -> Result<Vec<MarketKline>, AppError> {
    let url = format!(
        "https://fapi.binance.com/fapi/v1/klines?symbol={}&interval={}&limit={}",
        symbol,
        interval,
        limit.min(1500)
    );
    let response = send_text(
        reqwest::Client::new().get(&url),
        OutboundRequestLog::new("market.binance.klines", Method::GET, &url),
    )
    .await?;
    if !response.status.is_success() {
        return Err(AppError::BadGateway(format!(
            "Binance klines request failed with status {}",
            response.status
        )));
    }
    let rows = serde_json::from_str::<Vec<Vec<Value>>>(&response.body)?;

    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        if r.len() < 7 {
            continue;
        }
        let open_time = r.first().and_then(Value::as_i64).unwrap_or(0);
        let open = r
            .get(1)
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let high = r
            .get(2)
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let low = r
            .get(3)
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let close = r
            .get(4)
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let volume = r
            .get(5)
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let close_time = r.get(6).and_then(Value::as_i64).unwrap_or(open_time);
        let quote_volume = r
            .get(7)
            .and_then(Value::as_str)
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(volume * close);

        out.push(MarketKline {
            open_time,
            open,
            high,
            low,
            close,
            volume,
            quote_volume,
            close_time,
        });
    }

    Ok(out)
}

pub(crate) async fn fetch_binance_symbols() -> Result<Vec<MarketSymbol>, AppError> {
    let url = "https://fapi.binance.com/fapi/v1/exchangeInfo";
    let response = send_text(
        reqwest::Client::new().get(url),
        OutboundRequestLog::new("market.binance.symbols", Method::GET, url),
    )
    .await?;
    if !response.status.is_success() {
        return Err(AppError::BadGateway(format!(
            "Binance symbols request failed with status {}",
            response.status
        )));
    }
    let payload = serde_json::from_str::<Value>(&response.body)?;
    let symbols = payload
        .get("symbols")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut out = Vec::new();
    for s in symbols {
        let status = s.get("status").and_then(Value::as_str).unwrap_or_default();
        let symbol = s.get("symbol").and_then(Value::as_str).unwrap_or_default();
        let quote = s
            .get("quoteAsset")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if status != "TRADING" || quote != "USDT" || symbol.is_empty() {
            continue;
        }
        out.push(MarketSymbol {
            symbol: symbol.to_string(),
            name: symbol.to_string(),
            category: "crypto".to_string(),
        });
        if out.len() >= 400 {
            break;
        }
    }

    Ok(out)
}
