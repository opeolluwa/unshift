#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
CARGO_TOML="$REPO_ROOT/kernel/Cargo.toml"
PACKAGE_JSON="$REPO_ROOT/console/package.json"

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

if [[ $# -ge 1 ]]; then
  BUMP_TYPE="$1"
else
  echo "Current version: $CURRENT_VERSION"
  echo ""
  echo "  1) patch"
  echo "  2) minor"
  echo "  3) major"
  echo ""
  read -rp "Bump type [1]: " choice
  case "${choice:-1}" in
    1|patch|pat) BUMP_TYPE="patch" ;;
    2|minor|min) BUMP_TYPE="minor" ;;
    3|major|maj) BUMP_TYPE="major" ;;
    *) echo "Invalid choice: $choice"; exit 1 ;;
  esac
fi

NEW_VERSION=$(bump "$CURRENT_VERSION" "$BUMP_TYPE")

DEFAULT_MSG="release v$NEW_VERSION"
read -rp "Commit message [$DEFAULT_MSG]: " COMMIT_MSG
COMMIT_MSG="${COMMIT_MSG:-$DEFAULT_MSG}"

sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" "$CARGO_TOML"
sed -i '' "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" "$PACKAGE_JSON"

echo ""
echo "Bumped $CURRENT_VERSION -> $NEW_VERSION"

cd "$REPO_ROOT"
git add "$CARGO_TOML" "$PACKAGE_JSON"
git commit -m "$COMMIT_MSG"
git tag "v$NEW_VERSION"

echo ""
read -rp "Push to remote? [Y/n]: " push_choice
if [[ "${push_choice:-Y}" =~ ^[Yy]$ ]]; then
  git push && git push --tags
  echo "Pushed v$NEW_VERSION to remote"
else
  echo "Skipped push. Tag v$NEW_VERSION created locally."
fi
