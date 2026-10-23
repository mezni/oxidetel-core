# Implementation Plan: Rust Workspace and Tooling (Phase 2)

**Branch**: `002-rust-workspace-and-tooling` | **Date**: 2026-10-23 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/002-rust-workspace-and-tooling/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command; its definition describes the execution workflow.

## Summary

Establish a compilable Rust workspace for the Telecom Core Backend with six service crates (`apps/auth-service`, `apps/crm-service`, `apps/billing-service`, `apps/inventory-service`, `apps/ticketing-service`, `apps/catalog-service`) and shared infrastructure crates (`crates/api-common`, `crates/config`, `crates/observability`). Configure Rust stable toolchain, workspace dependencies, formatting, linting, and testing. Ensure `cargo build`, `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` all pass.

## Technical Context

**Language/Version**: Rust stable (pinned via `rust-toolchain.toml`)  
**Primary Dependencies**: Tokio, Axum, SQLx, Serde, thiserror, anyhow, tracing, tracing-subscriber, validator, utoipa, argon2, jsonwebtoken (or approved JWT crate), rust_decimal, uuid  
**Storage**: PostgreSQL (accessed via SQLx) — workspace setup only, no schema implementation in this phase  
**Testing**: `cargo test --workspace` (unit/integration test infrastructure ready)  
**Target Platform**: Linux server (Docker-based deployment)  
**Project Type**: Multi-crate Rust workspace (backend services)  
**Performance Goals**: N/A for workspace setup  
**Constraints**: Must follow constitution (six services, shared crates not owning domain logic), two-container deployment model, no AI/UI  
**Scale/Scope**: 6 services + 3 shared crates + workspace tooling

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **Article II (Tech stack)**: Using Rust stable, Axum, Tokio, SQLx, Serde, validator, thiserror/anyhow, tracing, utoipa, argon2, JWT library, rust_decimal/decimal, uuid as specified.
- [x] **Article III (Service architecture)**: Creating six service crates with independent structure; shared crates limited to infrastructure.
- [x] **Article XI (Testing & quality)**: Will enforce `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`.
- [x] **Article IV/V general constraints**: Workspace structure supports two-container deployment and schema ownership model.
- [x] **No violations** — gates pass.

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
telecom-core/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── .cargo/config.toml
├── apps/
│   ├── auth-service/
│   ├── crm-service/
│   ├── billing-service/
│   ├── inventory-service/
│   ├── ticketing-service/
│   └── catalog-service/
├── crates/
│   ├── api-common/
│   ├── config/
│   └── observability/
├── migrations/        # (empty structure now; populated in Phase 3)
├── specs/
├── docs/
└── tests/
```

**Structure Decision**: Use Cargo workspace with `apps/*` for the six services and `crates/*` for shared infrastructure per ADR-0008 and architecture.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
