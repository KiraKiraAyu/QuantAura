use super::service::*;

pub async fn sync_balance(
    app: &SharedState,
    user_id: &str,
    id: &str,
) -> AppResult<TraderBalanceSyncPayload> {
    let trader = match get_trader_by_owner(app, user_id, id).await {
        Ok(Some(t)) => t,
        Ok(None) => return Err(app_error(AppErrorKind::NotFound, "Trader does not exist")),
        Err(_) => {
            return Err(app_error(AppErrorKind::Internal, "Failed to load trader"));
        }
    };

    let account = match app.trading_repo.latest_account(user_id, id).await {
        Ok(Some(account)) => account,
        Ok(None) => TraderAccountRecord {
            trader_id: id.to_string(),
            total_balance: trader.initial_balance.max(0.0),
            available_balance: trader.initial_balance.max(0.0),
            used_margin: 0.0,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            currency: "USDT".to_string(),
            snapshot_at: now_ts(),
        },
        Err(_) => {
            return Err(app_error(AppErrorKind::Internal, "Failed to sync account"));
        }
    };

    let now = now_ts();
    let snapshot_id = Uuid::now_v7().to_string();
    let result = app
        .trading_repo
        .insert_account_snapshot(snapshot_id, user_id, id, &trader.exchange_id, &account, now)
        .await;

    match result {
        Ok(_) => Ok(TraderBalanceSyncPayload {
            message: "Balance synced",
            account: account_payload(id.to_string(), account.with_snapshot_at(now)),
        }),
        Err(_) => Err(app_error(
            AppErrorKind::Internal,
            "Failed to persist balance snapshot",
        )),
    }
}

pub async fn close_position(
    app: &SharedState,
    user_id: &str,
    id: &str,
    req: ClosePositionRequest,
) -> AppResult<ClosePositionPayload> {
    let symbol = req.symbol.trim().to_uppercase();
    let side = req.side.trim().to_uppercase();
    if symbol.is_empty() || (side != "LONG" && side != "SHORT") {
        return Err(app_error(
            AppErrorKind::BadRequest,
            "symbol and side(LONG/SHORT) are required",
        ));
    }

    if trader_owner_missing(app, user_id, id).await {
        return Err(app_error(
            AppErrorKind::NotFound,
            "Trader does not exist or no permission",
        ));
    }

    let open_positions = app
        .trading_repo
        .open_position_records(user_id, id, Some(&symbol), Some(&side))
        .await;
    let position_count = match open_positions {
        Ok(v) if !v.is_empty() => v.len(),
        Ok(_) => return Err(app_error(AppErrorKind::NotFound, "No open position found")),
        Err(_) => {
            return Err(app_error(AppErrorKind::Internal, "Failed to load position"));
        }
    };

    let trade_ids = (0..position_count)
        .map(|_| Uuid::now_v7().to_string())
        .collect();
    let closed = app
        .trading_repo
        .close_open_positions(user_id, id, &symbol, &side, trade_ids, now_ts())
        .await;

    match closed {
        Ok(_) => Ok(ClosePositionPayload {
            message: "Position closed",
            symbol,
            side,
        }),
        Err(_) => Err(app_error(
            AppErrorKind::Internal,
            "Failed to close position",
        )),
    }
}

