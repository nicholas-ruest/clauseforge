use clauseforge_kernel::{AmendmentRequest, DomainError};
#[test]
fn rejects_impossible_recovery_window() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 25,
        cure_days: 0,
        reviewer: "sre".into(),
    };
    assert_eq!(r.validate_identity(), Err(DomainError::Range("cure_days")));
}
