# Feature Specification: Rust Workspace and Tooling (Phase 2)

**Feature Branch**: `002-rust-workspace-and-tooling`  
**Created**: 2026-10-23  
**Status**: Draft  
**Input**: User description: "Phase 2: Rust workspace and tooling - create Cargo workspace with six services and shared crates, configure fmt/clippy/tests"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Developer can build the workspace (Priority: P1)

As a developer, I need the Rust workspace to compile so I can start implementing services.

**Why this priority**: Foundation for all subsequent development.

**Independent Test**: Run `cargo build --workspace` successfully.

**Acceptance Scenarios**:
1. **Given** a fresh checkout, **When** running `cargo build --workspace`, **Then** all crates compile without errors.

---

### User Story 2 - Developer can run quality checks (Priority: P1)

As a developer, I need formatting/linting/tests to pass to meet the Definition of Done.

**Why this priority**: Enforces code quality from the start.

**Independent Test**: Run the required checks.

**Acceptance Scenarios**:
1. **Given** the workspace, **When** running `cargo fmt --all -- --check`, **Then** no formatting issues.
2. **Given** the workspace, **When** running `cargo clippy --workspace --all-targets --all-features -- -D warnings`, **Then** no warnings.
3. **Given** the workspace, **When** running `cargo test --workspace`, **Then** all tests pass.

---

### User Story 3 - Six services and shared crates exist (Priority: P1)

As a developer, I need the six service packages and shared infrastructure crates to exist per architecture.

**Why this priority**: Matches the constitution/architecture requirements.

**Independent Test**: Verify directory structure and Cargo.toml members.

**Acceptance Scenarios**:
1. **Given** the workspace, **When** inspecting Cargo.toml, **Then** it includes all six services and shared crates.
2. **Given** the workspace, **Then** `apps/auth-service`, `apps/crm-service`, `apps/billing-service`, `apps/inventory-service`, `apps/ticketing-service`, `apps/catalog-service` exist.
3. **Given** the workspace, **Then** `crates/api-common`, `crates/config`, `crates/observability` exist.

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: Workspace MUST use Rust stable with `rust-toolchain.toml`.
- **FR-002**: Workspace MUST define all six service packages as members.
- **FR-003**: Workspace MUST define shared infrastructure crates as members.
- **FR-004**: Dependencies MUST be managed in workspace dependencies where appropriate.
- **FR-005**: `Cargo.lock` MUST be committed.
- **FR-006**: Quality tooling MUST be configured (formatting/linting).
- **FR-007**: Each service crate MUST compile as a binary/library as appropriate.
- **FR-008**: Shared crates MUST compile.

### Key Entities
- **Workspace**: Root Cargo.toml defining members and workspace dependencies.
- **Service Crate**: One of the six services under `apps/`.
- **Shared Crate**: Infrastructure crate under `crates/`.

## Success Criteria *(mandatory)*

### Measurable Outcomes
- **SC-001**: `cargo build --workspace` completes successfully.
- **SC-002**: `cargo fmt --all -- --check` passes with no changes needed.
- **SC-003**: `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes with no warnings.
- **SC-004**: `cargo test --workspace` passes.
- **SC-005**: All six services and three shared crates are present and compile.

## Assumptions
- Using Rust stable toolchain
- Following constitution/architecture for service layout
