# ADR-0001: Use one PostgreSQL instance with six service-owned schemas

**Status:** Accepted (subject to review)  
**Date:** 2026-10-23  

## Context
Need clear data ownership across 6 services with minimal operational complexity.

## Decision
Use single PostgreSQL instance with schemas: auth, crm, billing, inventory, trouble_ticket, catalog. Each service owns its schema and migrations; no cross-service direct writes.

## Consequences
+ Simple deployment (1 DB container)
- Less isolation than separate DBs
- Must enforce least-privilege and migration control
