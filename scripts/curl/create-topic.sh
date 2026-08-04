#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "$SCRIPT_DIR/_lib.sh"

usage() {
  echo "Usage: create-topic.sh <name> [num_partitions=1] [replication_factor=1]"
  exit 1
}

if [[ $# -lt 1 ]]; then
  usage
fi

name="$(json_escape "${1:?}")"
num_partitions="${2:-1}"
replication_factor="${3:-1}"

body=$(printf '{"name":"%s","numPartitions":%s,"replicationFactor":%s}' \
  "$name" "$num_partitions" "$replication_factor")

curl -sS -X POST "$BASE_URL/topics" \
  -H "Content-Type: application/json" \
  -d "$body" | format_json
