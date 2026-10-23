# Telecom Core Backend — Assumptions and Open Questions

**File:** `docs/assumptions-and-open-questions.md`  
**Version:** 1.0  
**Status:** Proposed  

## Assumptions
- Two-container deployment acceptable initially
- Synchronous HTTP for inter-service calls initially
- PostgreSQL single instance with schemas
- Rust/Axum/Tokio/SQLx stack as specified

## Open Questions
1. Does CRM own subscribers and subscriptions, or separate service? (ADR needed)
2. Where are billing accounts owned and relation to customers? (ADR needed)
3. Authentication mechanism and token lifecycle (JWT details)? (ADR needed)
4. Which operations require SUPERVISOR approval?
5. Ledger posting/reversal/reconciliation rules (double-entry vs single-entry)?
6. Service-to-service auth independent of user auth?
7. Availability/latency/throughput/recovery targets?
8. Async workflows needed?
9. Data retention/privacy/audit retention?
10. Migration/recovery strategy for prod changes?

## Resolution process
All open questions must be resolved or have approved ADRs before implementing dependent features.
