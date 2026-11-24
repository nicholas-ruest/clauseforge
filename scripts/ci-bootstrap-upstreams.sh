#!/usr/bin/env bash
set -euo pipefail
root=${1:?upstream checkout directory required}; mkdir -p "$root"
clone_pin(){
  local name=$1 url=$2 sha=$3 dir="$root/$1"
  test ! -e "$dir"; git init -q "$dir"
  git -C "$dir" remote add origin "$url"
  git -C "$dir" fetch -q --depth 1 origin "$sha"
  git -C "$dir" checkout -q --detach FETCH_HEAD
  test "$(git -C "$dir" rev-parse HEAD)" = "$sha"
}
clone_pin ruvector https://github.com/ruvnet/RuVector.git 979c31acc57b970c1a7e0771ed527c1703c137e8
clone_pin ruv-fann https://github.com/ruvnet/ruv-FANN.git 02431fd3b3d5639f38feee8f964ec9a8bcc27fc5
clone_pin midstream https://github.com/ruvnet/midstream.git 0934fd3b2f92b4abf93d0d56e6d9085a8d4ad92e
clone_pin qudag https://github.com/ruvnet/QuDAG.git b53466814554737c93f916d458783f7dff6c8acd
clone_pin guardrail https://github.com/ruvnet/guardrail.git 65df339ce21322290fd4154c9fde96c0c7ed1c96
clone_pin opa https://github.com/open-policy-agent/opa.git ba39415db4b3c90cd68942a74b3cc2223ec2b23c
clone_pin dspy.ts https://github.com/ruvnet/dspy.ts.git 200c03a47043b5d1303b631c5d7b4d29d9fe88fa
clone_pin logact https://github.com/facebookresearch/logact.git e24c1fba1f03e205ed410da5f8c68e7c89b1ff4c
python3 -m venv "$root/guardrail/.venv"
"$root/guardrail/.venv/bin/pip" install --disable-pip-version-check -q -r "$root/guardrail/requirements.txt"
(
  cd "$root/opa"
  go build -trimpath -o opa .
)
test -x "$root/opa/opa"
test -x "$root/guardrail/.venv/bin/python"
printf 'CI_UPSTREAMS_READY=%s\n' "$root"
