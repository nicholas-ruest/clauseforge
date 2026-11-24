use clauseforge_kernel::AmendmentRequest;
use clauseforge_telemetry::ingest;
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
    let r = ingest(&request()).unwrap();
    assert_eq!(r.stage, "telemetry");
    assert!(!r.evidence.is_empty());
}
