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
        .env("CLAUSEFORGE_SUPPLIER", &request.supplier)
        .env("CLAUSEFORGE_AMENDMENT", &request.amendment)
        .env(
            "CLAUSEFORGE_AFFECTED_CONTROLS",
            request.affected_controls.to_string(),
        )
        .env("CLAUSEFORGE_CURE_DAYS", request.cure_days.to_string())
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
    let fields: Vec<&str> = text.trim().split(':').collect();
    let raw = fields
        .get(1)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(request.affected_controls as f64 / 100.0);
    let score = ((raw + 1.0) * 50.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "upstream-fann".into(),
        score,
        message: "infer calibrated amendment risk".into(),
        evidence: vec![text.trim().into()],
    })
}
