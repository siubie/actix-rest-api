.PHONY: help build up down logs restart clean test dev prod

# Default target
help:
	@echo "Available commands:"
	@echo "  make dev          - Start development environment"
	@echo "  make prod         - Start production environment"
	@echo "  make build        - Build Docker images"
	@echo "  make up           - Start services in detached mode"
	@echo "  make down         - Stop services"
	@echo "  make logs         - View logs (follow mode)"
	@echo "  make restart      - Restart services"
	@echo "  make clean        - Stop and remove volumes"
	@echo "  make test         - Run tests"
	@echo "  make db-shell     - Access MySQL shell"
	@echo "  make app-shell    - Access application container"
	@echo "  make migrate      - Run database migrations"

# Development commands
dev:
	docker-compose up -d
	@echo "Development environment started!"
	@echo "API: http://localhost:8080"
	@echo "Swagger UI: http://localhost:8080/swagger-ui/"

dev-logs:
	docker-compose logs -f app

# Production commands
prod:
	docker-compose -f docker-compose.prod.yml --env-file .env.prod up -d
	@echo "Production environment started!"

prod-logs:
	docker-compose -f docker-compose.prod.yml logs -f

# General Docker Compose commands
build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

logs:
	docker-compose logs -f

restart:
	docker-compose restart

clean:
	docker-compose down -v
	@echo "Containers and volumes removed!"

# Database commands
db-shell:
	docker-compose exec db mysql -u$${DB_USER:-rustuser} -p$${DB_PASSWORD:-devpassword} $${DB_NAME:-rust}

migrate:
	docker-compose exec db mysql -uroot -p$${DB_ROOT_PASSWORD:-rootpassword} $${DB_NAME:-rust} < migrations/01_create_users_table.sql

# Application commands
app-shell:
	docker-compose exec app sh

# Testing
test:
	cargo test

# Local development (without Docker)
run-local:
	cargo run

check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy

# Rebuild and restart
rebuild:
	docker-compose up -d --build
	@echo "Services rebuilt and restarted!"

# Show container status
ps:
	docker-compose ps
