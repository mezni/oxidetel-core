# Domain Bounded Contexts

Detailed breakdown of each bounded context in the OxideTel BSS modular monolith. Each context owns a **Cargo crate** and a **PostgreSQL schema** (or set of schemas), and communicates with other contexts only through explicit contracts — never by reaching into another context's tables ad hoc.

## Context Map

```
   ┌─────────┐      ┌─────────┐      ┌──────────┐
   │   iam   │─────►│   crm   │◄─────│ catalog  │
   └─────────┘      └────┬────┘      └────┬─────┘
        (users)          │          (subscriptions)
                         ▼                ▲
                  ┌────────────┐          │
                  │  ordering  │──────────┘
                  └─────┬──────┘
                        │ orders
        ┌───────────────┼───────────────┐
        ▼               ▼               ▼
   ┌─────────┐    ┌───────────┐    ┌──────────┐
   │ rating  │───►│  billing  │───►│ dunning  │
   └─────────┘    └───────────┘    └──────────┘
     (CDRs)        (invoices)      (collections)
```

---

## IAM — Identity & Access Management

**Crate:** `crates/iam` · **Schema:** `iam`

Manages operators and subscriber portal accounts.

| Table | Purpose |
|---|---|
| `iam.roles` | `ADMIN`, `CSR`, `CUSTOMER` (seeded with fixed UUIDs) |
| `iam.users` | Email + Argon2 `password_hash`, `is_active`, optional `customer_id` FK → `crm.customers` |
| `iam.user_roles` | Many-to-many user↔role join |

**Domain notes:**
- `iam::domain::User` exposes `register()` (Argon2 hashing) and `verify_password()`.
- The `customer_id` link is what ties a portal login to a subscriber account — IAM *references* CRM but CRM never references IAM.
- JWT bearer extraction is sketched in `src/auth.rs` but not yet wired into the router.

**Ownership rule:** only the IAM crate may write `iam.*` tables.

---

## CRM — Customer Relationship Management

**Crate:** `crates/crm` · **Schema:** `crm`

The customer master record — the hub that every other context references by `customer_id`.

| Table | Purpose |
|---|---|
| `crm.customers` | `name`, unique `email`, `status` ∈ `ACTIVE \| SUSPENDED \| TERMINATED` |

**Domain notes:**
- `Customer` aggregate with `suspend()` / `activate()` transitions (`crates/crm/src/domain.rs`).
- Status transitions are driven by the dunning worker (suspend) and the payment webhook (activate).

**Ownership rule:** `crm.customers.status` is written by CRM-facing operations only (API registration, dunning worker, payment webhook) — never by billing or catalog directly.

---

## Catalog — Products & Inventory

**Crate:** `crates/catalog` · **Schema:** `catalog`

Everything sellable and everything physically assigned to a subscriber.

| Table | Purpose |
|---|---|
| `catalog.rate_plans` | `code` (e.g. `PLAN_5G_10GB`), `monthly_fee_cents`, `included_data_bytes`, `overage_rate_per_mb_cents` |
| `catalog.sim_cards` | Unique `iccid`, `imsi`, `msisdn`; status `IN_STOCK \| ACTIVATED \| SUSPENDED` |
| `catalog.devices` | Brand/model/SKU, retail price |
| `catalog.device_inventory` | Physical `imei` / `serial_number`, assigned `customer_id` |
| `catalog.subscriptions` | The join that matters: `customer_id` + `rate_plan_id` + `sim_id` + optional device, `billing_cycle_day` (1–28) |

**Domain notes:**
- `RatePlan::calculate_overage(total_used_bytes)` performs ceil-MB overage math used by the rating worker.
- A subscriber's network identity is `catalog.subscriptions.sim_id` — this is the handle the OSS barring/unbarring logic operates on.

**Ownership rule:** the subscription is the *contract* between catalog and billing: billing reads `rate_plans` for pricing, dunning reads `subscriptions.sim_id` for barring.

---

## Ordering — Sales & Provisioning

**Crate:** `crates/ordering` · **Schema:** `ordering`

Order capture and hand-off to the network.

| Table | Purpose |
|---|---|
| `ordering.orders` | `customer_id`, `status` ∈ `PENDING \| PROCESSING \| COMPLETED \| FAILED`, `total_amount_cents` |
| `ordering.order_items` | `product_type` ∈ `RATE_PLAN \| DEVICE \| SIM`, `product_id`, `price_cents` |
| `ordering.provisioning_logs` | Audit trail: order, sim, `status` ∈ `ACTIVATED \| BARRED`, payload, `executed_at` |

