use clauseforge_kernel::AmendmentRequest;
use clauseforge_orchestration::{aggregate_score, run_domain};
#[test]
fn composes_domain_stages() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "regional failover".into(),
        affected_controls: 25,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    let s = run_domain(&r).unwrap();
    assert_eq!(s.len(), 10);
    assert!(aggregate_score(&s).unwrap() > 0.0);
}
