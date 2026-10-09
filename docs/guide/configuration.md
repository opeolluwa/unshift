# Configuration

Unshift is configured entirely through environment variables. At startup the
kernel loads a `.env` file from the working directory (via `dotenvy`) and then
reads the process environment.

## Environment variables

| Variable                 | Required | Default                                       | Description                                                        |
| ------------------------ | -------- | --------------------------------------------- | ------------------------------------------------------------------ |
| `KAFKA_BROKER`           | Yes      | —                                             | Comma-separated Kafka bootstrap servers, e.g. `kafka:29092`.        |
| `PORT`                   | No       | `8000`                                        | Port the API and built UI listen on.                                |
| `ENVIRONMENT`            | No       | `development`                                 | `development`/`dev` or `production`/`prod`; controls log verbosity. |
| `ALLOWED_ORIGINS`        | No       | `http://localhost:3000,http://localhost:5173` | Comma-separated CORS allowed origins.                              |
| `REQUESTS_TIME_OUT_SECS` | No       | `10`                                          | Request timeout in seconds; timed-out requests return `408`.        |
| `TOPICS_FILE`            | No       | `./topics.yaml`                               | YAML file of topics to seed at startup.                             |
| `DATA_DIR`               | No       | `./data`                                      | Directory for the SQLite database backing saved messages.           |

`ENVIRONMENT` also picks the log level: `DEBUG` in development, `INFO` in
production. `KAFKA_BROKER` is the only variable without a fallback — the kernel
fails to start if it is unset.

## Topic seeding

On boot, the kernel reads `TOPICS_FILE` (if it exists) and creates any missing
topics. The file is a YAML document with a `topics` list:

```yaml
topics:
  - name: orders
    partitions: 3
    replication_factor: 1
  - name: payments
    partitions: 2
    replication_factor: 1
```

`partitions` and `replication_factor` both default to `1` when omitted. Existing
topics are left untouched, and a missing or malformed file is logged and
skipped rather than treated as fatal.

For the bundled local compose stack, `just bootstrap-topics` runs
`scripts/topics.sh` against `scripts/topics.yaml`, which lists only the missing
topics and creates them with `kafka-topics`.

## Data directory

The saved-messages feature stores reusable message templates in a local SQLite
database at `DATA_DIR/unshift.db`. The table is created on first boot. In the
production image `DATA_DIR` is `/app/data`, exposed as a volume so saved
messages survive container restarts.

## Running in Docker

`TOPICS_FILE` and `DATA_DIR` are resolved against the process working
directory — `/app` in the production image. Relative values therefore behave
differently inside a container than on the host:

- Use absolute paths in `docker run`/compose (`DATA_DIR: /app/data`).
- Bind-mount your topics file into the container and point `TOPICS_FILE` at
  the mounted path, e.g. `- ./kafka-topics.yaml:/app/kafka-topics.yaml:ro`
  with `TOPICS_FILE: /app/kafka-topics.yaml`. A missing file is skipped, so
  an unmounted path silently disables seeding.
- Keep `DATA_DIR` on the image's `/app/data` volume so saved messages persist
  across container recreation.

`DATA_DIR` must not collide with an existing file in the workdir (for
example, the server binary itself is `/app/unshift`). If it does, startup
fails with `failed to create data dir: File exists (os error 17)`.

## CORS

Only origins listed in `ALLOWED_ORIGINS` receive CORS headers. Add your console
origin (e.g. `http://localhost:3000` during local development, or your deployed
domain) when the browser and the API are served from different origins.

Next: [Architecture](/guide/architecture) for how the pieces fit together.
