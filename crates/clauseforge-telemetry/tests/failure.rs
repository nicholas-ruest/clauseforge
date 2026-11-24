use clauseforge_kernel::AmendmentRequest;
use clauseforge_telemetry::ingest;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "NaN".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(ingest(&r).is_err());
}
