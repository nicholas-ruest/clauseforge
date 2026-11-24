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
            required_marker: "calibrated_interval".into(),
        }
    }
}

pub fn infer_risk(
    config: &AdapterConfig,
    request: &AmendmentRequest,
) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    let output = Command::new(&config.command)
        .args(&config.args)
        .env("CLAUSEFORGE_SERVICE", &request.supplier)
        .env("CLAUSEFORGE_SCENARIO", &request.amendment)
        .output()
        .map_err(|e| DomainError::Upstream(format!("upstream-fann: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "upstream-fann exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "upstream-fann missing marker {}",
            config.required_marker
        )));
    }
    Ok(StageResult {
        stage: "upstream-fann".into(),
        score: 100.0,
        message: "infer calibrated recovery risk".into(),
        evidence: vec![text.trim().into()],
    })
}
