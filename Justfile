# Game Slop Machine
# Usage: just [recipe]
# Run `just` to see all available recipes.

# Cargo commands

build:
    cargo build --release

lint:
    cargo clippy --all-targets --all-features -- -D warnings

fmt:
    cargo fmt --all

clean:
    cargo clean

# Run commands

run-inspector:
    cargo run --release --bin game-inspector

