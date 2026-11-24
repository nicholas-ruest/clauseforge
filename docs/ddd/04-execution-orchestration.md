# Execution Orchestration bounded context

## Purpose and language

ExecutionPlan orders accountable domain stages and selected upstream responsibilities for one immutable intent.

## Aggregate boundary and invariants

All ten domain stages run before seven adapters. Any stage failure aborts the result; partial stage lists are diagnostic only and never sealed as completed.

## Commands, events and failure semantics

ExecutionCompleted contains exactly seventeen named stages. ExecutionFailed carries the correlation ID and typed cause. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
