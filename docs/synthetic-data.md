# Synthetic Data Strategy

**Version:** 1.0  
**Status:** Draft  
**Project:** Telecom Core Backend  
**Stack:** Rust, SQLx, PostgreSQL  
**Schemas:** `auth`, `crm`, `billing`, `inventory`, `trouble_ticket`, `catalog`

---

## 1. Purpose

Define a repeatable process for generating synthetic telecom data for:

- Local development.
- Database migration testing.
- API integration testing.
- Cross-service workflow testing.
- Billing and ledger integrity testing.
- Performance testing.
- Demonstrations and development environments.

Synthetic data must be reproducible, internally consistent, and safe to use without exposing real customer information.

## 2. Design Principles

1. **Deterministic:** the same seed and configuration produce the same logical dataset.
2. **Referentially consistent:** relationships between generated records use valid identifiers.
3. **Domain-valid:** generated records follow the approved business rules and lifecycle states.
4. **Isolated:** test scenarios can be created and cleaned up without affecting unrelated data.
5. **Privacy-safe:** all customer information and identifiers are synthetic.
6. **Configurable:** record counts and scenario distributions are controlled by configuration.
7. **Auditable:** financial records and state transitions have consistent histories.
8. **Service-owned:** data is inserted through the owning service or an approved seed/migration mechanism, not by unrestricted cross-schema writes.

## 3. Dataset Categories

Maintain three dataset categories.

| Category | Purpose | Characteristics |
|---|---|---|
| Minimal | Fast unit and API tests | A few records covering the normal path |
| Standard | Development and integration testing | Multiple customers, offers, resources, invoices, payments, and tickets |
| Stress | Load and performance testing | Configurable high-volume data with controlled distributions |

Use the minimal dataset by default for automated tests. Do not generate a large dataset during every unit-test run.

## 4. Schema Coverage

### 4.1 Auth — `auth`

Generate:

- Users.
- Roles.
- Permissions.
- User-role assignments.
- Role-permission assignments.
- Service identities, if included in the approved model.

Required scenarios:

- An active agent.
- An active supervisor.
- A disabled user.
- A user with restricted permissions.
- A service identity with a limited scope.

Use fake credentials and approved password-hashing functions. Never store production credentials or reusable real secrets in fixtures.

### 4.2 CRM — `crm`

Generate:

- Customers.
- Customer addresses.
- Synthetic customer identifiers.
- Approved customer status values.

Required scenarios:

- Active customer.
- Inactive customer.
- Customer with multiple addresses.
- Customer with missing optional information.
- Invalid input fixtures for validation tests.

Use reserved or clearly synthetic email domains, such as `example.com`, and fictitious contact information.

### 4.3 Catalog — `catalog`

Generate:

- Products.
- Product offers.
- Rate plans.
- Rate-plan versions.
- Add-ons.
- Offer components.

Include:

- Active and inactive offers.
- Current and historical rate-plan versions.
- Offers with compatible add-ons.
- Invalid or incompatible combinations for negative tests.
- Effective-date boundaries.

Pricing, currency, and offer eligibility must follow the approved Catalog specification.

### 4.4 Inventory — `inventory`

Generate:

- Devices.
- SIM cards.
- eSIM profiles.
- Resource assignments.
- Resource status history.

Include:

- Available resources.
- Assigned resources.
- Suspended resources.
- Retired resources.
- Resources with assignment history.
- Duplicate-assignment attempts for negative tests.

Use fictitious identifiers rather than real subscriber credentials, device identities, or operational network credentials.

### 4.5 Billing — `billing`

Generate:

- Billing accounts.
- Invoices.
- Invoice line items.
- Payments.
- Payment allocations.
- Credit memos.
- Ledger entries.
- Reconciliation records.

Include scenarios for:

- Unpaid invoice.
- Partially paid invoice.
- Fully paid invoice.
- Overdue invoice.
- Approved credit memo.
- Reversed or compensated transaction.
- Duplicate idempotency request.
- Reconciliation discrepancy.

Financial amounts must use the approved exact monetary representation and rounding policy.

Never mark a transaction as posted unless the corresponding ledger and balance invariants are satisfied.

### 4.6 Ticketing — `trouble_ticket`

Generate:

- Tickets.
- Ticket comments.
- Ticket events.

Include:

- Open ticket.
- Assigned ticket.
- In-progress ticket.
- Pending-customer ticket.
- Resolved ticket.
- Closed ticket.
- Internal and customer-visible comments.
- Valid transition history.
- Invalid transition attempts for negative tests.

Ticket categories, priorities, statuses, and assignment rules must match the approved Ticketing specification.

## 5. Cross-Domain Scenarios

The generator must support complete scenarios rather than creating unrelated records independently.

### Scenario A: New customer

1. Create a synthetic customer in CRM.
2. Create or select an eligible offer in Catalog.
3. Create an available SIM in Inventory.
4. Establish the approved subscription or order reference once ownership is decided.
5. Create the required billing account through Billing.
6. Verify the relationships using documented service interfaces.

### Scenario B: Monthly billing

1. Select an eligible commercial plan.
2. Generate an invoice with line items.
3. Calculate the invoice total using approved rules.
4. Create a payment.
5. Allocate the payment.
6. Verify the resulting invoice state and financial records.

### Scenario C: Support ticket

1. Select a synthetic customer.
2. Create a ticket referencing an approved customer identifier.
3. Assign the ticket.
4. Add comments and events.
5. Transition the ticket through valid states.
6. Verify authorization and event history.

