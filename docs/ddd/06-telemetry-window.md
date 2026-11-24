# Telemetry Window bounded context

## Purpose and language

TelemetryWindow represents evidence observed during the bounded contractual cure objective, not indefinite monitoring.

## Aggregate boundary and invariants

Midstream contributes safety-window ingestion while the domain telemetry stage checks that the requested recovery window and service context are usable.

## Commands, events and failure semantics

TelemetryAccepted advances execution; upstream process or marker failure produces an UpstreamFailure and no write. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
