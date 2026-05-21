use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    contracts::debates::{
        CreateDebateRequest, DebateActionPayload, DebateDetailPayload, DebateExecutionPayload,
        DebateListPayload, DebateMessagePayload, DebateMessagesPayload, DebatePersonalitiesPayload,
        DebateVotesPayload, StartDebateRequest,
    },
    error::Result,
    http::{extractors::AuthUser, response::ApiResponse},
    state,
};

pub async fn handle_get_debates(
    State(app): State<state::AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<DebateListPayload>>> {
    let payload = app.services.debate_service.list(&user.sub).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_create_debate(
    State(app): State<state::AppState>,
    user: AuthUser,
    Json(request): Json<CreateDebateRequest>,
) -> Result<Json<ApiResponse<DebateActionPayload>>> {
    let payload = app
        .services
        .debate_service
        .create(&user.sub, request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_get_debate_personalities(
    State(app): State<state::AppState>,
) -> Result<Json<ApiResponse<DebatePersonalitiesPayload>>> {
    let payload = app.services.debate_service.personalities();
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_get_debate(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateDetailPayload>>> {
    let payload = app.services.debate_service.get(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_delete_debate(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateMessagePayload>>> {
    let payload = app.services.debate_service.delete(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_start_debate(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
    Json(_request): Json<StartDebateRequest>,
) -> Result<Json<ApiResponse<DebateActionPayload>>> {
    let payload = app.services.debate_service.start(&user.sub, &id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_cancel_debate(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateActionPayload>>> {
    let payload = app.services.debate_service.cancel(&user.sub, &id)?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_execute_debate(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateExecutionPayload>>> {
    let payload = app
        .services
        .debate_service
        .execution(&user.sub, &id)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_get_debate_messages(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateMessagesPayload>>> {
    let payload = app.services.debate_service.messages(&user.sub, &id).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_get_debate_votes(
    State(app): State<state::AppState>,
    user: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateVotesPayload>>> {
    let payload = app.services.debate_service.votes(&user.sub, &id).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
