use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    contracts::trading::{
        common::TraderQuery,
        traders::{
            CreateTraderRequest, ToggleCompetitionRequest, TraderCreatedPayload, TraderListPayload,
            TraderMessagePayload, TraderPayload, TraderStatusPayload, UpdatePromptRequest,
            UpdateTraderRequest,
        },
    },
    error::Result,
    http::{extractors::AuthUser, response::ApiResponse},
    state,
};

use super::trading_service;

pub async fn list(
    State(app): State<state::AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<TraderListPayload>>> {
    let payload = trading_service(&app).list_traders(&user.sub).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn get(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TraderPayload>>> {
    let payload = trading_service(&app).get_trader(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn config(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TraderPayload>>> {
    let payload = trading_service(&app)
        .get_trader_config(&user.sub, &id)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn create(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<CreateTraderRequest>,
) -> Result<Json<ApiResponse<TraderCreatedPayload>>> {
    let payload = trading_service(&app)
        .create_trader(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn update(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(request): Json<UpdateTraderRequest>,
) -> Result<Json<ApiResponse<TraderMessagePayload>>> {
    let payload = trading_service(&app)
        .update_trader(&user.sub, &id, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn delete(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TraderMessagePayload>>> {
    let payload = trading_service(&app).delete_trader(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn start(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TraderMessagePayload>>> {
    let payload = trading_service(&app).start_trader(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn stop(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<TraderMessagePayload>>> {
    let payload = trading_service(&app).stop_trader(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn update_prompt(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(request): Json<UpdatePromptRequest>,
) -> Result<Json<ApiResponse<TraderMessagePayload>>> {
    let payload = trading_service(&app)
        .update_trader_prompt(&user.sub, &id, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn toggle_competition(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(request): Json<ToggleCompetitionRequest>,
) -> Result<Json<ApiResponse<TraderMessagePayload>>> {
    let payload = trading_service(&app)
        .toggle_competition(&user.sub, &id, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn status(
    State(app): State<state::AppState>,
    user: AuthUser,
    Query(q): Query<TraderQuery>,
) -> Result<Json<ApiResponse<TraderStatusPayload>>> {
    let payload = trading_service(&app).status(&user.sub, q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
