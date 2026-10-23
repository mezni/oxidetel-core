# Implementation Plan: Authentication and RBAC (Phase 5)

**Branch**: `006-auth-rbac` | **Date**: 2026-10-23 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/006-auth-rbac/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command; its definition describes the execution workflow.

## Summary

Implement auth service with users, roles, permissions in auth schema, JWT, RBAC.

## Technical Context

**Language/Version**: Rust  
**Primary Dependencies**: Axum, SQLx, jsonwebtoken, bcrypt  
**Storage**: PostgreSQL auth schema  
**Testing**: Auth tests  
**Target Platform**: Linux  
**Project Type**: Service  
**Constraints**: Schema-per-service, no cross-service writes  
**Scale/Scope**: Auth/RBAC

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

[Gates determined based on constitution file]

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
apps/auth-service/
migrations/auth/
crates/api-common/
```

**Structure Decision**: Auth service owns auth schema.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |
