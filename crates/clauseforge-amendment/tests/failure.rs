use clauseforge_amendment::compile_plan;
use clauseforge_kernel::AmendmentRequest;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "x".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(compile_plan(&r).is_err());
}
