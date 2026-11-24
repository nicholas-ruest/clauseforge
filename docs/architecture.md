# Architecture

The React console speaks the versioned `/api/v1` contract on the same origin as the Axum composition service. Request layers establish correlation, authentication, authorization, a deadline and trace context before the handler constructs an immutable amendment review request. Orchestration calls ten typed domain stages, then eight fail-closed upstream adapters. The score and stage names become a canonical payload. Audit reconstruction verifies all prior records before the append-only store fsyncs the new record.

The production composition root is `clauseforge-service`; Cargo metadata proves reachability across 25 workspace packages. The browser never owns durable truth. The JSONL evidence file is an authorized shared-storage boundary and is mounted externally in a container or Cloud Run configuration. Static React assets are served by the Rust service after the Vite build.

Failures are intentionally asymmetric: validation, denial, upstream failure or integrity failure return no completion record. Read paths verify record integrity again. External evidence is collected outside the checkout and bound to a clean Git source SHA.
