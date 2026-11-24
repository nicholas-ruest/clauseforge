#!/usr/bin/env bash
set -euo pipefail
cargo test --locked -p clauseforge-service policy_denial_has_no_downstream_write -- --nocapture
grep -q 'middleware::from_fn(authn)' crates/clauseforge-service/src/app.rs
grep -q 'middleware::from_fn(authz_policy)' crates/clauseforge-service/src/app.rs
echo MIDDLEWARE_ORDER_AND_DENIAL_PASS
