use clauseforge_kernel::AmendmentRequest;
use clauseforge_risk::score_risk;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 0,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(score_risk(&r).is_err());
}