**Domain notes:**
- `OssProvisioningAdapter` (`crates/ordering/src/oss_adapter.rs`) is the seam between BSS and the operational support system (the actual mobile network). It is currently a mock — real `reqwest` calls are stubbed.
- Provisioning writes go through `provisioning_logs` so that network actions are auditable.

**Ownership rule:** ordering may *read* catalog to price order items; it never mutates catalog inventory outside its own tables.

---

## Rating — Usage Mediation

**Crate:** none (logic lives in `billing::mediation` + `generator`) · **Schema:** `rating`

| Table | Purpose |
|---|---|
| `rating.cdrs` | `customer_id`, `usage_type` ∈ `DATA \| VOICE \| SMS`, `bytes_used`; index `(customer_id, created_at)` |

**Domain notes:**
- CDRs are written by the `generator` crate (synthetic stream) or, in production, by network probes.
- Rating (turning raw usage into money) is owned by the **billing** context — the `rating` schema is deliberately just a staging area for raw usage.

**Ownership rule:** raw CDRs are append-mostly; the only mutation is the "rated" marker set by the mediation worker.

---

## Billing — Invoicing & Payments

**Crate:** `crates/billing` · **Schema:** `billing`

| Table | Purpose |
|---|---|
| `billing.invoices` | `amount_cents`, `currency` (ISO-4217, default `USD`), `status` ∈ `ISSUED \| PAID \| OVERDUE \| CANCELLED`, `due_date`; Net-30 terms |
| `billing.invoice_items` | Line items: description + `amount_cents` (cascade on invoice delete) |
| `billing.payments` | `amount_cents`, `gateway_reference` **UNIQUE** (idempotency key), `status` ∈ `SUCCESS \| FAILED` |

**Domain notes:**
- `BillingEngine::generate_customer_invoice()` composes the plan fee plus data overage.
- `process_cdr_mediation_batch()` runs on a 45s loop: read unrated CDRs → compute overage using `overage_rate_per_mb_cents` → append invoice items → increment invoice total → mark rated.
- All money is stored as `i64` **cents** — never floats.

**Ownership rule:** the payment webhook is the only ingress for payment state; gateways must never write tables directly.

---

## Dunning — Collections

**Crate:** `crates/dunning` · **Schema:** `dunning`

| Table | Purpose |
|---|---|
| `dunning.cases` | `invoice_id`, `customer_id`, `stage` ∈ `PAST_DUE \| SOFT_SUSPENSION \| HARD_SUSPENSION \| RESOLVED`, `days_overdue`, `updated_at`; index on `stage` |

**Domain notes:**
- `DunningCase` implements the state machine in `crates/dunning/src/domain.rs`: `escalate()` and `resolve()` transitions.
- The active worker (`crates/dunning/src/worker.rs`, 30s loop):
  1. Opens a `SOFT_SUSPENSION` case for newly overdue invoices and sets `crm.customers.status = 'Suspended'`.
  2. Escalates aged soft cases to `HARD_SUSPENSION`.
  3. Resolution happens only via the payment webhook (`RESOLVED` + OSS unbar).
- `DunningEngine` (`engine.rs`) is a fuller implementation (14-day soft / 30-day hard thresholds, direct OSS barring) that is not yet invoked by the binary — see [development/contributing.md](../development/contributing.md).

**Ownership rule:** dunning is the *only* context permitted to flip a customer into `Suspended`, and it does so through an explicit, logged transition.

---

## Cross-Context Contract Summary

| From → To | Contract | Mechanism |
|---|---|---|
| iam → crm | "This user belongs to this customer" | `iam.users.customer_id` FK |
| ordering → catalog | Price a plan / assign a SIM | Read `rate_plans`, `sim_cards` |
| billing → catalog | Overage pricing for a subscriber | Read `rate_plans`, `subscriptions` |
| dunning → crm | Suspend / (via webhook) reactivate customer | Write `crm.customers.status` |
| dunning → catalog | Bar / unbar network service | Read `subscriptions.sim_id`, OSS adapter |
| billing → dunning | Payment settles collections | Write `dunning.cases.stage = 'RESOLVED'` |
| rating → billing | Usage becomes money | Mediation worker appends invoice items |
| all → domain-core | Shared aggregate/repository primitives | Trait imports only |

Cross-schema foreign keys exist (e.g. `iam.users.customer_id → crm.customers`) and are enforced by PostgreSQL — this is intentional: boundaries are *logical*, integrity is *physical*.
