# Operational Runbook

Runbook for OxideTel BSS support and operations engineers. Covers the three most common incident classes: **hard suspensions**, **failed payment webhooks**, and **database migrations**.

**Service:** `oxidetel-bss` (single binary, port 8080)
**Database:** PostgreSQL 15, schemas `iam crm catalog ordering rating billing dunning`
**Connect:** `docker exec -it oxidetel-postgres psql -U postgres -d oxidetel`

> All monetary values are **cents**. All timestamps in the DB are UTC.

---

## 1. Handling Hard Suspensions

A subscriber is hard-suspended when an invoice is long overdue: the dunning worker opens a `SOFT_SUSPENSION` case, sets `crm.customers.status = 'Suspended'`, then escalates to `HARD_SUSPENSION`. Network barring is intended to follow via the OSS adapter.

### 1.1 Identify suspended subscribers

```sql
SELECT c.id, c.name, c.email, c.status            AS customer_status,
       dc.stage, dc.days_overdue, dc.updated_at,
       i.id AS invoice_id, i.amount_cents, i.status AS invoice_status, i.due_date
FROM   dunning.cases dc
JOIN   crm.customers  c  ON c.id = dc.customer_id
JOIN   billing.invoices i ON i.id = dc.invoice_id
WHERE  dc.stage = 'HARD_SUSPENSION'
ORDER  BY dc.updated_at DESC;
```

All hard suspensions (any stage of concern):

```sql
SELECT stage, COUNT(*) FROM dunning.cases GROUP BY stage ORDER BY 2 DESC;
```

### 1.2 Verify the case is legitimate (not a false positive)

Before barring/unbarring, confirm the money facts:

```sql
-- Payments recorded against the invoice (should be none / insufficient)
SELECT id, amount_cents, status, gateway_reference, created_at
FROM   billing.payments WHERE invoice_id = '<invoice_id>' ORDER BY created_at;

-- Full invoice state
SELECT id, amount_cents, currency, status, due_date, created_at
FROM   billing.invoices WHERE id = '<invoice_id>';
```

**Known trap — status casing:** writers mix casings. The API creates invoices as `'ISSUED'` while some workers/tests write `'Unpaid'`; customers are written as both `'ACTIVE'` and `'Active'`. When filtering, cover both:

```sql
WHERE lower(i.status) IN ('issued', 'unpaid')
```

A dunning case whose invoice already shows `PAID` is stale → close it (§1.4).

### 1.3 Verify OSS network unbarring state

Barring state lives in two places; check both:

```sql
-- 1. Subscription (what BSS believes)
SELECT s.id, s.status AS sub_status, s.sim_id, s.customer_id,
       sc.msisdn, sc.iccid, sc.status AS sim_status
FROM   catalog.subscriptions s
JOIN   catalog.sim_cards sc ON sc.id = s.sim_id
WHERE  s.customer_id = '<customer_id>';
```

```sql
-- 2. OSS (actual network state) — check the service logs
# docker logs oxidetel-bss 2>&1 | grep "OSS ADAPTER"
```

- Look for `📡 [OSS ADAPTER] Sending UNBAR / RESTORE command for SIM ID: …` — printed by the payment webhook on success.
- The OSS adapter is currently a **mock**: it logs the command and BSS only updates `catalog.subscriptions.status`. The authoritative barring flag must be confirmed in the real OSS/CORE if one is connected (see `crates/ordering/src/oss_adapter.rs`).

If the subscription shows `SUSPENDED`/barred but the network is not barred (or vice-versa), reconcile by recording the intended state (§1.5).

### 1.4 Manually resolve a dunning case (goodwill / corrected invoice)

Run **all statements in one transaction** — this mirrors exactly what the payment webhook does:

```sql
BEGIN;

UPDATE billing.invoices
SET    status = 'PAID'
WHERE  id = '<invoice_id>'
  AND  status <> 'PAID';          -- guard against double-settlement

INSERT INTO billing.payments (id, invoice_id, customer_id, amount_cents,
                              gateway_reference, status, created_at)
SELECT gen_random_uuid(), '<invoice_id>', customer_id, amount_cents,
       'manual-adjust-<ticket#>', 'SUCCESS', NOW()
FROM   billing.invoices WHERE id = '<invoice_id>'
AND NOT EXISTS (
  SELECT 1 FROM billing.payments
  WHERE  gateway_reference = 'manual-adjust-<ticket#>'   -- idempotency
);

UPDATE dunning.cases
SET    stage = 'RESOLVED', updated_at = NOW()
WHERE  invoice_id = '<invoice_id>' AND stage <> 'RESOLVED';

UPDATE crm.customers SET status = 'ACTIVE' WHERE id = '<customer_id>';

UPDATE catalog.subscriptions
SET    status = 'ACTIVE'
WHERE  customer_id = '<customer_id>';

COMMIT;
```

Verify:

```sql
SELECT (SELECT status FROM billing.invoices  WHERE id = '<invoice_id>') AS invoice,
       (SELECT stage  FROM dunning.cases     WHERE invoice_id = '<invoice_id>') AS dunning,
       (SELECT status FROM crm.customers     WHERE id = '<customer_id>') AS customer,
       (SELECT status FROM catalog.subscriptions WHERE customer_id = '<customer_id>') AS subscription;
-- expect: PAID | RESOLVED | ACTIVE | ACTIVE
```

Then confirm/trigger unbar in the OSS and note the ticket in the payment's `gateway_reference`.

### 1.5 Re-barr a subscriber (suspend without a webhook)

For a customer-initiated suspension or fraud hold:

```sql
BEGIN;
UPDATE crm.customers SET status = 'SUSPENDED' WHERE id = '<customer_id>';
UPDATE catalog.subscriptions SET status = 'SUSPENDED' WHERE customer_id = '<customer_id>';
-- Open or escalate a dunning case if this is collections-driven:
UPDATE dunning.cases SET stage = 'HARD_SUSPENSION', updated_at = NOW()
WHERE customer_id = '<customer_id>' AND stage <> 'RESOLVED';
COMMIT;
```

Log the corresponding `BAR` command in `ordering.provisioning_logs` for audit:

```sql
INSERT INTO ordering.provisioning_logs (id, order_id, sim_id, status, payload, executed_at)
SELECT gen_random_uuid(), o.id, s.sim_id, 'BARRED', 'manual: <ticket#>', NOW()
FROM   catalog.subscriptions s
JOIN   ordering.orders o ON o.customer_id = s.customer_id
WHERE  s.customer_id = '<customer_id>' LIMIT 1;
```

---

## 2. Replaying Failed Payment Webhooks

Endpoint: `POST /api/webhooks/payment`. Handler: `src/api.rs:185`.

### 2.1 Understand failure modes

The handler runs **one transaction**: insert payment → mark invoice `PAID` → resolve dunning → unbar subscription. Any error rolls everything back.

| Symptom | Cause | Action |
|---|---|---|
| HTTP 500, `gateway_reference` already exists | Duplicate delivery — original already applied | **No action.** Treat as success; tell gateway to stop retrying (unique constraint = idempotency) |
| HTTP 500, FK violation on `invoice_id` | Webhook arrived before invoice existed (or bad ID) | Replay **after** invoice is issued (§2.3) |
| HTTP 500, transient DB error | DB/network blip | Replay (§2.3) — safe, transaction rolled back |
| HTTP 200 but customer still barred | Subscription missing for customer, or OSS mock only logged | Inspect §1.3, apply §1.4/§1.5 |
| HTTP 404/405 | Wrong path/method | Gateway config — must be `POST /api/webhooks/payment`, JSON body |

### 2.2 Check what already happened

```sql
-- Did this reference already settle?
SELECT p.id, p.amount_cents, p.status, p.created_at,
       i.status AS invoice_status
FROM   billing.payments p
JOIN   billing.invoices  i ON i.id = p.invoice_id
WHERE  p.gateway_reference = '<gateway_reference>';
```

- Row exists + invoice `PAID` → **already settled, do not replay.**
- No row → the webhook never committed; safe to replay.

### 2.3 Replay

**Preferred — let the gateway retry.** Retries are safe by construction; a duplicate returns 500 with no state change.

**Manual replay** (gateway unavailable, or settling an unpaid invoice directly):

```bash
curl -sS -X POST http://localhost:8080/api/webhooks/payment \
  -H 'Content-Type: application/json' \
  -d '{
    "invoice_id":     "<invoice_uuid>",
    "customer_id":    "<customer_uuid>",
    "amount_cents":   4500,
    "gateway_reference": "ch_original_ref_123"
  }'
```

Expect:

```json
{"status":"success","payment_id":"…","invoice_id":"…","message":"Payment processed, dunning resolved, and service restored."}
```

Then verify the full chain (§1.4 query): invoice `PAID` → dunning `RESOLVED` → customer `ACTIVE` → subscription `ACTIVE`, and an `OSS ADAPTER` unbar line in `docker logs oxidetel-bss`.

**Never** invent a second `gateway_reference` for the same charge — that breaks idempotency and creates a duplicate payment row.

### 2.4 Settle an invoice with no gateway (offline/cash payment)

Use the manual-adjust transaction from §1.4 with a `gateway_reference` like `manual-adjust-<ticket#>` (≤100 chars, unique).

---

## 3. Database Maintenance — Running Migrations Safely

Migrations are SQL files in `migrations/` with timestamp prefixes (`2026101700000x_*.sql`), tracked in the `sqlx._sqlx_migrations` table.

### 3.1 How they run

- **Automatically at startup:** `sqlx::migrate!("./migrations").run(&pool)` in `src/main.rs:39` — every deploy applies pending migrations before serving traffic. The Docker image ships `/app/migrations`.
- **In tests:** `tests/common/mod.rs` runs them on test-DB setup.

