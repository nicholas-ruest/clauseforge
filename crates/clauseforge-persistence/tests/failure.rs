use clauseforge_evidence::seal;
use clauseforge_persistence::EvidenceStore;
#[test]
fn rejects_tampered_record() {
    let p = std::env::temp_dir().join(format!("clauseforge-bad-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    let mut r = seal("1", "c", "done", "genesis").unwrap();
    r.hash = "bad".into();
    assert!(s.append(&r).is_err());
    let _ = std::fs::remove_file(p);
}
