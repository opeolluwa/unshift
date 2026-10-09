# REST reference

The UI is backed by a small REST API mounted under `/api`. The base URL is:

```
http://localhost:8000/api
```

All request and response bodies are JSON. Field names are `camelCase`. Kafka
failures return `502`, storage failures `500`, and every error body is
`{ "message": "..." }`.

## Endpoints

| Method   | Path                                        | Description                                                    |
| -------- | ------------------------------------------- | -------------------------------------------------------------- |
| `GET`    | `/api/health`                               | Health check (plain-text `healthy`).                           |
| `GET`    | `/api/cluster/overview`                     | Cluster statistics.                                            |
| `GET`    | `/api/topics`                               | List topics.                                                   |
| `POST`   | `/api/topics`                               | Create a topic.                                                |
| `GET`    | `/api/topics/{topicName}`                   | Topic detail, health and configs.                              |
| `GET`    | `/api/topics/{topicName}/messages?limit=10` | Read messages.                                                 |
| `POST`   | `/api/topics/{topicName}/messages`          | Publish a message.                                             |
| `GET`    | `/api/saved-messages`                       | List saved message templates.                                  |
| `POST`   | `/api/saved-messages`                       | Save a message template.                                       |
| `DELETE` | `/api/saved-messages/{id}`                  | Delete a saved message (responds `204`).                       |

---

## Health

```
GET /api/health
```

Returns the plain text `healthy`.

```bash
curl http://localhost:8000/api/health
```

## Cluster overview

```
GET /api/cluster/overview
```

```json
{
  "bootstrapServers": "localhost:9092",
  "totalTopics": 12,
  "totalPartitions": 34,
  "preferredPartitionLeaderPercentage": 100.0,
  "totalUnderReplicatedPartitions": 0
}
```

## List topics

```
GET /api/topics
```

```json
{
  "topics": [
    {
      "topic": "orders",
      "partitions": 3,
      "preferredLeaderPercent": 100,
      "underReplicated": 0,
      "customConfigs": 1,
      "configs": [
        { "name": "cleanup.policy", "value": "compact", "readOnly": false }
      ]
    }
  ]
}
```

## Create a topic

```
POST /api/topics
Content-Type: application/json
```

| Field               | Type     | Description             |
| ------------------- | -------- | ----------------------- |
| `name`              | `string` | Topic name.             |
| `numPartitions`     | `int`    | Number of partitions.   |
| `replicationFactor` | `int`    | Replication factor.     |

```bash
curl -X POST http://localhost:8000/api/topics \
  -H "Content-Type: application/json" \
  -d '{"name": "orders", "numPartitions": 3, "replicationFactor": 1}'
```

```json
{ "name": "orders", "status": "created" }
```

## Topic detail

```
GET /api/topics/{topicName}
```

Returns a single `TopicSummary` (the same shape as an item of the topics list).

## Read messages

```
GET /api/topics/{topicName}/messages?limit=10
```

Reads from the earliest offset across all partitions. `limit` defaults to `10`
and is capped at `100`.

```json
{
  "topic": "orders",
  "messages": [
    { "key": "order-42", "payload": "{\"item\":\"espresso\"}", "partition": 0, "offset": 7 }
  ]
}
```

`key` is `null` for keyless messages.

## Publish a message

```
POST /api/topics/{topicName}/messages
Content-Type: application/json
```

| Field     | Type     | Description                      |
| --------- | -------- | -------------------------------- |
| `key`     | `string` | Message key; empty for no key.   |
| `payload` | `string` | Message payload (opaque string). |

```bash
curl -X POST http://localhost:8000/api/topics/orders/messages \
  -H "Content-Type: application/json" \
  -d '{"key": "order-42", "payload": "{\"item\":\"espresso\",\"qty\":1}"}'
```

```json
{ "topic": "orders", "status": "published" }
```

## Saved messages

Reusable publish templates, stored server-side in SQLite.

### List

```
GET /api/saved-messages
```

```json
{
  "messages": [
    {
      "id": 1,
      "label": null,
      "topic": "orders",
      "key": "order-42",
      "payload": "{\"item\":\"espresso\"}",
      "createdAt": "2026-01-01 12:00:00"
    }
  ]
}
```

### Save

```
POST /api/saved-messages
Content-Type: application/json
```

| Field     | Type             | Description                       |
| --------- | ---------------- | --------------------------------- |
| `label`   | `string \| null` | Optional label.                   |
| `topic`   | `string`         | Topic the template targets.       |
| `key`     | `string`         | Message key.                      |
| `payload` | `string`         | Message payload.                  |

```bash
curl -X POST http://localhost:8000/api/saved-messages \
  -H "Content-Type: application/json" \
  -d '{"label": "espresso order", "topic": "orders", "key": "order-42", "payload": "{\"item\":\"espresso\"}"}'
```

Returns the created `SavedMessage` (with its generated `id` and `createdAt`).

### Delete

```
DELETE /api/saved-messages/{id}
```

Responds `204 No Content` on success.
