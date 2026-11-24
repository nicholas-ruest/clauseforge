#!/usr/bin/env bash
set -euo pipefail
slug=${1:?upstream slug required}
root=${CLAUSEFORGE_UPSTREAM_ROOT:?exact-pinned upstream root required}
export CARGO_TARGET_DIR=${CLAUSEFORGE_UPSTREAM_TARGET:-${XDG_CACHE_HOME:-$HOME/.cache}/clauseforge-upstream-target}
verify(){ test -d "$root/$1/.git"; test "$(git -C "$root/$1" rev-parse HEAD)" = "$2"; }
case "$slug" in
  ruvector) verify ruvector 979c31acc57b970c1a7e0771ed527c1703c137e8; cargo test --quiet --manifest-path "$root/ruvector/Cargo.toml" -p ruvector-core --test filtered_search -- --nocapture >/dev/null; echo similarity_set ;;
  fann) verify ruv-fann 02431fd3b3d5639f38feee8f964ec9a8bcc27fc5; cargo test --quiet --manifest-path "$root/ruv-fann/Cargo.toml" reference_ranks_and_deterministic_sorted_intervals -- --nocapture >/dev/null; echo calibrated_interval ;;
  midstream) verify midstream 0934fd3b2f92b4abf93d0d56e6d9085a8d4ad92e; cargo test --quiet --manifest-path "$root/midstream/Cargo.toml" -p midstream test_metric_ingestion -- --nocapture >/dev/null; echo safety_window ;;
  qudag) verify qudag b53466814554737c93f916d458783f7dff6c8acd; cargo test --quiet --manifest-path "$root/qudag/Cargo.toml" -p qudag-network test_ml_kem_encapsulation_decapsulation -- --nocapture >/dev/null; echo kem_ciphertext ;;
  guardrail) verify guardrail 65df339ce21322290fd4154c9fde96c0c7ed1c96; python_bin=${CLAUSEFORGE_GUARDRAIL_PYTHON:-$root/guardrail/.venv/bin/python}; (cd "$root/guardrail" && "$python_bin" -m unittest tests.test_conditions.ConditionTests.test_present_nested_array_value >/dev/null); echo guardrail_valid ;;
  opa) verify opa ba39415db4b3c90cd68942a74b3cc2223ec2b23c; test "$("$root/opa/opa" eval --fail -f raw -d "$(dirname "$0")/policy.rego" -i "$(dirname "$0")/allow.json" 'data.clauseforge.allow')" = true; echo policy_allow ;;
  dspy) verify dspy.ts 200c03a47043b5d1303b631c5d7b4d29d9fe88fa; (cd "$root/dspy.ts" && node --experimental-strip-types --input-type=module -e 'import {topK} from "./src/utils/top-k.ts"; const rows=[{id:"audit",risk:9},{id:"notice",risk:4},{id:"transfer",risk:9}]; const out=topK(rows,2,(a,b)=>b.risk-a.risk); if(out.map(x=>x.id).join(",")!=="audit,transfer") throw Error("unstable ranking");' >/dev/null); echo dspy_rank ;;
  logact) verify logact e24c1fba1f03e205ed410da5f8c68e7c89b1ff4c; cargo test --quiet --locked --manifest-path "$root/logact/Cargo.toml" -p logact_commit_service_policy_register synchronous_provider::tests::register_updates_advance_the_provider_generation -- --exact >/dev/null; echo logact_policy ;;
  *) echo "unknown upstream: $slug" >&2; exit 64 ;;
esac
