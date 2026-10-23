# Tasks: Backend Container and Service Startup (Phase 4)

**Input**: Design documents from `/specs/004-backend-container/`
**Prerequisites**: plan.md, spec.md, research.md

## Phase 1: Setup
- [ ] T001 Create `Dockerfile` (multi-stage) for Rust
- [ ] T002 Create `docker-compose.yml` with telecom-db and telecom-backend

## Phase 2: Foundational
- [ ] T003 Create process supervisor entrypoint
- [ ] T004 Configure networking and volumes
- [ ] T005 Add `.env.example`

## Phase 3: User Story 1 - Two containers (P1)
- [ ] T006 [US1] Define exactly two services in docker-compose
- [ ] T007 [US1] Configure Postgres service with volume

## Phase 4: User Story 2 - Six services running (P1)
- [ ] T008 [US2] Configure backend to run all six services on 8001-8006
- [ ] T009 [US2] Add health endpoints to services

## Phase 5: User Story 3 - Health and graceful shutdown (P1)
- [ ] T010 [US3] Add /health/live and /health/ready
- [ ] T011 [US3] Implement graceful shutdown
- [ ] T012 [US3] Add controlled migration step

## Phase 6: Polish
- [ ] T013 Test startup/shutdown
- [ ] T014 Document deployment
