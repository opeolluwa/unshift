

cfg:
    #!/usr/bin/env sh
    set -e
    if ! command -v pnpm >/dev/null 2>&1; then
        echo "Installing pnpm..."
        curl -fsSL https://get.pnpm.io/install.sh | sh -
        source /Users/$USER/.zshrc
    else
        echo "pnpm is already installed"
    fi
    if ! command -v cargo-watch >/dev/null 2>&1; then
        echo "Installing cargo-watch..."
        cargo install cargo-watch
    else
        echo "cargo-watch is already installed"
    fi
    if [ ! -f .env ]; then
        cp .env.example .env
        echo "Created .env from .env.example"
    else
        echo ".env already exists, skipping"
    fi

[working-directory: "console"]
run-console: 
    pnpm run dev 

[working-directory: "kernel"]
run-kernel:
    docker compose up -d 
    docker compose logs -f --tail=30 app 

[script]
dev:
    pnpm exec concurrently --names console,kernel --prefix-colors blue,red "just run-console" "just run-kernel"

[script]
release type:
    ./scripts/release.sh {{type}}


[working-directory: "scripts"]
bootstrap-topics:
    bash ./topics.sh
