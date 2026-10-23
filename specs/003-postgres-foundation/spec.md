# Feature Specification: PostgreSQL Foundation (Phase 3)

**Feature Branch**: `003-postgres-foundation`  
**Created**: 2026-10-23  
**Status**: Draft  
**Input**: User description: "read phase 3"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Schemas exist (Priority: P1)
As a developer, I need six service-owned schemas in PostgreSQL so data ownership is enforced.
**Acceptance Scenarios**:
1. Given a fresh database, When migrations run, Then schemas `auth`, `crm`, `billing`, `inventory`, `trouble_ticket`, `catalog` exist.

### User Story 2 - Migrations are versioned (Priority: P1)
As a developer, I need SQLx migrations per schema so schema changes are controlled.
**Acceptance Scenarios**:
1. Migrations exist under `migrations/` organized by schema.
2. Migrations are version-controlled and testable.

### User Story 3 - Least privilege enforced (Priority: P1)
As a developer, I need restricted database roles so services only access their schemas.
**Acceptance Scenarios**:
1. Runtime roles have minimal privileges.
2. Migration credentials separate from runtime credentials.

## Requirements *(mandatory)*
- **FR-001**: Create schemas auth, crm, billing, inventory, trouble_ticket, catalog
- **FR-002**: Create migration directories per schema under migrations/
- **FR-003**: Define migration execution strategy (controlled, not per-service startup racing all)
- **FR-004**: Define restricted runtime roles per service
- **FR-005**: Separate migration and runtime credentials
- **FR-006**: Add PostgreSQL integration test setup
- **FR-007**: Document backup/recovery requirements

## Success Criteria *(mandatory)*
- **SC-001**: All six schemas exist after migrations
- **SC-002**: Each service has defined DB access policy
- **SC-003**: Migrations run in clean test DB
- **SC-004**: Permissions/constraints tested
- **SC-005**: Runtime credentials not superuser

## Assumptions
- Single PostgreSQL instance (ADR-0001)
- SQLx for migrations
