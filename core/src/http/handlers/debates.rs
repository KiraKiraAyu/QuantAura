use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    contracts::debates::{
        CreateDebateRequest, DebateActionPayload, DebateDetailPayload, DebateExecutionPayload,
        DebateListPayload, DebateMessagePayload, DebateMessagesPayload, DebatePersonalitiesPayload,
        DebateVotesPayload,
    },
    error::{AppError, Result},
    http::response::{ApiResponse, AppJson},
    state,
};

fn validate_debate_id(id: &str) -> Result<&str> {
    let trimmed = id.trim();
    if trimmed.is_empty() {
        return Err(AppError::BadRequest("debate id must not be empty".into()));
    }
    uuid::Uuid::parse_str(trimmed)
        .map_err(|_| AppError::BadRequest("invalid debate id, expected valid UUID".into()))?;
    Ok(trimmed)
}

pub async fn handle_get_debates(
    State(app): State<state::AppState>,
) -> Result<Json<ApiResponse<DebateListPayload>>> {
    let payload = app.services.debate_service.list().await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_create_debate(
    State(app): State<state::AppState>,
    AppJson(CreateDebateRequest {
        name,
        symbol,
        max_rounds,
        participants,
    }): AppJson<CreateDebateRequest>,
) -> Result<Json<ApiResponse<DebateActionPayload>>> {
    if let Some(ref n) = name
        && n.trim().is_empty()
    {
        return Err(AppError::BadRequest(
            "name cannot be empty if provided".into(),
        ));
    }
    if let Some(ref s) = symbol
        && s.trim().is_empty()
    {
        return Err(AppError::BadRequest(
            "symbol cannot be empty if provided".into(),
        ));
    }
    if let Some(r) = max_rounds
        && r < 1
    {
        return Err(AppError::BadRequest("max_rounds must be positive".into()));
    }
    let payload = app
        .services
        .debate_service
        .create(name, symbol, max_rounds, participants)
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
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateDetailPayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.get(id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_delete_debate(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateMessagePayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.delete(id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_start_debate(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateActionPayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.start(id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_cancel_debate(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateActionPayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.cancel(id)?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_execute_debate(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateExecutionPayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.execution(id).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_get_debate_messages(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateMessagesPayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.messages(id).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_get_debate_votes(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<DebateVotesPayload>>> {
    let id = validate_debate_id(&id)?;
    let payload = app.services.debate_service.votes(id).await;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
