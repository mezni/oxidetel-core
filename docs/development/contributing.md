# Contributing Guidelines

Guidelines for contributing to OxideTel BSS — adding new crates, writing database migrations, and keeping the modular-monolith boundaries intact.

## Project Layout

```
oxidetel-core/
├── Cargo.toml            # workspace + oxidetel-bss binary (Axum server)
├── src/                  # binary: main.rs (bootstrap/workers), api.rs (routes)
├── crates/               # one crate per bounded context
│   ├── domain-core/      # shared DDD primitives (AggregateRoot, DomainEvent, Repository)
│   ├── iam/              # identity & access
│   ├── crm/              # customers
│   ├── catalog/          # rate plans, SIMs, devices, subscriptions
│   ├── ordering/         # orders + OSS provisioning adapter
│   ├── billing/          # invoices, payments, mediation
│   ├── dunning/          # collections state machine + worker
│   └── generator/        # synthetic data seeding / CDR stream
├── migrations/           # SQLx migrations (timestamped, forward-only)
├── tests/                # integration tests
└── docs/                 # this documentation
```

Architecture context: [system-overview.md](../architecture/system-overview.md) and the [bounded-context map](../architecture/domain-bounded-contexts.md).

## Ground Rules

1. **Respect schema boundaries.** A crate writes only to its own PostgreSQL schema. Cross-schema reads must follow the contract table in [domain-bounded-contexts.md](../architecture/domain-bounded-contexts.md). Fully qualify every table (`billing.invoices`, never `invoices`).
2. **Bound parameters only.** Always `sqlx::query(...).bind($1)` — never interpolate values into SQL strings.
3. **Money is `i64` cents.** Never floats; currency as ISO-4217 string where needed.
4. **UUID v4 IDs** generated in the app (`Uuid::new_v4()`), not serial sequences.
5. **Normalize status strings to UPPERCASE** (`'ACTIVE'`, `'ISSUED'`, `'HARD_SUSPENSION'`) — see "Known inconsistencies" below.
6. **No commented-out code, no stale TODOs.** Delete; git remembers.

## Adding a New Crate

1. **Scaffold it:**
   ```bash
   cargo new --lib crates/<name>
   ```
2. **Register it in the workspace** — add `"<name>"` to `[workspace] members` and to `[dependencies]` in the root `Cargo.toml` (the binary is the consumer).
3. **Declare the module boundary:**
   ```rust
   // crates/<name>/src/lib.rs
   pub mod domain;   // don't leave lib.rs as the cargo-new placeholder!
   ```
   > Known issue: `crates/crm/src/lib.rs` and `crates/catalog/src/lib.rs` are still scaffold placeholders and don't declare `pub mod domain;` — their domain modules are orphaned. Don't copy that pattern.
4. **Reuse `domain-core`** for the aggregate pattern:
   ```rust
   use domain_core::{AggregateRoot, DomainEvent};
   pub struct Thing { /* ... */ }
   ```
5. **Create its schema** (see next section) and keep tables inside it.
6. **Add a cross-context entry** to [domain-bounded-contexts.md](../architecture/domain-bounded-contexts.md) — if the context isn't documented, it isn't integrated.
7. **Add tests** — see [testing.md](testing.md).

### When to add a crate vs. extend one

| Situation | Do this |
|---|---|
| New bounded context (own data + own lifecycle) | New crate + new schema |
| New entity inside an existing context | Extend the existing crate |
| Pure shared primitive used by 2+ contexts | `domain-core` (keep it logic-free) |
| One-off helper used by the binary only | `src/` in the root package |

## Database Migrations

Migrations are plain SQL files in `migrations/`, applied automatically at startup (`sqlx::migrate!("./migrations")`) and tracked in `sqlx._sqlx_migrations`.

### Creating a migration

```bash
touch migrations/$(date -u +%Y%m%d%H%M%S)_short_description.sql
```

- **Timestamp prefix must sort after every existing migration** — never edit a migration that may have been applied; add a new one instead.
- **One logical change per file.**
- **Forward-only:** to undo, write a new migration that reverses the change.

### Writing safe migrations

```sql
-- GOOD: additive, idempotent-guarded
CREATE TABLE IF NOT EXISTS billing.notes (
    id UUID PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES billing.invoices(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_billing_notes_invoice ON billing.notes (invoice_id);

ALTER TABLE crm.customers ADD COLUMN IF NOT EXISTS tier VARCHAR(20) NOT NULL DEFAULT 'STANDARD';
```

```sql
-- RISKY: destructive, requires two-phase rollout + backfill
ALTER TABLE billing.invoices DROP COLUMN legacy_total;   -- deploy code that stops reading it FIRST
```

Rules:

