#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "$SCRIPT_DIR/_lib.sh"

usage() {
  echo "Usage: publish-message.sh <topic> <key> <payload>"
  exit 1
}

if [[ $# -lt 3 ]]; then
  usage
fi

topic="${1:?}"
key="$(json_escape "${2:?}")"
payload="$(json_escape "${3:?}")"

body=$(printf '{"key":"%s","payload":"%s"}' "$key" "$payload")

curl -sS -X POST "$BASE_URL/topics/$topic/messages" \
  -H "Content-Type: application/json" \
  -d "$body" | format_json
