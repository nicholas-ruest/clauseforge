use clauseforge_persistence::EvidenceStore;
use clauseforge_replay::replay;
#[test]
fn missing_amendment_fails() {
    let p = std::env::temp_dir().join(format!("clauseforge-none-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    assert!(replay(&s, "missing").is_err());
    let _ = std::fs::remove_file(p);
}
