use clauseforge_authorization::authorize_control_scope;
use clauseforge_kernel::AmendmentRequest;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 90,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(authorize_control_scope(&r).is_err());
}
