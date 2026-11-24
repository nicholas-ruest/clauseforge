use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn ingest(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.amendment.contains("NaN") {
        return Err(DomainError::Denied(
            "telemetry contains invalid sample".into(),
        ));
    }
    let score = ((request.affected_controls as f64).sqrt() * 10.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "telemetry".into(),
        score,
        message: "ingested normalized amendment telemetry".into(),
        evidence: vec![format!("telemetry:{}:{:.2}", request.stable_key(), score)],
    })
}
