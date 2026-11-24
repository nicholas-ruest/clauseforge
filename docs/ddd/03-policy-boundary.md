# Policy Boundary bounded context

## Purpose and language

BlastBoundary combines requested radius, reviewer role and the enterprise policy decision.

## Aggregate boundary and invariants

The local precondition radius ≤80 is enforced before execution and the pinned OPA operation independently demonstrates the same responsibility.

## Commands, events and failure semantics

BoundaryHeld is a first-class negative outcome. It is intentionally not persisted as a successful amendment review record. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
