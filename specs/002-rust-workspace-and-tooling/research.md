# Research: Rust Workspace and Tooling (Phase 2)

**Feature**: 002-rust-workspace-and-tooling  
**Date**: 2026-10-23

## Decisions

### Decision: Rust toolchain
- **Choice**: Pin to `rust-toolchain.toml` with stable channel
- **Rationale**: Reproducible builds, matches constitution (Rust stable)
- **Alternatives**: Manual installation - rejected for reproducibility

### Decision: Workspace layout
- **Choice**: `apps/` for six service binaries, `crates/` for shared infrastructure
- **Rationale**: Matches ADR-0008, enforces service boundaries
- **Alternatives**: All under `src/` - rejected (doesn't scale to 6 services)

### Decision: Dependency management
- **Choice**: Use `[workspace.dependencies]` in root `Cargo.toml`, commit `Cargo.lock`
- **Rationale**: Consistent versions across workspace
- **Alternatives**: Per-crate deps - rejected for consistency

### Decision: Quality gates
- **Choice**: Enforce `fmt --check`, `clippy --all-targets --all-features -- -D warnings`, `cargo test`
- **Rationale**: Matches constitution Article XI
- **Alternatives**: Relaxed gates - rejected

### Decision: Shared crates scope
- **Choice**: `api-common` (errors/types), `config` (env/config), `observability` (tracing/metrics setup)
- **Rationale**: Infrastructure only, no domain logic
- **Alternatives**: Shared domain crate - rejected per constitution Article III

## Resolved Clarifications
All items from Technical Context are resolved (no NEEDS CLARIFICATION).
