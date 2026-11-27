use crate::middleware::{
    authn, authz_policy, correlation, normalize_errors, timeouts, validate_request, CorrelationId,
    Principal,
};
use axum::http::StatusCode;
use axum::{
    extract::{Path, State},
    middleware,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use clauseforge_api_schema::{AmendmentResponse, ErrorResponse, StageEvidence, SubmitAmendment};
use clauseforge_audit::AuditChain;
use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};
use clauseforge_persistence::EvidenceStore;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<EvidenceStore>,
}

impl AppState {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, DomainError> {
        Ok(Self {
            store: Arc::new(EvidenceStore::open(path)?),
        })
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route(
            "/api/v1/amendments",
            post(create_amendment).get(list_amendments),
        )
        .route("/api/v1/amendments/{correlation}", get(get_amendment))
        .route("/api/v1/audit", get(audit))
        .fallback_service(
            ServeDir::new("frontend/dist")
                .not_found_service(ServeFile::new("frontend/dist/index.html")),
        )
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn(timeouts))
        .layer(middleware::from_fn(normalize_errors))
        .layer(middleware::from_fn(validate_request))
        .layer(middleware::from_fn(authz_policy))
        .layer(middleware::from_fn(authn))
        .layer(middleware::from_fn(correlation))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    Json(serde_json::json!({"status":"ready","api":"v1","supplier":"clauseforge"}))
}

fn error(error: DomainError, correlation: &str) -> (StatusCode, Json<ErrorResponse>) {
    let status = match error {
        DomainError::Denied(_) => StatusCode::FORBIDDEN,
        DomainError::Missing(_) | DomainError::Range(_) => StatusCode::UNPROCESSABLE_ENTITY,
        DomainError::Upstream(_) => StatusCode::BAD_GATEWAY,
        DomainError::Integrity(_) | DomainError::Storage(_) => StatusCode::INTERNAL_SERVER_ERROR,
    };
    (
        status,
        Json(ErrorResponse {
            code: status.as_u16().to_string(),
            message: error.to_string(),
            correlation_id: correlation.into(),
        }),
    )
}

fn config(
    name: &str,
    marker: &str,
) -> Result<clauseforge_upstream_ruvector::AdapterConfig, DomainError> {
    let script = std::env::var(name)
        .map_err(|_| DomainError::Upstream(format!("missing runtime binding {name}")))?;
    Ok(clauseforge_upstream_ruvector::AdapterConfig {
        command: "/bin/sh".into(),
        args: vec!["-c".into(), script],
        required_marker: marker.into(),
    })
}

fn run_upstreams(request: &AmendmentRequest) -> Result<Vec<StageResult>, DomainError> {
    let mut stages = Vec::new();
    let c = config("CLAUSEFORGE_RUVECTOR_CMD", "similarity_set")?;
    stages.push(clauseforge_upstream_ruvector::retrieve_neighbors(
        &c, request,
    )?);
    let raw = config("CLAUSEFORGE_FANN_CMD", "calibrated_interval")?;
    let c = clauseforge_upstream_fann::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_fann::infer_risk(&c, request)?);
    let raw = config("CLAUSEFORGE_MIDSTREAM_CMD", "impact_metric")?;
    let c = clauseforge_upstream_midstream::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_midstream::aggregate_stream(
        &c, request,
    )?);
    let raw = config("CLAUSEFORGE_QUDAG_CMD", "kem_ciphertext")?;
    let c = clauseforge_upstream_qudag::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_qudag::seal_envelope(&c, request)?);
    let raw = config("CLAUSEFORGE_GUARDRAIL_CMD", "guardrail_valid")?;
    let c = clauseforge_upstream_guardrail::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_guardrail::validate_clause_structure(
        &c, request,
    )?);
    let raw = config("CLAUSEFORGE_OPA_CMD", "policy_allow")?;
    let c = clauseforge_upstream_opa::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_opa::evaluate_policy(&c, request)?);
    let raw = config("CLAUSEFORGE_DSPY_CMD", "dspy_rank")?;
    let c = clauseforge_upstream_dspy::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_dspy::rank_clause_priority(
        &c, request,
    )?);
    let raw = config("CLAUSEFORGE_LOGACT_CMD", "logact_policy")?;
    let c = clauseforge_upstream_logact::AdapterConfig {
        command: raw.command,
        args: raw.args,
        required_marker: raw.required_marker,
    };
    stages.push(clauseforge_upstream_logact::generate_policy_actions(
        &c, request,
    )?);
    Ok(stages)
}

