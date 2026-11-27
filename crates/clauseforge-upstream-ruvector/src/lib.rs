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
            required_marker: "similarity_set".into(),
        }
    }
}

pub fn retrieve_neighbors(
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
        .map_err(|e| DomainError::Upstream(format!("upstream-ruvector: {e}")))?;
    if !output.status.success() {
        return Err(DomainError::Upstream(format!(
            "upstream-ruvector exited {}",
            output.status
        )));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    if !text.contains(&config.required_marker) {
        return Err(DomainError::Upstream(format!(
            "upstream-ruvector missing marker {}",
            config.required_marker
        )));
    }
    let fields: Vec<&str> = text.trim().split(':').collect();
    let matches = fields
        .get(1)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(0.0);
    let controls = fields
        .get(2)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(request.affected_controls as f64);
    let score = (controls + matches * 5.0).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "upstream-ruvector".into(),
        score,
        message: "retrieve similar obligation and control neighborhoods".into(),
        evidence: vec![text.trim().into()],
    })
}
