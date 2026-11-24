use clauseforge_correlation::correlate;
use clauseforge_kernel::AmendmentRequest;
#[test]
fn rejects_domain_boundary() {
    let r = AmendmentRequest {
        supplier: "".into(),
        amendment: "x".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "x".into(),
    };
    assert!(correlate(&r).is_err());
}
