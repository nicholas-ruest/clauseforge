#!/usr/bin/env bash
set -euo pipefail
for crate in ruvector fann midstream qudag guardrail opa dspy logact; do cargo test --locked -p "clauseforge-upstream-$crate"; done
run_bound() {
  local slug=$1 supplier=$2 amendment=$3 controls=$4 cure=$5
  CLAUSEFORGE_SUPPLIER="$supplier" \
  CLAUSEFORGE_AMENDMENT="$amendment" \
  CLAUSEFORGE_AFFECTED_CONTROLS="$controls" \
  CLAUSEFORGE_CURE_DAYS="$cure" \
    scripts/upstream-runtime/run.sh "$slug"
}

for slug in ruvector fann midstream qudag guardrail opa dspy logact; do
  printf '%s=' "$slug"
  first=$(run_bound "$slug" Northstar "audit transfer" 24 30)
  second=$(run_bound "$slug" Meridian "subprocessor notice" 61 45)
  test -n "$first"
  test -n "$second"
  if test "$first" = "$second"; then
    echo "request-independent output from $slug" >&2
    exit 1
  fi
  if CLAUSEFORGE_AMENDMENT="audit transfer" \
      CLAUSEFORGE_AFFECTED_CONTROLS=24 \
      CLAUSEFORGE_CURE_DAYS=30 \
      scripts/upstream-runtime/run.sh "$slug" >/dev/null 2>&1; then
    echo "missing-input failure path unexpectedly passed for $slug" >&2
    exit 1
  fi
  printf '%s -> %s\n' "$first" "$second"
done
echo INTEGRATION_EXECUTION_PASS
