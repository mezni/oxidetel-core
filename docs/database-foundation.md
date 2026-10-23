# Database Foundation

## Schemas
- auth (Auth)
- crm (CRM)
- billing (Billing)
- inventory (Inventory)
- trouble_ticket (Ticketing)
- catalog (Catalog)

## Migrations
Versioned SQLx migrations per schema under `migrations/`. Controlled execution (not per-service startup racing all).

## Roles
Least privilege per service, separate migration/runtime credentials. Cross-schema FKs require ADR.

## Testing
PostgreSQL integration tests against disposable DB.
