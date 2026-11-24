# Browser Experience bounded context

## Purpose and language

The Studio is the user-facing application service for composing, denying, recovering and reviewing amendments.

## Aggregate boundary and invariants

The form maintains intent state across denial recovery. Result and Evidence views consume only API responses; no browser-local durable truth is credited.

## Commands, events and failure semantics

AmendmentRendered and EvidenceRendered are journey outcomes. Loading, empty, error and success are explicit UI states. Commands entering this context carry the shared correlation identity. Failures are typed and may not be converted into a successful stage or a durable completion record.

## Ownership and dependencies

This context owns its terms and acceptance tests. It depends only on typed contracts from `clauseforge-kernel` or the explicitly named adapter boundary; transport, UI and storage concerns remain outside unless this document assigns them. Reciprocal mappings live in the crate call-path and traceability documents.
