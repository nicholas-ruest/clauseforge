use clauseforge_identity::verify_reviewer;
use clauseforge_kernel::AmendmentRequest;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "invalid".into(),
    };
    assert!(verify_reviewer(&r).is_err());
}
