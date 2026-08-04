

cfg:
    curl -fsSL https://get.pnpm.io/install.sh | sh - 
    source /Users/$USER/.zshrc
    cp .env.example .env

[working-directory: "console"]
run-console: 
    pnpm run dev 

[working-directory: "kernel"]
run-kernel:
    cargo watch -qcx run 

[script]
dev:
    pnpm exec concurrently --names console,kernel --prefix-colors blue,red "just run-console" "just run-kernel"

[script]
release type:
    ./scripts/release.sh {{type}}

[script]
bootstrap-topics:
    ./scripts/topics.sh
