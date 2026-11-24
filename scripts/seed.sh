#!/usr/bin/env bash
set -euo pipefail
curl -fsS -X POST http://127.0.0.1:8787/api/v1/amendments -H 'content-type: application/json' -H 'authorization: Bearer clauseforge-reviewer' -H 'x-reviewer: seed@example.com' -H 'x-role: amendment-reviewer' --data '{"supplier":"VectorCloud AI","amendment":"permit subprocessor model training with 72-hour notice","affected_controls":32,"cure_days":14}'
