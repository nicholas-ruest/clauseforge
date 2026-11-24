use clauseforge_correlation::correlate;
use clauseforge_kernel::AmendmentRequest;
fn request() -> AmendmentRequest {
    AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "regional failover exercise".into(),
        affected_controls: 25,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    }
}
#[test]
fn produces_domain_evidence() {
    let r = correlate(&request()).unwrap();
    assert_eq!(r.stage, "correlation");
    assert!(!r.evidence.is_empty());
}
