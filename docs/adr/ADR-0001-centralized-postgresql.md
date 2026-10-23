# ADR-0001: Centralized PostgreSQL with Domain Schemas

- Status: Accepted
- Date: 2026-10-23

## Context
Need six services with clear ownership but simple deployment (two containers).

## Decision
Use one PostgreSQL instance with schemas: auth, crm, billing, inventory, trouble_ticket, catalog. Each service owns its schema/migrations; least privilege; no default cross-schema FKs.

## Consequences
+ Simple ops, clear ownership
- Shared DB resource/failure domain
