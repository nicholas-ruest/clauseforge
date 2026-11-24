use clauseforge_kernel::AmendmentRequest;
use clauseforge_upstream_qudag::{seal_envelope, AdapterConfig};
#[test]
fn executes_pinned_responsibility() {
    let c = AdapterConfig::pinned("/bin/sh", vec!["-c".into(), "printf kem_ciphertext".into()]);
    let r = AmendmentRequest {
        supplier: "checkout".into(),
        amendment: "failover".into(),
        affected_controls: 20,
        cure_days: 30,
        reviewer: "sre@example.com".into(),
    };
    assert_eq!(seal_envelope(&c, &r).unwrap().stage, "upstream-qudag");
}
