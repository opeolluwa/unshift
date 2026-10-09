# Stage 1: Frontend build
FROM node:26-slim AS frontend
RUN npm install -g pnpm@11.18.0
WORKDIR /app
COPY console/package*.json console/pnpm-lock.yaml console/pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY console/ ./
    RUN pnpm run build

# Stage 2: Rust build
FROM rust:1.94-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    cmake \
    perl \
    git \
    libssl-dev \
    zlib1g-dev \
    libcurl4-openssl-dev \
    libclang-dev \
    llvm-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY kernel/ ./kernel/
WORKDIR /app/kernel
RUN cargo build --release \
    && strip target/release/unshift

# Stage 3: Final image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --create-home --home-dir /app --shell /sbin/nologin app

WORKDIR /app

COPY --from=builder /app/kernel/target/release/unshift ./unshift
COPY --from=frontend /app/.output/public ./assets

RUN mkdir -p /app/data && chown -R app:app /app

USER app

VOLUME ["/app/data"]

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -qO /dev/null http://localhost:8000/health || exit 1

CMD ["./unshift"]
