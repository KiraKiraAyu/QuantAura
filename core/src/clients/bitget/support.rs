use super::*;

#[derive(Debug, Deserialize)]
pub(super) struct BitgetEnvelope<T> {
    pub(super) code: String,
    #[serde(default)]
    pub(super) msg: String,
    pub(super) data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetTickerRow {
    pub(super) last_pr: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetOrderAck {
    pub(super) order_id: String,
    #[serde(default)]
    pub(super) client_oid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetCancelAck {
    pub(super) order_id: String,
    #[serde(default)]
    pub(super) client_oid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetAccountRow {
    pub(super) margin_coin: String,
    #[serde(default)]
    pub(super) account_equity: String,
    #[serde(default)]
    pub(super) available: String,
    #[serde(default)]
    pub(super) unrealized_pl: String,
    #[serde(default)]
    pub(super) pos_mode: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BitgetPositionMode {
    OneWay,
    Hedge,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetPositionRow {
    pub(super) symbol: String,
    pub(super) hold_side: String,
    pub(super) total: String,
    #[serde(default)]
    pub(super) open_price_avg: String,
    #[serde(default)]
    pub(super) mark_price: String,
    #[serde(default)]
    pub(super) unrealized_pl: String,
    #[serde(default)]
    pub(super) leverage: String,
    #[serde(default)]
    pub(super) liquidation_price: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetOrderPage {
    #[serde(default)]
    pub(super) entrusted_list: Vec<BitgetOrderRow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetFillPage {
    #[serde(default)]
    pub(super) fill_list: Vec<BitgetFillRow>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetOrderRow {
    pub(super) order_id: String,
    #[serde(default)]
    pub(super) client_oid: String,
    pub(super) symbol: String,
    pub(super) side: String,
    #[serde(default)]
    pub(super) trade_side: String,
    #[serde(default)]
    pub(super) reduce_only: String,
    pub(super) order_type: String,
    #[serde(default)]
    pub(super) price: String,
    #[serde(default)]
    pub(super) size: String,
    #[serde(default)]
    pub(super) base_volume: String,
    pub(super) status: String,
    #[serde(default)]
    pub(super) u_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetFillRow {
    pub(super) trade_id: String,
    pub(super) order_id: String,
    pub(super) symbol: String,
    pub(super) side: String,
    pub(super) price: String,
    pub(super) size: String,
    pub(super) fee: String,
    pub(super) fee_coin: String,
    #[serde(default)]
    pub(super) profit: String,
    #[serde(default)]
    pub(super) c_time: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct BitgetContractRow {
    pub(super) symbol: String,
    #[serde(default)]
    pub(super) base_coin: String,
    #[serde(default)]
    pub(super) quote_coin: String,
    #[serde(default)]
    pub(super) min_trade_num: String,
    #[serde(default)]
    pub(super) max_trade_num: String,
    #[serde(default)]
    pub(super) size_multiplier: String,
    #[serde(default)]
    pub(super) min_trade_usdt: String,
    #[serde(default)]
    pub(super) price_place: i32,
    #[serde(default)]
    pub(super) volume_place: i32,
}

pub(super) fn parse_bitget_response<T: for<'de> Deserialize<'de>>(
    resp: OutboundResponse,
) -> Result<T, AppError> {
    let status = resp.status;
    let body = resp.body;
    if !status.is_success() {
        return Err(exchange_api_error(status, 0, body));
    }
    let envelope: BitgetEnvelope<T> =
        serde_json::from_str(&body).map_err(AppError::ExchangeJson)?;
    if envelope.code != "00000" {
        return Err(exchange_api_error(
            status,
            envelope.code.parse::<i64>().unwrap_or(0),
            envelope.msg,
        ));
    }
    Ok(envelope.data)
}

pub(super) fn exchange_api_error(status: StatusCode, code: i64, message: String) -> AppError {
    AppError::ExchangeApi {
        status: status.as_u16(),
        code: if code == 0 {
            i64::from(status.as_u16())
        } else {
            code
        },
        message,
    }
}

pub(super) fn sign_bitget(
    secret: &str,
    timestamp: &str,
    method: &str,
    request_path: &str,
    body: &str,
) -> Result<String, AppError> {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| AppError::ExchangeCrypto(e.to_string()))?;
    mac.update(format!("{timestamp}{method}{request_path}{body}").as_bytes());
    Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}

pub(super) fn request_path(path: &str, params: Vec<(&str, String)>) -> String {
    let query = build_query(params);
    if query.is_empty() {
        path.to_string()
    } else {
        format!("{path}?{query}")
    }
}

pub(super) fn build_query(params: Vec<(&str, String)>) -> String {
    params
        .into_iter()
        .map(|(k, v)| format!("{}={}", encode_component(k), encode_component(&v)))
        .collect::<Vec<_>>()
        .join("&")
}

pub(super) fn encode_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(b))
            }
            _ => {
                out.push('%');
                out.push_str(&format!("{:02X}", b));
            }
        }
    }
    out
}

pub(super) fn bitget_symbol(symbol: &str) -> String {
    symbol.trim().to_uppercase().replace('-', "")
}

pub(super) fn bitget_side(side: ExchangeSide) -> &'static str {
    match side {
        ExchangeSide::Buy => "buy",
        ExchangeSide::Sell => "sell",
    }
}

pub(super) fn bitget_trade_side(reduce_only: bool) -> &'static str {
    if reduce_only { "close" } else { "open" }
}

pub(super) fn bitget_margin_mode(mode: ExchangeMarginMode) -> &'static str {
    match mode {
        ExchangeMarginMode::Cross => "crossed",
        ExchangeMarginMode::Isolated => "isolated",
    }
}

pub(super) fn bitget_position_mode(pos_mode: &str) -> BitgetPositionMode {
    match pos_mode.trim().to_ascii_lowercase().as_str() {
        "hedge_mode" => BitgetPositionMode::Hedge,
        _ => BitgetPositionMode::OneWay,
    }
}

pub(super) fn bitget_ack_order_id(payload: &BitgetOrderAck) -> String {
    if payload.order_id.trim().is_empty() {
        payload.client_oid.clone()
    } else {
        payload.order_id.clone()
    }
}

pub(super) fn bitget_order_type(order_type: ExchangeOrderType) -> &'static str {
    match order_type {
        ExchangeOrderType::Market => "market",
        ExchangeOrderType::Limit => "limit",
    }
}

pub(super) fn bitget_force(tif: Option<TimeInForce>) -> &'static str {
    match tif.unwrap_or(TimeInForce::Gtc) {
        TimeInForce::Gtc => "gtc",
        TimeInForce::Ioc => "ioc",
        TimeInForce::Fok => "fok",
    }
}

