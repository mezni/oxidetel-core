# Quickstart: Rust Workspace and Tooling (Phase 2)

## Prerequisites
- Rust toolchain (stable) installed or managed via `rust-toolchain.toml`
- Cargo available

## Setup
```bash
# From repo root
cargo --version
rustc --version
```

## Validation
```bash
# Build all
cargo build --workspace

# Format check
cargo fmt --all -- --check

# Lint
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Test
cargo test --workspace
```

## Expected
All commands pass with no errors/warnings.
