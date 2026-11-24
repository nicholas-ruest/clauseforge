#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); runtime="$root/.runtime"; mkdir -p "$runtime" "$root/var/clauseforge"
"$root/scripts/stop.sh" >/dev/null 2>&1 || true
export PATH="$HOME/.cargo/bin:/usr/bin:$PATH" PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-${XDG_CACHE_HOME:-$HOME/.cache}/ms-playwright}"
export PORT=${CLAUSEFORGE_PORT:-8787} CLAUSEFORGE_DATA_PATH=${CLAUSEFORGE_DATA_PATH:-$root/var/clauseforge/evidence.jsonl}
export CLAUSEFORGE_RUVECTOR_CMD="$root/scripts/upstream-runtime/run.sh ruvector"
export CLAUSEFORGE_FANN_CMD="$root/scripts/upstream-runtime/run.sh fann"
export CLAUSEFORGE_MIDSTREAM_CMD="$root/scripts/upstream-runtime/run.sh midstream"
export CLAUSEFORGE_QUDAG_CMD="$root/scripts/upstream-runtime/run.sh qudag"
export CLAUSEFORGE_GUARDRAIL_CMD="$root/scripts/upstream-runtime/run.sh guardrail"
export CLAUSEFORGE_OPA_CMD="$root/scripts/upstream-runtime/run.sh opa"
export CLAUSEFORGE_DSPY_CMD="$root/scripts/upstream-runtime/run.sh dspy"
export CLAUSEFORGE_LOGACT_CMD="$root/scripts/upstream-runtime/run.sh logact"
setsid sh -c "cd '$root' && exec cargo run --locked -p clauseforge-service" >"$runtime/backend.log" 2>&1 & echo $! >"$runtime/backend.pid"
setsid sh -c "cd '$root/frontend' && exec npm run dev -- --port 4173 --strictPort" >"$runtime/frontend.log" 2>&1 & echo $! >"$runtime/frontend.pid"
"$root/scripts/readiness.sh"
echo "ClauseForge ready at http://127.0.0.1:4173"
