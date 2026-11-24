use clauseforge_kernel::AmendmentRequest;
use clauseforge_validation::validate_cure_window;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 200,
        reviewer: "sre@example.com".into(),
    };
    assert!(validate_cure_window(&r).is_err());
}
