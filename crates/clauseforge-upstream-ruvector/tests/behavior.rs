use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_ruvector::{retrieve_neighbors, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf similarity_set".into()]);
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(
        retrieve_neighbors(&c, &r).unwrap().stage,
        "upstream-ruvector"
    );
}
