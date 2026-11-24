use axum::{
    body::Body,
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
