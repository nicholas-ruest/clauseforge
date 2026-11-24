use clauseforge_evidence::seal;
use clauseforge_persistence::EvidenceStore;
use clauseforge_replay::replay;
#[test]
fn replays_saved_event() {
    let p = std::env::temp_dir().join(format!("clauseforge-replay-{}.jsonl", std::process::id()));
    let s = EvidenceStore::open(&p).unwrap();
    s.append(&seal("1", "c", "done", "genesis").unwrap())
        .unwrap();
    assert_eq!(replay(&s, "c").unwrap().events, 1);
    let _ = std::fs::remove_file(p);
}
