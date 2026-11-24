# Audit and Replay bounded context

## Purpose and language

AuditChain is the consistency boundary across every amendment review record. ReplaySummary is a correlation-focused projection.

## Aggregate boundary and invariants

Before append, persisted records are reconstructed and compared. Replay verifies both record hashes and chain order, then returns payloads and final digest.

## Commands, events and failure semantics

AuditVerified enables write or retrieval. ChainDiscontinuity blocks both and preserves the source file for investigation. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
