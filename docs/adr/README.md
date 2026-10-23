# Architecture Decision Records

This directory contains the Architecture Decision Records (ADRs) for the Telecom Core Backend.

## ADR Index

| ADR | Decision | Status |
|---|---|---|
| ADR-0001 | Centralized PostgreSQL with domain schemas | Accepted |
| ADR-0002 | Two-container runtime architecture | Accepted |
| ADR-0003 | Separate Catalog and Inventory domains | Accepted |
| ADR-0004 | Cross-service communication strategy | Proposed |
| ADR-0005 | Authentication and authorization strategy | Proposed |
| ADR-0006 | Subscription and order ownership | Open |
| ADR-0007 | Financial ledger integrity | Proposed |
| ADR-0008 | Rust workspace and service boundaries | Proposed

## Status Definitions
- Proposed: Recommended decision awaiting approval
- Accepted: Approved and guides implementation
- Rejected: Evaluated and not selected
- Superseded: Replaced by later ADR
- Open: Additional information required

## ADR Rules
1. Create ADR for decisions affecting architecture, security, data ownership, or operations
2. Record alternatives and rationale
3. Link affected specs/requirements/tests
4. Do not silently change accepted decisions - supersede instead
5. Keep unresolved decisions explicit
6. Update architecture docs and traceability when ADR affects requirements
