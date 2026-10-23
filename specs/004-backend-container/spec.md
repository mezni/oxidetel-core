# Feature Specification: Backend Container and Service Startup (Phase 4)

**Feature Branch**: `004-backend-container`  
**Created**: 2026-10-23  
**Status**: Draft  
**Input**: User description: "read phase 4"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Two containers (Priority: P1)
As an operator, I need exactly two containers (telecom-db, telecom-backend) so deployment matches architecture.
**Acceptance Scenarios**:
1. docker-compose defines exactly two application services
2. Backend hosts all six services

### User Story 2 - Six services running (Priority: P1)
As a developer, I need all six APIs on ports 8001-8006 responding.
**Acceptance Scenarios**:
1. auth:8001, crm:8002, billing:8003, inventory:8004, ticketing:8005, catalog:8006 respond to health checks

### User Story 3 - Health and graceful shutdown (Priority: P1)
As an operator, I need health endpoints and graceful shutdown.
**Acceptance Scenarios**:
1. /health/live and /health/ready available
2. Graceful shutdown works

## Requirements *(mandatory)*
- **FR-001**: Multi-stage Dockerfile for Rust
- **FR-002**: Process supervisor managing all six services
- **FR-003**: Ports 8001-8006 assigned
- **FR-004**: Health and readiness endpoints
- **FR-005**: Graceful shutdown and failure handling
- **FR-006**: Network to PostgreSQL
- **FR-007**: Persistent volume for Postgres
- **FR-008**: Controlled migration execution

## Success Criteria *(mandatory)*
- **SC-001**: Exactly two containers defined
- **SC-002**: All six APIs start and respond
- **SC-003**: Health endpoints exist
- **SC-004**: Backend connects to PostgreSQL
- **SC-005**: Startup/shutdown tested

## Assumptions
- One backend container, not six separate backend containers
