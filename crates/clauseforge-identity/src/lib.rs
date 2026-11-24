use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub fn verify_reviewer(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if !request.reviewer.contains("@") {
        return Err(DomainError::Denied(
            "reviewer identity is not an email".into(),
        ));
    }
    let score = (request.reviewer.split("@").count() as f64 * 25.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "identity".into(),
        score,
        message: "verified reviewer identity".into(),
        evidence: vec![format!("identity:{}:{:.2}", request.stable_key(), score)],
    })
}
