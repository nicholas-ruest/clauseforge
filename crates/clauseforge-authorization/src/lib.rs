use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn authorize_control_scope(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.affected_controls > 80 {
        return Err(DomainError::Denied(
            "affected control scope exceeds policy".into(),
        ));
    }
    let score = (100.0 - request.affected_controls as f64).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "authorization".into(),
        score,
        message: "authorized bounded control scope".into(),
        evidence: vec![format!(
            "authorization:{}:{:.2}",
            request.stable_key(),
            score
        )],
    })
}
