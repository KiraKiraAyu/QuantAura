use axum::{Json, extract::State};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::{
    contracts::auth::{
        ChangePasswordRequest, CurrentUserPayload, LoginRequest, MessagePayload, RegisterRequest,
        TokenPayload,
    },
    error::Result,
    http::{extractors::AuthUser, response::ApiResponse},
    state::AppState,
};

pub async fn register(
    State(app): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<TokenPayload>>> {
    let payload = app
        .services
        .auth_service
        .register(&request.email, &request.password)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn login(
    State(app): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<TokenPayload>>> {
    let payload = app
        .services
        .auth_service
        .login(&request.email, &request.password)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn logout(
    State(app): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<ApiResponse<MessagePayload>>> {
    let payload = app.services.auth_service.logout(bearer.token())?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn me(
    State(app): State<AppState>,
    user: AuthUser,
) -> Result<Json<ApiResponse<CurrentUserPayload>>> {
    let payload = app.services.auth_service.current_user(&user.sub).await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}

pub async fn change_password(
    State(app): State<AppState>,
    user: AuthUser,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<MessagePayload>>> {
    let payload = app
        .services
        .auth_service
        .change_password(&user.sub, &request.current_password, &request.new_password)
        .await?;
    Ok(Json(ApiResponse::success(Some(payload), None)))
}
