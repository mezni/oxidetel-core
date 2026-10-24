# Telecom Core Backend — Traceability Matrix

**File:** `docs/traceability-matrix.md`  
**Version:** 1.0  
**Status:** Proposed  

| Requirement ID | Source (Constitution/Spec) | Service/Component | Acceptance Criteria | Implementation | Tests | Status |
|---|---|---|---|---|---|---|
| REQ-ARCH-001 | Constitution Art. IV | Deployment | Exactly 2 containers | docker-compose.yml | Deployment checks | Pending |
| REQ-ARCH-002 | Constitution Art. III | Services | 6 logical services | apps/* | Service startup tests | Pending |
| REQ-DB-001 | Constitution Art. V | DB | 6 schemas owned by services | migrations/* | Schema ownership tests | Pending |
| REQ-DATA-001 | Constitution Art. VI | Synthetic data | Seed data per schema respecting ownership | migrations/*/20241023000003*, specs/011-synthetic-data/ | Seed data verification tests | Pending |
| REQ-SEC-001 | Constitution Art. VII | AuthZ/AuthN | RBAC + least privilege | auth-service, middleware | AuthZ tests | Pending |
| REQ-FIN-001 | Constitution Art. VIII | Billing | Immutable ledger, compensating entries | billing-service | Financial invariants tests | Pending |
| REQ-API-001 | Constitution Art. IX | All APIs | /api/v1 versioned | all services | API contract tests | Pending |
| REQ-TKT-001 | Constitution Art. X | Ticketing | State machine + events | ticketing-service | Transition tests | Pending |
