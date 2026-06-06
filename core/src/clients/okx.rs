use std::collections::HashMap;

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use chrono::{SecondsFormat, Utc};
use hmac::{Hmac, Mac};
use reqwest::{Client, Method, StatusCode};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    clients::{
        exchanges::{
            CancelOrderResponse, ExchangeBalance, ExchangeCredentials, ExchangeMarginMode,
            ExchangeOpenOrder, ExchangeOrderDetail, ExchangeOrderType, ExchangePosition,
            ExchangeSide, ExchangeSymbolConstraints, ExchangeTradeFill, ExchangeUserStreamSession,
            LiveExchangeAdapter, PlaceOrderRequest, PlaceOrderResponse, PositionSide, TimeInForce,
        },
        outbound_http::{OutboundRequestLog, OutboundResponse, send_text},
    },
    error::AppError,
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
pub struct OkxFuturesAdapter {
    client: Client,
    api_key: String,
    secret_key: String,
    passphrase: String,
    simulated: bool,
    base_url: String,
    ws_url: String,
}

impl OkxFuturesAdapter {
    pub fn new(credentials: ExchangeCredentials) -> Result<Self, AppError> {
        if credentials.api_key.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "okx api_key is required".to_string(),
            ));
        }
        if credentials.secret_key.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "okx secret_key is required".to_string(),
            ));
        }
        let Some(passphrase) = credentials.passphrase else {
            return Err(AppError::InvalidExchangeConfig(
                "okx passphrase is required".to_string(),
            ));
        };
        if passphrase.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "okx passphrase is required".to_string(),
            ));
        }

        Ok(Self {
            client: Client::builder().build().map_err(AppError::ExchangeHttp)?,
            api_key: credentials.api_key,
            secret_key: credentials.secret_key,
            passphrase,
            simulated: credentials.testnet,
            base_url: "https://www.okx.com".to_string(),
            ws_url: if credentials.testnet {
                "wss://wspap.okx.com:8443/ws/v5/private"
            } else {
                "wss://ws.okx.com:8443/ws/v5/private"
            }
            .to_string(),
        })
    }

    async fn public_get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        params: Vec<(&str, String)>,
    ) -> Result<T, AppError> {
        let request_path = request_path(path, params);
        let url = format!("{}{}", self.base_url, request_path);
        let resp = send_text(
            self.client.get(&url),
            OutboundRequestLog::new("exchange.okx.public", Method::GET, &url),
        )
        .await
        .map_err(AppError::ExchangeHttp)?;
        parse_okx_response(resp)
    }

    async fn private_get<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        params: Vec<(&str, String)>,
    ) -> Result<T, AppError> {
        self.private_send(Method::GET, path, params, None).await
    }

    async fn private_post<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: Value,
    ) -> Result<T, AppError> {
        self.private_send(Method::POST, path, vec![], Some(body))
            .await
    }

    async fn private_send<T: for<'de> Deserialize<'de>>(
        &self,
        method: Method,
        path: &str,
        params: Vec<(&str, String)>,
        body: Option<Value>,
    ) -> Result<T, AppError> {
        let request_path = request_path(path, params);
        let url = format!("{}{}", self.base_url, request_path);
        let body_text = body
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(AppError::ExchangeJson)?
            .unwrap_or_default();
        let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
        let signature = sign_okx(
            &self.secret_key,
            &timestamp,
            method.as_str(),
            &request_path,
            &body_text,
        )?;

        let mut req = self
            .client
            .request(method.clone(), &url)
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", signature)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .header("Content-Type", "application/json");

        if self.simulated {
            req = req.header("x-simulated-trading", "1");
        }

        if let Some(body) = body {
            req = req.json(&body);
        }

        let resp = send_text(
            req,
            OutboundRequestLog::new("exchange.okx.private", method, &url).body(body_text),
        )
        .await
        .map_err(AppError::ExchangeHttp)?;
        parse_okx_response(resp)
    }

    async fn instrument(&self, symbol: &str) -> Result<OkxInstrument, AppError> {
        let inst_id = okx_inst_id(symbol);
        let mut rows: Vec<OkxInstrument> = self
            .public_get(
                "/api/v5/public/instruments",
                vec![
                    ("instType", "SWAP".to_string()),
                    ("instId", inst_id.clone()),
                ],
            )
            .await?;
        rows.pop().ok_or_else(|| {
            AppError::InvalidExchangeConfig(format!("okx instrument not found: {inst_id}"))
        })
    }

    async fn instruments_by_inst_id(&self) -> Result<HashMap<String, OkxInstrument>, AppError> {
        let rows: Vec<OkxInstrument> = self
            .public_get(
                "/api/v5/public/instruments",
                vec![("instType", "SWAP".to_string())],
            )
            .await?;
        Ok(rows
            .into_iter()
            .map(|inst| (inst.inst_id.clone(), inst))
            .collect())
    }

    async fn position_mode(&self) -> Result<OkxPositionMode, AppError> {
        let rows: Vec<OkxAccountConfig> =
            self.private_get("/api/v5/account/config", vec![]).await?;
        Ok(rows
            .first()
            .map(|row| okx_position_mode(&row.pos_mode))
            .unwrap_or(OkxPositionMode::Net))
    }
}

