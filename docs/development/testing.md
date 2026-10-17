# Testing Guide

How to run and write tests for OxideTel BSS: unit tests, integration tests, and data seeding.

## Quick Start

```bash
make test      # starts Postgres via compose, runs workspace tests + lifecycle test
```

`make test` runs `docker compose up -d`, waits, then:

```bash
cargo test --workspace
cargo test --test customer_lifecycle_test -- --nocapture
```

Individual pieces:

```bash
cargo test --workspace                 # all unit + lib tests
cargo test -p billing                  # one crate
cargo test --test customer_lifecycle_test   # integration only (needs live DB)
cargo test -- --nocapture              # show stdout/log output
```

**Requirement:** integration tests need a reachable PostgreSQL with `DATABASE_URL` set (default `postgres://postgres:postgres@localhost:5432/oxidetel`). `tests/common/mod.rs` runs migrations on setup. Use `make up` to start the stack first.

> Compile-time `sqlx::query!` macros also need a live `DATABASE_URL` at **build** time — `cargo test` on a clean checkout without a DB will fail to compile if macro queries are present.

## Test Layout

```
tests/
├── common/mod.rs                  # setup_test_db(): connect + run sqlx migrations
└── customer_lifecycle_test.rs     # end-to-end subscriber lifecycle
crates/*/src/                      # unit tests in #[cfg(test)] modules
```

### Existing tests

| Test | Type | What it covers |
|---|---|---|
| `test_full_telecom_customer_lifecycle` (`tests/customer_lifecycle_test.rs`) | Integration, live DB | customer → rate plan → order → invoice → dunning (`SOFT_SUSPENSION`) → payment → `RESOLVED` |
| `it_works` in `crates/crm`, `crates/catalog` | Unit (scaffold) | placeholder only — expand these |

**Caveats:**

- The lifecycle test drives **direct SQL**, not the HTTP API — it validates the data model and cross-schema FKs, not the routes.
- It **silently skips** plan-dependent assertions if no `catalog.rate_plans` rows exist (the catalog is empty on a fresh DB). Seed first (see below) or assertions won't run.
- It writes legacy-cased statuses (`'Active'`, `'Unpaid'`, `'Paid'`) that differ from API-written values — see the casing note in [contributing.md](contributing.md).

## Writing Unit Tests

Pure domain logic lives in the crates — test it without a database:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overage_rounds_up_to_whole_mb() {
        let plan = RatePlan { /* ... */ };
        assert_eq!(plan.calculate_overage(1_500_000), /* expected MB */);
    }
}
```

Good unit-test targets (currently under-tested):

- `catalog::RatePlan::calculate_overage` — ceil-MB math
- `dunning::domain::DunningStage::escalate/resolve` — state-machine transitions & illegal moves
- `iam::domain::User::register/verify_password` — Argon2 round-trip
- `ordering::OrderStatus` transitions
- Money arithmetic in `billing::engine` — always assert in **cents**, exact integers

Run them: `cargo test -p <crate>` (no DB needed).

## Writing Integration Tests

Follow the `tests/common/mod.rs` pattern:

```rust
mod common;

#[tokio::test]
async fn invoice_survives_payment_webhook() {
    let pool = common::setup_test_db().await;   // applies migrations

    sqlx::query(r#"INSERT INTO crm.customers (id, name, email, status)
                   VALUES ($1, 'Test', 't@example.com', 'ACTIVE')"#)
        .bind(customer_id)
        .execute(&pool).await.unwrap();

    // ... arrange → act → assert
}
```

Guidelines:

1. **Always go through `setup_test_db()`** so migrations are applied — never assume a schema exists.
2. **Use unique emails/references per test** (`Uuid::new_v4()`), since the DB persists between runs.
3. **Assert cross-context effects together** — a payment test should check `billing.payments`, `billing.invoices`, `dunning.cases`, `crm.customers`, and `catalog.subscriptions` in one test (that's the whole point of the webhook's transaction).
4. **Idempotency tests:** send the same `gateway_reference` twice; expect the second insert to fail and *no* state change.
5. Prefer exercising the **HTTP API** (`tower::ServiceExt::oneshot` on `create_router(pool)`) for route-level coverage — the current lifecycle test doesn't, so new tests should.

### Suggested integration cases (gaps worth filling)

- [ ] `POST /api/customers` → 201, row in `crm.customers` with `ACTIVE`
- [ ] Duplicate email behavior (currently 500 — document/assert actual)
- [ ] `POST /api/orders` with unknown `rate_plan_id` → 404
- [ ] `GET /api/invoices/:id` returns rows / empty array
- [ ] Payment webhook happy path end-to-end (all 5 tables)
- [ ] Payment webhook **duplicate `gateway_reference`** → rollback, single payment row
- [ ] Dunning worker: overdue invoice → `SOFT_SUSPENSION` + customer `Suspended`
- [ ] CDR mediation batch: unrated CDRs → invoice items + amount increment + marked rated

## Seeding Test Data

The `generator` crate provides `TelephonyDataGenerator`:

- `seed_base_system(count)` — fake customers (via `fake`), SIM ICCID/IMSI/MSISDN, and a default rate plan.
- `stream_cdrs(...)` — streams synthetic CDRs into `rating.cdrs` every 500ms.

**Known gap:** the generator is **not wired** into `main.rs`, tests, or any binary — it never runs on its own. To use it today, invoke it from a test or a temporary `#[tokio::main]` example. Wiring a `seed` binary/example is a welcome contribution.

Minimal manual seed via SQL (useful right now):

```sql
INSERT INTO catalog.rate_plans (id, name, code, monthly_fee_cents,
                                included_data_bytes, overage_rate_per_mb_cents, is_active)
VALUES (gen_random_uuid(), '5G 10GB', 'PLAN_5G_10GB', 2500, 10737418240, 10, TRUE)
ON CONFLICT (code) DO NOTHING;
```

For a **scratch database** (never point tests at production):

```bash
createdb oxidetel_test
DATABASE_URL=postgres://postgres:postgres@localhost:5432/oxidetel_test cargo test
```

## CI

`.github/workflows/ci.yml` runs, on every push/PR:

1. `cargo fmt --all -- --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo test --test customer_lifecycle_test -- --nocapture` (Postgres 15-alpine service container)
5. Release build + Docker image build (no push)

Match CI locally before pushing — see the pre-push commands in [contributing.md](contributing.md).

## Troubleshooting

| Problem | Fix |
|---|---|
| `connection refused` / tests hang | `make up` — Postgres must be running; check `docker ps` |
| `database does not exist` | Ensure db `oxidetel` exists (compose creates it); or run migrations via `cargo run` |
| Compile error in `sqlx::query!` macros | Export `DATABASE_URL` to a DB that has the migrations applied |
| Lifecycle test passes but does nothing | Catalog empty → plan assertions skipped; seed `catalog.rate_plans` first |
| Tests fail only in CI | Casing differences (`'Unpaid'` vs `'ISSUED'`) — assert with `lower(status)` or normalize |
| Stale schema after migration change | `make clean` (⚠️ destroys the compose volume) or `docker compose down -v` |
