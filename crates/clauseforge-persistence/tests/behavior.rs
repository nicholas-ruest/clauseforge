use clauseforge_evidence::seal;
use clauseforge_persistence::EvidenceStore;
#[test]
fn survives_reopen() {
    let p = std::env::temp_dir().join(format!("clauseforge-store-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    s.append(&seal("1", "c", "done", "genesis").unwrap())
        .unwrap();
    drop(s);
    assert_eq!(EvidenceStore::open(&p).unwrap().load().unwrap().len(), 1);
    let _ = std::fs::remove_file(p);
}
