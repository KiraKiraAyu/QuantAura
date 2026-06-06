use async_trait::async_trait;
use k256::ecdsa::SigningKey;
use reqwest::{Client, Method, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sha3::{Digest, Keccak256};
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

const HYPERLIQUID_MIN_NOTIONAL_USDC: f64 = 10.0;
const HYPERLIQUID_MAX_QTY_UNKNOWN: f64 = 0.0;

#[derive(Debug, Clone)]
pub struct HyperliquidAdapter {
    client: Client,
    wallet_addr: String,
    private_key: String,
    base_url: String,
    ws_url: String,
    testnet: bool,
}

impl HyperliquidAdapter {
    pub fn new(credentials: ExchangeCredentials) -> Result<Self, AppError> {
        let Some(wallet_addr) = credentials.wallet_addr else {
            return Err(AppError::InvalidExchangeConfig(
                "hyperliquid wallet address is required".to_string(),
            ));
        };
        if wallet_addr.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "hyperliquid wallet address is required".to_string(),
            ));
        }
        if credentials.secret_key.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "hyperliquid private key is required".to_string(),
            ));
        }

        let base_url = if credentials.testnet {
            "https://api.hyperliquid-testnet.xyz"
        } else {
            "https://api.hyperliquid.xyz"
        };
        let ws_url = if credentials.testnet {
            "wss://api.hyperliquid-testnet.xyz/ws"
        } else {
            "wss://api.hyperliquid.xyz/ws"
        };

        Ok(Self {
            client: Client::builder().build().map_err(AppError::ExchangeHttp)?,
            wallet_addr: normalize_address(&wallet_addr),
            private_key: credentials.secret_key,
            base_url: base_url.to_string(),
            ws_url: ws_url.to_string(),
            testnet: credentials.testnet,
        })
    }

    async fn info(&self, body: Value) -> Result<Value, AppError> {
        let url = format!("{}/info", self.base_url);
        let body_text = serde_json::to_string(&body).map_err(AppError::ExchangeJson)?;
        let resp = send_text(
            self.client.post(&url).json(&body),
            OutboundRequestLog::new("exchange.hyperliquid.info", Method::POST, &url)
                .body(body_text),
        )
        .await
        .map_err(AppError::ExchangeHttp)?;
        parse_json_response(resp)
    }

    async fn exchange<T: Serialize>(&self, action: &T, nonce: i64) -> Result<Value, AppError> {
        let signature = sign_l1_action(
            &self.private_key,
            action,
            nonce,
            None,
            if self.testnet { "b" } else { "a" },
        )?;
        let body = HyperliquidExchangeRequest {
            action,
            nonce,
            signature,
            vault_address: None,
        };
        let url = format!("{}/exchange", self.base_url);
        let body_text = serde_json::to_string(&body).map_err(AppError::ExchangeJson)?;
        let resp = send_text(
            self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(body_text.clone()),
            OutboundRequestLog::new("exchange.hyperliquid.exchange", Method::POST, &url)
                .body(body_text),
        )
        .await
        .map_err(AppError::ExchangeHttp)?;
        parse_json_response(resp)
    }

    async fn meta(&self) -> Result<Vec<HyperliquidAsset>, AppError> {
        let value = self.info(json!({ "type": "meta" })).await?;
        let universe = value
            .get("universe")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        Ok(universe
            .into_iter()
            .enumerate()
            .filter_map(|(index, value)| {
                let coin = value.get("name")?.as_str()?.to_string();
                let sz_decimals =
                    value.get("szDecimals").and_then(Value::as_i64).unwrap_or(4) as i32;
                let max_leverage = value
                    .get("maxLeverage")
                    .and_then(Value::as_i64)
                    .unwrap_or(1);
                Some(HyperliquidAsset {
                    index,
                    coin,
                    sz_decimals,
                    max_leverage,
                })
            })
            .collect())
    }

    async fn asset(&self, symbol: &str) -> Result<HyperliquidAsset, AppError> {
        let coin = hyperliquid_coin(symbol);
        self.meta()
            .await?
            .into_iter()
            .find(|asset| asset.coin.eq_ignore_ascii_case(&coin))
            .ok_or_else(|| {
                AppError::InvalidExchangeConfig(format!("hyperliquid asset not found: {coin}"))
            })
    }

    async fn all_mids(&self) -> Result<Map<String, Value>, AppError> {
        self.info(json!({ "type": "allMids" }))
            .await?
            .as_object()
            .cloned()
            .ok_or_else(|| {
                AppError::InvalidExchangeConfig("hyperliquid allMids missing".to_string())
            })
    }
}

