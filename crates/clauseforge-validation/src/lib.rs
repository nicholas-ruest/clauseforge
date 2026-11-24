use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn validate_cure_window(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.cure_days > 365 {
        return Err(DomainError::Denied(
            "contractual cure period exceeds one year".into(),
        ));
    }
    let score = ((365 - request.cure_days) as f64 / 3.65).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "validation".into(),
        score,
        message: "validated contractual cure period".into(),
        evidence: vec![format!("validation:{}:{:.2}", request.stable_key(), score)],
    })
}