async fn create_amendment(
    State(state): State<AppState>,
    Extension(principal): Extension<Principal>,
    Extension(correlation): Extension<CorrelationId>,
    Json(input): Json<SubmitAmendment>,
) -> Result<(StatusCode, Json<AmendmentResponse>), (StatusCode, Json<ErrorResponse>)> {
    let request = input.into_domain(principal.0);
    request
        .validate_identity()
        .map_err(|e| error(e, &correlation.0))?;
    if request.affected_controls > 80 {
        return Err(error(
            DomainError::Denied("affected control scope exceeds OPA policy boundary".into()),
            &correlation.0,
        ));
    }
    let mut stages =
        clauseforge_orchestration::run_domain(&request).map_err(|e| error(e, &correlation.0))?;
    let upstream_request = request.clone();
    let upstream_stages = tokio::task::spawn_blocking(move || run_upstreams(&upstream_request))
        .await
        .map_err(|join_error| {
            error(
                DomainError::Upstream(format!("upstream worker failed: {join_error}")),
                &correlation.0,
            )
        })?
        .map_err(|e| error(e, &correlation.0))?;
    stages.extend(upstream_stages);
    let risk_score = clauseforge_orchestration::aggregate_score(&stages)
        .map_err(|e| error(e, &correlation.0))?;
    let stage_names = stages
        .iter()
        .map(|stage| stage.stage.clone())
        .collect::<Vec<_>>();
    let stage_evidence = stages
        .iter()
        .map(|stage| StageEvidence {
            stage: stage.stage.clone(),
            score: stage.score,
            evidence: stage.evidence.clone(),
        })
        .collect::<Vec<_>>();
    let id = Uuid::new_v4().to_string();
    let payload =
        serde_json::json!({"id":id,"supplier":request.supplier,"amendment":request.amendment,
        "risk_score":risk_score,"stages":stage_names,"stage_evidence":stage_evidence})
        .to_string();
    let persisted = state.store.load().map_err(|e| error(e, &correlation.0))?;
    clauseforge_evidence::ledger::verify_chain(&persisted).map_err(|e| error(e, &correlation.0))?;
    let mut chain = AuditChain::default();
    for item in &persisted {
        let rebuilt = chain
            .append(&item.id, &item.correlation_id, &item.payload)
            .map_err(|e| error(e, &correlation.0))?;
        if rebuilt.hash != item.hash {
            return Err(error(
                DomainError::Integrity("persisted audit hash mismatch".into()),
                &correlation.0,
            ));
        }
    }
    let record = chain
        .append(&id, &correlation.0, &payload)
        .map_err(|e| error(e, &correlation.0))?
        .clone();
    state
        .store
        .append(&record)
        .map_err(|e| error(e, &correlation.0))?;
    Ok((
        StatusCode::CREATED,
        Json(AmendmentResponse {
            id,
            correlation_id: correlation.0,
            status: "completed".into(),
            risk_score,
            stages: stage_names,
            stage_evidence,
            evidence_hash: record.hash,
        }),
    ))
}

async fn list_amendments(
    State(state): State<AppState>,
) -> Result<Json<Vec<clauseforge_evidence::EvidenceRecord>>, (StatusCode, Json<ErrorResponse>)> {
    state.store.load().map(Json).map_err(|e| error(e, "list"))
}

async fn audit(
    State(state): State<AppState>,
) -> Result<Json<Vec<clauseforge_evidence::EvidenceRecord>>, (StatusCode, Json<ErrorResponse>)> {
    state.store.load().map(Json).map_err(|e| error(e, "audit"))
}

async fn get_amendment(
    State(state): State<AppState>,
    Path(correlation): Path<String>,
) -> Result<Json<clauseforge_replay::ReplaySummary>, (StatusCode, Json<ErrorResponse>)> {
    clauseforge_replay::replay(&state.store, &correlation)
        .map(Json)
        .map_err(|e| error(e, &correlation))
}
