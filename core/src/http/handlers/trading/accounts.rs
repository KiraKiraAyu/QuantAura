use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    contracts::trading::{
        accounts::{
            ClosePositionPayload, ClosePositionRequest, GridRiskInfoPayload, TraderAccountPayload,
            TraderBalanceSyncPayload,
        },
        common::{PaginationQuery, TraderQuery},
        positions::{PositionListPayload, PositionQuery},
    },
    error::Result,
    http::{extractors::AuthUser, response::ApiResponse},
    state,
};

use super::trading_service;

pub async fn sync_balance(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TraderBalanceSyncPayload>>> {
    let payload = trading_service(&app).sync_balance(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn close_position(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(request): Json<ClosePositionRequest>,
) -> Result<Json<ApiResponse<ClosePositionPayload>>> {
    let payload = trading_service(&app)
        .close_position(&user.sub, &id, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn grid_risk(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<GridRiskInfoPayload>>> {
    let payload = trading_service(&app).grid_risk_info(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn account(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<TraderQuery>,
) -> Result<Json<ApiResponse<TraderAccountPayload>>> {
    let payload = trading_service(&app).account(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn positions(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<PositionQuery>,
) -> Result<Json<ApiResponse<PositionListPayload>>> {
    let payload = trading_service(&app).positions(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn positions_history(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PositionListPayload>>> {
    let payload = trading_service(&app)
        .positions_history(&user.sub, q)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
