# ADR-0002: Two-Container Runtime Architecture

- Status: Accepted
- Date: 2026-10-23

## Context
Constitution requires exactly two containers.

## Decision
telecom-db (Postgres) and telecom-backend (supervises 6 logical services on ports 8001-8006).

## Consequences
+ Meets constraint
- Shared failure/domain isolation
