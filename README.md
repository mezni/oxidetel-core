# Telecom Core Backend

Telecom Core Backend per constitution v3.0.1. Two-container deployment (telecom-db + telecom-backend) with six Rust services, schema-per-service PostgreSQL, SDD workflow.

## Architecture
- Six services: auth, crm, billing, inventory, ticketing, catalog (ports 8001-8006)
- Six schemas: auth, crm, billing, inventory, trouble_ticket, catalog
- No cross-service DB writes by default
- Rust + Axum + Tokio + SQLx

## Quick Start
```bash
docker-compose up --build
```

## Docs
See `docs/` for architecture, constitution, build plan, and ADRs.
