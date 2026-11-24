use clauseforge_kernel::AmendmentRequest;
#[test]
fn accepts_well_formed_amendment() {
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "regional failover".into(),
        affected_controls: 25,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert!(r.validate_identity().is_ok());
    assert_eq!(r.stable_key(), "checkout:regional failover:sre@example.com");
}
