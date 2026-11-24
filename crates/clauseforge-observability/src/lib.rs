use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn instrument(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.supplier.len() > 80 {
        return Err(DomainError::Denied("service label is unbounded".into()));
    }
    let score = ((request.supplier.len() + request.amendment.len()) as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "observability".into(),
        score,
        message: "opened correlated observation span".into(),
        evidence: vec![format!(
            "observability:{}:{:.2}",
            request.stable_key(),
            score
        )],
    })
}
