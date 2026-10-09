# Architecture

Unshift is a pnpm/Rust workspace made of two deployable halves that share one
image in production.

```
unshift/
├── kernel/        Rust API (axum + rdkafka) and the compiled binary
│   └── src/
│       ├── bin/unshift.rs   server bootstrap, routing, static file serving
│       ├── router.rs        /api route table
│       ├── handlers.rs      request handlers
│       ├── adapters.rs      request/response DTOs
│       ├── kafka/           clients, topic seeding, read/publish helpers
│       ├── db.rs            SQLite store for saved messages
│       └── config/          env, CORS, logger, timeouts
├── console/       Nuxt UI frontend (Pinia store + axios plugin)
├── docker/        dev + prod Dockerfiles
├── scripts/       release.sh, topics.sh/topics.yaml, curl helpers
├── docs/          this VitePress site
└── Justfile       cfg, dev, build, release, bootstrap-topics
```

## Kernel

The Rust API is built on [axum] and talks to Kafka through [rdkafka]. On boot,
`bin/unshift.rs`:

1. Loads configuration and initializes logging.
2. Builds a `KafkaState` holding a **producer**, a shared **metadata client**
   (`BaseConsumer`) and a shared **admin client**.
3. Seeds topics from `TOPICS_FILE`.
4. Opens the SQLite database in `DATA_DIR`.
5. Mounts the API under `/api`, wraps it with a request-timeout layer and CORS,
   and serves the built frontend as the fallback service.

### Kafka clients

| Purpose          | Client                   | Notes                                             |
| ---------------- | ------------------------ | ------------------------------------------------- |
| Produce messages | `FutureProducer`         | 5s message timeout, 100k buffered messages.       |
| List/describe    | `BaseConsumer`           | Used for metadata queries only.                   |
| Create topics    | `AdminClient`            | Topic creation and seeding.                       |
| Read messages    | on-demand `BaseConsumer` | Group `unshift-console`, `auto.offset.reset=earliest`. |

Reading messages uses a short-lived consumer that reads from the earliest offset
across all partitions, bounded by the request's `limit`.

### Storage

Saved messages (reusable publish templates) live in a small SQLite database at
`DATA_DIR/unshift.db`, created on first boot. This is the only stateful part of
the kernel; everything else is derived from the live cluster.

### Errors

Handlers return `AppError`, which maps to HTTP responses:

| Error                       | Status |
| --------------------------- | ------ |
| Kafka / Kafka admin failure | `502`  |
| Database failure            | `500`  |
| Config / startup / other    | `500`  |

Error bodies are JSON: `{ "message": "..." }`.

## Console

`console/` is a Nuxt 4 app using [Nuxt UI], [Pinia] and axios. A single axios
instance (`app/plugins/api.ts`) points at `http://localhost:8000/api` and
normalizes errors into plain `Error`s. A Pinia store (`app/stores/kafka.ts`)
wraps every endpoint, and the pages render the cluster overview, topic detail,
message browser, publish form and saved messages.

Types under `console/app/bindings/` mirror the kernel's `adapters.rs` DTOs.

## Single image

`docker/prod/Dockerfile` is a three-stage build:

1. **frontend** — `pnpm install` + `pnpm run build` in `console/`, producing
   `.output/public`.
2. **builder** — `cargo build --release` in `kernel/`.
3. **final** — a slim Debian image with the `unshift` binary and the static
   frontend copied into `assets/`. The binary serves `assets/` via `ServeDir`,
   falling back to `assets/index.html` so client-side routes resolve.

Because the API (`/api/*`) and the static app share a port, no reverse proxy is
needed in production. `DATA_DIR` is a volume so saved messages persist.

[axum]: https://github.com/tokio-rs/axum
[rdkafka]: https://github.com/fede1024/rust-rdkafka
[Nuxt UI]: https://ui.nuxt.com
[Pinia]: https://pinia.vuejs.org
