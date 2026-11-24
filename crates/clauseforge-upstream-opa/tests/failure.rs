use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_opa::{evaluate_policy, AdapterConfig};
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
    assert!(evaluate_policy(&c, &r).is_err());
}
