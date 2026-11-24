use clauseforge_kernel::AmendmentRequest;
use clauseforge_scheduler::schedule;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 1,
        reviewer: "sre@example.com".into(),
    };
    assert!(schedule(&r).is_err());
}
