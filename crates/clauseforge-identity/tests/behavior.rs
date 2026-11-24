use clauseforge_identity::verify_reviewer;
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
    let r = verify_reviewer(&request()).unwrap();
    assert_eq!(r.stage, "identity");
    assert!(!r.evidence.is_empty());
}
