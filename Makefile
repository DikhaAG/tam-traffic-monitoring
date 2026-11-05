.PHONY: help build test run-infra stop-infra clean

help:
	@echo "TAM Traffic & PM2 Server Fleet Monitoring Commands:"
	@echo "  make run-infra    - Start PostgreSQL & Redis via Docker Compose"
	@echo "  make stop-infra   - Stop PostgreSQL & Redis"
	@echo "  make build        - Compile all Cargo workspace crates"
	@echo "  make test         - Run all unit and integration tests across workspace"
	@echo "  make clean        - Clean Cargo artifacts"

run-infra:
	docker compose up -d

stop-infra:
	docker compose down

build:
	cargo build --workspace

test:
	cargo test --workspace

clean:
	cargo clean

