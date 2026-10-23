# Feature Specification: Authentication and RBAC (Phase 5)

**Feature Branch**: `006-auth-rbac`  
**Created**: 2026-10-23  
**Status**: Draft  
**Input**: Phase 5 from build plan

## User Scenarios & Testing *(mandatory)*
### User Story 1 - Authentication (Priority: P1)
As a user, I need to authenticate.
**Acceptance Scenarios**:
1. Login works
2. Tokens issued

### User Story 2 - RBAC (Priority: P1)
As an admin, I need role-based access control.
**Acceptance Scenarios**:
1. Roles enforced

### User Story 3 - Security (Priority: P1)
As a system, I need secure auth.
**Acceptance Scenarios**:
1. Passwords hashed

## Requirements *(mandatory)*
- **FR-001**: Auth service with users/roles/permissions
- **FR-002**: JWT tokens
- **FR-003**: RBAC middleware
- **FR-004**: Schema auth

## Success Criteria *(mandatory)*
- **SC-001**: Auth endpoints work
- **SC-002**: RBAC enforced
- **SC-003**: No cross-service writes

## Assumptions
- JWT-based
