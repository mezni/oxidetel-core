# Tasks: CRM (Phase 6)

**Input**: Design documents from `/specs/007-crm/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, quickstart.md

## Phase 1: Setup (Shared Infrastructure)

- [ ] T001 Create CRM migrations directory structure per plan
- [ ] T002 Configure SQLx migration setup for crm schema

## Phase 2: Foundational (Blocking Prerequisites)

- [ ] T003 Create crm schema migration
- [ ] T004 Create base tables (customers, accounts)
- [ ] T005 Setup RBAC middleware integration

**Checkpoint**: Foundation ready

## Phase 3: User Story 1 - Customers (Priority: P1) 🎯 MVP

**Goal**: Manage customers

**Independent Test**: CRUD operations on customers

### Implementation for User Story 1

- [ ] T006 [P] [US1] Create customer model in apps/crm-service/src/
- [ ] T007 [US1] Implement customer service in apps/crm-service/src/
- [ ] T008 [US1] Add customer API endpoints in apps/crm-service/src/
- [ ] T009 [US1] Add validation for customers

**Checkpoint**: US1 complete

## Phase 4: User Story 2 - Accounts (Priority: P1)

**Goal**: Manage accounts

**Independent Test**: CRUD operations on accounts

### Implementation for User Story 2

- [ ] T010 [P] [US2] Create account model in apps/crm-service/src/
- [ ] T011 [US2] Implement account service in apps/crm-service/src/
- [ ] T012 [US2] Add account API endpoints in apps/crm-service/src/
- [ ] T013 [US2] Add validation for accounts

**Checkpoint**: US2 complete

## Phase 5: User Story 3 - Relationships (Priority: P1)

**Goal**: Track relationships

**Independent Test**: Manage customer-account relationships

### Implementation for User Story 3

- [ ] T014 [P] [US3] Create relationship model in apps/crm-service/src/
- [ ] T015 [US3] Implement relationship service in apps/crm-service/src/
- [ ] T016 [US3] Add relationship API endpoints in apps/crm-service/src/

**Checkpoint**: All user stories functional

## Phase 6: Polish & Cross-Cutting Concerns

- [ ] T017 [P] Update OpenAPI spec for crm service
- [ ] T018 Add integration tests
- [ ] T019 Run quickstart validation
- [ ] T020 Document CRM APIs

## Dependencies & Execution Order

### Phase Dependencies
- Setup (Phase 1): No dependencies
- Foundational (Phase 2): Depends on Setup - BLOCKS all stories
- User Stories: All depend on Foundational
- Polish: Depends on all stories complete

### Implementation Strategy
**MVP First**: Complete Phases 1-3 (US1), validate, then proceed.
