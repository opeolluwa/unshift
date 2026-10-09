# Development

## Prerequisites

- [Just](https://just.systems) — command runner
- [Node.js](https://nodejs.org) and [pnpm](https://pnpm.io)
- [Rust](https://rust-lang.org) toolchain
- [Docker](https://www.docker.com) + Compose
- [cmake](https://cmake.org) — required to build `rdkafka`

## Bootstrap

```bash
git clone https://github.com/opeolluwa/unshift.git
cd unshift
just cfg
```

`just cfg` installs `pnpm` and `cargo-watch` if missing, creates `.env` from
`.env.example`, and installs root + `console/` dependencies.

## Run everything

```bash
just dev
```

`just dev` uses `concurrently` to start both halves of the workspace:

| Process   | Command           | What it does                                                 |
| --------- | ----------------- | ------------------------------------------------------------ |
| `console` | `just run-console`| Nuxt dev server on port `3000`                               |
| `kernel`  | `just run-kernel` | `docker compose up -d`, then tails the `app` container logs  |

The `kernel` service runs `cargo watch -x run` inside the dev container, so Rust
edits rebuild and restart automatically. The Nuxt dev server proxies API calls
to `http://localhost:8000/api` (see `console/app/plugins/api.ts`).

You can also run either side on its own:

```bash
just run-console   # Nuxt only
just run-kernel    # API + Kafka + Kafdrop + Zookeeper via compose
```

The dev compose stack also brings up `kafka`, `zookeeper` and a Kafdrop
instance on port `9000`.

## Topics

Seed the local broker with the topics declared in `scripts/topics.yaml`:

```bash
just bootstrap-topics
```

The script waits for the `kafka` container, lists existing topics and creates
only the missing ones.

## Build the image

```bash
just build
```

This builds `opeolluwa/unshift:<version>` and `:latest` from
`docker/prod/Dockerfile`, passing `PORT`, `ENVIRONMENT`, `ALLOWED_ORIGINS` and
`REQUESTS_TIME_OUT_SECS` as build args. The version is read from
`kernel/Cargo.toml`.

## Docs

```bash
pnpm docs:dev       # VitePress dev server
pnpm docs:build     # static build into docs/.vitepress/dist
pnpm docs:preview   # preview the built site
```

## Release

```bash
just release patch   # or minor | major
```

Delegates to `scripts/release.sh`, which bumps `kernel/Cargo.toml` and pushes
the matching git tag. The `docker-publish` workflow builds and pushes the image
on `v*` tags.
