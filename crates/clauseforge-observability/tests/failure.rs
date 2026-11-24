use clauseforge_kernel::AmendmentRequest;
use clauseforge_observability::instrument;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier:
            "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
                .into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(instrument(&r).is_err());
}
