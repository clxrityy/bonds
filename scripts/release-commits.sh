#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  scripts/release-commits.sh v0.1.5 [output-file]

What it does:
- Fails if the target GitHub Release already exists
- Fails if the target tag already exists locally or on origin
- Finds the newest older published GitHub release matching v*
- Collects commits after that release up to current HEAD
- Writes a local Markdown file for copy/paste into release notes

Requirements:
- git
- gh (authenticated)
- python3
EOF
}

fail() {
  echo "error: $*" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || fail "missing required command: $1"
}

extract_repo_slug() {
  python3 - "$1" <<'PY'
import re
import sys

url = sys.argv[1].strip()
m = re.search(r'github\.com[:/](.+?)(?:\.git)?$', url)
if not m:
    raise SystemExit(1)
print(m.group(1))
PY
}

TARGET_TAG="${1:-}"
OUTPUT_FILE="${2:-}"

[[ -n "$TARGET_TAG" ]] || { usage; exit 1; }
[[ "$TARGET_TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || \
  fail "target tag must look like vMAJOR.MINOR.PATCH (example: v0.1.5)"

require_cmd git
require_cmd gh
require_cmd python3

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || fail "run this from inside the repository"

git remote get-url origin >/dev/null 2>&1 || fail "missing origin remote"
REPO_URL="$(git remote get-url origin)"
REPO="$(extract_repo_slug "$REPO_URL")" || fail "could not determine GitHub repo from origin: $REPO_URL"

gh auth status >/dev/null 2>&1 || fail "gh is not authenticated; run: gh auth login"

if gh release view "$TARGET_TAG" --repo "$REPO" >/dev/null 2>&1; then
  fail "GitHub Release '$TARGET_TAG' already exists in $REPO"
fi

if git rev-parse -q --verify "refs/tags/$TARGET_TAG" >/dev/null; then
  fail "local tag '$TARGET_TAG' already exists"
fi

if git ls-remote --exit-code --tags origin "refs/tags/$TARGET_TAG" >/dev/null 2>&1; then
  fail "remote tag '$TARGET_TAG' already exists on origin"
fi

RELEASE_TAGS_JSON="$(
  gh release list \
    --repo "$REPO" \
    --exclude-drafts \
    --exclude-pre-releases \
    --limit 200 \
    --json tagName
)"

PREV_TAG="$(
  RELEASE_TAGS_JSON="$RELEASE_TAGS_JSON" python3 - "$TARGET_TAG" <<'PY'
import json
import os
import re
import sys

target = sys.argv[1]
data = json.loads(os.environ["RELEASE_TAGS_JSON"])

def parse(tag: str):
    m = re.fullmatch(r"v(\d+)\.(\d+)\.(\d+)", tag)
    return tuple(int(x) for x in m.groups()) if m else None

target_version = parse(target)
if target_version is None:
    print("__INVALID_TARGET__")
    raise SystemExit(0)

tags = []
for item in data:
    tag = item.get("tagName", "")
    version = parse(tag)
    if version is not None:
        tags.append((version, tag))

if not tags:
    print("")
    raise SystemExit(0)

latest_version, latest_tag = max(tags)
if target_version <= latest_version:
    print(f"__NOT_NEWER__:{latest_tag}")
    raise SystemExit(0)

older = [pair for pair in tags if pair[0] < target_version]
if not older:
    print("")
else:
    print(max(older)[1])
PY
)"

case "$PREV_TAG" in
  __INVALID_TARGET__)
    fail "invalid target tag format: $TARGET_TAG"
    ;;
  __NOT_NEWER__*)
    fail "target '$TARGET_TAG' is not newer than the latest published v* release '${PREV_TAG#__NOT_NEWER__:}'"
    ;;
esac

OUTPUT_FILE="${OUTPUT_FILE:-release-notes/${TARGET_TAG}-commits.md}"
mkdir -p "$(dirname "$OUTPUT_FILE")"

if [[ -n "$PREV_TAG" ]]; then
  RANGE="${PREV_TAG}..HEAD"
  COMPARE_URL="https://github.com/${REPO}/compare/${PREV_TAG}...HEAD"
  COMMIT_COUNT="$(git rev-list --count "${PREV_TAG}..HEAD")"
else
  RANGE="HEAD"
  COMPARE_URL=""
  COMMIT_COUNT="$(git rev-list --count HEAD)"
fi

GENERATED_AT="$(date -u '+%Y-%m-%d %H:%M:%S UTC')"

{
  echo "# Commits for ${TARGET_TAG}"
  echo
  echo "- Repository: \`${REPO}\`"
  echo "- Target release: \`${TARGET_TAG}\`"
  if [[ -n "$PREV_TAG" ]]; then
    echo "- Previous release: \`${PREV_TAG}\`"
  else
    echo "- Previous release: _none found_"
  fi
  echo "- Commit range: \`${RANGE}\`"
  if [[ -n "$COMPARE_URL" ]]; then
    echo "- Compare view: ${COMPARE_URL}"
  fi
  echo "- Generated: ${GENERATED_AT}"
  echo
  echo "## Commits"
  echo

  if [[ "$COMMIT_COUNT" -eq 0 ]]; then
    echo "_No commits found for this release range._"
  else
    git log --reverse --format='%H%x09%s' "$RANGE" | \
    while IFS=$'\t' read -r sha subject; do
      short_sha="${sha:0:7}"
      url="https://github.com/${REPO}/commit/${sha}"
      printf -- "- %s — [\`%s\`](%s)\n" "$subject" "$short_sha" "$url"
    done
  fi
} > "$OUTPUT_FILE"

echo "Wrote $OUTPUT_FILE"
