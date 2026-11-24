use clauseforge_api_schema::{SubmitAmendment, API_VERSION};
#[test]
fn maps_typed_contract() {
    let d = SubmitAmendment {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
    }
    .into_domain("sre@example.com".into());
    assert_eq!(d.supplier, "checkout");
    assert_eq!(API_VERSION, "v1");
}
