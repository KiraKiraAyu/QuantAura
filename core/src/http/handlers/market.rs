use axum::{Json, extract::Query};

use crate::{
    contracts::public::{ExchangeSymbolsPayload, KlinePayload, KlinesQuery, SymbolsQuery},
    error::Result,
    http::response::ApiResponse,
    services::market,
};

pub async fn handle_symbols(
    Query(q): Query<SymbolsQuery>,
) -> Result<Json<ApiResponse<ExchangeSymbolsPayload>>> {
    let payload = market::symbols(q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_klines(
    Query(q): Query<KlinesQuery>,
) -> Result<Json<ApiResponse<Vec<KlinePayload>>>> {
    let payload = market::klines(q).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
