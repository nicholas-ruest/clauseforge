use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_fann::{infer_risk, AdapterConfig};
#[test]
fn propagates_upstream_failure() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "exit 17".into()]);
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(infer_risk(&c, &r).is_err());
}
