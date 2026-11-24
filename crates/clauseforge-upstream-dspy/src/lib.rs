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
        .env("CLAUSEFORGE_SERVICE", &request.supplier)
        .env("CLAUSEFORGE_SCENARIO", &request.amendment)
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
    Ok(StageResult {
        stage: "upstream-dspy".into(),
        score: 100.0,
        message: "publish correlated amendment event".into(),
        evidence: vec![text.trim().into()],
    })
}
