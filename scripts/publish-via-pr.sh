#!/usr/bin/env bash
set -euo pipefail
owner=nicholas-ruest; repo=clauseforge; branch=product/clauseforge-2026-11-23
test "$(gh api user --jq .login)" = "$owner" || { echo PUBLISH_DENIED:wrong-owner >&2; exit 71; }
test -z "$(git status --porcelain)"; source_sha=$(git rev-parse HEAD); base_sha=$(git rev-list --max-parents=0 HEAD)
test "$(git ls-tree -r --name-only "$base_sha" | wc -l)" -ge 10
remote_main=""; remote_product=""
if gh repo view "$owner/$repo" >/dev/null 2>&1; then
  test "$(gh pr list --repo "$owner/$repo" --state all --json number --jq length)" = "0" || { echo PUBLISH_DENIED:repository-has-prs >&2; exit 72; }
  test "$(gh release list --repo "$owner/$repo" --json tagName --jq length)" = "0" || { echo PUBLISH_DENIED:repository-has-releases >&2; exit 72; }
  refs_json=$(gh api "repos/$owner/$repo/git/matching-refs/heads")
  remote_main=$(printf '%s' "$refs_json" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{const r=JSON.parse(s).find(x=>x.ref==="refs/heads/main");if(r)process.stdout.write(r.object.sha)})')
  remote_product=$(printf '%s' "$refs_json" | BRANCH="$branch" node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{const r=JSON.parse(s).find(x=>x.ref===`refs/heads/${process.env.BRANCH}`);if(r)process.stdout.write(r.object.sha)})')
  if [[ -n "$remote_main" ]]; then
    test "$remote_main" = "$base_sha" || { echo PUBLISH_DENIED:unexpected-main >&2; exit 72; }
  else
    test "$(gh api "repos/$owner/$repo" --jq .size)" = "0" || { echo PUBLISH_DENIED:repository-not-empty >&2; exit 72; }
  fi
  if [[ -n "$remote_product" && "$remote_product" != "$source_sha" ]]; then
    git cat-file -e "$remote_product^{commit}" 2>/dev/null || { echo PUBLISH_DENIED:unknown-product-commit >&2; exit 72; }
    git merge-base --is-ancestor "$remote_product" "$source_sha" || { echo PUBLISH_DENIED:non-fast-forward-product-branch >&2; exit 72; }
  fi
  expected_refs=0
  [[ -n "$remote_main" ]] && ((expected_refs += 1))
  [[ -n "$remote_product" ]] && ((expected_refs += 1))
  actual_refs=$(printf '%s' "$refs_json" | node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>process.stdout.write(String(JSON.parse(s).length)))')
  test "$actual_refs" -eq "$expected_refs" || { echo PUBLISH_DENIED:unexpected-branches >&2; exit 72; }
else
  gh repo create "$owner/$repo" --public --description "Evidence-bearing supplier AI amendment impact studio"
fi
remote="https://github.com/$owner/$repo.git"
if git remote get-url origin >/dev/null 2>&1; then
  test "$(git remote get-url origin)" = "$remote"
else
  git remote add origin "$remote"
fi
if [[ -z "$remote_main" ]]; then
  if ! git push origin "$base_sha:refs/heads/main" 2>&1 | tee "$DREAM_MACHINE_EVIDENCE_DIR/publisher-main-push.log"; then
    observed_main=$(gh api "repos/$owner/$repo/git/ref/heads/main" --jq .object.sha 2>/dev/null || true)
    test "$observed_main" = "$base_sha" || exit 73
  fi
  test "$(gh api "repos/$owner/$repo/git/ref/heads/main" --jq .object.sha)" = "$base_sha"
fi
if [[ "$remote_product" != "$source_sha" ]]; then
  if ! git push origin "$source_sha:refs/heads/$branch" 2>&1 | tee "$DREAM_MACHINE_EVIDENCE_DIR/publisher-product-push.log"; then
    observed_product=$(gh api "repos/$owner/$repo/git/ref/heads/$branch" --jq .object.sha 2>/dev/null || true)
    test "$observed_product" = "$source_sha" || exit 73
  fi
fi
test "$(gh api "repos/$owner/$repo/git/ref/heads/$branch" --jq .object.sha)" = "$source_sha"
stage=$(mktemp -d); trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/product-evidence" "$stage/seed/selection-evidence"
cp -a "${DREAM_MACHINE_EVIDENCE_DIR:?}"/. "$stage/product-evidence/"
cp "${DREAM_MACHINE_SEMANTIC_REVIEW:?}" "$stage/seed/semantic-review.md"
cp "${DREAM_MACHINE_SELECTION_REPORT:?}" "$stage/seed/selection-report.json"
cp -a "${DREAM_MACHINE_SELECTION_EVIDENCE:?}"/. "$stage/seed/selection-evidence/"
asset="clauseforge-evidence-$source_sha.tgz"; archive="$stage/$asset"
tar -czf "$archive" -C "$stage" product-evidence seed
tag="evidence-$source_sha"
gh release create "$tag" "$archive#$asset" --repo "$owner/$repo" --target "$branch" --title "Exact-source verification evidence" --notes "Source-bound product evidence and CI seed for $source_sha."
evidence_url="https://github.com/$owner/$repo/releases/download/$tag/$asset"
gh pr create --repo "$owner/$repo" --base main --head "$branch" --title "Build ClauseForge complete product" --body "Complete React, typed API, executable middleware, 26-crate Rust backend, eight source-pinned integrations, restart-safe audit evidence, containers and three browser journeys. Human merge required."
printf '{"owner":"%s","repository":"%s","source_sha":"%s","branch":"%s","base_sha":"%s","evidence_url":"%s"}\n' "$owner" "$repo" "$source_sha" "$branch" "$base_sha" "$evidence_url"