#[async_trait]
impl LiveExchangeAdapter for HyperliquidAdapter {
    fn exchange_type(&self) -> &'static str {
        "hyperliquid"
    }

    async fn ping(&self) -> Result<(), AppError> {
        let _ = self.info(json!({ "type": "meta" })).await?;
        Ok(())
    }

    async fn get_price(&self, symbol: &str) -> Result<f64, AppError> {
        let coin = hyperliquid_coin(symbol);
        self.all_mids()
            .await?
            .get(&coin)
            .and_then(Value::as_str)
            .map(parse_f64)
            .ok_or_else(|| {
                AppError::InvalidExchangeConfig(format!("hyperliquid mid missing: {coin}"))
            })
    }

    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, AppError> {
        if req.quantity <= 0.0 {
            return Err(AppError::InvalidExchangeConfig(
                "quantity must be > 0".to_string(),
            ));
        }

        let asset = self.asset(&req.symbol).await?;
        let coin = asset.coin.clone();
        let reference_price = match req.order_type {
            ExchangeOrderType::Limit => req.price.ok_or_else(|| {
                AppError::InvalidExchangeConfig("limit order requires price".to_string())
            })?,
            ExchangeOrderType::Market => {
                let mid = self.get_price(&coin).await?;
                if matches!(req.side, ExchangeSide::Buy) {
                    mid * 1.05
                } else {
                    mid * 0.95
                }
            }
        };
        let tif = match req.order_type {
            ExchangeOrderType::Market => "Ioc",
            ExchangeOrderType::Limit => match req.time_in_force.unwrap_or(TimeInForce::Gtc) {
                TimeInForce::Gtc => "Gtc",
                TimeInForce::Ioc => "Ioc",
                TimeInForce::Fok => "Ioc",
            },
        };
        let cloid = hyperliquid_cloid(req.client_order_id);
        let nonce = now_millis();
        let action = HyperliquidAction::Order {
            orders: vec![HyperliquidOrderAction {
                a: asset.index,
                b: matches!(req.side, ExchangeSide::Buy),
                p: format_hyperliquid_price(reference_price, asset.sz_decimals),
                s: format_hyperliquid_size(req.quantity, asset.sz_decimals),
                r: req.reduce_only,
                t: HyperliquidOrderWireType {
                    limit: HyperliquidLimitType {
                        tif: tif.to_string(),
                    },
                },
                c: Some(cloid.clone()),
            }],
            grouping: "na".to_string(),
        };
        let response = self.exchange(&action, nonce).await?;
        ensure_hyperliquid_status_ok(&response)?;
        let order_id = response
            .pointer("/response/data/statuses/0/resting/oid")
            .and_then(Value::as_i64)
            .or_else(|| {
                response
                    .pointer("/response/data/statuses/0/filled/oid")
                    .and_then(Value::as_i64)
            })
            .map(|v| v.to_string())
            .unwrap_or_default();
        if order_id.is_empty() {
            return Err(AppError::ExchangeApi {
                status: 200,
                code: 0,
                message: format!("hyperliquid order response missing oid: {response}"),
            });
        }

        Ok(PlaceOrderResponse {
            order_id,
            client_order_id: cloid,
            symbol: internal_symbol(&coin),
            side: exchange_side_label(req.side).to_string(),
            position_side: req
                .position_side
                .map(position_side_label)
                .unwrap_or_default(),
            reduce_only: req.reduce_only,
            status: "submitted".to_string(),
            order_type: order_type_label(req.order_type).to_string(),
            price: reference_price,
            orig_qty: req.quantity,
            executed_qty: response
                .pointer("/response/data/statuses/0/filled/totalSz")
                .and_then(Value::as_str)
                .map(parse_f64)
                .unwrap_or(0.0),
            update_time: nonce,
        })
    }

    async fn ensure_symbol_settings(
        &self,
        symbol: &str,
        leverage: i64,
        margin_mode: ExchangeMarginMode,
    ) -> Result<(), AppError> {
        let asset = self.asset(symbol).await?;
        let leverage = leverage.clamp(1, 125);
        if leverage > asset.max_leverage {
            return Err(AppError::InvalidExchangeConfig(format!(
                "hyperliquid leverage {} exceeds max {} for {}",
                leverage, asset.max_leverage, asset.coin
            )));
        }
        let response = self
            .exchange(
                &HyperliquidAction::UpdateLeverage {
                    asset: asset.index,
                    is_cross: matches!(margin_mode, ExchangeMarginMode::Cross),
                    leverage: leverage as u32,
                },
                now_millis(),
            )
            .await?;
        ensure_hyperliquid_status_ok(&response)?;
        Ok(())
    }

    async fn cancel_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<CancelOrderResponse, AppError> {
        let asset = self.asset(symbol).await?;
        let oid = order_id.trim().parse::<i64>().map_err(|_| {
            AppError::InvalidExchangeConfig("hyperliquid numeric order_id is required".to_string())
        })?;
        let response = self
            .exchange(
                &HyperliquidAction::Cancel {
                    cancels: vec![HyperliquidCancelAction {
                        a: asset.index,
                        o: oid,
                    }],
                },
                now_millis(),
            )
            .await?;
        ensure_hyperliquid_status_ok(&response)?;

        Ok(CancelOrderResponse {
            order_id: order_id.trim().to_string(),
            client_order_id: String::new(),
            symbol: internal_symbol(&asset.coin),
            status: "canceled".to_string(),
        })
    }

    async fn get_balances(&self) -> Result<Vec<ExchangeBalance>, AppError> {
        let state = self
            .info(json!({ "type": "clearinghouseState", "user": self.wallet_addr }))
            .await?;
        let margin = state.get("marginSummary").cloned().unwrap_or(Value::Null);
        let unrealized_pnl = state
            .get("assetPositions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|row| {
                row.get("position")
                    .and_then(|p| p.get("unrealizedPnl"))
                    .and_then(Value::as_str)
                    .map(parse_f64)
                    .unwrap_or(0.0)
            })
            .sum::<f64>();
        Ok(vec![ExchangeBalance {
            asset: "USDC".to_string(),
            wallet_balance: margin
                .get("accountValue")
                .and_then(Value::as_str)
                .map(parse_f64)
                .unwrap_or(0.0),
            available_balance: state
                .get("withdrawable")
                .and_then(Value::as_str)
                .map(parse_f64)
                .unwrap_or(0.0),
            unrealized_pnl,
        }])
    }

    async fn get_positions(&self) -> Result<Vec<ExchangePosition>, AppError> {
        let mids = self.all_mids().await?;
        let state = self
            .info(json!({ "type": "clearinghouseState", "user": self.wallet_addr }))
            .await?;
        Ok(state
            .get("assetPositions")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|row| {
                let position = row.get("position")?;
                let coin = position.get("coin")?.as_str()?.to_string();
                let signed_qty = position.get("szi")?.as_str().map(parse_f64)?;
                if signed_qty.abs() <= f64::EPSILON {
                    return None;
                }
                let mark_price = mids
                    .get(&coin)
                    .and_then(Value::as_str)
                    .map(parse_f64)
                    .unwrap_or(0.0);
                Some(ExchangePosition {
                    symbol: internal_symbol(&coin),
                    position_side: if signed_qty < 0.0 { "SHORT" } else { "LONG" }.to_string(),
                    quantity: signed_qty.abs(),
                    entry_price: position
                        .get("entryPx")
                        .and_then(Value::as_str)
                        .map(parse_f64)
                        .unwrap_or(0.0),
                    mark_price,
                    unrealized_pnl: position
                        .get("unrealizedPnl")
                        .and_then(Value::as_str)
                        .map(parse_f64)
                        .unwrap_or(0.0),
                    leverage: position
                        .get("leverage")
                        .and_then(|v| v.get("value"))
                        .and_then(Value::as_i64)
                        .unwrap_or(1),
                    liquidation_price: position
                        .get("liquidationPx")
                        .and_then(Value::as_str)
                        .map(parse_f64)
                        .unwrap_or(0.0),
                })
            })
            .collect())
    }

    async fn get_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<ExchangeOpenOrder>, AppError> {
        let target = symbol.map(hyperliquid_coin);
        let rows = self
            .info(json!({ "type": "frontendOpenOrders", "user": self.wallet_addr }))
            .await?;
        Ok(rows
            .as_array()
            .into_iter()
            .flatten()
            .filter(|row| {
                target
                    .as_ref()
                    .is_none_or(|coin| row.get("coin").and_then(Value::as_str) == Some(coin))
            })
            .map(hyperliquid_open_order)
            .collect())
    }

    async fn get_order(
        &self,
        _symbol: &str,
        order_id: &str,
    ) -> Result<ExchangeOrderDetail, AppError> {
        let oid = order_id.trim().parse::<i64>().map_err(|_| {
            AppError::InvalidExchangeConfig("hyperliquid numeric order_id is required".to_string())
        })?;
        let row = self
            .info(json!({ "type": "orderStatus", "user": self.wallet_addr, "oid": oid }))
            .await?;
        Ok(hyperliquid_order_status_detail(&row, order_id.trim()))
    }

    async fn get_order_fills(
        &self,
        _symbol: &str,
        order_id: &str,
    ) -> Result<Vec<ExchangeTradeFill>, AppError> {
        let oid = order_id.trim();
        let rows = self
            .info(json!({ "type": "userFills", "user": self.wallet_addr }))
            .await?;
        Ok(rows
            .as_array()
            .into_iter()
            .flatten()
            .filter(|row| {
                row.get("oid")
                    .and_then(Value::as_i64)
                    .map(|v| v.to_string())
                    .as_deref()
                    == Some(oid)
            })
            .map(hyperliquid_fill)
            .collect())
    }

    async fn get_symbol_constraints(
        &self,
        symbol: &str,
    ) -> Result<ExchangeSymbolConstraints, AppError> {
        let asset = self.asset(symbol).await?;
        let price = self.get_price(symbol).await?;
        let step_size = 10_f64.powi(-asset.sz_decimals);
        Ok(ExchangeSymbolConstraints {
            symbol: internal_symbol(&asset.coin),
            base_asset: asset.coin,
            quote_asset: "USDC".to_string(),
            min_qty: step_size,
            max_qty: HYPERLIQUID_MAX_QTY_UNKNOWN,
            step_size,
            min_notional: HYPERLIQUID_MIN_NOTIONAL_USDC,
            tick_size: hyperliquid_price_tick(price, asset.sz_decimals),
        })
    }

    async fn start_user_stream(&self) -> Result<String, AppError> {
        Err(AppError::UnsupportedExchange(
            "hyperliquid user stream is not wired into runtime yet".to_string(),
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
            "hyperliquid user stream is not wired into runtime yet".to_string(),
        ))
    }

    async fn user_stream_session(&self) -> Result<ExchangeUserStreamSession, AppError> {
        Ok(
            ExchangeUserStreamSession::private_ws("hyperliquid", self.ws_url.clone())
                .without_heartbeat()
                .with_initial_json(json!({
                    "method": "subscribe",
                    "subscription": {"type": "userEvents", "user": self.wallet_addr}
                }))
                .with_initial_json(json!({
                    "method": "subscribe",
                    "subscription": {"type": "orderUpdates", "user": self.wallet_addr}
                }))
                .with_initial_json(json!({
                    "method": "subscribe",
                    "subscription": {"type": "webData2", "user": self.wallet_addr}
                })),
        )
    }
}

mod support;

use support::*;
