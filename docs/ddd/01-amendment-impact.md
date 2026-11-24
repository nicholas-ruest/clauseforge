# Amendment Impact bounded context

## Purpose and language

A AmendmentIntent captures supplier, amendment, affected control scope, contractual cure objective and accountable reviewer before execution.

## Aggregate boundary and invariants

The aggregate rejects empty supplier/amendment fields, cure periods outside 1–365 days and affected-control counts outside the closed 1–100 range. A validated intent is immutable for the duration of one execution.

## Commands, events and failure semantics

ScenarioAccepted and ValidationCompleted are emitted as typed stages; rejection produces no durable record. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
