use axum::{
    Json,
    extract::{Query, State},
};

use crate::{
    contracts::backtest::{
        BacktestDecisionsPayload, BacktestEquityPayload, BacktestExportPayload,
        BacktestLabelRequest, BacktestMessagePayload, BacktestMetricsPayload, BacktestQueryParams,
        BacktestRunActionPayload, BacktestRunIdRequest, BacktestRunsPayload, BacktestStartRequest,
        BacktestStatusPayload, BacktestTracePayload, BacktestTradesPayload, KlinePayload,
        KlinesQuery,
    },
    error::Result,
    http::{extractors::AuthUser, response::ApiResponse},
    state,
};

pub async fn handle_backtest_start(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<BacktestStartRequest>,
) -> Result<Json<ApiResponse<BacktestRunActionPayload>>> {
    let payload = app
        .services
        .backtest_service
        .start(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_pause(
    State(app): State<state::AppState>,
    Json(request): Json<BacktestRunIdRequest>,
) -> Result<Json<ApiResponse<BacktestRunActionPayload>>> {
    let payload = app.services.backtest_service.pause(request);
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_resume(
    State(app): State<state::AppState>,
    Json(request): Json<BacktestRunIdRequest>,
) -> Result<Json<ApiResponse<BacktestRunActionPayload>>> {
    let payload = app.services.backtest_service.resume(request);
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_stop(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<BacktestRunIdRequest>,
) -> Result<Json<ApiResponse<BacktestRunActionPayload>>> {
    let payload = app.services.backtest_service.stop(&user.sub, request)?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_label(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<BacktestLabelRequest>,
) -> Result<Json<ApiResponse<BacktestMessagePayload>>> {
    let payload = app
        .services
        .backtest_service
        .label(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_delete(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<BacktestRunIdRequest>,
) -> Result<Json<ApiResponse<BacktestMessagePayload>>> {
    let payload = app
        .services
        .backtest_service
        .delete(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_status(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestStatusPayload>>> {
    let payload = app.services.backtest_service.status(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_runs(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestRunsPayload>>> {
    let payload = app.services.backtest_service.runs(&user.sub, q).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_equity(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestEquityPayload>>> {
    let payload = app.services.backtest_service.equity(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_trades(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestTradesPayload>>> {
    let payload = app.services.backtest_service.trades(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_metrics(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestMetricsPayload>>> {
    let payload = app.services.backtest_service.metrics(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_trace(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestTracePayload>>> {
    let payload = app.services.backtest_service.trace(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_decisions(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestDecisionsPayload>>> {
    let payload = app
        .services
        .backtest_service
        .decisions(&user.sub, q)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_export(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<BacktestQueryParams>,
) -> Result<Json<ApiResponse<BacktestExportPayload>>> {
    let payload = app.services.backtest_service.export(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_backtest_klines(
    State(app): State<state::AppState>,
    Query(q): Query<KlinesQuery>,
) -> Result<Json<ApiResponse<Vec<KlinePayload>>>> {
    let payload = app.services.backtest_service.klines(q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
