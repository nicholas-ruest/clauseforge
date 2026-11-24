# Upstream Capabilities bounded context

## Purpose and language

An UpstreamResponsibility pairs canonical project identity, exact revision, product responsibility and typed StageResult.

## Aggregate boundary and invariants

Each adapter launches its configured command and requires a capability-specific marker. The wrapper first checks checkout HEAD against the selected pin.

## Commands, events and failure semantics

UpstreamSucceeded adds one named stage. ExitFailure, MarkerMissing and RevisionMismatch fail closed. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
