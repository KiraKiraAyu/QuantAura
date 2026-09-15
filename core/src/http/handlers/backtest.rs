use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    contracts::backtest::{
        BacktestMessagePayload, BacktestQueryParams, BacktestRunActionPayload, BacktestRunsPayload,
        BacktestStartRequest,
    },
    error::{AppError, Result},
    http::response::{ApiResponse, AppJson},
    state,
};

pub async fn handle_backtest_start(
    State(app): State<state::AppState>,
    AppJson(request): AppJson<BacktestStartRequest>,
) -> Result<Json<ApiResponse<BacktestRunActionPayload>>> {
    let BacktestStartRequest {
        trader_id,
        run_id,
        start_ts,
        end_ts,
        initial_balance,
        interval,
    } = request;

    if trader_id.trim().is_empty() {
        return Err(AppError::BadRequest("trader_id must not be empty".into()));
    }
    if let Some(ref id) = run_id
        && id.trim().is_empty()
    {
        return Err(AppError::BadRequest("run_id must not be empty".into()));
    }
    if let Some(ref iv) = interval
        && iv.trim().is_empty()
    {
        return Err(AppError::BadRequest("interval must not be empty".into()));
    }
    if let Some(b) = initial_balance
        && b <= 0.0
    {
        return Err(AppError::BadRequest(
            "initial_balance must be positive".into(),
        ));
    }

    let payload = app
        .services
        .backtest_service
        .start(
            &trader_id,
            run_id,
            start_ts,
            end_ts,
            initial_balance,
            interval,
        )
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_stop(
    State(app): State<state::AppState>,
    Path(run_id): Path<String>,
) -> Result<Json<ApiResponse<BacktestRunActionPayload>>> {
    let run_id = run_id.trim();
    if run_id.is_empty() {
        return Err(AppError::BadRequest("run_id must not be empty".into()));
    }
    let payload = app.services.backtest_service.stop(run_id.to_string())?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_delete(
    State(app): State<state::AppState>,
    Path(run_id): Path<String>,
) -> Result<Json<ApiResponse<BacktestMessagePayload>>> {
    let run_id = run_id.trim();
    if run_id.is_empty() {
        return Err(AppError::BadRequest("run_id must not be empty".into()));
    }
    let payload = app
        .services
        .backtest_service
        .delete(run_id.to_string())
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_runs(
    State(app): State<state::AppState>,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestRunsPayload>>> {
    let BacktestQueryParams { limit } = q;
    let limit = limit.map(|l| l.clamp(1, 200));
    let payload = app.services.backtest_service.runs(limit).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
