use clauseforge_kernel::AmendmentRequest;
use clauseforge_orchestration::run_domain;
#[test]
fn denial_stops_composition() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 90,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(run_domain(&r).is_err());
}
