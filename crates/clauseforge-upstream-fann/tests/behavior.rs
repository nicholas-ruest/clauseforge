use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_fann::{infer_risk, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned(
        "/bin/sh",
        vec!["-c".into(), "printf calibrated_interval".into()],
    );
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(infer_risk(&c, &r).unwrap().stage, "upstream-fann");
}
