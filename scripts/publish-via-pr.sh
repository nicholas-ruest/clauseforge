#!/usr/bin/env bash
set -euo pipefail
owner=nicholas-ruest; repo=clauseforge; branch=product/clauseforge-2026-11-23
test "$(gh api user --jq .login)" = "$owner" || { echo PUBLISH_DENIED:wrong-owner >&2; exit 71; }
test -z "$(git status --porcelain)"; source_sha=$(git rev-parse HEAD); base_sha=$(git rev-list --max-parents=0 HEAD)
test "$(git ls-tree -r --name-only "$base_sha" | wc -l)" -ge 10
if gh repo view "$owner/$repo" >/dev/null 2>&1; then
  test "$(gh pr list --repo "$owner/$repo" --state all --json number --jq length)" = "0" || { echo PUBLISH_DENIED:repository-has-prs >&2; exit 72; }
  test "$(gh release list --repo "$owner/$repo" --json tagName --jq length)" = "0" || { echo PUBLISH_DENIED:repository-has-releases >&2; exit 72; }
  remote_main=$(gh api "repos/$owner/$repo/git/ref/heads/main" --jq .object.sha 2>/dev/null || true)
  remote_product=$(gh api "repos/$owner/$repo/git/ref/heads/$branch" --jq .object.sha 2>/dev/null || true)
  if [[ -n "$remote_main" ]]; then
    test "$remote_main" = "$base_sha" || { echo PUBLISH_DENIED:unexpected-main >&2; exit 72; }
  else
    test "$(gh api "repos/$owner/$repo" --jq .size)" = "0" || { echo PUBLISH_DENIED:repository-not-empty >&2; exit 72; }
  fi
  [[ -z "$remote_product" || "$remote_product" = "$source_sha" ]] || { echo PUBLISH_DENIED:unexpected-product-branch >&2; exit 72; }
  expected_refs=1
  [[ -n "$remote_product" ]] && expected_refs=2
  actual_refs=$(gh api "repos/$owner/$repo/git/matching-refs/heads" --jq length 2>/dev/null || echo 0)
  test "$actual_refs" -le "$expected_refs" || { echo PUBLISH_DENIED:unexpected-branches >&2; exit 72; }
else
  gh repo create "$owner/$repo" --public --description "Evidence-bearing supplier AI amendment impact studio"
fi
remote="https://github.com/$owner/$repo.git"
if git remote get-url origin >/dev/null 2>&1; then
  test "$(git remote get-url origin)" = "$remote"
else
  git remote add origin "$remote"
fi
git push origin "$base_sha:refs/heads/main"; git push origin "$source_sha:refs/heads/$branch"
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
