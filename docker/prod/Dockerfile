# Stage 1: Frontend build
FROM node:26-slim AS frontend
RUN npm install -g pnpm@11.18.0
WORKDIR /app
COPY console/package*.json console/pnpm-lock.yaml console/pnpm-workspace.yaml ./
RUN pnpm install --frozen-lockfile
COPY console/ ./
    RUN pnpm run build

# Stage 2: Rust build
FROM rust:1.94-alpine AS builder

RUN apk add --no-cache \
    build-base \
    pkgconf \
    cmake \
    perl \
    git \
    openssl-dev \
    openssl-libs-static \
    zlib-dev \
    zlib-static \
    curl-dev \
    clang-dev \
    llvm-dev

WORKDIR /app
COPY kernel/ ./kernel/
WORKDIR /app/kernel
RUN cargo build --release \
    && strip target/release/unshift

# Stage 3: Final image
FROM alpine:3.21

RUN addgroup -S app && adduser -S -G app -h /app -s /sbin/nologin app \
    && apk add --no-cache ca-certificates curl

WORKDIR /app

COPY --from=builder /app/kernel/target/release/unshift ./unshift
COPY --from=frontend /app/.output/public ./assets

RUN chown -R app:app /app

USER app

EXPOSE 8000

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

CMD ["./unshift"]
