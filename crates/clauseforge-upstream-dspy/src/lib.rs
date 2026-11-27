use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct AdapterConfig {
    pub command: String,
    pub args: Vec<String>,
    pub required_marker: String,
}
impl AdapterConfig {
    pub fn pinned(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            command: command.into(),
            args,
            required_marker: "dspy_rank".into(),
        }
    }
}

pub fn rank_clause_priority(
    config: &AdapterConfig,
    request: &AmendmentRequest,
) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let output = Command::new(&config.command)
        .args(&config.args)
        .env("CLAUSEFORGE_SUPPLIER", &request.supplier)
        .env("CLAUSEFORGE_AMENDMENT", &request.amendment)
        .env(
            "CLAUSEFORGE_AFFECTED_CONTROLS",
            request.affected_controls.to_string(),
        )
        .env("CLAUSEFORGE_CURE_DAYS", request.cure_days.to_string())
        .output()
        .map_err(|e| DomainError::Upstream(format!("upstream-dspy: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "upstream-dspy exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "upstream-dspy missing marker {}",
            config.required_marker
        )));
    }
    let score_values = text.trim().split(':').nth(2).unwrap_or_default();
    let scores: Vec<f64> = score_values
        .split(',')
        .filter_map(|v| v.parse::<f64>().ok())
        .collect();
    let score = if scores.is_empty() {
        request.affected_controls as f64
    } else {
        scores.iter().sum::<f64>() / scores.len() as f64
    }
    .clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "upstream-dspy".into(),
        score,
        message: "publish correlated amendment event".into(),
        evidence: vec![text.trim().into()],
    })
}
