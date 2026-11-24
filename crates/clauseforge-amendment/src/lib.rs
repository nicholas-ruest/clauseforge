use clauseforge_kernel::{AmendmentRequest, DomainError, StageResult};

pub mod topology;

pub fn compile_plan(request: &AmendmentRequest) -> Result<StageResult, DomainError> {
    request.validate_identity()?;
    if request.amendment.len() < 6 {
        return Err(DomainError::Denied("amendment is too vague".into()));
    }
    let analysis = topology::analyze_amendment(request)?;
    let vocabulary = request.amendment.split_whitespace().count() as f64 * 4.0;
    let topology_exposure = analysis.maximum_weighted_impact * 0.45;
    let score = (vocabulary + topology_exposure).clamp(0.0, 100.0);
    Ok(StageResult {
        stage: "amendment".into(),
        score,
        message: format!(
            "compiled clause impact across {} affected obligation nodes",
            analysis.affected_nodes
        ),
        evidence: vec![
            format!("amendment:{}:{:.2}", request.stable_key(), score),
            format!(
                "obligation_graph:depth={}:remediation_days={}",
                analysis.longest_path, analysis.remediation_burden_days
            ),
        ],
    })
}
