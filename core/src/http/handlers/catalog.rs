use axum::Json;

use crate::{
    contracts::public::{SupportedExchangePayload, SupportedProviderTypePayload},
    error::Result,
    http::response::ApiResponse,
    services::catalog,
};

pub async fn supported_provider_types()
-> Result<Json<ApiResponse<Vec<SupportedProviderTypePayload>>>> {
    let payload = catalog::supported_provider_types();
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn supported_exchanges() -> Result<Json<ApiResponse<Vec<SupportedExchangePayload>>>> {
    let payload = catalog::supported_exchanges();
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
