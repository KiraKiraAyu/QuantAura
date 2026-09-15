//! Route-level authorization tests.
//!
//! `routes::build_app` splits the protected surface into a "standard timeout"
//! group, a "long-running" group, and a "streaming" group, then hangs the auth
//! middleware off the merged result. Nothing in the type system stops a future
//! edit from nesting a router *after* the `route_layer` call, which would
//! silently expose that group — these tests are the tripwire for that.
//!
//! They boot the real application (`AppState::new` + `build_app`) rather than a
//! hand-assembled router, so a regression in boot-time wiring is caught too.

use axum::{
    body::Body,
    http::{Method, Request, StatusCode, header},
    response::Response,
};
use http_body_util::BodyExt;
use quantaura::{routes::build_app, state::AppState};
use serde_json::{Value, json};
use totp_rs::{Algorithm, Secret, TOTP};
use tower::ServiceExt;

/// Fixed base32 authenticator secret for the test boot.
const TEST_TOTP_SECRET: &str = "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP";

/// A single test drives the whole flow: `AppState::new` reads process-level
/// configuration, and keeping the boot in one place avoids racing on `DB_URL`
/// with any other test in this binary.
#[tokio::test(flavor = "multi_thread")]
async fn protected_routes_require_a_valid_token() {
    unsafe {
        // SAFETY: this test is the only place in this test binary that mutates
        // the environment, and it does so before any other test can observe it.
        std::env::set_var("DB_URL", "sqlite::memory:");
        std::env::set_var("AUTH_TOTP_SECRET", TEST_TOTP_SECRET);
    }

    let state = AppState::new().await;
    let app = build_app(state, 15);

    // Public routes stay reachable without a session.
    let response = app.clone().oneshot(get("/api/health")).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK, "health must stay public");

    // A protected route with no token is rejected.
    let response = app.clone().oneshot(get("/api/config")).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = read_json(response).await;
    assert_eq!(body["success"], json!(false));

    // The nested `/api/trading` group is protected as well. This is the group
    // that would leak if the auth layer were attached before the nesting.
    let response = app
        .clone()
        .oneshot(get("/api/trading/runtime-alerts"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // A malformed token is rejected on the streaming route too.
    let response = app
        .clone()
        .oneshot(get("/api/events?token=not-a-real-token"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Unknown routes answer with the JSON envelope, not an empty body.
    let response = app
        .clone()
        .oneshot(get("/api/definitely-not-a-route"))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = read_json(response).await;
    assert_eq!(body["success"], json!(false));
    assert_eq!(body["error"], json!("Route not found"));

    // Authenticate, then check how the token is allowed to travel.
    let token = login(&app).await;

    // Non-streaming routes ignore `?token=`; only the header is accepted there.
    let response = app
        .clone()
        .oneshot(get(&format!("/api/config?token={token}")))
        .await
        .unwrap();
    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED,
        "non-streaming routes must not accept a query-parameter token"
    );

    // The same token in a header is accepted.
    let response = app
        .clone()
        .oneshot(get_with_bearer("/api/config", &token))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Streaming routes accept the token as a query parameter (EventSource
    // cannot set headers). Only the response head is inspected — the SSE body
    // never completes on its own.
    let response = app
        .clone()
        .oneshot(get(&format!("/api/events?token={token}")))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("text/event-stream"),
    );
}

/// Verify a freshly generated TOTP code and return the issued session token.
async fn login(app: &axum::Router) -> String {
    let code = current_totp_code();
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/auth/verify")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json!({ "code": code }).to_string()))
        .expect("build verify request");

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "TOTP login should succeed with the configured secret"
    );

    read_json(response).await["data"]["token"]
        .as_str()
        .expect("login response should carry a token")
        .to_string()
}

/// Generate the TOTP code the server expects right now, using the same
/// parameters `AuthService` verifies with (SHA1 / 6 digits / 30s step).
fn current_totp_code() -> String {
    let secret = Secret::Encoded(TEST_TOTP_SECRET.to_string())
        .to_bytes()
        .expect("decode test totp secret");
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret,
        Some("QuantAura".to_string()),
        "local".to_string(),
    )
    .expect("build test totp");
    totp.generate_current().expect("generate totp code")
}

fn get(uri: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .expect("build get request")
}

fn get_with_bearer(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .expect("build authenticated get request")
}

async fn read_json(response: Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("read response body")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response body should be JSON")
}
