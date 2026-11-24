#!/usr/bin/env bash
set -euo pipefail
for crate in ruvector fann midstream qudag guardrail opa dspy logact; do cargo test --locked -p "clauseforge-upstream-$crate"; done
for slug in ruvector fann midstream qudag guardrail opa dspy logact; do printf '%s=' "$slug"; scripts/upstream-runtime/run.sh "$slug"; done
echo INTEGRATION_EXECUTION_PASS
