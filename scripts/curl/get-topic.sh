#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "$SCRIPT_DIR/_lib.sh"

usage() {
  echo "Usage: get-topic.sh <topic>"
  exit 1
}

if [[ $# -lt 1 ]]; then
  usage
fi

topic="${1:?}"

curl -sS "$BASE_URL/topics/$topic" | format_json
