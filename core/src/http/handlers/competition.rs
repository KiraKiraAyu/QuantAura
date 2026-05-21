use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    contracts::public::{
        CompetitionListPayload, EquityHistoryBatchPayload, EquityHistoryBatchRequest,
        EquityHistoryPointPayload, EquityHistoryQuery, PublicCompetitionTraderPayload,
        PublicTraderConfigPayload,
    },
    error::Result,
    http::response::ApiResponse,
    state,
};

pub async fn handle_public_competition(
    State(app): State<state::AppState>,
) -> Result<Json<ApiResponse<CompetitionListPayload>>> {
    let traders = app.services.competition_service.competition().await?;
    let payload = CompetitionListPayload {
        count: traders.len(),
        traders,
    };
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_top_traders(
    State(app): State<state::AppState>,
) -> Result<Json<ApiResponse<Vec<PublicCompetitionTraderPayload>>>> {
    let payload = app.services.competition_service.top_traders().await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_equity_history(
    State(app): State<state::AppState>,
    Query(q): Query<EquityHistoryQuery>,
) -> Result<Json<ApiResponse<Vec<EquityHistoryPointPayload>>>> {
    let payload = app.services.competition_service.equity_history(q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_equity_history_batch(
    State(app): State<state::AppState>,
    Json(request): Json<EquityHistoryBatchRequest>,
) -> Result<Json<ApiResponse<EquityHistoryBatchPayload>>> {
    let payload = app
        .services
        .competition_service
        .equity_history_batch(request)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_public_trader_config(
    State(app): State<state::AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<PublicTraderConfigPayload>>> {
    let payload = app
        .services
        .competition_service
        .public_trader_config(&id)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
