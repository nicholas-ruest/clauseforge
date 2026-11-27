use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use clauseforge_service::app::{router, AppState};
use tower::ServiceExt;

#[tokio::test]
async fn policy_denial_has_no_downstream_write() {
    let path =
        std::env::temp_dir().join(format!("clauseforge-denial-{}.jsonl", std::process::id()));
    let state = AppState::open(&path).unwrap();
    let app = router(state.clone());
    let body = r#"{"supplier":"checkout","amendment":"unsafe global failover","affected_controls":95,"cure_days":30}"#;
    let response = app
        .oneshot(
            Request::post("/api/v1/amendments")
                .header("authorization", "Bearer clauseforge-reviewer")
                .header("x-role", "amendment-reviewer")
                .header("x-reviewer", "sre@example.com")
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert!(state.store.load().unwrap().is_empty());
    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn validation_middleware_rejects_wrong_media_type_without_write() {
    let path = std::env::temp_dir().join(format!(
        "clauseforge-validation-{}.jsonl",
        std::process::id()
    ));
    let state = AppState::open(&path).unwrap();
    let response = router(state.clone())
        .oneshot(
            Request::post("/api/v1/amendments")
                .header("authorization", "Bearer clauseforge-reviewer")
                .header("x-role", "amendment-reviewer")
                .header("content-type", "text/plain")
                .header("x-correlation-id", "validation-matrix")
                .body(Body::from("not-json"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(response.headers()["content-type"], "application/json");
    assert!(state.store.load().unwrap().is_empty());
    let _ = std::fs::remove_file(path);
}

#[tokio::test]
async fn error_middleware_normalizes_unknown_api_routes() {
    let path =
        std::env::temp_dir().join(format!("clauseforge-errors-{}.jsonl", std::process::id()));
    let state = AppState::open(&path).unwrap();
    let response = router(state)
        .oneshot(
            Request::get("/api/v1/unknown")
                .header("x-correlation-id", "error-matrix")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = to_bytes(response.into_body(), 16_384).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(value["code"], "404");
    assert_eq!(value["correlation_id"], "error-matrix");
}

#[tokio::test]
async fn authn_failure_is_structured_and_correlated() {
    let path = std::env::temp_dir().join(format!("clauseforge-authn-{}.jsonl", std::process::id()));
    let state = AppState::open(&path).unwrap();
    let response = router(state)
        .oneshot(
            Request::post("/api/v1/amendments")
                .header("content-type", "application/json")
                .header("x-correlation-id", "authn-matrix")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    let body = to_bytes(response.into_body(), 16_384).await.unwrap();
    let value: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(value["code"], "401");
    assert_eq!(value["correlation_id"], "authn-matrix");
}
