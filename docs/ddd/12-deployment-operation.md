# Deployment and Operation bounded context

## Purpose and language

ProductRuntime contains one Rust supplier, built React assets, an external evidence volume and externally prepared upstream bindings.

## Aggregate boundary and invariants

Start/stop/reset/seed commands resolve absolute repo paths. Container and Cloud Run configurations accept PORT and storage bindings without performing deployment.

## Commands, events and failure semantics

RuntimeReady requires API and UI health. Restarted proves a new process can render the same sealed record. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
