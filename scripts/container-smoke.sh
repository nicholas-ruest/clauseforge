#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); image=localhost/clauseforge:${SOURCE_SHA:-local}
podman build --network=host --tag "$image" "$root"
cid=$(podman run -d --rm --network=host -e PORT=8877 -e CLAUSEFORGE_DATA_PATH=/tmp/evidence.jsonl -e CLAUSEFORGE_RUVECTOR_CMD="printf similarity_set" -e CLAUSEFORGE_FANN_CMD="printf calibrated_interval" -e CLAUSEFORGE_MIDSTREAM_CMD="printf impact_metric" -e CLAUSEFORGE_QUDAG_CMD="printf kem_ciphertext" -e CLAUSEFORGE_GUARDRAIL_CMD="printf guardrail_valid" -e CLAUSEFORGE_OPA_CMD="printf policy_allow" -e CLAUSEFORGE_DSPY_CMD="printf dspy_rank" -e CLAUSEFORGE_LOGACT_CMD="printf logact_policy" "$image"); trap 'podman stop "$cid" >/dev/null 2>&1 || true' EXIT
for _ in $(seq 1 40); do curl -fsS http://127.0.0.1:8877/healthz >/dev/null && curl -fsS http://127.0.0.1:8877/ >/dev/null && curl -fsS -X POST http://127.0.0.1:8877/api/v1/amendments -H 'content-type: application/json' -H 'authorization: Bearer clauseforge-reviewer' -H 'x-reviewer: container@example.com' -H 'x-role: amendment-reviewer' --data '{"supplier":"Container AI","amendment":"extend model retention to 30 days","affected_controls":25,"cure_days":14}' | grep -q '"status":"completed"' && curl -fsS http://127.0.0.1:8877/api/v1/audit | grep -q 'Container AI' && { echo CONTAINER_TOPOLOGY_PASS; exit 0; }; sleep 1; done
podman logs "$cid"; exit 1
