# Feature Specification: CRM (Phase 6)

**Feature Branch**: `007-crm`  
**Created**: 2026-10-23  
**Status**: Draft  
**Input**: Phase 6 from build plan

## User Scenarios & Testing *(mandatory)*
### User Story 1 - Customers (Priority: P1)
As a CSR, I need customer management.
**Acceptance Scenarios**:
1. Create/read/update customer

### User Story 2 - Accounts (Priority: P1)
As a CSR, I need accounts.
**Acceptance Scenarios**:
1. Manage accounts

### User Story 3 - Relationships (Priority: P1)
As a CSR, I need relationships.
**Acceptance Scenarios**:
1. Track relationships

## Requirements *(mandatory)*
- **FR-001**: CRM service owns crm schema
- **FR-002**: Customer, account entities
- **FR-003**: APIs with validation
- **FR-004**: No cross-service writes

## Success Criteria *(mandatory)*
- **SC-001**: CRM APIs work
- **SC-002**: Schema owned by crm
- **SC-003**: RBAC enforced

## Assumptions
- REST APIs
