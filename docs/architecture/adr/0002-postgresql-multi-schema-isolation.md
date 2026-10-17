# ADR-0002: PostgreSQL Multi-Schema Isolation

## Status
Accepted

## Date
2026-10-17

## Context

Following the decision to build a modular monolith ([ADR-0001](0001-modular-monolith-over-microservices.md)), we needed a way to keep each bounded context's data genuinely isolated. Constraints:

- Every context lives in the same PostgreSQL database (single-node, single-transaction model).
- Contexts *do* legitimately reference each other — IAM users belong to CRM customers, dunning cases reference billing invoices and catalog subscriptions.
- We wanted a future path to "one schema → one service" without rewriting tables.
- Access control should be enforceable *outside* application code, not merely by convention.

## Decision

Give each bounded context its **own PostgreSQL schema** inside a single database. Migration `20261017000000_init_multi_schema.sql` creates seven schemas up front:

```
iam      crm      catalog      ordering
rating   billing  dunning
```

Rules:

1. **A crate only writes to its own schema.** Reads across schemas are allowed but must follow the cross-context contract table in [domain-bounded-contexts.md](../domain-bounded-contexts.md).
2. **Foreign keys cross schemas freely** (e.g. `iam.users.customer_id → crm.customers`, `dunning.cases.invoice_id → billing.invoices`). Logical boundaries are enforced by review; *referential* integrity is enforced by PostgreSQL.
3. **Fully qualify every table** in SQL: `INSERT INTO crm.customers ...`, never `INSERT INTO customers ...`. The `search_path` is never used to hide schema names.
4. Tables are named in plural snake_case within their schema; constraints and indexes are prefixed with the table name (`idx_dunning_stage`, `idx_invoices_customer`).

## Alternatives Considered

### Separate databases per context
- Pros: Strongest isolation; independent backup/restore and resource control.
- Cons: Kills cross-context transactions — the payment webhook's atomic "pay invoice + resolve dunning + unbar service" would become a distributed saga. Requires foreign-data-wrapper or app-level joins for reporting.
- Rejected: We would lose exactly the atomicity the monolith was chosen for.

### Single shared schema (tables prefixed by domain, e.g. `crm_customers`)
- Pros: Trivial to set up; no schema qualification needed.
- Cons: Purely convention-based; a bad `DROP TABLE customers` or a careless join silently crosses boundaries. No meaningful migration path to service extraction.
- Rejected: Provides no enforcement and no extraction path.

### Table-level RBAC only (one DB role, GRANT per table)
- Pros: Fine-grained.
- Cons: Grants multiply with every new table and are painful to keep in migrations; doesn't give logical grouping for reporting or search-path control.
- Rejected as the *primary* mechanism, but schema-level `GRANT`s remain a good hardening step for production (see [operations/runbook.md](../../operations/runbook.md)).

### Row-Level Security
- Pros: Great for multi-tenant SaaS on shared tables.
- Cons: Our isolation unit is the *domain*, not the tenant; RLS policies would add per-query overhead and complexity with no benefit here.
- Rejected.

## Consequences

**Positive**
- Boundaries are visible in the database itself: `\dn` shows the domain map; `\dt billing.*` shows exactly what billing owns.
- A future service extraction is `pg_dump --schema=billing` plus a connection-string change — the data layout already matches the service boundary.
- Cross-schema foreign keys give us integrity without coupling schemas logically.
- Reporting can be scoped deliberately (`SELECT * FROM billing.invoices` cannot accidentally wander into `iam.users` without an explicit qualified name).
- Schema-level privileges can later restrict a runtime DB role to only the schemas its process touches.

**Negative**
- Every query must schema-qualify — slightly more verbose, and forgetting a prefix fails loudly (usually a good thing, occasionally annoying in psql).
- Migrations must be ordered carefully: schema creation must precede any table, and cross-schema FKs require the referenced table to exist first (handled by the timestamped migration sequence `2026101700000x`).
- Cross-schema joins in ad-hoc reporting require knowing the domain map.

**Risks / follow-ups**
- **Status-string casing drift**: writers currently mix `'ISSUED'`/`'Unpaid'` and `'ACTIVE'`/`'Active'` across schemas. This is a schema-adjacent consistency bug — normalize to uppercase enum-like strings (tracked in [development/contributing.md](../../development/contributing.md)).
- Ensure application DB roles are granted per-schema in production rather than using the superuser.
