use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use clauseforge_service::app::{router, AppState};
use tower::ServiceExt;

fn configure() {
    for (name, marker) in [
        ("CLAUSEFORGE_RUVECTOR_CMD", "similarity_set"),
        ("CLAUSEFORGE_FANN_CMD", "calibrated_interval"),
        ("CLAUSEFORGE_MIDSTREAM_CMD", "safety_window"),
        ("CLAUSEFORGE_QUDAG_CMD", "kem_ciphertext"),
        ("CLAUSEFORGE_GUARDRAIL_CMD", "guardrail_valid"),
        ("CLAUSEFORGE_OPA_CMD", "policy_allow"),
        ("CLAUSEFORGE_DSPY_CMD", "dspy_rank"),
        ("CLAUSEFORGE_LOGACT_CMD", "logact_policy"),
    ] {
        std::env::set_var(name, format!("printf {marker}"));
    }
}

#[tokio::test]
async fn browser_to_durable_result_slice() {
    configure();
    let path =
        std::env::temp_dir().join(format!("clauseforge-service-{}.jsonl", std::process::id()));
    let app = router(AppState::open(&path).unwrap());
    let body = r#"{"supplier":"checkout","amendment":"regional failover","affected_controls":25,"cure_days":30}"#;
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
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(
        AppState::open(&path).unwrap().store.load().unwrap().len(),
        1
    );
    let _ = std::fs::remove_file(path);
}
