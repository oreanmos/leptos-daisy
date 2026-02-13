#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/release-tag.sh [--dry-run] <version>

Examples:
  scripts/release-tag.sh 0.1.1
  scripts/release-tag.sh v0.1.1
  scripts/release-tag.sh --dry-run 0.1.1

What it does:
  1) Updates crates/leptos-daisyui/Cargo.toml version
  2) Validates tag does not already exist
  3) Prints exact git commands to commit, tag, and push

Notes:
  - This script does not commit or push for you.
  - Version format must be semantic version: X.Y.Z (optional pre-release/build suffix).
EOF
}

DRY_RUN=false
VERSION_INPUT=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      if [[ -n "$VERSION_INPUT" ]]; then
        echo "error: only one version argument is supported"
        usage
        exit 1
      fi
      VERSION_INPUT="$1"
      shift
      ;;
  esac
done

if [[ -z "$VERSION_INPUT" ]]; then
  echo "error: missing version argument"
  usage
  exit 1
fi

VERSION="${VERSION_INPUT#v}"
SEMVER_RE='^[0-9]+\.[0-9]+\.[0-9]+([-.][0-9A-Za-z.-]+)?(\+[0-9A-Za-z.-]+)?$'
if [[ ! "$VERSION" =~ $SEMVER_RE ]]; then
  echo "error: invalid version '$VERSION_INPUT'"
  echo "expected format: X.Y.Z (e.g. 0.1.1)"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CRATE_TOML="$REPO_ROOT/crates/leptos-daisyui/Cargo.toml"

if [[ ! -f "$CRATE_TOML" ]]; then
  echo "error: expected crate manifest at $CRATE_TOML"
  exit 1
fi

CURRENT_VERSION="$(grep '^version = ' "$CRATE_TOML" | head -n1 | cut -d '"' -f2)"
TARGET_TAG="v$VERSION"

if [[ "$CURRENT_VERSION" == "$VERSION" ]]; then
  echo "info: crate version is already $VERSION"
fi

if git -C "$REPO_ROOT" rev-parse "$TARGET_TAG" >/dev/null 2>&1; then
  echo "error: local tag '$TARGET_TAG' already exists"
  exit 1
fi

if git -C "$REPO_ROOT" ls-remote --tags origin "refs/tags/$TARGET_TAG" | grep -q "$TARGET_TAG"; then
  echo "error: remote tag '$TARGET_TAG' already exists on origin"
  exit 1
fi

if [[ "$DRY_RUN" == true ]]; then
  echo "[dry-run] Would update $CRATE_TOML version: $CURRENT_VERSION -> $VERSION"
else
  sed -i -E "0,/^version = \"[^\"]+\"/s//version = \"$VERSION\"/" "$CRATE_TOML"
  UPDATED_VERSION="$(grep '^version = ' "$CRATE_TOML" | head -n1 | cut -d '"' -f2)"
  if [[ "$UPDATED_VERSION" != "$VERSION" ]]; then
    echo "error: failed to update crate version to $VERSION"
    exit 1
  fi
  echo "updated crates/leptos-daisyui/Cargo.toml: $CURRENT_VERSION -> $VERSION"
fi

cat <<EOF

Next commands:
  git add crates/leptos-daisyui/Cargo.toml
  git commit -m "release: $TARGET_TAG"
  git tag $TARGET_TAG
  git push origin main --tags

When pushed, workflow 'Private Crate Release' will run and package the crate.
EOF
