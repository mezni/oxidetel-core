# OxideTel BSS — System Overview

High-level architecture and data flow for the OxideTel Business Support System (BSS), the billing and subscriber-management backend for a telecommunications operator.

## What It Is

A **modular monolith**: a single Rust binary (`oxidetel-bss`) built on an Axum HTTP server, with domain logic partitioned into Cargo workspace crates and data partitioned into PostgreSQL schemas. One deployable artifact, one database, strict internal boundaries.

- **Language / runtime:** Rust (edition 2021), Tokio async runtime
- **HTTP framework:** Axum 0.7
- **Database:** PostgreSQL 15, accessed via SQLx 0.7 (no ORM)
- **Observability:** `tracing` (JSON logs) + Prometheus (`/metrics`)

## Topology

```
                         ┌──────────────────────────────────────────────┐
                         │              oxidetel-bss (single            │
                         │                 Rust binary)                 │
   Clients / ──────────► │                                              │
   Frontend              │  Axum Router (src/api.rs)                    │
   Portal                │   ├── /health                                │
                         │   ├── /api/customers            ┌────────────┴───────────┐
   Payment   ──────────► │   ├── /api/catalog/plans        │  Background Workers     │
   Gateway               │   ├── /api/orders               │   ├── Dunning worker    │
   (webhooks)            │   ├── /api/invoices/:customer_id│   │    (30s loop)        │
                         │   ├── /api/webhooks/payment     │   └── CDR mediation     │
   Prometheus ─────────► │   └── /metrics                  │        (45s loop)       │
                         │                                 └────────────┬───────────┘
                         └──────────────────────────────────────────────┘
                                          │  SQLx pool (max 20 conns)
                                          ▼
                         ┌──────────────────────────────────────────────┐
                         │           PostgreSQL 15                      │
                         │  iam │ crm │ catalog │ ordering │ rating     │
                         │  billing │ dunning        (7 schemas)        │
                         └──────────────────────────────────────────────┘
```

## Request / Data Flow

### 1. Customer onboarding
`POST /api/customers` inserts a row into `crm.customers` with status `ACTIVE`.

### 2. Ordering & provisioning
`GET /api/catalog/plans` lists active rate plans from `catalog.rate_plans`.
`POST /api/orders` looks up the plan fee and inserts a row into `ordering.orders` inside a single transaction. Real network provisioning is handled by the OSS adapter (`crates/ordering/src/oss_adapter.rs`) — currently a mock that logs UNBAR/ACTIVATE commands.

### 3. Usage → rating → billing
1. CDRs (call detail records) land in `rating.cdrs` (seeded synthetically by the `generator` crate).
2. The **CDR mediation worker** (`src/main.rs`, 45s loop) calls `billing::mediation::process_cdr_mediation_batch`: it reads unrated CDRs, computes data overage against the subscriber's plan, appends `billing.invoice_items`, increments `billing.invoices.amount_cents`, and marks the CDR rated.
3. Invoices live in `billing.invoices` with status `ISSUED → PAID | OVERDUE | CANCELLED` and Net-30 terms.

### 4. Dunning (collections)
The **dunning worker** (`crates/dunning/src/worker.rs`, 30s loop) sweeps overdue invoices:

| Condition | Stage | Side effects |
|---|---|---|
| Overdue invoice, no case yet | `SOFT_SUSPENSION` | Customer set to `Suspended` in `crm.customers` |
| Soft case aging past threshold | `HARD_SUSPENSION` | Escalated; OSS barring logged |
| Payment webhook received | `RESOLVED` | Service restored |

Stage progression: `PAST_DUE → SOFT_SUSPENSION → HARD_SUSPENSION → RESOLVED`.

### 5. Payment settlement (webhook)
`POST /api/webhooks/payment` runs a **single transaction** that:
1. Inserts into `billing.payments` (unique `gateway_reference` for idempotency),
2. Marks the invoice `PAID`,
3. Resolves any `dunning.cases` for that invoice,
4. Restores the subscription to `ACTIVE` and logs the OSS UNBAR command.

See [api/webhooks.md](../api/webhooks.md) for the full contract.

## Workspace Layout

| Crate | Domain responsibility |
|---|---|
| `domain-core` | Shared DDD primitives: `AggregateRoot`, `DomainEvent`, `Repository<A>` |
| `iam` | Users, Argon2 password hashing, roles (`ADMIN`/`CSR`/`CUSTOMER`) |
| `crm` | Customer aggregate, status lifecycle |
| `catalog` | Rate plans, SIMs, devices, inventory, subscriptions |
| `ordering` | Orders, order items, provisioning logs, OSS adapter |
| `billing` | Invoices, payments, billing engine, CDR mediation |
| `dunning` | Dunning cases, collection state machine, background worker |
| `generator` | Synthetic data seeding & CDR stream generation |

The root package is the binary: `src/main.rs` (bootstrap, workers, metrics) + `src/api.rs` (HTTP routes).

## Key Design Decisions

| Decision | See |
|---|---|
| Modular monolith over microservices | [ADR-0001](adr/0001-modular-monolith-over-microservices.md) |
| PostgreSQL multi-schema isolation | [ADR-0002](adr/0002-postgresql-multi-schema-isolation.md) |
| SQLx compile-time queries over ORM | [ADR-0003](adr/0003-sqlx-compile-time-query-checking.md) |

## Known Gaps

These are documented deliberately so operators and contributors don't get surprised:

- **No authentication is enforced** — the JWT extractor in `src/auth.rs` exists but is not wired into the router.
- **`/metrics` exposes no custom application metrics yet** — see [operations/monitoring.md](../operations/monitoring.md).
- **Status-string casing is inconsistent** across writers (`'ISSUED'` vs `'Unpaid'`, `'ACTIVE'` vs `'Active'`) — see [development/contributing.md](../development/contributing.md).
- **Payment webhook has no signature verification** — see [api/webhooks.md](../api/webhooks.md).
