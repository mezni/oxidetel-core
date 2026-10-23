# ADR-0008: Rust Workspace and Service Boundaries

- Status: Proposed
- Date: 2026-10-23

## Decision
Cargo workspace with services/* and crates/* for shared infrastructure only. No cross-service private imports.
