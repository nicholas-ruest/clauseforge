use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_opa::{evaluate_policy, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf policy_allow".into()]);
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(evaluate_policy(&c, &r).unwrap().stage, "upstream-opa");
}
