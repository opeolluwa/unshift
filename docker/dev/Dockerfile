FROM rust:1.94.0-slim-bullseye

ENV CARGO_TARGET_DIR=/cargo-target

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        libgcc-s1 \
        libssl-dev \
        pkg-config \
        clang \
        libclang-dev \
        zlib1g-dev \
        llvm-dev \
        build-essential; \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m -s /bin/bash appuser

RUN cargo install cargo-watch

WORKDIR /app

COPY kernel/ ./kernel/

WORKDIR /app/kernel

RUN cargo build

RUN chown -R appuser:appuser /app

USER appuser

VOLUME ["/cargo-target", "/usr/local/cargo/registry"]

CMD ["cargo", "watch", \
     "--why", \
     "--clear", \
     "--quiet", \
     "--ignore", ".git/", \
     "--ignore", "target/", \
     "-w", "src", \
     "-w", "Cargo.toml", \
     "-x", "check", \
     "-x", "run"]
