<!--
## Sync Impact Report

- **Version change**: 3.0.0 → 3.0.1
- **Modified principles**: None
- **Added sections**: Documentation Structure under Operations & Process
- **Removed sections**: None
- **Follow-up TODOs**: None
-->

# Telecom Core Backend Constitution

## Core Principles

### I. Project Purpose & Domain Ownership

Build a modular, enterprise-oriented telecom backend covering identity and access control, customer management, commercial catalog, inventory, billing, and trouble ticketing.

The project SHALL prioritize domain ownership, data integrity, security, testability, observability, and operational reliability. AI features and user-interface applications are outside the project scope.

**Rationale**: Clear scope boundaries prevent mission creep and ensure focused delivery of core telecom capabilities.

### II. Service Architecture & Independence

The backend SHALL contain six logically independent services: `auth-service`, `crm-service`, `billing-service`, `catalog-service`, `inventory-service`, and `ticketing-service`.

Each service SHALL have a separate Rust binary or independently runnable application target, its own HTTP routes and API contract, explicit request and response types, domain logic and error handling, database access restricted to its owned schema, SQLx migrations, unit/integration/API tests, and service-specific configuration with health endpoints.

Shared Rust crates MAY provide stable infrastructure capabilities but SHALL NOT bypass service ownership or create tightly coupled domain logic. Services SHALL NOT directly invoke another service's internal business functions or write to its tables. Cross-service interactions SHALL use documented HTTP APIs or approved integration mechanisms.

**Rationale**: Service independence enables autonomous development, deployment, and scaling while preventing implicit coupling through shared state.

### III. Security & Authorization

The system SHALL support `AGENT`, `SUPERVISOR`, and scoped service identities. Authentication and authorization SHALL be distinct—every protected endpoint SHALL validate the caller's identity, permissions, and resource access.

JWT validation SHALL verify signature, allowed algorithm, issuer, audience, expiration, and applicable time claims. Passwords SHALL be hashed using Argon2id. Secrets SHALL NOT be committed to Git or included in logs. Database access SHALL use least-privilege credentials.

External input SHALL be validated before domain operations. SQL queries SHALL use parameterized SQL and safe SQLx query interfaces. Security-sensitive actions SHALL produce auditable events without recording credentials, tokens, or unnecessary personal information.

**Rationale**: Defense-in-depth security prevents unauthorized access and ensures auditability of sensitive operations.

### IV. Financial Integrity

Money SHALL use exact decimal arithmetic (`rust_decimal`) or integer minor currency units, with an explicit currency. Billing rules SHALL define rounding, taxation, proration, payment allocation, adjustments, and invoice finalization.

Posted ledger entries SHALL be immutable through ordinary application operations. Corrections SHALL use compensating entries or another approved accounting mechanism. Financial operations SHALL support idempotency where duplicate requests could create duplicate effects. Approval requirements SHALL be specified for sensitive financial adjustments.

Automated tests SHALL cover duplicate requests, concurrency, monetary rounding, accounting invariants, and reconciliation.

**Rationale**: Immutable ledger entries and idempotent operations prevent double-posting and ensure reconcilable financial records.

### V. Testing & Code Quality

The project SHALL enforce code quality through `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`. PostgreSQL integration tests, migration tests, API contract tests, authorization tests, concurrency/idempotency tests, and dependency security audits are mandatory.

Critical domain rules SHALL be tested independently of HTTP handlers. Database tests SHALL run against PostgreSQL rather than relying exclusively on mocks. Tests SHALL cover valid requests, invalid input, authorization failures, boundary conditions, transaction behavior, and dependency failures. Unsafe Rust SHALL be avoided unless documented and safety-reviewed.

**Rationale**: Comprehensive testing across layers catches defects early and provides confidence in correctness of critical business logic.

## Technical Constraints

### Technology Stack

The backend SHALL use Rust stable, Axum for HTTP APIs, Tokio for async execution, SQLx for PostgreSQL access with migrations, Serde for serialization, `validator` crate for validation, `thiserror` for typed errors with `anyhow` for context, `tracing`/`tracing-subscriber` for structured logging, `utoipa` for OpenAPI docs, `argon2` for password hashing, a maintained JWT library with strict validation, `rust_decimal` or integer minor units for money, and `uuid` for identifiers.

Dependency versions SHALL be locked through `Cargo.lock`. Updates SHALL be reviewed and tested. The project SHALL prefer maintained libraries with appropriate security practices documented in the dependency baseline.

### Two-Container Deployment

Initial deployment uses two Docker containers: `telecom-db` (PostgreSQL with persistent storage) and `telecom-backend` (all six Rust service processes).

| Service | Port |
|---|---:|
| Auth | 8001 |
| CRM | 8002 |
| Billing | 8003 |
| Inventory | 8004 |
| Ticketing | 8005 |
| Catalog | 8006 |

The backend container SHALL use multi-stage Docker build and process supervisor managing all six services. Health, restart behavior, shutdown handling, and logs SHALL be explicitly configured. A single container SHALL NOT be treated as equivalent to six independent services. The codebase SHALL preserve ability to move each service into its own container later.

