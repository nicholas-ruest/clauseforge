use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_logact::{generate_policy_actions, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf logact_policy".into()]);
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(
        generate_policy_actions(&c, &r).unwrap().stage,
        "upstream-logact"
    );
}
