# Tasks: Rust Workspace and Tooling (Phase 2)

**Input**: Design documents from `/specs/002-rust-workspace-and-tooling/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Create workspace root files: `Cargo.toml`, `rust-toolchain.toml`, `.cargo/config.toml` in `/home/dali/WORK/telco-si/`
- [ ] T002 Create directory structure: `apps/` with six service dirs and `crates/` with three shared crate dirs in `/home/dali/WORK/telco-si/`
- [ ] T003 [P] Initialize gitignore entries for build artifacts if needed in `/home/dali/WORK/telco-si/.gitignore`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core workspace configuration and shared infrastructure crates

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T004 [P] Create `crates/api-common/Cargo.toml` and minimal library in `/home/dali/WORK/telco-si/crates/api-common/`
- [ ] T005 [P] Create `crates/config/Cargo.toml` and minimal library in `/home/dali/WORK/telco-si/crates/config/`
- [ ] T006 [P] Create `crates/observability/Cargo.toml` and minimal library in `/home/dali/WORK/telco-si/crates/observability/`
- [ ] T007 Configure workspace dependencies in root `Cargo.toml` (tokio, axum, sqlx, serde, thiserror, anyhow, tracing, tracing-subscriber, validator, utoipa, argon2, jsonwebtoken, rust_decimal, uuid)
- [ ] T008 Set up workspace members in root `Cargo.toml` for all apps/* and crates/*

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Developer can build the workspace (Priority: P1) 🎯 MVP

**Goal**: Workspace compiles successfully

**Independent Test**: `cargo build --workspace`

### Implementation for User Story 1

- [ ] T009 [P] [US1] Create `apps/auth-service/Cargo.toml` and minimal binary in `/home/dali/WORK/telco-si/apps/auth-service/`
- [ ] T010 [P] [US1] Create `apps/crm-service/Cargo.toml` and minimal binary in `/home/dali/WORK/telco-si/apps/crm-service/`
- [ ] T011 [P] [US1] Create `apps/billing-service/Cargo.toml` and minimal binary in `/home/dali/WORK/telco-si/apps/billing-service/`
- [ ] T012 [P] [US1] Create `apps/inventory-service/Cargo.toml` and minimal binary in `/home/dali/WORK/telco-si/apps/inventory-service/`
- [ ] T013 [P] [US1] Create `apps/ticketing-service/Cargo.toml` and minimal binary in `/home/dali/WORK/telco-si/apps/ticketing-service/`
- [ ] T014 [P] [US1] Create `apps/catalog-service/Cargo.toml` and minimal binary in `/home/dali/WORK/telco-si/apps/catalog-service/`
- [ ] T015 [US1] Verify workspace builds with `cargo build --workspace`

**Checkpoint**: Workspace compiles

---

## Phase 4: User Story 2 - Developer can run quality checks (Priority: P1)

**Goal**: Formatting, linting, and tests pass

**Independent Test**: Run fmt/clippy/test as specified

### Implementation for User Story 2

- [ ] T016 [US2] Configure rustfmt/clippy settings if needed in `/home/dali/WORK/telco-si/.cargo/config.toml`
- [ ] T017 [US2] Add minimal unit test to at least one crate to validate test infrastructure
- [ ] T018 [US2] Run `cargo fmt --all -- --check` and fix formatting
- [ ] T019 [US2] Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` and fix issues
- [ ] T020 [US2] Run `cargo test --workspace` and ensure all pass

**Checkpoint**: All quality gates pass

---

## Phase 5: User Story 3 - Six services and shared crates exist (Priority: P1)

**Goal**: Structure matches architecture

**Independent Test**: Verify structure and membership

### Implementation for User Story 3

- [ ] T021 [US3] Verify all six services present under `apps/`
- [ ] T022 [US3] Verify all three shared crates present under `crates/`
- [ ] T023 [US3] Verify workspace members match architecture
- [ ] T024 [US3] Commit `Cargo.lock` if generated

**Checkpoint**: Structure validated

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Finalize and document

- [ ] T025 [P] Update quickstart/README as needed
- [ ] T026 Run full validation per quickstart.md
- [ ] T027 Commit workspace changes with appropriate message

---

## Dependencies & Execution Order

### Phase Dependencies
- Setup (Phase 1): No dependencies
- Foundational (Phase 2): Depends on Setup - BLOCKS all user stories
- User Stories (Phase 3+): Depend on Foundational
- Polish (Phase 6): Depends on all user stories

### Parallel Opportunities
- T004, T005, T006 can run in parallel
- T009-T014 can run in parallel (all six services)
- Quality checks are sequential

### MVP Scope
MVP is User Story 1 (workspace compiles). All P1 stories are needed for Phase 2 completion per build plan.
