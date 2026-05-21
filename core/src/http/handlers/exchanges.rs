use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    contracts::exchanges::{
        CreateExchangePayload, CreateExchangeRequest, MessagePayload, SafeExchangeConfig,
        UpdateExchangeConfigRequest,
    },
    error::Result,
    http::{extractors::AuthUser, response::ApiResponse},
    state,
};

pub async fn get_exchange_configs(
    State(app): State<state::AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<Vec<SafeExchangeConfig>>>> {
    let payload = app
        .services
        .exchange_config_service
        .list_configs(&user.sub)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn create_exchange(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<CreateExchangeRequest>,
) -> Result<Json<ApiResponse<CreateExchangePayload>>> {
    let payload = app
        .services
        .exchange_config_service
        .create_exchange(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn update_exchange_configs(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<UpdateExchangeConfigRequest>,
) -> Result<Json<ApiResponse<MessagePayload>>> {
    let payload = app
        .services
        .exchange_config_service
        .update_configs(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn delete_exchange(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<MessagePayload>>> {
    let payload = app
        .services
        .exchange_config_service
        .delete_exchange(&user.sub, &id)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
