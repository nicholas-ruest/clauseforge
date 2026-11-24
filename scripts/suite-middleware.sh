#!/usr/bin/env bash
set -euo pipefail
cargo test --locked -p clauseforge-service --test failure -- --nocapture
expected='correlation authn authz_policy validate_request normalize_errors timeouts TraceLayer'
for control in $expected; do
  grep -q "$control" crates/clauseforge-service/src/app.rs
done
test "$(grep -n 'middleware::from_fn' crates/clauseforge-service/src/app.rs | wc -l)" -ge 6
echo MIDDLEWARE_ORDER_AND_DENIAL_PASS
