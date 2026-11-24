use clauseforge_deadline::budget;
use clauseforge_kernel::AmendmentRequest;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 900,
        reviewer: "sre@example.com".into(),
    };
    assert!(budget(&r).is_err());
}
