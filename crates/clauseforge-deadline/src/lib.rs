use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn budget(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let score = ((365 - request.cure_days) as f64 / 3.65).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "deadline".into(),
        score,
        message: "scored contractual cure urgency".into(),
        evidence: vec![format!("deadline:{}:{:.2}", request.stable_key(), score)],
    })
}
