# ADR-0003: SQLx with Compile-Time Query Checking over an ORM

## Status
Accepted

## Date
2026-10-17

## Context

The BSS performs a mix of simple CRUD and genuinely SQL-shaped work: transactional payment settlement, set-based dunning sweeps, batch CDR mediation with aggregation, and cross-schema reads. Requirements:

- Raw SQL performance and full control over query plans for batch jobs (rating runs every 45s over `rating.cdrs`).
- Safety under the multi-schema layout from [ADR-0002](0002-postgresql-multi-schema-isolation.md) — a typo'd table or column must fail at compile time, not at 3am in production.
- No impedance mismatch: money is `i64` cents, IDs are `uuid`, timestamps are `chrono::DateTime<Utc>` — these should map to Postgres types with zero translation layers.
- Keep dependencies lean for a single-binary deploy.

## Decision

Use **SQLx 0.7** with runtime `sqlx::query` / `query_as` for API handlers and compile-time `sqlx::query!` macros where static checking adds the most value — instead of Diesel, SeaORM, or any heavy ORM.

- All queries are written as **plain SQL against schema-qualified tables** (`SELECT ... FROM billing.invoices`).
- Row types are explicit `#[derive(FromRow)]` structs (`RatePlanResponse`, `InvoiceResponse`) or ad-hoc `sqlx::Row::get::<T>("col")`.
- Transactions use SQLx's native `pool.begin()` / `tx.commit()` API.
- Migrations are SQL files under `migrations/`, applied at startup via `sqlx::migrate!("./migrations")`.

## Alternatives Considered

### Diesel
- Pros: Mature, compile-time checked query builder, great ` diesel print-schema` workflow.
- Cons: Query-builder DSL becomes painful for set-based sweeps and CTEs; schema-generated code needs regeneration on every migration; heavier compile times; cross-schema handling is clunkier than writing `billing.invoices` by hand.
- Rejected: We write a lot of SQL anyway — an ORM that hides it adds a second language to learn.

### SeaORM
- Pros: Async-native, entity/codegen, nice for CRUD apps.
- Cons: Entity definitions duplicate the schema we already express in migrations; abstraction leaks on window functions and batch updates; still pulls in a query-builder layer for queries we want to hand-tune.
- Rejected: Abstraction cost without proportional benefit for a workload this SQL-heavy.

### sqlx runtime-only (no macros at all)
- Pros: No `DATABASE_URL` needed at compile time; fastest builds.
- Cons: Loses the compile-time column/table check that is the main safety win for multi-schema SQL.
- Rejected as the *only* mode — we keep runtime queries for dynamic/optional paths but want macros where feasible.

### Raw driver (`tokio-postgres`) + hand-rolled mapping
- Pros: Minimal dependencies.
- Cons: We'd reimplement connection pooling, type mapping, and migration tooling ourselves.
- Rejected: SQLx is effectively "tokio-postgres + batteries" without giving up raw SQL.

## Consequences

**Positive**
- Queries stay readable and portable to psql — an operator debugging with the [runbook](../../operations/runbook.md) runs the *same* SQL the app runs.
- `#[derive(FromRow)]` gives typed row extraction with no reflection or runtime string maps.
- Migrations are plain `.sql` files: reviewable in PRs, runnable by DBAs, no codegen step in the deploy path.
- Compile-time macros catch schema drift (renamed/missing columns) before merge — critical when 7 schemas evolve in parallel.
- Money stays `i64` end-to-end; no decimal/float coercion layer.

**Negative**
- **Compile-time macros need a live `DATABASE_URL` at build time** (or a checked-in `.sqlx/` offline cache). CI must provision a Postgres service before `cargo check` — `.github/workflows/ci.yml` already does this; local builds need `make up` or a running local Postgres.
- No automatic relationship loading — joins are written by hand (acceptable: our joins are few and deliberate).
- Schema changes require touching every affected query string manually (mitigated by compile-time checking flagging them).
- The abstraction is thin, so SQL injection safety depends on always using bound parameters (`$1`, `.bind(...)`) — never string interpolation. This is a hard review rule.

**Risks / follow-ups**
- **Offline mode**: no `.sqlx/` cache is currently committed, so builds without a database fail on macro queries. Either commit `cargo sqlx prepare` output or keep a DB service in every build environment.
- Keep runtime `sqlx::query(...)` and macro `sqlx::query!(...)` usage consistent per layer; document the choice in [development/contributing.md](../../development/contributing.md).
