# ADR-0001: Modular Monolith over Microservices

## Status
Accepted

## Date
2026-10-17

## Context

OxideTel BSS must handle the classic telecom backend workload: subscriber onboarding, catalog/pricing, ordering, CDR rating, invoicing, payment settlement, and collections (dunning). Early design questions:

- These domains are naturally distinct and map cleanly to separate teams/owners — a strong argument for **microservices**.
- However, nearly every operation crosses domain lines: placing an order touches catalog pricing and customer status; a payment touches billing, dunning, and catalog simultaneously; rating reads catalog pricing on every batch.
- The team is small, deployment capacity is limited, and the product is pre-scale. A distributed system would introduce network partitions, distributed transactions, and service sprawl before any of them bought real value.
- The core requirement is **domain isolation** — the ability to reason about (and eventually split out) each context — not runtime separation *today*.

## Decision

Build a **modular monolith**: a single Rust binary (`oxidetel-bss`) organized as a Cargo workspace, where each bounded context is its own crate (`iam`, `crm`, `catalog`, `ordering`, `billing`, `dunning`, plus shared `domain-core`).

- Contexts communicate through explicit crate-level contracts, not by reaching into each other's internals.
- Domain data is isolated in separate PostgreSQL schemas (see [ADR-0002](0002-postgresql-multi-schema-isolation.md)) so a later split into services is a schema-to-service move, not a data-rewriting exercise.
- Background work (dunning sweep, CDR mediation) runs as in-process Tokio tasks rather than separate services.

## Alternatives Considered

### Microservices (one service per context)
- Pros: Independent deploy/scale, failure isolation, team autonomy, tech-stack freedom per service.
- Cons: Every cross-context call becomes a network hop; a payment settlement would need a saga spanning billing/dunning/catalog services; requires message broker, service discovery, distributed tracing, and 4× the deployment surface from day one.
- Rejected: At current scale the operational cost dwarfs the benefit, and the transactional workflows (payment → invoice → dunning → unbar) are far simpler inside one database transaction.

### Modular monolith with a single shared schema
- Pros: Simplest possible data model; no schema-per-context bookkeeping.
- Cons: Nothing enforces boundaries — any crate can join any table, and the "modular" part degrades into a big ball of mud within months.
- Rejected: The whole point of modularity is *enforced* boundaries. Schema separation provides a hard, database-level guardrail that linting alone cannot.

### Django/Rails-style full-stack framework
- Pros: Fast CRUD, batteries included.
- Cons: We need tight control over async I/O, CDR stream throughput, and precise money arithmetic; the team is Rust-first.
- Rejected: Mismatch with performance profile and team expertise.

## Consequences

**Positive**
- One `cargo build`, one Docker image, one deployment — CI runs the whole workspace in a single job.
- Cross-context transactions (payment webhook: insert payment → mark invoice paid → resolve dunning → restore subscription) are a single PostgreSQL transaction with no distributed-commit risk.
- Compile-time coupling across crates surfaces boundary violations at build time.
- Each crate can still be extracted into a standalone service later, because its data already lives in its own schema.

**Negative**
- No independent scaling: a rating-heavy workload scales the whole binary. Mitigated by running workers in the same process cheaply; revisit if CDR volume demands it.
- A panic or resource leak in one context affects the whole process. Mitigated by Tokio task isolation and connection-pool limits.
- Database is a single point of failure — there is no service-level redundancy of the datastore itself.

**Risks / follow-ups**
- Modular discipline is only as good as the boundary rules in [domain-bounded-contexts.md](../domain-bounded-contexts.md); enforce them in review.
- The dunning worker currently writes `crm.customers.status` directly — cross-context writes should eventually go through a domain event rather than raw SQL.
