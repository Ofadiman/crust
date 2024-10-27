include .env

.PHONY:
build:
	docker compose build --no-cache

.PHONY:
up:
	docker compose up --detach

.PHONY:
stop:
	docker compose stop

.PHONY:
down: stop
	docker compose down --volumes --remove-orphans --rmi all

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
	docker compose exec api sqlx migrate run --database-url "postgres://${POSTGRES__USERNAME}:${POSTGRES__PASSWORD}@${POSTGRES__HOST}:${POSTGRES__PORT}/${POSTGRES__DATABASE}"

.PHONY:
migrations_recreate:
	docker compose exec postgres psql -U "${POSTGRES__USERNAME}" -d "${POSTGRES__DATABASE}" -c "drop schema if exists public cascade;"
	docker compose exec postgres psql -U "${POSTGRES__USERNAME}" -d "${POSTGRES__DATABASE}" -c "create schema public;"
	$(MAKE) migrations_run
