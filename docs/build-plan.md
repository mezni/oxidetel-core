# Telecom Core Backend — Build Plan

**File:** `docs/build-plan.md`  
**Version:** 3.0  
**Status:** Proposed  
**Stack:** Rust, Axum, Tokio, SQLx, PostgreSQL  
**Methodology:** Spec-Driven Development (SDD)

## 1. Project objective

Build an enterprise-style telecom backend that supports customer management, authentication, billing, commercial catalog, resource inventory, and trouble-ticket management. Develop incrementally with specs, tests, and traceability. Constraints: exactly two Docker containers (telecom-db, telecom-backend), six logical Rust services, six schemas, no cross-service DB writes, no AI/UI.

## 2. Target repository structure

```text
telecom-core/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── Dockerfile
├── docker-compose.yml
├── .env.example
├── .gitignore
├── README.md
├── CONTRIBUTING.md
├── apps/
│   ├── auth-service/
│   ├── crm-service/
│   ├── billing-service/
│   ├── inventory-service/
│   ├── ticketing-service/
│   └── catalog-service/
├── crates/
│   ├── api-common/
│   ├── config/
│   └── observability/
├── migrations/
│   ├── auth/
│   ├── crm/
│   ├── billing/
│   ├── inventory/
│   ├── trouble_ticket/
│   └── catalog/
├── specs/
├── docs/
│   ├── constitution.md
│   ├── architecture.md
│   ├── build-plan.md
│   ├── domain-model.md
│   ├── definition-of-done.md
│   ├── test-strategy.md
│   ├── traceability-matrix.md
│   ├── assumptions-and-open-questions.md
│   └── adr/
└── tests/
```

## 3. Phase overview

| Phase | Name | Main deliverable |
|---|---|---|
| 0 | SDD governance | Constitution, templates, Definition of Done |
| 1 | Domain discovery | Domain model, requirements, open questions |
| 2 | Rust workspace | Compilable workspace and quality tooling |
| 3 | PostgreSQL foundation | Schemas, migrations, restricted roles |
| 4 | Backend container | Six running HTTP services in one container |
| 5 | Authentication and RBAC | Secure identities and authorization |
| 6 | CRM | Customer and account APIs |
| 7 | Commercial catalog | Products, offers, rate plans, add-ons |
| 8 | Inventory | Devices, SIM/eSIM, assignments |
| 9 | Billing | Invoices, payments, ledger, reconciliation |
| 10 | Ticketing | Tickets, comments, transitions, audit events |
| 11 | Synthetic data | Reproducible test and demo data |
| 12 | Cross-service integration | Validated end-to-end business workflows |
| 13 | Production readiness | Security, performance, recovery, release checks |

(Details per phases follow as before - abbreviated for brevity, full plan already conceptually covered)

## 4. Standard feature implementation workflow
1. Define problem/spec
2. Define acceptance criteria
3. Review architecture/ADRs
4. Design API/data
5. Plan implementation
6. Implement
7. Test
8. Verify (fmt/clippy/test)
9. Update docs/traceability
10. Commit

## 5. Continuous integration
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```
Plus integration/migration/authz/financial/concurrency/dependency/audit checks.

## 6. Definition of Done
See `docs/definition-of-done.md`.

## 7. Recommended implementation order
Follow phases 0-13 as above.

## 8. First implementation milestone
Phase 2: Rust workspace and tooling - compilable workspace with 6 services + shared crates, fmt/clippy/tests passing.
