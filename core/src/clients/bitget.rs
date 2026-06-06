use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use reqwest::{Client, Method, StatusCode};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::Sha256;
use uuid::Uuid;

use crate::{
    clients::{
        exchanges::{
            CancelOrderResponse, ExchangeBalance, ExchangeCredentials, ExchangeOpenOrder,
            ExchangeOrderDetail, ExchangeOrderType, ExchangePosition, ExchangeSide,
            ExchangeSymbolConstraints, ExchangeTradeFill, LiveExchangeAdapter, PlaceOrderRequest,
            PlaceOrderResponse, PositionSide, TimeInForce,
        },
        outbound_http::{OutboundRequestLog, OutboundResponse, send_text},
    },
    error::AppError,
};

type HmacSha256 = Hmac<Sha256>;

const PRODUCT_TYPE: &str = "USDT-FUTURES";
const MARGIN_COIN: &str = "USDT";

#[derive(Debug, Clone)]
pub struct BitgetFuturesAdapter {
    client: Client,
    api_key: String,
    secret_key: String,
    passphrase: String,
    simulated: bool,
    base_url: String,
}

impl BitgetFuturesAdapter {
    pub fn new(credentials: ExchangeCredentials) -> Result<Self, AppError> {
        if credentials.api_key.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "bitget api_key is required".to_string(),
            ));
        }
        if credentials.secret_key.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "bitget secret_key is required".to_string(),
            ));
        }
        let Some(passphrase) = credentials.passphrase else {
            return Err(AppError::InvalidExchangeConfig(
                "bitget passphrase is required".to_string(),
            ));
        };
        if passphrase.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "bitget passphrase is required".to_string(),
            ));
        }

        Ok(Self {
            client: Client::builder().build().map_err(AppError::ExchangeHttp)?,
            api_key: credentials.api_key,
            secret_key: credentials.secret_key,
            passphrase,
            simulated: credentials.testnet,
            base_url: "https://api.bitget.com".to_string(),
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
            OutboundRequestLog::new("exchange.bitget.public", Method::GET, &url),
        )
        .await
        .map_err(AppError::ExchangeHttp)?;
        parse_bitget_response(resp)
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
        let timestamp = now_millis().to_string();
        let body_text = body
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(AppError::ExchangeJson)?
            .unwrap_or_default();
        let signature = sign_bitget(
            &self.secret_key,
            &timestamp,
            method.as_str(),
            &request_path,
            &body_text,
        )?;

        let mut req = self
            .client
            .request(method.clone(), &url)
            .header("ACCESS-KEY", &self.api_key)
            .header("ACCESS-SIGN", signature)
            .header("ACCESS-TIMESTAMP", timestamp)
            .header("ACCESS-PASSPHRASE", &self.passphrase)
            .header("locale", "en-US")
            .header("Content-Type", "application/json");

        if self.simulated {
            req = req.header("paptrading", "1");
        }

        if let Some(body) = body {
            req = req.json(&body);
        }

        let resp = send_text(
            req,
            OutboundRequestLog::new("exchange.bitget.private", method, &url).body(body_text),
        )
        .await
        .map_err(AppError::ExchangeHttp)?;
        parse_bitget_response(resp)
    }

    async fn position_mode(&self) -> Result<BitgetPositionMode, AppError> {
        let rows: Vec<BitgetAccountRow> = self
            .private_get(
                "/api/v2/mix/account/accounts",
                vec![("productType", PRODUCT_TYPE.to_string())],
            )
            .await?;
        Ok(rows
            .first()
            .map(|row| bitget_position_mode(&row.pos_mode))
            .unwrap_or(BitgetPositionMode::OneWay))
    }
}

