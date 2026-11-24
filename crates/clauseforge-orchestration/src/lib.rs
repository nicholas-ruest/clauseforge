use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn run_domain(request: &AmendmentRequest) -> Result<Vec<StageResult>, DomainError> {
    let stages = vec![
        clauseforge_amendment::compile_plan(request)?,
        clauseforge_identity::verify_reviewer(request)?,
        clauseforge_authorization::authorize_control_scope(request)?,
        clauseforge_validation::validate_cure_window(request)?,
        clauseforge_correlation::correlate(request)?,
        clauseforge_deadline::budget(request)?,
        clauseforge_observability::instrument(request)?,
        clauseforge_risk::score_risk(request)?,
        clauseforge_telemetry::ingest(request)?,
        clauseforge_scheduler::schedule(request)?,
    ];
    if stages.iter().any(|stage| stage.evidence.is_empty()) {
        return Err(DomainError::Integrity(
            "domain stage omitted evidence".into(),
        ));
    }
    Ok(stages)
}

pub fn aggregate_score(stages: &[StageResult]) -> Result<f64, DomainError> {
    if stages.is_empty() {
        return Err(DomainError::Missing("stages"));
    }
    Ok(stages.iter().map(|s| s.score).sum::<f64>() / stages.len() as f64)
}
