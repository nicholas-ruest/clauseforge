# Evidence Sealing bounded context

## Purpose and language

EvidenceRecord is the durable fact: unique ID, correlation, canonical payload, previous digest and current digest.

## Aggregate boundary and invariants

Seal hashes length-delimited fields with SHA-256. Verify recomputes the digest; audit reconstruction also proves previous-hash continuity.

## Commands, events and failure semantics

EvidenceSealed precedes PersistenceCommitted. HashMismatch is a hard integrity failure exposed as server error. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
