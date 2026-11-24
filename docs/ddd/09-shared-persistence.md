# Shared Persistence bounded context

## Purpose and language

EvidenceStore is an adapter over an authorized shared directory; it is not a service-owned database.

## Aggregate boundary and invariants

Append uses create/append, newline-delimited JSON and sync_data. Load rejects malformed lines instead of skipping them.

## Commands, events and failure semantics

PersistenceCommitted makes the record visible after process restart. StorageFailure prevents a completed response. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
