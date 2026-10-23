# Data Model: Rust Workspace and Tooling (Phase 2)

**Feature**: 002-rust-workspace-and-tooling  
**Date**: 2026-10-23

This phase does not introduce domain entities or database schemas. It establishes workspace structure only.

## Entities (conceptual)
- **Workspace**: Root configuration defining members and shared dependencies
- **ServiceCrate**: Binary crate under `apps/` for each logical service
- **SharedCrate**: Library crate under `crates/` for infrastructure

No persistence model changes in this phase.