### 3.2 Pre-deploy checklist (production)

1. **Write the migration as additive-first.** New table/column/index → safe. Renames/drops → two-phase (deploy code reading both, then a follow-up migration to drop).
2. **Order matters across schemas:** a migration adding a cross-schema FK must run after the referenced table exists (follow the timestamp sequence; never edit an applied migration — add a new one).
3. **Review the SQL as a DBA would:** every statement must be idempotent-safe or guarded (`IF NOT EXISTS`, `ADD COLUMN IF NOT EXISTS`) where the platform allows.
4. **Back up before schema-changing deploys:**
   ```bash
   docker exec oxidetel-postgres pg_dump -U postgres -d oxidetel \
     -Fc -f /tmp/oxidetel-$(date +%Y%m%d%H%M).dump
   docker cp oxidetel-postgres:/tmp/oxidetel-$(date +%Y%m%d%H%M).dump ./backups/
   ```
5. **Test on a scratch DB first:**
   ```bash
   createdb oxidetel_migrate_test
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/oxidetel_migrate_test \
     cargo sqlx migrate run   # if sqlx-cli installed
   # or simply: cargo run   # startup applies migrations to the configured DB
   ```

### 3.3 Applying migrations

**Managed path (recommended for prod):** deploy the new image; startup applies migrations automatically. Watch logs:

```bash
docker logs -f oxidetel-bss 2>&1 | grep -E "migration|Migrations applied"
# expect: "Executing database multi-schema migrations..." then
#         "Database migrations applied successfully."
```

**Manual path (sqlx-cli):**

```bash
cargo install sqlx-cli --no-default-features --features postgres
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/oxidetel

sqlx migrate info     # list pending vs applied  (read-only, safe)
sqlx migrate run      # apply all pending
sqlx migrate revert   # revert last (only if the migration supports it)
```

> **Rollback policy:** SQLx migrations are forward-only by convention. To undo, write a *new* migration that reverses the change — don't hand-edit `_sqlx_migrations`.

### 3.4 If a migration fails mid-deploy

SQLx applies each migration file in a transaction (Postgres supports transactional DDL), so a failure leaves the schema at the last successfully applied migration — not half-applied.

1. Read the error (usually a duplicate object, missing referenced table, or lock timeout).
2. Confirm state:
   ```sql
   SELECT version, success, installed_on FROM sqlx._sqlx_migrations ORDER BY version DESC LIMIT 5;
   ```
3. Fix the migration file **only if it was never recorded as successful**; otherwise add a new corrective migration.
4. Redeploy / re-run `sqlx migrate run`.

### 3.5 Locks & long migrations

Postgres DDL takes `ACCESS EXCLUSIVE` locks. On a large table:

- Prefer `CREATE INDEX CONCURRENTLY` (must run **outside** a transaction — put it in its own migration file with `-- no-transaction` handling or apply manually during a window).
- Add columns as nullable / with defaults computed in app code to avoid full-table rewrites.
- Take a maintenance window for anything touching `rating.cdrs` or `billing.invoices` at scale.

### 3.6 Routine maintenance

```sql
-- Connection pool: app caps at 20 (PgPoolOptions::max_connections(20))
SELECT count(*) AS conns, state FROM pg_stat_activity
WHERE datname = 'oxidetel' GROUP BY state;

-- Table sizes
SELECT schemaname, relname,
       pg_size_pretty(pg_total_relation_size(schemaname||'.'||relname)) AS total
FROM pg_tables WHERE schemaname IN
  ('iam','crm','catalog','ordering','rating','billing','dunning')
ORDER BY pg_total_relation_size(schemaname||'.'||relname) DESC;

-- Bloat / vacuum after bulk mediation runs
VACUUM ANALYZE rating.cdrs;
VACUUM ANALYZE billing.invoices;
```

---

## 4. Quick Incident Index

| Symptom | Go to |
|---|---|
| Customer reports "I paid but I'm still cut off" | §2.2 → §2.3 → §1.3 |
| Bulk of accounts suddenly suspended | Check dunning worker loop & status-casing trap (§1.2) |
| Gateway reporting webhook 500s | §2.1 table |
| Deploy stuck / migration error | §3.4 |
| High DB connections / pool exhaustion | §3.6 + [monitoring.md](monitoring.md) alerts |
| `/metrics` missing expected series | [monitoring.md](monitoring.md) — metric gaps section |

## 5. Useful Commands

```bash
make up          # start postgres + bss
make down        # stop
make run         # up + cargo run
make test        # full workspace + lifecycle tests
make clean       # cargo clean + compose down -v (DESTROYS DB VOLUME)

docker logs -f oxidetel-bss                 # JSON structured logs
curl -s localhost:8080/health               # liveness
curl -s localhost:8080/metrics | head       # Prometheus exposition
```
