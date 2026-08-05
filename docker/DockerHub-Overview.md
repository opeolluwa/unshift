# unshift

**Kafka admin UI** — a browser-based console for managing and inspecting your Kafka cluster, like [Kafkadrop](https://github.com/obsidiandynamics/kafdrop), with the added ability to **publish messages** directly from the UI.

| | |
| --- | --- |
| **Image** | `opeolluwa/unshift` |
| **Port** | `8000` |
| **Stack** | Rust (axum + rdkafka) API + Nuxt UI in a single container |

## Features

- **Cluster overview** — bootstrap servers, topic/partition counts, preferred leader and under-replicated metrics
- **Browse topics** — list every topic with partitions, replication health and dynamic configs
- **Create topics** — add a topic with a chosen partition count and replication factor
- **View messages** — read messages from a topic across all partitions (earliest offset)
- **Publish messages** — send a message with a key and payload straight from the UI
- **Single image** — the API and the built frontend are served together on one port, no extra proxy needed

## Quick start

Point the container at an existing Kafka broker:

```bash
docker run -d \
  --name unshift \
  -p 8000:8000 \
  -e KAFKA_BROKER=host.docker.internal:9092 \
  opeolluwa/unshift
```

Then open <http://localhost:8000>.

> On Linux, `host.docker.internal` may require `--add-host=host.docker.internal:host-gateway`. Otherwise, use your broker's reachable address (e.g. `-e KAFKA_BROKER=192.168.1.10:9092`).

## Docker Compose

The quickest way to try everything, including a broker:

```yaml
services:
  unshift:
    image: opeolluwa/unshift
    ports:
      - "8000:8000"
    environment:
      KAFKA_BROKER: kafka:9092
    depends_on:
      - kafka

  kafka:
    image: apache/kafka:3.7.0
    environment:
      KAFKA_NODE_ID: 1
      KAFKA_PROCESS_ROLES: broker,controller
      KAFKA_LISTENERS: PLAINTEXT://:9092,CONTROLLER://:9093
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:9092
      KAFKA_CONTROLLER_LISTENER_NAMES: CONTROLLER
      KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: CONTROLLER:PLAINTEXT,PLAINTEXT:PLAINTEXT
      KAFKA_CONTROLLER_QUORUM_VOTERS: 1@kafka:9093
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
```

```bash
docker compose up -d
```

## Use in an existing Docker Compose project

Add `unshift` as a service to a stack you already run:

```yaml
services:
  unshift:
    image: opeolluwa/unshift
    ports:
      - "8000:8000"
    environment:
      KAFKA_BROKER: kafka:29092
    depends_on:
      kafka:
        condition: service_healthy
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/api/health"]
      interval: 10s
      timeout: 5s
      retries: 3
    networks:
      - internal
```

Notes:

- If your broker service isn't named `kafka`, point `KAFKA_BROKER` at the right name and port (e.g. `broker:29092` for Confluent's `cp-kafka` internal listener).
- The snippet places `unshift` on an explicit `internal` network; the broker must be on that same network (define it under `networks:` if it isn't already).
- No extra proxy is required — the UI and API are both served on port `8000`.

## Environment variables

| Variable | Required | Default | Description |
| --- | --- | --- | --- |
| `KAFKA_BROKER` | Yes | — | Comma-separated Kafka bootstrap servers, e.g. `kafka:29092` |
| `PORT` | No | `8000` | Port the API and UI listen on |
| `ENVIRONMENT` | No | `development` | `development`/`dev` or `production`/`prod` |
| `ALLOWED_ORIGINS` | No | `http://localhost:3000,http://localhost:8000` | Comma-separated CORS allowed origins |
| `REQUESTS_TIME_OUT_SECS` | No | `10` | Request timeout in seconds |

## API reference

The UI is backed by a small REST API under `/api`:

| Method | Path | Description |
| --- | --- | --- |
| `GET` | `/api/health` | Health check |
| `GET` | `/api/cluster/overview` | Cluster statistics |
| `GET` | `/api/topics` | List topics |
| `POST` | `/api/topics` | Create a topic (`name`, `numPartitions`, `replicationFactor`) |
| `GET` | `/api/topics/{topicName}` | Topic detail and configs |
| `GET` | `/api/topics/{topicName}/messages?limit=10` | Read messages |
| `POST` | `/api/topics/{topicName}/messages` | Publish a message (`key`, `payload`) |

Example: publish a message

```bash
curl -X POST http://localhost:8000/api/topics/orders/messages \
  -H "Content-Type: application/json" \
  -d '{"key": "order-42", "payload": "{\"item\":\"espresso\",\"qty\":1}"}'
```

## Health check

```bash
curl http://localhost:8000/api/health
# healthy
```

## License

MIT
