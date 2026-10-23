# Telecom Core Backend — Definition of Done

**File:** `docs/definition-of-done.md`  
**Version:** 1.0  
**Status:** Proposed  
**Stack:** Rust, Axum, Tokio, SQLx, PostgreSQL  
**Methodology:** Spec-Driven Development (SDD)

## 1. Purpose

The Definition of Done (DoD) establishes the minimum quality requirements for every feature, bug fix, database migration, API, and release in the Telecom Core Backend.

A task is not complete merely because its code compiles or its API returns a successful response. It is complete only when the applicable requirements, implementation, automated tests, security controls, documentation, and verification checks are satisfied.

## 2. Scope

Applies to all six logical services (Auth, CRM, Billing, Inventory, Ticketing, Catalog), shared crates, migrations, Docker configuration, integration tests, and documentation. Deployment remains exactly two Docker containers: `telecom-db` and `telecom-backend`.

## 3. Universal Definition of Done

### 3.1 Specification and requirements
- [ ] Business problem and intended outcome documented
- [ ] Feature specification exists under `specs/`
- [ ] Acceptance criteria measurable and testable
- [ ] Confirmed requirements distinguished from assumptions
- [ ] Open questions resolved or explicitly deferred
- [ ] API contracts and data changes documented
- [ ] Dependencies/failure scenarios identified
- [ ] Linked in `docs/traceability-matrix.md`

### 3.2 Architecture and ownership
- [ ] Correct service identified
- [ ] Owning PostgreSQL schema identified
- [ ] Cross-service interactions use approved contracts
- [ ] No direct writes to other service schemas
- [ ] Shared crates are infrastructure only
- [ ] Architecture changes have ADR if needed
- [ ] Two-container constraint preserved

### 3.3 Implementation quality
- [ ] Follows approved specification
- [ ] Request validation implemented
- [ ] Domain invariants enforced
- [ ] Consistent error handling
- [ ] Parameterized queries
- [ ] Transactions for related changes
- [ ] Timeouts/retries/idempotency where applicable
- [ ] No hardcoded secrets
- [ ] Follows project conventions

### 3.4 Automated testing
- [ ] Unit tests for business rules
- [ ] Integration tests for DB where applicable
- [ ] API tests (success + validation failures)
- [ ] Authorization tests (allowed/denied)
- [ ] Boundary conditions tested
- [ ] Error paths/dependency failures tested
- [ ] Regression tests for bug fixes
- [ ] Deterministic tests

