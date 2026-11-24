# Risk Calibration bounded context

## Purpose and language

RiskScore is a bounded composite derived from evidence-bearing stages rather than a predictive probability.

## Aggregate boundary and invariants

Each stage supplies a finite score in [0,1]; aggregation rejects an empty set or non-finite values and computes a deterministic arithmetic mean.

## Commands, events and failure semantics

RiskAggregated is stored alongside the stage list. Benchmarks compare full and ablated paths to show the integration contribution. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
