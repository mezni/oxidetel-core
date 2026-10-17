.PHONY: all up down run clean db-create format lint test tests

# Default target
all: run

# Start PostgreSQL database container
up:
	docker compose up -d
	@echo "⏳ Waiting for PostgreSQL to be ready..."
	@sleep 3

# Stop PostgreSQL database container
down:
	docker compose down

# Create database if it doesn't exist yet
db-create:
	@PGPASSWORD=postgres createdb -h localhost -U postgres oxidetel || true

# Run the OxideTel BSS server
run: up db-create
	cargo run

# Clean build artifacts
clean:
	cargo clean

# Format code across the entire workspace using rustfmt
format:
	cargo fmt --all

# Lint code using clippy (treats warnings as errors for high code quality)
lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run all unit and integration tests across the workspace
test: up db-create
	cargo test --workspace

# Alias target for running tests
tests: test