1. **Cross-schema FKs** require the referenced table to exist — put the migration *after* the referenced table's migration in timestamp order.
2. **Never `search_path`-dependent DDL** — schema-qualify everything (`billing.notes`, not `notes`).
3. **Large-table safety:** prefer `CREATE INDEX CONCURRENTLY` (runs outside a transaction — its own migration) and avoid rewrites on `rating.cdrs` / `billing.invoices`.
4. **Seed data** (roles, default plans) goes in migrations with `ON CONFLICT DO NOTHING` and fixed UUIDs where other rows reference them.
5. **Test before deploying** — see the checklist in [operations/runbook.md §3](../operations/runbook.md).

### Migration checklist

- [ ] Filename: `YYYYMMDDHHMMSS_description.sql`, sorts last
- [ ] All tables/indexes schema-qualified
- [ ] Additive / idempotent where possible
- [ ] Cross-schema FK ordering verified
- [ ] No destructive change without a prior code deploy that stops using the object
- [ ] Reviewed against [ADR-0002](../architecture/adr/0002-postgresql-multi-schema-isolation.md)

## SQLx: Runtime Queries vs. Compile-Time Macros

Per [ADR-0003](../architecture/adr/0003-sqlx-compile-time-query-checking.md), both styles are in use:

| Style | Use when | Note |
|---|---|---|
| `sqlx::query!("SELECT ...")` macro | Static SQL you want column-checked at compile time | Needs `DATABASE_URL` at build time (no `.sqlx/` offline cache is committed — run `make up` or have a local Postgres first) |
| `sqlx::query(r#"..."#)` runtime | Dynamic/optional filters, and currently most handlers in `src/api.rs` | No compile-time checking — double-check column names by hand |

Be consistent within a layer, and prefer the macro for new static queries once an offline cache (or a DB in CI) is in place.

## HTTP API Changes

- Add routes in `src/api.rs::create_router`; keep DTOs (`#[derive(Deserialize)]` requests, `#[derive(Serialize, FromRow)]` responses) next to their handler.
- Update **`docs/api/openapi.yaml`** in the same PR — the spec is the contract for client/SDK generation.
- Update `docs/api/webhooks.md` for anything under `/api/webhooks/*`.
- Use meaningful status codes: `201` on create, `404` for missing lookups, `500` for query failure. (Note: unique-constraint violations currently surface as `500` rather than `409` — mapping them to `409` is a welcome improvement.)
- **Auth:** no route is currently authenticated (the JWT extractor in `src/auth.rs` is unwired). If you wire it up, document the scheme in the OpenAPI `securitySchemes`.

## Background Workers

Two loops run in-process (`src/main.rs`): dunning (30s) and CDR mediation (45s).

- Workers take a `PgPool` clone and must be **idempotent** — every tick may re-observe the same rows.
- Log with `tracing` macros (`tracing::warn!` etc.), not `println!`, so events carry structured context and respect `RUST_LOG`.
- Long loops must not hold a transaction open across the sleep/interval.

## Known Inconsistencies (fix opportunistically)

These are real bugs/gaps — if you touch the surrounding code, fix them and note it here:

1. **Status casing drift** — API/migrations use `'ISSUED'`/`'PAID'`/`'ACTIVE'`; the dunning worker filters `status = 'Unpaid'` and sets `'Suspended'`; the integration test writes `'Active'`/`'Unpaid'`/`'Paid'`. The worker's `'Unpaid'` filter will **not** match API-created `'ISSUED'` invoices. Normalize to UPPERCASE (rule 5 above) and use `lower(x) = ...` defensively while migrating.
2. **Dunning stage serialization** — `DunningEngine` persists `format!("{:?}", stage)` (`PastDue`, `SoftSuspension`) while the worker/API use `'SOFT_SUSPENSION'`/`'HARD_SUSPENSION'`/`'RESOLVED'`. Pick the SCREAMING_SNAKE form everywhere.
3. **Orphaned modules** — `crm::domain` and `catalog::domain` aren't compiled (see "Adding a New Crate"); `src/auth.rs` is never declared as a module; `BillingEngine` and `TelephonyDataGenerator` are never invoked by the binary.
4. **`billing/src/mediation.rs` compile issues** — a malformed statement and compile-time `query!` macros referencing columns (`duration_seconds`, `data_megabytes`, `rated`) that don't exist in the `rating.cdrs` migration. Fix the migration or the query — don't hand-edit an applied migration.
5. **License mismatch** — `LICENSE` is Apache-2.0; README says MIT. Align README.
6. **Unused deps** — `rust_decimal` (root) and `reqwest` (ordering, mock-only) are declared but unused; remove or use them.

## Commit & PR Hygiene

- Keep PRs to one concern (a crate addition, a migration set, a bug fix).
- Include migrations, docs, and tests **with** the code they support — not in a follow-up.
- Run before pushing:
  ```bash
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace
  ```
  CI enforces exactly these (`.github/workflows/ci.yml`) plus the Postgres-backed lifecycle test.
- Link the relevant ADR or docs page in the PR description when a decision is involved.