#[async_trait]
impl LiveExchangeAdapter for OkxFuturesAdapter {
    fn exchange_type(&self) -> &'static str {
        "okx"
    }

    async fn ping(&self) -> Result<(), AppError> {
        let _: Vec<OkxTimePayload> = self.public_get("/api/v5/public/time", vec![]).await?;
        Ok(())
    }

    async fn get_price(&self, symbol: &str) -> Result<f64, AppError> {
        let rows: Vec<OkxTicker> = self
            .public_get(
                "/api/v5/market/ticker",
                vec![("instId", okx_inst_id(symbol))],
            )
            .await?;
        rows.first()
            .map(|row| parse_f64(&row.last))
            .ok_or_else(|| AppError::InvalidExchangeConfig("okx ticker missing".to_string()))
    }

    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, AppError> {
        if req.quantity <= 0.0 {
            return Err(AppError::InvalidExchangeConfig(
                "quantity must be > 0".to_string(),
            ));
        }

        let inst = self.instrument(&req.symbol).await?;
        let position_mode = self.position_mode().await?;
        let size_contracts = okx_order_size_contracts(req.quantity, &inst)?;
        let cl_ord_id = req
            .client_order_id
            .unwrap_or_else(|| format!("amx{}", Uuid::now_v7().simple()));

        let mut body = json!({
            "instId": inst.inst_id,
            "tdMode": okx_td_mode(req.margin_mode.unwrap_or(ExchangeMarginMode::Cross)),
            "side": okx_side(req.side),
            "ordType": okx_order_type(req.order_type, req.time_in_force),
            "sz": format_decimal(size_contracts),
            "clOrdId": cl_ord_id,
            "reduceOnly": if req.reduce_only { "true" } else { "false" },
        });

        if matches!(position_mode, OkxPositionMode::LongShort) {
            let pos_side = req
                .position_side
                .unwrap_or_else(|| inferred_position_side_for_order(req.side, req.reduce_only));
            if matches!(pos_side, PositionSide::Both) {
                return Err(AppError::InvalidExchangeConfig(
                    "okx long/short mode requires position side".to_string(),
                ));
            }
            body["posSide"] = Value::String(okx_pos_side(pos_side).to_string());
        }

        if let ExchangeOrderType::Limit = req.order_type {
            let price = req.price.ok_or_else(|| {
                AppError::InvalidExchangeConfig("limit order requires price".to_string())
            })?;
            body["px"] = Value::String(format_decimal(price));
        }

        let rows: Vec<OkxOrderAck> = self.private_post("/api/v5/trade/order", body).await?;
        let ack = rows.first().ok_or_else(|| {
            AppError::InvalidExchangeConfig("okx order response missing".to_string())
        })?;
        ensure_okx_item_success(ack.s_code.as_deref(), ack.s_msg.as_deref())?;
        if ack.ord_id.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "okx order response missing ordId".to_string(),
            ));
        }

        Ok(PlaceOrderResponse {
            order_id: ack.ord_id.clone(),
            client_order_id: ack.cl_ord_id.clone(),
            symbol: internal_symbol(&ack.inst_id),
            side: exchange_side_label(req.side).to_string(),
            position_side: req
                .position_side
                .map(position_side_label)
                .unwrap_or("".to_string()),
            reduce_only: req.reduce_only,
            status: "submitted".to_string(),
            order_type: order_type_label(req.order_type).to_string(),
            price: req.price.unwrap_or(0.0),
            orig_qty: req.quantity,
            executed_qty: 0.0,
            update_time: now_millis(),
        })
    }

    async fn ensure_symbol_settings(
        &self,
        symbol: &str,
        leverage: i64,
        margin_mode: ExchangeMarginMode,
    ) -> Result<(), AppError> {
        let inst_id = okx_inst_id(symbol);
        let leverage = leverage.clamp(1, 125).to_string();
        let mgn_mode = okx_td_mode(margin_mode);
        let position_mode = self.position_mode().await?;

        let pos_sides = if matches!(position_mode, OkxPositionMode::LongShort) {
            vec![Some("long"), Some("short")]
        } else {
            vec![None]
        };

        for pos_side in pos_sides {
            let mut body = json!({
                "instId": inst_id,
                "lever": leverage,
                "mgnMode": mgn_mode,
            });
            if let Some(pos_side) = pos_side {
                body["posSide"] = Value::String(pos_side.to_string());
            }
            let _: Vec<Value> = self
                .private_post("/api/v5/account/set-leverage", body)
                .await?;
        }

        Ok(())
    }

    async fn cancel_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<CancelOrderResponse, AppError> {
        let rows: Vec<OkxOrderAck> = self
            .private_post(
                "/api/v5/trade/cancel-order",
                json!({
                    "instId": okx_inst_id(symbol),
                    "ordId": order_id.trim(),
                }),
            )
            .await?;
        let ack = rows.first().ok_or_else(|| {
            AppError::InvalidExchangeConfig("okx cancel response missing".to_string())
        })?;
        ensure_okx_item_success(ack.s_code.as_deref(), ack.s_msg.as_deref())?;
        if ack.ord_id.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "okx cancel response missing ordId".to_string(),
            ));
        }

        Ok(CancelOrderResponse {
            order_id: ack.ord_id.clone(),
            client_order_id: ack.cl_ord_id.clone(),
            symbol: internal_symbol(&ack.inst_id),
            status: "canceled".to_string(),
        })
    }

    async fn get_balances(&self) -> Result<Vec<ExchangeBalance>, AppError> {
        let rows: Vec<OkxBalanceRoot> = self
            .private_get("/api/v5/account/balance", vec![("ccy", "USDT".to_string())])
            .await?;
        Ok(rows
            .into_iter()
            .flat_map(|root| root.details)
            .map(|detail| ExchangeBalance {
                asset: detail.ccy,
                wallet_balance: parse_optional_f64(&detail.cash_bal)
                    .unwrap_or(parse_f64(&detail.eq)),
                available_balance: first_present([
                    parse_optional_f64(&detail.avail_eq),
                    parse_optional_f64(&detail.avail_bal),
                    parse_optional_f64(&detail.cash_bal),
                ])
                .unwrap_or(0.0),
                unrealized_pnl: parse_f64(&detail.upl),
            })
            .collect())
    }

    async fn get_positions(&self) -> Result<Vec<ExchangePosition>, AppError> {
        let rows: Vec<OkxPositionRow> = self
            .private_get(
                "/api/v5/account/positions",
                vec![("instType", "SWAP".to_string())],
            )
            .await?;
        let instruments = self.instruments_by_inst_id().await?;
        let mut positions = Vec::new();
        for row in rows {
            let pos_contracts = parse_f64(&row.pos);
            if pos_contracts.abs() <= f64::EPSILON {
                continue;
            }
            let inst = okx_instrument_from_map(&instruments, &row.inst_id)?;
            let quantity = pos_contracts.abs() * okx_contract_size(inst);
            positions.push(ExchangePosition {
                symbol: internal_symbol(&row.inst_id),
                position_side: okx_position_side_label(&row.pos_side, pos_contracts),
                quantity,
                entry_price: parse_f64(&row.avg_px),
                mark_price: parse_f64(&row.mark_px),
                unrealized_pnl: parse_f64(&row.upl),
                leverage: parse_f64(&row.lever).round().max(1.0) as i64,
                liquidation_price: parse_f64(&row.liq_px),
            });
        }
        Ok(positions)
    }

    async fn get_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<ExchangeOpenOrder>, AppError> {
        let mut params = vec![("instType", "SWAP".to_string())];
        if let Some(symbol) = symbol.filter(|s| !s.trim().is_empty()) {
            params.push(("instId", okx_inst_id(symbol)));
        }
        let rows: Vec<OkxOrderRow> = self
            .private_get("/api/v5/trade/orders-pending", params)
            .await?;
        let instruments = self.instruments_by_inst_id().await?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            let ct_val = okx_contract_size(okx_instrument_from_map(&instruments, &row.inst_id)?);
            out.push(okx_open_order(row, ct_val));
        }
        Ok(out)
    }

    async fn get_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<ExchangeOrderDetail, AppError> {
        let rows: Vec<OkxOrderRow> = self
            .private_get(
                "/api/v5/trade/order",
                vec![
                    ("instId", okx_inst_id(symbol)),
                    ("ordId", order_id.trim().to_string()),
                ],
            )
            .await?;
        let row = rows
            .into_iter()
            .next()
            .ok_or_else(|| AppError::InvalidExchangeConfig("okx order missing".to_string()))?;
        let ct_val = okx_contract_size(&self.instrument(&row.inst_id).await?);
        Ok(okx_order_detail(row, ct_val))
    }

    async fn get_order_fills(
        &self,
        _symbol: &str,
        order_id: &str,
    ) -> Result<Vec<ExchangeTradeFill>, AppError> {
        let rows: Vec<OkxFillRow> = self
            .private_get(
                "/api/v5/trade/fills",
                vec![
                    ("instType", "SWAP".to_string()),
                    ("ordId", order_id.trim().to_string()),
                ],
            )
            .await?;
        let instruments = self.instruments_by_inst_id().await?;
        let mut fills = Vec::with_capacity(rows.len());
        for row in rows {
            let ct_val = okx_contract_size(okx_instrument_from_map(&instruments, &row.inst_id)?);
            fills.push(ExchangeTradeFill {
                trade_id: row.trade_id,
                order_id: row.ord_id,
                symbol: internal_symbol(&row.inst_id),
                side: row.side.to_ascii_uppercase(),
                price: parse_f64(&row.fill_px),
                quantity: parse_f64(&row.fill_sz) * ct_val,
                fee: parse_f64(&row.fee),
                fee_asset: row.fee_ccy,
                realized_pnl: parse_f64(&row.fill_pnl),
                executed_at: row.ts.parse::<i64>().unwrap_or(0),
            });
        }
        Ok(fills)
    }

    async fn get_symbol_constraints(
        &self,
        symbol: &str,
    ) -> Result<ExchangeSymbolConstraints, AppError> {
        let inst = self.instrument(symbol).await?;
        let ct_val = okx_contract_size(&inst);
        let min_qty = parse_f64(&inst.min_sz) * ct_val;
        let max_qty = okx_max_qty(&inst) * ct_val;
        let min_notional = self.get_price(symbol).await? * min_qty;
        Ok(ExchangeSymbolConstraints {
            symbol: internal_symbol(&inst.inst_id),
            base_asset: inst.base_ccy,
            quote_asset: inst.quote_ccy,
            min_qty,
            max_qty,
            step_size: parse_f64(&inst.lot_sz) * ct_val,
            min_notional,
            tick_size: parse_f64(&inst.tick_sz),
        })
    }

    async fn start_user_stream(&self) -> Result<String, AppError> {
        Err(AppError::UnsupportedExchange(
            "okx user stream is not wired into runtime yet".to_string(),
        ))
    }

    async fn keepalive_user_stream(&self, _listen_key: &str) -> Result<(), AppError> {
        Ok(())
    }

    async fn close_user_stream(&self, _listen_key: &str) -> Result<(), AppError> {
        Ok(())
    }

    fn user_stream_ws_url(&self, _listen_key: &str) -> Result<String, AppError> {
        Err(AppError::UnsupportedExchange(
            "okx user stream is not wired into runtime yet".to_string(),
        ))
    }

    async fn user_stream_session(&self) -> Result<ExchangeUserStreamSession, AppError> {
        let timestamp = Utc::now().timestamp().to_string();
        let signature = sign_okx(
            &self.secret_key,
            &timestamp,
            "GET",
            "/users/self/verify",
            "",
        )?;
        let multipliers = self
            .instruments_by_inst_id()
            .await?
            .into_iter()
            .flat_map(|(inst_id, inst)| {
                let ct_val = okx_contract_size(&inst);
                [
                    (inst_id.clone(), ct_val),
                    (internal_symbol(&inst_id), ct_val),
                ]
            })
            .collect::<HashMap<_, _>>();

        Ok(
            ExchangeUserStreamSession::private_ws("okx", self.ws_url.clone())
                .with_quantity_multipliers(multipliers)
                .with_initial_json(json!({
                    "op": "login",
                    "args": [{
                        "apiKey": self.api_key,
                        "passphrase": self.passphrase,
                        "timestamp": timestamp,
                        "sign": signature,
                    }]
                }))
                .with_initial_json(json!({
                    "op": "subscribe",
                    "args": [
                        {"channel": "orders", "instType": "SWAP"},
                        {"channel": "account"},
                        {"channel": "positions", "instType": "SWAP"},
                        {"channel": "balance_and_position"}
                    ]
                })),
        )
    }
}

mod support;

use support::*;
