# OxideTel BSS (Business Support Systems & Operations Support Systems)

[![OxideTel BSS CI/CD Pipeline](https://github.com/your-username/oxidetel-core/actions/workflows/ci.yml/badge.svg)](https://github.com/your-username/oxidetel-core/actions/workflows/ci.yml)
[![Rust Version](https://img.shields.io/badge/rust-edition%202021-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**OxideTel BSS** is a production-grade, modular monolith telecommunications BSS/OSS engine built completely from scratch in **Rust**. It implements core telecom workflows ranging from customer onboarding and product catalog management to order provisioning, usage rating, invoicing, payment webhooks, automated dunning collections, and real-time observability.

---

## 🏛️ Architecture & Bounded Contexts

OxideTel BSS follows **Domain-Driven Design (DDD)** principles within a Cargo workspace. It runs on a **single PostgreSQL instance** partitioned into isolated **domain schemas** to enforce strict modular boundaries while maintaining transactional integrity:

┌─────────────────────────────────────────────────────────────┐│                       Axum REST API                         │├──────────────┬─────────────┬──────────────┬─────────────────┤│   iam        │     crm     │   catalog    │    ordering     │├──────────────┼─────────────┼──────────────┼─────────────────┤│   billing    │   dunning   │    rating    │   OSS Adapter   │└──────────────┴─────────────┴──────────────┴─────────────────┘│PostgreSQL (Multi-Schema Storage)
- **`iam`**: Identity & Access Management, Argon2 password hashing, and JWT Role-Based Access Control (RBAC).
- **`crm`**: Customer records, account status, profiles, and subscriber life-cycle states.
- **`catalog`**: Rate plans (tariffs), SIM cards, hardware devices, and active subscriptions.
- **`ordering`**: Service order placement, workflow validation, and OSS network provisioning mock adapters.
- **`rating`**: Call Detail Record (CDR) ingestion and mediation.
- **`billing`**: Transactional invoice generation, line-item calculations, overage mediation, and payment ledgers.
- **`dunning`**: Collections state machine (soft suspension, hard service barring, and automated unbarring via payment webhook settlement).

---

## ⚙️ Tech Stack

- **Language:** Rust (Edition 2021 / Latest Stable)
- **Web Framework:** Axum & Tower-HTTP (CORS, TraceLayer)
- **Database & Migrations:** SQLx (with pure Rust TLS `runtime-tokio-rustls` and multi-schema PostgreSQL migrations)
- **Async Runtime:** Tokio
- **Security:** Argon2 (password hashing) &jsonwebtoken (JWT RBAC)
- **Observability:** `tracing` (structured JSON logging) & `metrics-exporter-prometheus` (`/metrics` endpoint)
- **Containerization:** Multi-stage Docker build & Docker Compose orchestration

---

## 📁 Workspace Directory Structure

```text
oxidetel-core/
├── Cargo.toml                  # Workspace root manifest
├── Dockerfile                  # Multi-stage release build container
├── docker-compose.yml          # Postgres & BSS service orchestration
├── Makefile                    # Streamlined local workflow commands
├── migrations/                 # Multi-schema SQL migrations (1 to 7)
├── crates/                     
│   ├── domain-core/            # Shared domain primitives & aggregate roots
│   ├── iam/                    # Authentication & RBAC domain crate
│   ├── crm/                    # Customer relationship management domain crate
│   ├── catalog/                # Rate plans, SIMs, and inventory domain crate
│   ├── ordering/               # Orders & OSS network provisioning mock crate
│   ├── generator/              # Synthetic CDR & customer data stream generator
│   ├── billing/                # Invoices, billing ledgers, & rating mediation crate
│   └── dunning/                # Collections state machine & background worker crate
├── src/                        
│   ├── main.rs                 # Application bootstrap, workers, & telemetry init
│   └── api.rs                  # Axum router & REST endpoint controllers
└── tests/                      
    └── customer_lifecycle_test.rs # E2E automated customer lifecycle integration tests
🚀 Getting StartedPrerequisitesRust Toolchain (latest stable)Docker & Docker ComposeMake (optional, for convenience commands)Environment ConfigurationCreate a .env file in the root directory (defaults are automatically provided if omitted):Code snippetDATABASE_URL=postgres://postgres:postgres@localhost:5432/oxidetel
RUST_LOG=info
💻 Local Development & ExecutionYou can run the entire system locally using Make:Start PostgreSQL and run the BSS engine:Bashmake run
Run the integration test suite:Bashmake test
Tear down local containers and volumes:Bashmake clean
🐳 Containerized DeploymentBuild and run the full-stack containerized environment using Docker Compose:Bashdocker compose up --build
This spins up:oxidetel-postgres: PostgreSQL 15 database container with automated health checks.oxidetel-bss: Optimized multi-stage Rust binary container running auto-migrations, background dunning/mediation workers, and the Axum API server.🔌 API Endpoints & ObservabilityEndpointMethodDescription/healthGETService health check/api/customersPOSTRegister a new customer/api/catalog/plansGETList available rate plans and tariffs/api/ordersPOSTPlace a service order and trigger OSS provisioning/api/invoices/:customer_idGETRetrieve customer invoice billing history/api/webhooks/paymentPOSTInbound payment gateway webhook (settles invoices & unbars services)/metricsGETPrometheus operational metrics scraper endpoint🧪 TestingRun workspace unit tests and full end-to-end customer lifecycle integration tests (covering onboarding, ordering, billing, dunning suspension, payment settlement, and service unbarring):Bashcargo test --workspace
cargo test --test customer_lifecycle_test -- --nocapture
🛡️ CI/CD PipelineAutomated Continuous Integration is configured via GitHub Actions (.github/workflows/ci.yml), performing:Clippy Linter Checks (-D warnings)Workspace Unit TestsE2E Customer Lifecycle Integration Tests against a live PostgreSQL service containerRelease Binary CompilationDocker Multi-Stage Build Validation📄 LicenseDistributed under the MIT License. See LICENSE for more information.
