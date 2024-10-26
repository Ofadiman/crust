.PHONY:
build:
	docker compose build

.PHONY:
build_no_cache:
	docker compose build --no-cache

.PHONY:
up:
	docker compose up --detach

.PHONY:
stop:
	docker compose stop

.PHONY:
logs:
	docker compose logs api --follow --timestamps

.PHONY:
bash:
	docker compose exec -it api bash

# make migrations_new name=create_users
.PHONY:
migrations_new:
	docker compose exec api sqlx migrate add $(name)

.PHONY:
migrations_run:
	docker compose exec api sqlx migrate run --database-url postgres://user:password@postgres:5432/postgres

.PHONY:
migrations_recreate:
	docker compose exec postgres psql -U user -d postgres -c "drop schema if exists public cascade;"
	docker compose exec postgres psql -U user -d postgres -c "create schema public;"
	$(MAKE) migrations_run
