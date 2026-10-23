# Telecom Core Backend — Architecture

**File:** `docs/architecture.md`  
**Version:** 3.0  
**Status:** Proposed  
**Technology stack:** Rust, Axum, Tokio, SQLx, PostgreSQL  
**Architecture style:** Modular backend with logically independent services and schema-based data ownership

---

## 1. Purpose

This document defines the target architecture for the Telecom Core Backend, an enterprise-style telecom Business Support System (BSS) and Operations Support System (OSS) backend.

The system provides APIs for authentication, customer management, billing, commercial catalog, resource inventory, and trouble-ticket management.

The architecture must support:

- Clear service and domain boundaries.
- Secure, role-based access control.
- Reliable financial transactions and auditability.
- Independent database schema ownership.
- Automated testing and repeatable deployments.
- Future migration to independently deployed services if required.

This is a backend-only system. User interfaces and AI components are outside the scope of this architecture.

## 2. Architectural principles

1. **Two containers only:** one PostgreSQL container and one backend container.
2. **Logical service separation:** six services retain independent routes, domain logic, API contracts, database access, and tests.
3. **Schema ownership:** each service owns its PostgreSQL schema and controls its migrations.
4. **No cross-service writes:** a service must not directly modify another service's tables.
5. **API-based integration:** cross-service operations use documented APIs or explicitly approved integration mechanisms.
6. **Security by default:** enforce authentication, authorization, least privilege, and audit logging.
7. **Financial correctness:** use exact monetary representations, idempotency, immutable posted ledger entries, and compensating entries.
8. **Specification-driven development:** features require specifications, acceptance criteria, tests, and traceability.
9. **Observable execution:** use structured logs, metrics, and distributed tracing where applicable.
10. **No unsupported compliance claims:** telecom standards compliance must be established through explicit requirements and verification.

## 3. High-level architecture

```mermaid
flowchart TB
    Client["API Client / External System"]

    subgraph Backend["Container 2: telecom-backend"]
        Auth["auth-service :8001"]
        CRM["crm-service :8002"]
        Billing["billing-service :8003"]
        Inventory["inventory-service :8004"]
        Ticketing["ticketing-service :8005"]
        Catalog["catalog-service :8006"]

        Common["Shared Rust Crates<br/>Configuration · Errors · Observability"]

        Auth --- Common
        CRM --- Common
        Billing --- Common
        Inventory --- Common
        Ticketing --- Common
        Catalog --- Common
    end

    subgraph Database["Container 1: telecom-db"]
        PG[("PostgreSQL")]
        S1[("auth")]
        S2[("crm")]
        S3[("billing")]
        S4[("inventory")]
        S5[("trouble_ticket")]
        S6[("catalog")]

        PG --- S1
        PG --- S2
        PG --- S3
        PG --- S4
        PG --- S5
        PG --- S6
    end

    Client --> Auth
    Client --> CRM
    Client --> Billing
    Client --> Inventory
    Client --> Ticketing
    Client --> Catalog

    Auth --> PG
    CRM --> PG
    Billing --> PG
    Inventory --> PG
    Ticketing --> PG
    Catalog --> PG

    classDef service fill:#DCEBFA,stroke:#245A85,color:#132B42
    classDef infrastructure fill:#E5F3E5,stroke:#357A38,color:#173B1A
    classDef database fill:#FCE8D5,stroke:#B56B24,color:#55300D

    class Auth,CRM,Billing,Inventory,Ticketing,Catalog service
    class Common infrastructure
    class PG,S1,S2,S3,S4,S5,S6 database
```

**Deployment clarification:** the six service boxes represent logical services, not six Docker containers. The backend container hosts all six service processes. The PostgreSQL container hosts the database and its six schemas.

The shared Rust crates provide reusable infrastructure; they must not merge the services' business responsibilities.
