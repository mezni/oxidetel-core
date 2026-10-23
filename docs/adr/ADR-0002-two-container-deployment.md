# ADR-0002: Run six logical services inside one backend container

**Status:** Accepted (subject to review)  
**Date:** 2026-10-23  

## Context
Constitution requires exactly two containers initially.

## Decision
telecom-db (Postgres) and telecom-backend (supervises all 6 service processes).

## Consequences
+ Meets constraint, simple orchestration
- Shared process/failure domain
- Resource limits shared
