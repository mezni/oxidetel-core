# Tasks: PostgreSQL Foundation (Phase 3)

**Input**: Design documents from `/specs/003-postgres-foundation/`
**Prerequisites**: plan.md, spec.md, research.md

## Phase 1: Setup
- [ ] T001 Create migration directories per schema under `migrations/` (auth, crm, billing, inventory, trouble_ticket, catalog)
- [ ] T002 Document migration strategy in `docs/` or plan notes

## Phase 2: Foundational (Blocking)
- [ ] T003 Create initial migration files to create schemas
- [ ] T004 Define SQLx migration setup/config
- [ ] T005 Define database roles and least privilege grants (document or scripts)

## Phase 3: User Story 1 - Schemas exist (P1)
- [ ] T006 [US1] Create migration for schema creation
- [ ] T007 [US1] Test schema creation

## Phase 4: User Story 2 - Migrations are versioned (P1)
- [ ] T008 [US2] Organize migrations by schema
- [ ] T009 [US2] Document controlled migration execution

## Phase 5: User Story 3 - Least privilege enforced (P1)
- [ ] T010 [US3] Create role/grant scripts or docs
- [ ] T011 [US3] Separate migration/runtime credentials
- [ ] T012 [US3] Add integration test setup

## Phase 6: Polish
- [ ] T013 Document backup/recovery
- [ ] T014 Validate migrations against test DB (if possible)
