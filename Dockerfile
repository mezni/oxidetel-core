# --- Stage 1: Build Stage ---
FROM rust:latest AS builder

WORKDIR /app

# Install system dependencies required for compilation
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests
COPY Cargo.toml Cargo.lock ./
COPY crates/domain-core/Cargo.toml crates/domain-core/
COPY crates/iam/Cargo.toml crates/iam/
COPY crates/crm/Cargo.toml crates/crm/
COPY crates/catalog/Cargo.toml crates/catalog/
COPY crates/ordering/Cargo.toml crates/ordering/
COPY crates/generator/Cargo.toml crates/generator/
COPY crates/billing/Cargo.toml crates/billing/
COPY crates/dunning/Cargo.toml crates/dunning/

# Create dummy source files to cache dependencies
RUN mkdir -p src \
    crates/domain-core/src \
    crates/iam/src \
    crates/crm/src \
    crates/catalog/src \
    crates/ordering/src \
    crates/generator/src \
    crates/billing/src \
    crates/dunning/src

RUN echo "fn main() {}" > src/main.rs
RUN echo "" > crates/domain-core/src/lib.rs
RUN echo "" > crates/iam/src/lib.rs
RUN echo "" > crates/crm/src/lib.rs
RUN echo "" > crates/catalog/src/lib.rs
RUN echo "" > crates/ordering/src/lib.rs
RUN echo "" > crates/generator/src/lib.rs
RUN echo "" > crates/billing/src/lib.rs
RUN echo "" > crates/dunning/src/lib.rs

RUN cargo build --release

# Copy actual source code and migrations
COPY src/ src/
COPY crates/ crates/
COPY migrations/ migrations/

# Build release binary with real source code
RUN touch src/main.rs && cargo build --release

# --- Stage 2: Runtime Stage ---
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binary from builder stage
COPY --from=builder /app/target/release/oxidetel-bss /app/oxidetel-bss

# Copy migrations folder so the app can run auto-migrations on startup
COPY --from=builder /app/migrations /app/migrations

EXPOSE 8080

CMD ["/app/oxidetel-bss"]