### Scenario D: Inventory conflict

1. Create one available SIM.
2. Submit two competing assignment operations.
3. Verify that only the permitted assignment succeeds.
4. Verify that resource history remains consistent.

### Scenario E: Financial idempotency

1. Submit a financial operation with an idempotency key.
2. Repeat the same request.
3. Verify that duplicate financial effects are not created.
2. Submit a conflicting payload with the same key.
3. Verify the documented conflict response.

## 6. Generator Configuration

Use a configuration file to define dataset size and reproducibility.

Example `config/synthetic-data.toml`:

```toml
seed = 42
profile = "standard"

[counts]
customers = 100
customer_addresses_per_customer = 1
products = 10
product_offers = 20
rate_plans = 8
addons = 12
devices = 50
sim_cards = 100
esim_profiles = 30
billing_accounts = 100
invoices = 200
tickets = 150
```

These counts are illustrative, not business requirements. Final counts and distributions should be adjusted to the test environment and approved domain rules.

## 7. Rust Implementation Structure

Proposed directory layout:

```text
services/
  synthetic-data/
    Cargo.toml
    src/
      main.rs
      config.rs
      generator.rs
      ids.rs
      scenarios/
        mod.rs
        customer.rs
        catalog.rs
        inventory.rs
        billing.rs
        ticketing.rs
    tests/
      determinism.rs
      referential_integrity.rs
      billing_invariants.rs
      scenario_generation.rs

config/
  synthetic-data.toml
```

Use:

- `serde` and a TOML parser for configuration.
- A seeded random-number generator for reproducibility.
- `uuid` for generated identifiers where appropriate.
- `rust_decimal` or integer minor units for monetary values.
- SQLx for database access.
- Domain validation functions from the owning service or an approved shared domain crate.

The generator must not bypass service ownership by writing arbitrarily into all six schemas.

For cross-service scenarios, use the public APIs or an explicitly approved test-fixture mechanism with appropriate database privileges.

## 8. Generator Execution Modes

Support distinct execution modes:

```text
synthetic-data validate-config
synthetic-data generate --profile minimal
synthetic-data generate --profile standard
synthetic-data generate --profile stress
synthetic-data verify
```

These are proposed CLI commands; they are not implemented yet.

Expected behavior:

- `validate-config`: validate configuration without changing data.
- `generate`: create the requested synthetic dataset.
- `verify`: check referential consistency and approved domain invariants.
- Stress generation: require an explicit target environment and record count limits.

Database write operations must require explicit configuration. Never default the generator to a production database.

## 10. Data Integrity Requirements

The generator must verify:

- Every generated foreign key points to a valid record where the schema defines that relationship.
- Cross-service references follow approved API contracts.
- No duplicate active inventory assignments violate domain rules.
- Invoice totals match their line items and adjustments.
- Payment allocations respect the approved balance rules.
- Posted ledger transactions satisfy the selected accounting model.
- Ticket event history matches generated state transitions.
- Re-running a generation profile follows the documented reset or idempotency policy.

If the database uses separate schemas without cross-schema foreign keys, the generator must verify approved cross-domain relationships at the application or integration-test layer.

## 11. Test Requirements

| ID | Requirement | Acceptance criteria |
|---|---|---|
| SYN-001 | Deterministic generation | The same seed and configuration produce the same logical dataset |
| SYN-002 | Configurable volume | Each profile generates the configured record counts |
| SYN-003 | Referential consistency | Generated references satisfy the approved relationships |
| SYN-004 | Privacy-safe data | No real customer records or production secrets are used |
| SYN-005 | Financial validity | Generated invoices, allocations, and ledger records satisfy approved invariants |
| SYN-006 | Inventory validity | Generated assignments respect resource lifecycle and uniqueness rules |
| SYN-007 | Ticket history validity | Generated transitions follow the ticket state machine |
| SYN-008 | Safe execution | Production targets are rejected by default |
| SYN-009 | Repeatable cleanup | Test data can be isolated and removed using the documented procedure |
| SYN-010 | Cross-service consistency | Generated scenarios satisfy approved service contracts |

## 12. Implementation Plan

1. Define the generator configuration and seed strategy.
2. Implement deterministic identifier generation.
3. Implement minimal Auth and CRM fixtures.
4. Add Catalog data and commercial relationships.
5. Add Inventory resources and assignment scenarios.
6. Add Billing scenarios with financial invariants.
7. Add Ticketing scenarios and event histories.
8. Add cross-service workflows.
9. Implement dataset verification.
10. Add CI tests and document safe execution.

## 13. Definition of Done

The synthetic-data generator is complete when:

- Configuration is validated before database writes.
- Generation is deterministic for the same seed and configuration.
- All six domains are covered.
- Cross-domain relationships follow approved contracts.
- Financial and inventory invariants are verified.
- Invalid scenarios are available for negative testing without corrupting valid baseline data.
- Tests pass against a disposable PostgreSQL database.
- The generator cannot accidentally target production under its default configuration.
- Usage and cleanup procedures are documented.

## 14. Related Documents

- `docs/architecture.md`
- `docs/domain-model.md`
- `docs/test-strategy.md`
- `docs/traceability-matrix.md`
- `docs/assumptions-and-open-questions.md`
- `docs/adr/`
- Service specifications under `specs/`

## 15. Git Commit

```bash
git add docs/synthetic-data.md
git commit -m "docs(testing): define synthetic telecom data strategy"
```