use axum::{
    Json,
    extract::{FromRequestParts, Request, State},
    http::{StatusCode, header, request::Parts},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{contracts::auth::Claims, http::response::ApiResponse, state::AppState};

/// Authenticated user extractor that extracts validated Claims from request extensions.
#[derive(Debug, Clone)]
pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthUser)
            .ok_or_else(unauthorized)
    }
}

/// Where a request is allowed to carry its session token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenSource {
    /// `Authorization: Bearer <token>` header only.
    Header,
    /// Header first, then the `?token=` query parameter. Browsers cannot set
    /// request headers on `EventSource`/`WebSocket`, so streaming routes need it.
    HeaderOrQuery,
}

/// Route-level auth guard for regular API routes: the token must travel in the
/// `Authorization: Bearer` header.
pub async fn require_auth(State(app): State<AppState>, request: Request, next: Next) -> Response {
    authorize(app, request, next, TokenSource::Header).await
}

/// Route-level auth guard for streaming routes (SSE/WebSocket), which may also
/// pass the token through the `?token=` query parameter.
pub async fn require_auth_streaming(
    State(app): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    authorize(app, request, next, TokenSource::HeaderOrQuery).await
}

async fn authorize(
    app: AppState,
    mut request: Request,
    next: Next,
    token_source: TokenSource,
) -> Response {
    let Some(token) = extract_token(&request, token_source) else {
        return unauthorized();
    };

    match app.services.auth_service.validate_token(&token) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            next.run(request).await
        }
        Err(_) => unauthorized(),
    }
}

fn extract_token(request: &Request, token_source: TokenSource) -> Option<String> {
    if let Some(token) = bearer_token(request) {
        return Some(token);
    }

    match token_source {
        TokenSource::Header => None,
        TokenSource::HeaderOrQuery => query_token(request),
    }
}

fn bearer_token(request: &Request) -> Option<String> {
    request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_string)
}

fn query_token(request: &Request) -> Option<String> {
    let query = request.uri().query()?;

    form_urlencoded::parse(query.as_bytes()).find_map(|(key, value)| {
        (key == "token")
            .then(|| value.trim().to_string())
            .filter(|token| !token.is_empty())
    })
}

fn unauthorized() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiResponse::<()>::failure("Invalid token")),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use axum::http::Request as HttpRequest;

    use super::*;

    fn request_with(uri: &str, authorization: Option<&str>) -> Request {
        let mut builder = HttpRequest::builder().uri(uri);
        if let Some(value) = authorization {
            builder = builder.header(header::AUTHORIZATION, value);
        }
        builder.body(axum::body::Body::empty()).expect("request")
    }

    #[test]
    fn extracts_bearer_token_from_header() {
        let request = request_with("/api/config", Some("Bearer abc.def.ghi"));
        assert_eq!(
            extract_token(&request, TokenSource::Header).as_deref(),
            Some("abc.def.ghi")
        );
    }

    #[test]
    fn extracts_token_from_query_param_for_streaming_routes() {
        let request = request_with("/api/events?token=abc.def.ghi", None);
        assert_eq!(
            extract_token(&request, TokenSource::HeaderOrQuery).as_deref(),
            Some("abc.def.ghi")
        );
    }

    #[test]
    fn header_takes_precedence_over_query() {
        let request = request_with("/api/events?token=query", Some("Bearer header"));
        assert_eq!(
            extract_token(&request, TokenSource::HeaderOrQuery).as_deref(),
            Some("header")
        );
    }

    #[test]
    fn query_param_rejected_for_header_only_routes() {
        let request = request_with("/api/config?token=abc.def.ghi", None);
        assert!(extract_token(&request, TokenSource::Header).is_none());
    }

    #[test]
    fn missing_or_malformed_tokens_are_rejected() {
        assert!(extract_token(&request_with("/api/config", None), TokenSource::Header).is_none());
        assert!(
            extract_token(
                &request_with("/api/config", Some("Basic abc")),
                TokenSource::Header
            )
            .is_none()
        );
        assert!(
            extract_token(
                &request_with("/api/config", Some("Bearer ")),
                TokenSource::HeaderOrQuery
            )
            .is_none()
        );
        assert!(
            extract_token(
                &request_with("/api/events?token=", None),
                TokenSource::HeaderOrQuery
            )
            .is_none()
        );
    }
}
