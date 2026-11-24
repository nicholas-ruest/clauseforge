#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/.." && pwd); "$root/scripts/stop.sh" || true
target="$root/var/clauseforge"; test "$target" = "$root/var/clauseforge"; rm -rf "$target"; mkdir -p "$target"
