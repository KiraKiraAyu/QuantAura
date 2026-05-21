use axum::{
    Json, RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{Algorithm, Validation, decode};
use std::ops::Deref;
use thiserror::Error;

use crate::{contracts::auth::Claims, http::response::ApiResponse, state::AppState};

#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

impl Deref for AuthUser {
    type Target = Claims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
        };

        let body = Json(ApiResponse::<()>::failure(error_message));

        (status, body).into_response()
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let app = AppState::from_ref(state);

        if app
            .services
            .auth_service
            .is_token_blacklisted(bearer.token())
            .map_err(|_| AuthError::InvalidToken)?
        {
            return Err(AuthError::InvalidToken);
        }

        let claims = decode::<Claims>(
            bearer.token(),
            &app.config.jwt.decoding_key,
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|_| AuthError::InvalidToken)?
        .claims;

        Ok(AuthUser(claims))
    }
}