### 3.5 Rust quality checks
Must pass:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```
- [ ] No new warnings
- [ ] Dependency changes reviewed
- [ ] `Cargo.lock` updated if needed
- [ ] SQLx query checking follows CI strategy
- [ ] No skipped tests to pass

### 3.6 Security and privacy
- [ ] Auth required where appropriate
- [ ] Authz enforced server-side
- [ ] Roles/scopes respected
- [ ] Input validation/resource limits
- [ ] Sensitive fields protected
- [ ] No passwords/tokens/secrets logged
- [ ] Least privilege DB permissions
- [ ] Security-sensitive actions auditable
- [ ] Data retention/privacy respected

### 3.7 Database and migrations
- [ ] Versioned migration exists
- [ ] Applies to clean test DB
- [ ] Works with existing schema
- [ ] Constraints/indexes/uniqueness appropriate
- [ ] Migration recovery/forward-fix documented
- [ ] Respects schema ownership
- [ ] Separate migration/runtime credentials
- [ ] Invariants tested

### 3.8 Observability
- [ ] Useful error context
- [ ] Structured logging where appropriate
- [ ] Correlation IDs propagated
- [ ] Metrics for important behavior
- [ ] No sensitive data in logs/metric labels
- [ ] Health checks accurate

### 3.9 Documentation
- [ ] API docs updated
- [ ] OpenAPI matches behavior
- [ ] Config changes documented
- [ ] `.env.example` updated if needed
- [ ] Domain/DB docs updated
- [ ] ADRs updated if architecture changed
- [ ] Traceability updated

### 3.10 Git and review
- [ ] Scoped changes only
- [ ] No secrets/build artifacts
- [ ] Clear commit message
- [ ] CI passes
- [ ] Review/approval complete
- [ ] Known limitations/follow-ups documented

## 4. Service-specific requirements

### 4.1 Auth
- [ ] Auth behavior tested; invalid/expired rejected
- [ ] Strict token validation if JWT
- [ ] AGENT/SUPERVISOR enforced
- [ ] Service identities scoped correctly
- [ ] Argon2id for passwords if used
- [ ] Privileged changes audited

### 4.2 CRM
- [ ] Customer IDs/uniqueness enforced
- [ ] Lifecycle transitions validated
- [ ] Access authorized
- [ ] Pagination/filtering tested
- [ ] Audit/retention preserved
- [ ] Cross-service refs follow rules

### 4.3 Catalog
- [ ] Products/offers validated
- [ ] Rate-plan versions preserve history
- [ ] Effective dates enforced
- [ ] Exact money for pricing
- [ ] No rewrite of historical invoices
- [ ] Separate from inventory

### 4.4 Inventory
- [ ] Resource IDs/uniqueness enforced
- [ ] State transitions validated
- [ ] No conflicting active assignments
- [ ] Assignment/release transactional
- [ ] Status history recorded
- [ ] Sensitive SIM/eSIM protected

### 4.5 Billing
- [ ] Exact arithmetic; explicit currency; rounding tested
- [ ] Posted ledger entries immutable
- [ ] Corrections via compensating entries
- [ ] Transactions for related changes
- [ ] Idempotency prevents duplicates
- [ ] Concurrency preserves invariants
- [ ] Allocations within limits
- [ ] Approvals for sensitive adjustments
- [ ] Reconciliation detects discrepancies
- [ ] Changes auditable

### 4.6 Ticketing
- [ ] Required fields validated
- [ ] State machine enforced
- [ ] Assignment permissions enforced
- [ ] Internal comments protected
- [ ] Event history reflects committed changes
- [ ] Invalid transitions rejected without partial updates
- [ ] Search/pagination tested

## 5. API completion criteria
- [ ] Route/method documented
- [ ] Request/response schemas defined
- [ ] Validation rules specified
- [ ] Success/error codes defined
- [ ] AuthN/AuthZ documented
- [ ] Common error format
- [ ] Pagination where needed
- [ ] Idempotency where needed
- [ ] OpenAPI matches
- [ ] Positive/negative tests pass

## 6. Database migrations
- [ ] Versioned and reviewed
- [ ] Applies to clean PostgreSQL DB
- [ ] Works with existing schema state
- [ ] Constraints/indexes verified
- [ ] App queries work
- [ ] Integration tests cover invariants
- [ ] Recovery/forward-fix documented
- [ ] Respects schema ownership
- [ ] Controlled execution

## 7. Cross-service workflows
- [ ] Contracts documented
- [ ] Success/failure states specified
- [ ] AuthN/AuthZ at boundaries
- [ ] Bounded timeouts
- [ ] Safe retries/idempotency
- [ ] No duplicate consequential effects
- [ ] Partial failures recovery documented
- [ ] Correlation IDs
- [ ] Integration tests cover cases
- [ ] No cross-service DB writes to bypass rules

## 8. Release
- [ ] All feature DoDs satisfied
- [ ] fmt/clippy/test pass
- [ ] Integration/migration tests pass
- [ ] Security/dependency checks pass
- [ ] Docker builds pass
- [ ] Two containers configured
- [ ] All 6 APIs healthy
- [ ] Config/secrets safe
- [ ] Backup/restore tested
- [ ] Logs/metrics available
- [ ] Release notes/known limitations
- [ ] Deployment/rollback ready
- [ ] Approvals recorded

## 9. Exceptions
Must document: requirement waived, reason/risk, affected services/workflows, compensating controls, approver/role, expiration/remediation. Security, authz, financial integrity cannot be bypassed without explicit risk review/approval.

## 10. Traceability
Maintain in `docs/traceability-matrix.md`. Requirement without verification method is incomplete.

## 11. Final checklist
- [ ] Requirements satisfied
- [ ] Acceptance criteria pass
- [ ] Service/schema boundaries respected
- [ ] Code/migrations complete
- [ ] Tests pass
- [ ] Security verified
- [ ] Observability sufficient
- [ ] Docs/traceability updated
- [ ] CI passes
- [ ] Ready for review/merge

**Final rule:** DONE only when behavior implemented, verified, documented, and consistent with architecture.
