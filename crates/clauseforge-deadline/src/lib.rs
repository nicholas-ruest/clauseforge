use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn budget(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.cure_days > 720 {
        return Err(DomainError::Denied(
            "deadline consumes operational day".into(),
        ));
    }
    let score = ((1440 - request.cure_days) as f64 / 14.4).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "deadline".into(),
        score,
        message: "reserved execution deadline".into(),
        evidence: vec![format!("deadline:{}:{:.2}", request.stable_key(), score)],
    })
}
