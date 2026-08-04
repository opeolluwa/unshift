#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
COMPOSE_FILE="$REPO_ROOT/docker-compose.yaml"
CONFIG_PATH="${1:-$REPO_ROOT/scripts/topics.yaml}"
KAFKA_SERVICE="kafka"
KAFKA_BROKER="kafka:29092"

kafka_topics() {
  docker compose -f "$COMPOSE_FILE" exec -T "$KAFKA_SERVICE" \
    kafka-topics --bootstrap-server "$KAFKA_BROKER" "$@"
}

parse_topics() {
  awk '
    /^topics:/ { in_topics = 1; next }
    in_topics && /^[[:space:]]*-[[:space:]]*name:/ {
      if (name != "") print name, partitions, replication
      name = $NF; partitions = 1; replication = 1
      next
    }
    in_topics && /num_partitions:/ { partitions = $NF }
    in_topics && /replication_factor:/ { replication = $NF }
    END { if (in_topics && name != "") print name, partitions, replication }
  ' "$CONFIG_PATH"
}

if [[ ! -f "$CONFIG_PATH" ]]; then
  echo "error: config file not found: $CONFIG_PATH" >&2
  exit 1
fi

if ! docker compose -f "$COMPOSE_FILE" exec -T "$KAFKA_SERVICE" true 2>/dev/null; then
  echo "error: kafka container '$KAFKA_SERVICE' is not running. start it with: docker compose up -d $KAFKA_SERVICE" >&2
  exit 1
fi
existing_topics=$(kafka_topics --list)

mapfile -t definitions < <(parse_topics)

created=0
skipped=0

for definition in "${definitions[@]}"; do
  read -r name partitions replication <<< "$definition"

  if grep -qxF "$name" <<< "$existing_topics"; then
    echo "topic '$name' already exists, skipping"
    skipped=$((skipped + 1))
    continue
  fi

  kafka_topics --create --topic "$name" --partitions "$partitions" --replication-factor "$replication"
  created=$((created + 1))
done

echo "bootstrap complete: $created created, $skipped skipped"
