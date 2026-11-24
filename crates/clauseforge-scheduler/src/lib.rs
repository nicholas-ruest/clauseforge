use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub mod plan;

pub fn schedule(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.cure_days < 5 {
        return Err(DomainError::Denied(
            "amendment window is unschedulable".into(),
        ));
    }
    let recovery = plan::plan_request(request)?;
    let score = if recovery.slack_days >= 0 {
        (100.0 - recovery.critical_path_days as f64).clamp(0.0, 100.0)
    } else {
        (50.0 + recovery.slack_days as f64).clamp(0.0, 100.0)
    };
    Ok(StageResult {
        stage: "scheduler".into(),
        score,
        message: format!(
            "scheduled {} recovery tasks with {}-way parallelism",
            recovery.tasks.len(),
            recovery.maximum_parallelism
        ),
        evidence: vec![
            format!("scheduler:{}:{:.2}", request.stable_key(), score),
            format!(
                "recovery:critical={}:objective={}:slack={}",
                recovery.critical_path_days, recovery.cure_window_days, recovery.slack_days
            ),
        ],
    })
}