pub(super) fn bitget_position_side_label(hold_side: &str) -> String {
    match hold_side.trim().to_ascii_lowercase().as_str() {
        "short" => "SHORT".to_string(),
        _ => "LONG".to_string(),
    }
}

pub(super) fn inferred_position_side_for_order(
    side: ExchangeSide,
    reduce_only: bool,
) -> PositionSide {
    match (side, reduce_only) {
        (ExchangeSide::Sell, false) | (ExchangeSide::Buy, true) => PositionSide::Short,
        _ => PositionSide::Long,
    }
}

pub(super) fn validate_order_side(
    side: ExchangeSide,
    position_side: PositionSide,
    reduce_only: bool,
) -> Result<(), AppError> {
    let expected = inferred_position_side_for_order(side, reduce_only);
    if matches!(position_side, PositionSide::Both) || position_side == expected {
        return Ok(());
    }
    Err(AppError::InvalidExchangeConfig(format!(
        "bitget side is inconsistent with position side: side={} position_side={} reduce_only={}",
        exchange_side_label(side),
        position_side_label(position_side),
        reduce_only
    )))
}

pub(super) fn bitget_order_position_side_label(side: &str, reduce_only: bool) -> String {
    match (side.trim().to_ascii_lowercase().as_str(), reduce_only) {
        ("sell", false) | ("buy", true) => "SHORT".to_string(),
        _ => "LONG".to_string(),
    }
}

pub(super) fn position_side_label(side: PositionSide) -> String {
    match side {
        PositionSide::Both => "BOTH",
        PositionSide::Long => "LONG",
        PositionSide::Short => "SHORT",
    }
    .to_string()
}

pub(super) fn exchange_side_label(side: ExchangeSide) -> &'static str {
    match side {
        ExchangeSide::Buy => "BUY",
        ExchangeSide::Sell => "SELL",
    }
}

