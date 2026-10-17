# Monitoring & Metrics Guide

Reference for SREs operating OxideTel BSS: how metrics are exported, which series exist today, which are **missing**, and the alert rules worth configuring.

## How `/metrics` Works

- **Exporter:** [`metrics-exporter-prometheus`](https://docs.rs/metrics-exporter-prometheus) 0.13.1 (`http-listener` feature) + `metrics` 0.22.4.
- **Installation:** `PrometheusBuilder::new().install_recorder()` in `src/main.rs:25`.
- **Exposition:** `GET /metrics` on the same Axum server (`0.0.0.0:8080`), rendered per-request via `recorder_handle.render()` (`src/main.rs:63`).
- **Format:** standard Prometheus text exposition (`Content-Type: text/plain; version=0.0.4`).

Verify it's alive:

```bash
curl -s http://localhost:8080/metrics | head -20
```

### Prometheus scrape config

```yaml
scrape_configs:
  - job_name: oxidetel-bss
    scrape_interval: 15s
    metrics_path: /metrics
    static_configs:
      - targets: ['oxidetel-bss:8080']
        labels:
          service: oxidetel-bss
          env: production
```

### Liveness

`GET /health` → `{"status":"healthy","system":"OxideTel BSS","version":"1.0.0"}`. Use it for container healthchecks / LB targets; use `/metrics` presence for scrape success.

---

## Metric Inventory (current state)

> **Gap — read this first.** The recorder is installed, but **no custom application metrics are defined anywhere in the codebase**. There are no `counter!`/`gauge!`/`histogram!` call sites, and no process collector is registered. Today `/metrics` renders an essentially **empty** registry.

| Metric | Type | Source | Status |
|---|---|---|---|
| *(none custom)* | — | — | **Missing** — instrument the code |

Until instrumentation lands, rely on the proxies below.

### Interim observability (no code changes required)

| Signal | How to get it | What it tells you |
|---|---|---|
| HTTP request rate/errors/latency | `TraceLayer` JSON logs → log-based metrics (Loki / CloudWatch / Datadog logs pipeline) from `tower_http` request lines | API traffic & 5xx rate |
| Dunning escalations | grep logs for `[DUNNING]` / `Hard suspension` | Suspension pressure |
| OSS barring commands | grep logs for `OSS ADAPTER` | Unbar/bar activity |
| Webhook settlements | grep logs for `[WEBHOOK] Payment of` | Payment success flow |
| Mediation errors | `tracing::error!("Error in CDR mediation worker: …")` | Rating pipeline health |
| DB connections | `pg_stat_activity` count vs pool cap (20) | Pool exhaustion |
| Process/OS metrics | node_exporter on the host | CPU/RAM/disk of the BSS box |

Structured logs are JSON (`fmt::layer().json()`), filter via `RUST_LOG` (default `oxidetel_bss=debug,tower_http=debug`).

---

## Recommended Instrumentation (to close the gap)

Add with the `metrics` macros already in the dependency tree. Suggested set:

```rust
// API layer (per request, in middleware or handler)
metrics::counter!("http_requests_total", "route" => route, "status" => status).increment(1);
metrics::histogram!("http_request_duration_seconds", "route" => route).record(secs);

// Payment webhook
metrics::counter!("payments_webhook_total", "outcome" => "success"|"duplicate"|"error").increment(1);

// Dunning worker (per sweep)
metrics::counter!("dunning_cases_total", "stage" => "SOFT_SUSPENSION"|"HARD_SUSPENSION"|"RESOLVED").increment(1);
metrics::gauge!("dunning_cases_open").set(open_count as f64);

// Mediation worker
metrics::counter!("cdrs_rated_total").increment(batch);
metrics::counter!("mediation_errors_total").increment(1);

// DB pool
metrics::gauge!("db_pool_connections", "state" => "idle"|"in_use").set(n as f64);
```

Name convention: `<namespace>_<unit>_<type>` (`_total` for counters, base unit for histograms — seconds/bytes, not milliseconds).

---

## Recommended Alerting Rules

Prometheus rule group examples. Tune thresholds after observing a baseline.

```yaml
groups:
  - name: oxidetel-bss-sre
    rules:
      # --- Availability ---
      - alert: OxideTelDown
        expr: up{job="oxidetel-bss"} == 0
        for: 2m
        labels: {severity: critical}
        annotations:
          summary: "oxidetel-bss target down for 2m"
          runbook: "docs/operations/runbook.md"

      - alert: HighHttp5xxRate
        expr: |
          sum(rate(http_requests_total{job="oxidetel-bss", status=~"5.."}[5m]))
            / sum(rate(http_requests_total{job="oxidetel-bss"}[5m])) > 0.05
        for: 5m
        labels: {severity: warning}
        annotations:
          summary: "5xx ratio > 5% for 5m"

      # --- Dunning / collections ---
      - alert: HardSuspensionSpike
        expr: sum(rate(dunning_cases_total{stage="HARD_SUSPENSION"}[1h])) > 0.05
        for: 15m
        labels: {severity: warning}
        annotations:
          summary: "Elevated rate of HARD_SUSPENSION dunning cases (>3/hr sustained)"
          description: "May indicate a billing-run failure or mass invoice error — check billing.invoices and status casing."
          runbook: "docs/operations/runbook.md#1-handling-hard-suspensions"

      - alert: OpenDunningCasesGrowing
        expr: predict_linear(dunning_cases_open[2h], 4*3600) > 500 and dunning_cases_open > 100
        for: 30m
        labels: {severity: warning}
        annotations:
          summary: "Open dunning cases trending toward 500 in 4h"

      # --- Payments / webhooks ---
      - alert: PaymentWebhookErrors
        expr: sum(rate(payments_webhook_total{outcome="error"}[5m])) > 0.1
        for: 5m
        labels: {severity: critical}
        annotations:
          summary: "Payment webhook failing — settlements not landing"
          runbook: "docs/operations/runbook.md#2-replaying-failed-payment-webhooks"

      - alert: PaymentWebhookDuplicateStorm
        expr: sum(rate(payments_webhook_total{outcome="duplicate"}[5m])) > 1
        for: 10m
        labels: {severity: warning}
        annotations:
          summary: "Gateway retrying duplicates — original likely succeeded; check gateway stop-retry config"

      # --- Database / pool exhaustion ---
      - alert: DbPoolExhausted
        expr: db_pool_connections{state="in_use"} >= 20
        for: 2m
        labels: {severity: critical}
        annotations:
          summary: "SQLx pool at cap (20) — requests will queue/timeout"
          description: "Check pg_stat_activity for stuck queries, long transactions, or lock waits."
          runbook: "docs/operations/runbook.md#36-routine--maintenance"

      - alert: DbConnectionsNearLimit
        expr: db_pool_connections{state="in_use"} > 16
        for: 10m
        labels: {severity: warning}
        annotations:
          summary: "DB pool >80% utilized for 10m"

      - alert: PostgresDown
        expr: pg_up == 0
        for: 1m
        labels: {severity: critical}

      # --- Background workers ---
      - alert: MediationErrors
        expr: sum(rate(mediation_errors_total[10m])) > 0
        for: 10m
        labels: {severity: warning}
        annotations:
          summary: "CDR mediation worker erroring — rating/invoicing falling behind"

      - alert: CdrBacklogNotDraining
        expr: |
          rate(cdrs_rated_total[15m]) == 0
          and (rating_cdr_unrated_count > 1000)
        for: 15m
        labels: {severity: warning}
        annotations:
          summary: "Unrated CDR backlog growing with zero throughput"

      # --- Scrape health ---
      - alert: MetricsEndpointMissing
        expr: absent(up{job="oxidetel-bss"} == 1)
        for: 5m
        labels: {severity: warning}
```

### Alert priority matrix

| Severity | Examples | Response |
|---|---|---|
| **Critical** | Target down, webhook errors, pool exhausted | Page immediately — [runbook](runbook.md) |
| **Warning** | 5xx ratio, hard-suspension spike, mediation errors, pool >80% | Ticket + business hours |
| **Info/trend** | Duplicate webhook storm, case backlog growth | Review in weekly ops |

---

## Dashboards (Grafana)

Suggested panels for an `OxideTel BSS` dashboard:

1. **Golden signals** — request rate, 5xx ratio, p95 latency (from `http_requests_total` / `http_request_duration_seconds` once instrumented; log-derived before that), `up`.
2. **Money flow** — `payments_webhook_total` by outcome, invoices paid rate, open dunning cases by stage (stacked).
3. **Suspensions** — `dunning_cases_total` by stage (rate), current `dunning_cases_open`, customer status breakdown.
4. **Pipeline** — CDRs rated/s, mediation errors, unrated backlog depth.
5. **Resources** — `db_pool_connections` vs cap 20, Postgres connections, host CPU/RAM/disk (node_exporter), container restarts.

Import as JSON under `docs/operations/dashboards/` if you version-control them, and link dashboards back to this doc and the runbook.

---

## Metric Gaps — Known Issues

Track these so monitoring maturity doesn't silently stall:

1. **Zero custom metrics** — `/metrics` is effectively empty; everything above the "interim" table assumes instrumentation lands. Highest-priority gap.
2. **No process/runtime collector** — no CPU/RSS/GC-style series from the exporter itself; use node_exporter/cAdvisor.
3. **Log-only signals** — dunning, webhook, and OSS events are `println!`/`tracing` only; no counters. Webhook handler uses `println!` rather than `tracing`.
4. **No instrumentation for pool usage** — pool exhaustion can only be detected from Postgres side today (`pg_stat_activity`).
5. **No alerting wired for the mock OSS** — barring is logged, not measured; a real OSS integration must emit success/failure metrics per command.
6. **Cardinality caution** — when adding labels, never use `customer_id`, `invoice_id`, or `gateway_reference` as label values (unbounded cardinality will blow up the TSDB). Put those in logs/traces instead.
