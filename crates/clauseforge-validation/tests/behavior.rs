use clauseforge_kernel::AmendmentRequest;
use clauseforge_validation::validate_cure_window;
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
    let r = validate_cure_window(&request()).unwrap();
    assert_eq!(r.stage, "validation");
    assert!(!r.evidence.is_empty());
}
