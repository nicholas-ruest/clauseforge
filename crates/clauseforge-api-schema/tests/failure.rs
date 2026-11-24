use clauseforge_api_schema::SubmitAmendment;
#[test]
fn domain_validation_rejects_empty_service() {
    let d = SubmitAmendment {
        supplier: "".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
    }
    .into_domain("sre@example.com".into());
    assert!(d.validate_identity().is_err());
}
