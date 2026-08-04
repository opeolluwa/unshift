


cfg:
    curl -fsSL https://get.pnpm.io/install.sh | sh - 
    source /Users/$USER/.zshrc

[working-directory: "console"]
run-console: 
    pnpm run dev 

[script]
release type:
    ./scripts/release.sh {{type}}

[working-directory: "kernel"]
run-kernel:
    cargo watch -qcx run 