### Database Ownership

PostgreSQL SHALL contain six logical schemas:

| Schema | Owner |
|---|---|
| `auth` | Auth |
| `crm` | CRM |
| `billing` | Billing |
| `catalog` | Catalog |
| `inventory` | Inventory |
| `trouble_ticket` | Ticketing |

Each schema SHALL have a designated owning service. Database roles SHALL enforce least-privilege access. SQLx migrations SHALL be version-controlled, reviewed, and tested against PostgreSQL. Production schema changes SHALL follow controlled migration process. Cross-schema foreign keys require an approved ADR.

### Domain Modeling

The model SHALL distinguish customers, subscribers, billing accounts, commercial products, product offers, rate plans, add-ons, subscriptions, inventory resources, invoices, payments, ledger entries, and trouble tickets.

Catalog owns commercial definitions and effective-dated product terms. Inventory owns physical/logical resources, assignments, and lifecycle states. Billing owns financial records and accounting invariants. Ticketing owns ticket lifecycle, comments, assignments, and event history. Stable identifiers SHALL be used instead of mutable business attributes as permanent primary keys.

The project SHALL NOT claim TM Forum or 3GPP compliance without defining and verifying applicable standards and scope.

### API Standards

Public APIs SHALL use versioned routes (`/api/v1`). Axum handlers SHALL delegate business operations to domain/application services instead of embedding complex business rules in route handlers.

API contracts SHALL define request/response types, validation, authorization, error semantics, pagination, concurrency, and idempotency where relevant. OpenAPI documentation SHALL be generated and checked against implemented endpoints. Errors SHALL use consistent response structure without disclosing internal stack traces, secrets, or sensitive details.

Cross-service calls SHALL have explicit timeouts and bounded retries where safe. Retries SHALL NOT duplicate business or financial effects.

### Trouble Ticketing

The Ticketing service SHALL own tickets, assignments, comments, status transitions, and ticket event history. An approved status-transition matrix SHALL govern ticket updates.

Important state changes SHALL record actor, timestamp, event type, and relevant details. Internal comments SHALL be protected by authorization. Concurrent updates SHALL NOT silently overwrite newer changes. A ticket represents a reported/investigated issue; it SHALL NOT independently prove network faults or service restoration success.

## Operations & Process

### Observability

Each service SHALL expose health and readiness endpoints. The backend SHALL provide structured logs, request correlation identifiers, metrics, and tracing appropriate to deployment architecture.

Backup, restoration, migration recovery, performance, availability, and retention requirements SHALL be defined before production deployment. CI/CD SHALL run formatting, linting, tests, migration checks, and applicable security checks.

### Spec-Driven Development (SDD)

Every significant feature SHALL follow this sequence:

1. Define problem and business outcome.
2. Write and review feature specification.
3. Record assumptions, constraints, and open questions.
4. Define API contracts and data changes.
5. Prepare implementation plan with testable tasks.
6. Implement small, coherent increment.
7. Run required tests and quality checks.
8. Verify acceptance criteria.
9. Update documentation and traceability.
10. Commit completed increment.

Architecture changes SHALL be recorded in ADRs. Unresolved business rules SHALL be documented rather than silently assumed.

### Definition of Done

A feature is complete only when: specification and acceptance criteria satisfied, ownership boundaries respected, migrations versioned, security/authorization implemented, automated checks passed, observability adequate, documentation updated, and known risks recorded.

Exceptions SHALL document affected rule, rationale, risk, mitigation, approval, and review date.

### Synthetic Data Generator

A Rust-based synthetic-data generator SHALL produce reproducible, configurable datasets using deterministic seeds, dependency-ordered creation, referential validation, and realistic scenarios. Generated financial records SHALL satisfy accounting invariants. Synthetic data SHALL NOT contain real customer personal information or production credentials.

### Documentation Structure

The project SHALL maintain documentation under `docs/` with the following structure:

```text
docs/
├── constitution.md
├── architecture.md
├── build-plan.md
├── domain-model.md
├── definition-of-done.md
├── test-strategy.md
├── traceability-matrix.md
├── assumptions-and-open-questions.md
└── adr/
```

All significant architectural decisions SHALL be recorded in ADRs under `docs/adr/`. The constitution, architecture, and related governance/technical documents SHALL remain synchronized.

## Governance

This constitution becomes binding after project owner approval. Amendments SHALL follow the SDD process with explicit documentation of changes, rationale, and approval.

Versioning follows semantic versioning: MAJOR for backward-incompatible governance/principle removals or redefinitions; MINOR for new principles or materially expanded guidance; PATCH for clarifications and non-semantic refinements.

All implementations SHALL verify compliance with this constitution during code review. Complexity or deviation MUST be justified with documented rationale. Unresolved governance questions SHALL be raised before implementation proceeds.

**Status**: Proposed — pending project owner approval

**Version**: 3.0.1 | **Ratified**: 2026-10-23 | **Last Amended**: 2026-10-23