#[async_trait]
impl LiveExchangeAdapter for BitgetFuturesAdapter {
    fn exchange_type(&self) -> &'static str {
        "bitget"
    }

    async fn ping(&self) -> Result<(), AppError> {
        let _: Vec<BitgetContractRow> = self
            .public_get(
                "/api/v2/mix/market/contracts",
                vec![("productType", PRODUCT_TYPE.to_string())],
            )
            .await?;
        Ok(())
    }

    async fn get_price(&self, symbol: &str) -> Result<f64, AppError> {
        let rows: Vec<BitgetTickerRow> = self
            .public_get(
                "/api/v2/mix/market/tickers",
                vec![
                    ("productType", PRODUCT_TYPE.to_string()),
                    ("symbol", bitget_symbol(symbol)),
                ],
            )
            .await?;
        rows.first()
            .map(|row| parse_f64(&row.last_pr))
            .ok_or_else(|| AppError::InvalidExchangeConfig("bitget ticker missing".to_string()))
    }

    async fn place_order(&self, req: PlaceOrderRequest) -> Result<PlaceOrderResponse, AppError> {
        if req.quantity <= 0.0 {
            return Err(AppError::InvalidExchangeConfig(
                "quantity must be > 0".to_string(),
            ));
        }

        let symbol = bitget_symbol(&req.symbol);
        let position_side = req
            .position_side
            .unwrap_or_else(|| inferred_position_side_for_order(req.side, req.reduce_only));
        validate_order_side(req.side, position_side, req.reduce_only)?;
        let position_mode = self.position_mode().await?;
        let client_oid = req
            .client_order_id
            .unwrap_or_else(|| format!("amx{}", Uuid::now_v7().simple()));

        let mut body = json!({
            "symbol": symbol,
            "productType": PRODUCT_TYPE,
            "marginMode": "crossed",
            "marginCoin": MARGIN_COIN,
            "size": format_decimal(req.quantity),
            "side": bitget_side(req.side),
            "orderType": bitget_order_type(req.order_type),
            "clientOid": client_oid,
        });

        match position_mode {
            BitgetPositionMode::Hedge => {
                body["tradeSide"] = Value::String(bitget_trade_side(req.reduce_only).to_string());
            }
            BitgetPositionMode::OneWay => {
                if req.reduce_only {
                    body["reduceOnly"] = Value::String("YES".to_string());
                }
            }
        }

        if let ExchangeOrderType::Limit = req.order_type {
            let price = req.price.ok_or_else(|| {
                AppError::InvalidExchangeConfig("limit order requires price".to_string())
            })?;
            body["price"] = Value::String(format_decimal(price));
            body["force"] = Value::String(bitget_force(req.time_in_force).to_string());
        }

        let payload: BitgetOrderAck = self
            .private_post("/api/v2/mix/order/place-order", body)
            .await?;
        let order_id = bitget_ack_order_id(&payload);
        if order_id.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "bitget order response missing orderId and clientOid".to_string(),
            ));
        }

        Ok(PlaceOrderResponse {
            order_id,
            client_order_id: payload.client_oid,
            symbol,
            side: exchange_side_label(req.side).to_string(),
            position_side: position_side_label(position_side),
            reduce_only: req.reduce_only,
            status: "submitted".to_string(),
            order_type: order_type_label(req.order_type).to_string(),
            price: req.price.unwrap_or(0.0),
            orig_qty: req.quantity,
            executed_qty: 0.0,
            update_time: now_millis(),
        })
    }

    async fn cancel_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<CancelOrderResponse, AppError> {
        let payload: BitgetCancelAck = self
            .private_post(
                "/api/v2/mix/order/cancel-order",
                json!({
                    "symbol": bitget_symbol(symbol),
                    "productType": PRODUCT_TYPE,
                    "orderId": order_id.trim(),
                }),
            )
            .await?;
        if payload.order_id.trim().is_empty() {
            return Err(AppError::InvalidExchangeConfig(
                "bitget cancel response missing orderId".to_string(),
            ));
        }

        Ok(CancelOrderResponse {
            order_id: payload.order_id,
            client_order_id: payload.client_oid,
            symbol: bitget_symbol(symbol),
            status: "canceled".to_string(),
        })
    }

    async fn get_balances(&self) -> Result<Vec<ExchangeBalance>, AppError> {
        let rows: Vec<BitgetAccountRow> = self
            .private_get(
                "/api/v2/mix/account/accounts",
                vec![("productType", PRODUCT_TYPE.to_string())],
            )
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| ExchangeBalance {
                asset: row.margin_coin,
                wallet_balance: parse_f64(&row.account_equity),
                available_balance: parse_f64(&row.available),
                unrealized_pnl: parse_f64(&row.unrealized_pl),
            })
            .collect())
    }

    async fn get_positions(&self) -> Result<Vec<ExchangePosition>, AppError> {
        let rows: Vec<BitgetPositionRow> = self
            .private_get(
                "/api/v2/mix/position/all-position",
                vec![
                    ("productType", PRODUCT_TYPE.to_string()),
                    ("marginCoin", MARGIN_COIN.to_string()),
                ],
            )
            .await?;
        Ok(rows
            .into_iter()
            .filter_map(|row| {
                let quantity = parse_f64(&row.total).abs();
                if quantity <= f64::EPSILON {
                    return None;
                }
                Some(ExchangePosition {
                    symbol: bitget_symbol(&row.symbol),
                    position_side: bitget_position_side_label(&row.hold_side),
                    quantity,
                    entry_price: parse_f64(&row.open_price_avg),
                    mark_price: parse_f64(&row.mark_price),
                    unrealized_pnl: parse_f64(&row.unrealized_pl),
                    leverage: parse_f64(&row.leverage).round().max(1.0) as i64,
                    liquidation_price: parse_f64(&row.liquidation_price),
                })
            })
            .collect())
    }

    async fn get_open_orders(
        &self,
        symbol: Option<&str>,
    ) -> Result<Vec<ExchangeOpenOrder>, AppError> {
        let mut params = vec![("productType", PRODUCT_TYPE.to_string())];
        if let Some(symbol) = symbol.filter(|s| !s.trim().is_empty()) {
            params.push(("symbol", bitget_symbol(symbol)));
        }
        let payload: BitgetOrderPage = self
            .private_get("/api/v2/mix/order/orders-pending", params)
            .await?;
        Ok(payload
            .entrusted_list
            .into_iter()
            .map(bitget_open_order)
            .collect())
    }

    async fn get_order(
        &self,
        symbol: &str,
        order_id: &str,
    ) -> Result<ExchangeOrderDetail, AppError> {
        let row: BitgetOrderRow = self
            .private_get(
                "/api/v2/mix/order/detail",
                vec![
                    ("symbol", bitget_symbol(symbol)),
                    ("productType", PRODUCT_TYPE.to_string()),
                    ("orderId", order_id.trim().to_string()),
                ],
            )
            .await?;
        Ok(bitget_order_detail(row))
    }

    async fn get_order_fills(
        &self,
        _symbol: &str,
        order_id: &str,
    ) -> Result<Vec<ExchangeTradeFill>, AppError> {
        let payload: BitgetFillPage = self
            .private_get(
                "/api/v2/mix/order/fills",
                vec![
                    ("productType", PRODUCT_TYPE.to_string()),
                    ("orderId", order_id.trim().to_string()),
                ],
            )
            .await?;
        Ok(payload
            .fill_list
            .into_iter()
            .map(|row| ExchangeTradeFill {
                trade_id: row.trade_id,
                order_id: row.order_id,
                symbol: bitget_symbol(&row.symbol),
                side: row.side.to_ascii_uppercase(),
                price: parse_f64(&row.price),
                quantity: parse_f64(&row.size),
                fee: parse_f64(&row.fee),
                fee_asset: row.fee_coin,
                realized_pnl: parse_f64(&row.profit),
                executed_at: row.c_time.parse::<i64>().unwrap_or(0),
            })
            .collect())
    }

    async fn get_symbol_constraints(
        &self,
        symbol: &str,
    ) -> Result<ExchangeSymbolConstraints, AppError> {
        let target = bitget_symbol(symbol);
        let rows: Vec<BitgetContractRow> = self
            .public_get(
                "/api/v2/mix/market/contracts",
                vec![
                    ("productType", PRODUCT_TYPE.to_string()),
                    ("symbol", target.clone()),
                ],
            )
            .await?;
        let row = rows
            .into_iter()
            .find(|row| bitget_symbol(&row.symbol) == target)
            .ok_or_else(|| {
                AppError::InvalidExchangeConfig(format!("bitget contract not found: {target}"))
            })?;
        Ok(ExchangeSymbolConstraints {
            symbol: bitget_symbol(&row.symbol),
            base_asset: row.base_coin,
            quote_asset: row.quote_coin,
            min_qty: parse_f64(&row.min_trade_num),
            max_qty: parse_f64(&row.max_trade_num),
            step_size: parse_f64(&row.size_multiplier).max(decimal_step(row.volume_place)),
            min_notional: parse_f64(&row.min_trade_usdt),
            tick_size: decimal_step(row.price_place),
        })
    }

    async fn start_user_stream(&self) -> Result<String, AppError> {
        Err(AppError::UnsupportedExchange(
            "bitget user stream is not wired into runtime yet".to_string(),
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
            "bitget user stream is not wired into runtime yet".to_string(),
        ))
    }
}

mod support;

use support::*;
