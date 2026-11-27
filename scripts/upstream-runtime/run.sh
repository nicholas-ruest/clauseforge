#!/usr/bin/env bash
set -euo pipefail
slug=${1:?upstream slug required}
root=${CLAUSEFORGE_UPSTREAM_ROOT:?exact-pinned upstream root required}
export CARGO_TARGET_DIR=${CLAUSEFORGE_UPSTREAM_TARGET:-${XDG_CACHE_HOME:-$HOME/.cache}/clauseforge-upstream-target}
project=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$project/.upstream-links"
for source in ruvector ruv-fann midstream qudag logact; do
  ln -sfn "$root/$source" "$project/.upstream-links/$source"
done
verify(){ test -d "$root/$1/.git"; test "$(git -C "$root/$1" rev-parse HEAD)" = "$2"; }
case "$slug" in
  ruvector) verify ruvector 979c31acc57b970c1a7e0771ed527c1703c137e8; cargo run --quiet --locked --manifest-path "$project/scripts/upstream-runtime/probes/ruvector/Cargo.toml" ;;
  fann) verify ruv-fann 02431fd3b3d5639f38feee8f964ec9a8bcc27fc5; cargo run --quiet --locked --manifest-path "$project/scripts/upstream-runtime/probes/fann/Cargo.toml" ;;
  midstream) verify midstream 0934fd3b2f92b4abf93d0d56e6d9085a8d4ad92e; cargo run --quiet --locked --manifest-path "$project/scripts/upstream-runtime/probes/midstream/Cargo.toml" ;;
  qudag) verify qudag b53466814554737c93f916d458783f7dff6c8acd; cargo run --quiet --locked --manifest-path "$project/scripts/upstream-runtime/probes/qudag/Cargo.toml" ;;
  guardrail) verify guardrail 65df339ce21322290fd4154c9fde96c0c7ed1c96; "${CLAUSEFORGE_GUARDRAIL_PYTHON:-$root/guardrail/.venv/bin/python}" "$project/scripts/upstream-runtime/guardrail_probe.py" ;;
  opa) verify opa ba39415db4b3c90cd68942a74b3cc2223ec2b23c; node -e 'console.log(JSON.stringify({role:"amendment-reviewer",supplier:process.env.CLAUSEFORGE_SUPPLIER,amendment:process.env.CLAUSEFORGE_AMENDMENT,affected_controls:Number(process.env.CLAUSEFORGE_AFFECTED_CONTROLS),cure_days:Number(process.env.CLAUSEFORGE_CURE_DAYS)}))' | "$root/opa/opa" eval --fail --stdin-input -f raw -d "$project/scripts/upstream-runtime/policy.rego" 'data.clauseforge.allow' | grep -qx true; echo "policy_allow:${CLAUSEFORGE_AFFECTED_CONTROLS}:${CLAUSEFORGE_CURE_DAYS}" ;;
  dspy) verify dspy.ts 200c03a47043b5d1303b631c5d7b4d29d9fe88fa; node --experimental-strip-types "$project/scripts/upstream-runtime/dspy_probe.mjs" ;;
  logact) verify logact e24c1fba1f03e205ed410da5f8c68e7c89b1ff4c; cargo run --quiet --locked --manifest-path "$project/scripts/upstream-runtime/probes/logact/Cargo.toml" ;;
  *) echo "unknown upstream: $slug" >&2; exit 64 ;;
esac
