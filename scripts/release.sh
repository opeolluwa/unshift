#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CARGO_TOML="$REPO_ROOT/kernel/Cargo.toml"
PACKAGE_JSON="$REPO_ROOT/console/package.json"

if [[ $# -lt 1 ]]; then
  echo "Usage: release <major|maj|minor|min|patch|pat>"
  exit 1
fi

bump() {
  local parts
  IFS='.' read -r major minor patch <<< "$1"

  case "$2" in
    major|maj) major=$((major + 1)); minor=0; patch=0 ;;
    minor|min) minor=$((minor + 1)); patch=0 ;;
    patch|pat) patch=$((patch + 1)) ;;
    *) echo "Unknown bump type: $2"; exit 1 ;;
  esac

  echo "$major.$minor.$patch"
}

CURRENT_VERSION=$(grep -m1 '^version' "$CARGO_TOML" | sed 's/.*"\(.*\)".*/\1/')
NEW_VERSION=$(bump "$CURRENT_VERSION" "$1")

sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" "$PACKAGE_JSON"

echo "Released $CURRENT_VERSION -> $NEW_VERSION"
