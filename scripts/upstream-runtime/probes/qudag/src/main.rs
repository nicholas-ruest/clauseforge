use qudag_network::quantum_crypto::MlKem;
use std::{
    env,
    hash::{DefaultHasher, Hash, Hasher},
};

fn required(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("{name} is required"))
}

fn main() {
    let payload = format!(
        "{}|{}|{}|{}",
        required("CLAUSEFORGE_SUPPLIER"),
        required("CLAUSEFORGE_AMENDMENT"),
        required("CLAUSEFORGE_AFFECTED_CONTROLS"),
        required("CLAUSEFORGE_CURE_DAYS")
    );
    let mut kem = MlKem::new_default();
    let (public, secret) = kem.generate_keypair().unwrap();
    let (ciphertext, shared_a) = kem.encapsulate(&public).unwrap();
    let shared_b = kem.decapsulate(&secret, &ciphertext).unwrap();
    assert_eq!(shared_a.as_bytes(), shared_b.as_bytes());
    let mut request_hasher = DefaultHasher::new();
    payload.hash(&mut request_hasher);
    let request_digest = request_hasher.finish();
    let mut envelope_hasher = DefaultHasher::new();
    request_digest.hash(&mut envelope_hasher);
    shared_a.as_bytes().hash(&mut envelope_hasher);
    let envelope = envelope_hasher.finish();
    println!(
        "kem_ciphertext:{request_digest:016x}:{envelope:016x}:{}",
        ciphertext.ciphertext.len()
    );
}
