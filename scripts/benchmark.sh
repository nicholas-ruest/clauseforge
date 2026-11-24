#!/usr/bin/env bash
set -euo pipefail
out=${1:?output json required}; start=$(date +%s%N)
body=$(curl -fsS -X POST http://127.0.0.1:8787/api/v1/amendments -H 'content-type: application/json' -H 'authorization: Bearer clauseforge-reviewer' -H 'x-reviewer: benchmark@example.com' -H 'x-role: amendment-reviewer' --data '{"supplier":"VectorCloud","amendment":"permit cross-region model telemetry with 72-hour notice","affected_controls":35,"cure_days":12}')
end=$(date +%s%N); BODY="$body" START="$start" END="$end" node -e 'const r=JSON.parse(process.env.BODY); console.log(JSON.stringify({schema_version:1,mode:"full",elapsed_ms:Number((BigInt(process.env.END)-BigInt(process.env.START))/1000000n),stage_count:r.stages.length,risk_score:r.risk_score,correlation_id:r.correlation_id,all_upstreams:r.stages.length===18},null,2))' > "$out"
node -e 'const r=require(process.argv[1]);if(r.stage_count!==18||!r.all_upstreams)process.exit(1)' "$out"