pub(super) fn order_type_label(order_type: ExchangeOrderType) -> &'static str {
    match order_type {
        ExchangeOrderType::Market => "MARKET",
        ExchangeOrderType::Limit => "LIMIT",
    }
}

pub(super) fn bitget_open_order(row: BitgetOrderRow) -> ExchangeOpenOrder {
    let reduce_only =
        row.reduce_only.eq_ignore_ascii_case("yes") || row.trade_side.eq_ignore_ascii_case("close");
    ExchangeOpenOrder {
        order_id: row.order_id,
        client_order_id: row.client_oid,
        symbol: bitget_symbol(&row.symbol),
        side: row.side.to_ascii_uppercase(),
        position_side: bitget_order_position_side_label(&row.side, reduce_only),
        reduce_only,
        order_type: row.order_type.to_ascii_uppercase(),
        status: row.status,
        price: parse_f64(&row.price),
        orig_qty: parse_f64(&row.size),
        executed_qty: parse_f64(&row.base_volume),
        update_time: row.u_time.parse::<i64>().unwrap_or(0),
    }
}

pub(super) fn bitget_order_detail(row: BitgetOrderRow) -> ExchangeOrderDetail {
    let reduce_only =
        row.reduce_only.eq_ignore_ascii_case("yes") || row.trade_side.eq_ignore_ascii_case("close");
    ExchangeOrderDetail {
        order_id: row.order_id,
        client_order_id: row.client_oid,
        symbol: bitget_symbol(&row.symbol),
        side: row.side.to_ascii_uppercase(),
        position_side: bitget_order_position_side_label(&row.side, reduce_only),
        reduce_only,
        order_type: row.order_type.to_ascii_uppercase(),
        status: row.status,
        price: parse_f64(&row.price),
        orig_qty: parse_f64(&row.size),
        executed_qty: parse_f64(&row.base_volume),
        update_time: row.u_time.parse::<i64>().unwrap_or(0),
    }
}

pub(super) fn decimal_step(place: i32) -> f64 {
    if place <= 0 { 1.0 } else { 10_f64.powi(-place) }
}

pub(super) fn now_millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

pub(super) fn format_decimal(v: f64) -> String {
    let mut s = format!("{:.10}", v);
    while s.contains('.') && s.ends_with('0') {
        s.pop();
    }
    if s.ends_with('.') {
        s.pop();
    }
    if s.is_empty() { "0".to_string() } else { s }
}

pub(super) fn parse_f64(v: &str) -> f64 {
    v.parse::<f64>().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bitget_symbol_normalizes_internal_symbols() {
        assert_eq!(bitget_symbol("btc-usdt"), "BTCUSDT");
        assert_eq!(bitget_symbol("BTCUSDT"), "BTCUSDT");
    }

    #[test]
    fn bitget_signing_returns_base64_hmac() {
        let signature = sign_bitget(
            "secret",
            "16273667805456",
            "GET",
            "/api/v2/mix/account/accounts?productType=USDT-FUTURES",
            "",
        )
        .expect("sign");

        assert!(!signature.is_empty());
    }

    #[test]
    fn bitget_order_position_side_uses_side_and_reduce_only() {
        assert_eq!(bitget_order_position_side_label("sell", false), "SHORT");
        assert_eq!(bitget_order_position_side_label("buy", true), "SHORT");
        assert_eq!(bitget_order_position_side_label("buy", false), "LONG");
        assert_eq!(bitget_order_position_side_label("sell", true), "LONG");
        assert!(matches!(
            inferred_position_side_for_order(ExchangeSide::Buy, true),
            PositionSide::Short
        ));
    }

    #[test]
    fn bitget_validates_position_side_against_order_side() {
        assert!(validate_order_side(ExchangeSide::Buy, PositionSide::Long, false).is_ok());
        assert!(validate_order_side(ExchangeSide::Buy, PositionSide::Short, true).is_ok());
        assert!(validate_order_side(ExchangeSide::Buy, PositionSide::Short, false).is_err());
    }

    #[test]
    fn bitget_reduce_only_ack_can_fall_back_to_client_oid() {
        let ack = BitgetOrderAck {
            order_id: String::new(),
            client_oid: "client-1".to_string(),
        };

        assert_eq!(bitget_ack_order_id(&ack), "client-1");
    }
}
