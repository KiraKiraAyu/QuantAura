use axum::Json;

use crate::{
    contracts::public::{CryptoConfigPayload, CryptoPublicKeyPayload},
    error::Result,
    http::response::ApiResponse,
    services::crypto,
};

pub async fn handle_crypto_config() -> Result<Json<ApiResponse<CryptoConfigPayload>>> {
    let payload = crypto::crypto_config();
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_crypto_public_key() -> Result<Json<ApiResponse<CryptoPublicKeyPayload>>> {
    let payload = crypto::crypto_public_key();
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn handle_crypto_decrypt() -> Result<Json<ApiResponse<CryptoConfigPayload>>> {
    let payload = crypto::crypto_decrypt().await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
