# Research: PostgreSQL Foundation (Phase 3)

## Decisions
- **Schemas**: auth, crm, billing, inventory, trouble_ticket, catalog (per constitution)
- **Migrations**: SQLx migrations, versioned, per-schema directories
- **Execution**: Controlled migration process (not per-service startup racing)
- **Permissions**: Least privilege roles per service, separate migration/runtime credentials
- **Cross-schema FKs**: Prohibited by default, require ADR
- **Testing**: PostgreSQL integration tests against disposable DB

## Resolved
All clarifications resolved per constitution/architecture.
