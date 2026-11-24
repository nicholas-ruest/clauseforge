use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub mod plan;

pub fn schedule(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.cure_days < 5 {
        return Err(DomainError::Denied(
            "amendment window is unschedulable".into(),
        ));
    }
    let mitigation = plan::plan_request(request)?;
    let score = if mitigation.slack_days >= 0 {
        (100.0 - mitigation.critical_path_days as f64).clamp(0.0, 100.0)
    } else {
        (50.0 + mitigation.slack_days as f64).clamp(0.0, 100.0)
    };
    Ok(StageResult {
        stage: "scheduler".into(),
        score,
        message: format!(
            "scheduled {} mitigation tasks with {}-way parallelism",
            mitigation.tasks.len(),
            mitigation.maximum_parallelism
        ),
        evidence: vec![
            format!("scheduler:{}:{:.2}", request.stable_key(), score),
            format!(
                "mitigation:critical={}:cure={}:slack={}",
                mitigation.critical_path_days, mitigation.cure_window_days, mitigation.slack_days
            ),
        ],
    })
}
