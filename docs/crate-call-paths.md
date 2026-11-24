# Crate-to-backend production call paths

The `clauseforge-service` binary constructs the router and evidence store. The create handler calls identity validation and `clauseforge-orchestration::run_domain`, whose ordered path reaches amendment, identity, authorization, validation, correlation, deadline, observability, risk, telemetry and scheduler. It then calls each of the eight upstream adapter crates and aggregates their typed results.

`clauseforge-api-schema` converts the transport command into `clauseforge-kernel::AmendmentRequest`. The service reconstructs `clauseforge-audit::AuditChain`, whose seal/verify operations use `clauseforge-evidence`, and commits through `clauseforge-persistence`. GET by correlation calls `clauseforge-replay`, which re-verifies stored evidence. The service therefore reaches all 26 packages through a production route rather than development-only features.

Behavior and failure tests live next to every package. Reciprocal mapping is enforced by the composition report and semantic review: a package without a unique responsibility, caller and negative boundary is not counted.
