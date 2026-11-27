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
            required_marker: "logact_policy".into(),
        }
    }
}

pub fn generate_policy_actions(
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
        .map_err(|e| DomainError::Upstream(format!("upstream-logact: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "upstream-logact exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "upstream-logact missing marker {}",
            config.required_marker
        )));
    }
    let score = text
        .trim()
        .split(':')
        .next_back()
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(request.affected_controls as f64)
        .clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "upstream-logact".into(),
        score,
        message: "generate enforceable mitigation actions".into(),
        evidence: vec![text.trim().into()],
    })
}
