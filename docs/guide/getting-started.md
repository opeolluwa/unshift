# Getting started

**Unshift** is a browser-based console for managing and inspecting your Kafka
cluster, like [Kafdrop](https://github.com/obsidiandynamics/kafdrop), with the
added ability to **publish messages** directly from the UI.

The whole app ships as a single image — `opeolluwa/unshift` — that serves the
Rust API and the built frontend together on port **8000**.

## Point it at an existing broker

```bash
docker run -d \
  --name unshift \
  -p 8000:8000 \
  -e KAFKA_BROKER=host.docker.internal:9092 \
  opeolluwa/unshift
```

Then open `http://localhost:8000`.

> On Linux, `host.docker.internal` may require
> `--add-host=host.docker.internal:host-gateway`. Otherwise, use your broker's
> reachable address (e.g. `-e KAFKA_BROKER=192.168.1.10:9092`).

## Compose, with a broker

The quickest way to try everything, including a throwaway broker:

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

## Add it to an existing stack

Add `unshift` as a service in a stack you already run:

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

- If your broker service isn't named `kafka`, point `KAFKA_BROKER` at the right
  name and port (e.g. `broker:29092` for Confluent's `cp-kafka` internal
  listener).
- The snippet places `unshift` on an explicit `internal` network; the broker
  must be on that same network.
- No extra proxy is required — the UI and API share port `8000`.

## Health check

```bash
curl http://localhost:8000/api/health
# healthy
```

Next: [Configuration](/guide/configuration) for every environment variable, or
the [API reference](/api/) to talk to the cluster from scripts.
