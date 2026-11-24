use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_dspy::{rank_clause_priority, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf dspy_rank".into()]);
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(rank_clause_priority(&c, &r).unwrap().stage, "upstream-dspy");
}