pub async fn grid_risk_info(
    app: &SharedState,
    user_id: &str,
    id: &str,
) -> AppResult<GridRiskInfoPayload> {
    if trader_owner_missing(app, user_id, id).await {
        return Err(app_error(
            AppErrorKind::NotFound,
            "Trader does not exist or no permission",
        ));
    }

    let rows = app
        .trading_repo
        .open_position_records(user_id, id, None, None)
        .await;

    match rows {
        Ok(open_positions) => {
            let mut total_notional = 0.0;
            let mut by_symbol: HashMap<String, f64> = HashMap::new();
            for position in open_positions {
                let notional = (position.quantity * position.mark_price).abs();
                total_notional += notional;
                *by_symbol.entry(position.symbol).or_insert(0.0) += notional;
            }

            let mut symbol_concentration: Vec<SymbolConcentrationPayload> = by_symbol
                .into_iter()
                .map(|(symbol, value)| SymbolConcentrationPayload {
                    symbol,
                    notional: value,
                    weight_pct: if total_notional > 0.0 {
                        value * 100.0 / total_notional
                    } else {
                        0.0
                    },
                })
                .collect();

            symbol_concentration.sort_by(|a, b| {
                b.notional
                    .partial_cmp(&a.notional)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            Ok(GridRiskInfoPayload {
                trader_id: id.to_string(),
                total_notional,
                symbol_concentration,
            })
        }
        Err(_) => Err(app_error(
            AppErrorKind::Internal,
            "Failed to get grid risk info",
        )),
    }
}

pub async fn status(
    app: &SharedState,
    user_id: &str,
    q: TraderQuery,
) -> AppResult<TraderStatusPayload> {
    let trader_id = match resolve_trader_id(app, user_id, q.trader_id).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let trader = match get_trader_by_owner(app, user_id, &trader_id).await {
        Ok(Some(t)) => t,
        _ => return Err(app_error(AppErrorKind::NotFound, "Trader not found")),
    };

    let (open_positions, open_orders) = app
        .trading_repo
        .open_counts(&trader_id)
        .await
        .unwrap_or((0, 0));

    let runtime_engine = app.runtime_engine(&trader_id).ok().flatten();
    let runtime_running = runtime_engine
        .as_ref()
        .map(|v| v.is_running)
        .unwrap_or(trader.is_running != 0);

    Ok(TraderStatusPayload {
        trader_id,
        is_running: runtime_running,
        open_positions,
        open_orders,
        last_updated: trader.updated_at,
        runtime_engine: runtime_engine.as_ref().map(runtime_engine_payload),
    })
}

pub async fn account(
    app: &SharedState,
    user_id: &str,
    q: TraderQuery,
) -> AppResult<TraderAccountPayload> {
    let trader_id = match resolve_trader_id(app, user_id, q.trader_id).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    match app.trading_repo.latest_account(user_id, &trader_id).await {
        Ok(Some(account)) => Ok(account_payload(trader_id, account)),
        Ok(None) => Err(app_error(
            AppErrorKind::NotFound,
            "Account snapshot not found",
        )),
        Err(_) => Err(app_error(AppErrorKind::Internal, "Failed to load account")),
    }
}

pub async fn positions(
    app: &SharedState,
    user_id: &str,
    q: PositionQuery,
) -> AppResult<PositionListPayload> {
    let trader_id = match resolve_trader_id(app, user_id, q.trader_id).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let status = q
        .status
        .unwrap_or_else(|| "open".to_string())
        .trim()
        .to_lowercase();

    match app
        .trading_repo
        .positions_by_status(user_id, &trader_id, &status)
        .await
    {
        Ok(items) => {
            let items: Vec<PositionPayload> = items.into_iter().map(position_payload).collect();
            Ok(PositionListPayload {
                count: items.len(),
                items,
                trader_id,
            })
        }
        Err(_) => Err(app_error(
            AppErrorKind::Internal,
            "Failed to load positions",
        )),
    }
}

pub async fn positions_history(
    app: &SharedState,
    user_id: &str,
    q: PaginationQuery,
) -> AppResult<PositionListPayload> {
    let trader_id = match resolve_trader_id(app, user_id, q.trader_id).await {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    let limit = q.limit.unwrap_or(100).clamp(1, 500);
    let offset = q.offset.unwrap_or(0).max(0);

    match app
        .trading_repo
        .closed_positions(user_id, &trader_id, limit, offset)
        .await
    {
        Ok(items) => {
            let items: Vec<PositionPayload> = items.into_iter().map(position_payload).collect();
            Ok(PositionListPayload {
                count: items.len(),
                items,
                trader_id,
            })
        }
        Err(_) => Err(app_error(
            AppErrorKind::Internal,
            "Failed to load position history",
        )),
    }
}

trait AccountSnapshotExt {
    fn with_snapshot_at(self, snapshot_at: i64) -> TraderAccountRecord;
}

impl AccountSnapshotExt for TraderAccountRecord {
    fn with_snapshot_at(mut self, snapshot_at: i64) -> TraderAccountRecord {
        self.snapshot_at = snapshot_at;
        self
    }
}

fn account_payload(trader_id: String, account: TraderAccountRecord) -> TraderAccountPayload {
    TraderAccountPayload {
        trader_id,
        total_balance: account.total_balance,
        available_balance: account.available_balance,
        used_margin: account.used_margin,
        unrealized_pnl: account.unrealized_pnl,
        realized_pnl: account.realized_pnl,
        currency: account.currency,
        snapshot_at: account.snapshot_at,
    }
}

pub(crate) fn position_payload(position: TraderPositionRecord) -> PositionPayload {
    PositionPayload {
        id: position.id,
        trader_id: position.trader_id,
        symbol: position.symbol,
        side: position.side,
        quantity: position.quantity,
        entry_price: position.entry_price,
        mark_price: position.mark_price,
        liquidation_price: position.liquidation_price,
        leverage: position.leverage,
        margin_mode: position.margin_mode,
        unrealized_pnl: position.unrealized_pnl,
        realized_pnl: position.realized_pnl,
        status: position.status,
        opened_at: position.opened_at,
        closed_at: position.closed_at,
        updated_at: position.updated_at,
    }
}
