# Webhook Documentation

Inbound webhooks consumed by OxideTel BSS, and outbound integration points with external systems (payment gateway, OSS/network).

## Inbound: `POST /api/webhooks/payment`

The settlement callback from the payment gateway. This is the **only** path that moves an invoice to `PAID`, resolves a dunning case, and restores barred network service.

### Request

`Content-Type: application/json`

```json
{
  "invoice_id": "9c1f5a2e-3b7d-4e8a-9f01-2d6c4b8e7a55",
  "customer_id": "3f1d2a4e-8b6c-4f9a-9d2e-1c7b5a0e3f44",
  "amount_cents": 4500,
  "gateway_reference": "ch_3Mxyz123abcDEF"
}
```

| Field | Type | Required | Notes |
|---|---|---|---|
| `invoice_id` | UUID | yes | `billing.invoices.id` to settle |
| `customer_id` | UUID | yes | Used to locate `catalog.subscriptions` for unbar |
| `amount_cents` | int64 | yes | Amount charged, in cents |
| `gateway_reference` | string ≤100 | yes | **Idempotency key** — unique in `billing.payments` |

### Processing (single transaction)

Handler: `handle_payment_webhook` in `src/api.rs:185`.

1. **Insert payment** — `billing.payments` with `status = 'SUCCESS'` and the `gateway_reference`.
2. **Mark invoice paid** — `billing.invoices.status = 'PAID'`.
3. **Resolve dunning** — `dunning.cases.stage = 'RESOLVED'` for all cases on that invoice (no-op if none).
4. **Unbar service** — look up `catalog.subscriptions.sim_id` for the customer, log the OSS `UNBAR / RESTORE` command, and set `subscriptions.status = 'ACTIVE'`.

Any failure at any step **rolls back the whole transaction** — no partial settlement is possible.

### Response

`200 OK`

```json
{
  "status": "success",
  "payment_id": "…uuid…",
  "invoice_id": "…uuid…",
  "message": "Payment processed, dunning resolved, and service restored."
}
```

`500` — transaction failed/rolled back (see Retry semantics).

### Idempotency & Retry Semantics

- `gateway_reference` has a `UNIQUE` constraint. A **duplicate delivery inserts the same reference again → constraint violation → full rollback → HTTP 500**. State is unchanged, so the gateway's retry is safe; after the original success, subsequent retries will keep returning 500 until the gateway stops.
- **Recommended handling:** gateways should treat HTTP 500 on an already-seen `gateway_reference` as "already processed" and stop retrying. BSS-side, a future improvement is to catch the unique violation and return `200` with a duplicate flag.
- There is currently **no signature verification** (no HMAC header check). Anyone who can reach the endpoint can settle an invoice. Until a signing scheme is added:
  - Bind the service to a private network / gateway allowlist.
  - Never expose `/api/webhooks/payment` publicly.

### Ordering Guarantee

Webhooks may arrive out of order relative to invoice creation. A webhook for an invoice that doesn't exist still inserts the payment (FK to `billing.invoices` will reject it → 500). Ensure the invoice is issued before the gateway is told to charge.

---

## Outbound: OSS / Network Provisioning

The BSS tells the mobile network (OSS — Operational Support System) to bar/unbar subscribers.

| Direction | Where | Status |
|---|---|---|
| **UNBAR / RESTORE** | Payment webhook, step 4 (`src/api.rs:236`) | Logged (`println!`) + subscription row updated; no real HTTP call |
| **ACTIVATE** | `OssProvisioningAdapter::trigger_activation` (`crates/ordering/src/oss_adapter.rs`) | Mock — real `reqwest` POST is stubbed out |
| **BAR** | `DunningEngine` hard-suspension path (`crates/dunning/src/engine.rs`) | Defined, **not invoked** by the running binary (the active worker only escalates the stage) |

The adapter is the intended single seam for all network commands. When integrating a real OSS:

1. Implement the `reqwest` calls inside `OssProvisioningAdapter` only — never scatter network calls through handlers.
2. Record every command in `ordering.provisioning_logs` (`ACTIVATED` / `BARRED`, payload, `executed_at`) for audit.
3. Make commands idempotent: the OSS will see retries.

---

## Outbound: Payment Gateway

BSS does not currently *call* the gateway; it only receives the webhook. Gateway references (`ch_…`) suggest a Stripe-style flow where charges are created client-side or by a separate checkout service.

When adding outbound gateway integration:

- Keep credentials out of the codebase (env var / secret manager).
- Never log full card data or raw gateway secrets.
- Reuse `billing.payments.gateway_reference` as the shared idempotency key for both directions.

---

## Adding a New Webhook

1. Add a DTO (`#[derive(Deserialize)]`) and handler in `src/api.rs`.
2. Route it in `create_router` under `/api/webhooks/<name>`.
3. Wrap **all** state changes in one transaction.
4. Use a unique business key for idempotency (mirror the `gateway_reference` pattern).
5. Document it here **and** add it to `openapi.yaml`.
6. Add an alert/metric for its failure rate — see [operations/monitoring.md](../operations/monitoring.md).
