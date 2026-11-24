use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_guardrail::{validate_clause_structure, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned(
        "/bin/sh",
        vec!["-c".into(), "printf guardrail_valid".into()],
    );
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(
        validate_clause_structure(&c, &r).unwrap().stage,
        "upstream-guardrail"
    );
}
