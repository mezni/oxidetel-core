# ADR-0004: Cross-Service Communication Strategy

- Status: Proposed
- Date: 2026-10-23

## Decision
Use documented APIs/contracts; no direct cross-schema writes by default. Internal HTTP or in-process with same rules. Async only if justified.

## Consequences
+ Boundaries preserved
- Need timeouts/retries/idempotency/failure handling
