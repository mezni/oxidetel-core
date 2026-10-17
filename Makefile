.PHONY: up down run test clean

up:
	docker compose up -d
	@echo "⏳ Waiting for PostgreSQL to be ready..."
	@sleep 3

down:
	docker compose down

run: up
	cargo run

test: up
	cargo test --workspace
	cargo test --test customer_lifecycle_test -- --nocapture

clean:
	cargo clean
	docker compose down -v
