.PHONY: up down logs rebuild test migrate makemigrations shell fmt

up:
	docker compose up -d --build

down:
	docker compose down -v

logs:
	docker compose logs -f --tail=150

rebuild:
	docker compose build --no-cache

test:
	docker compose run --rm test-runner

migrate:
	docker compose run --rm api python manage.py migrate

makemigrations:
	docker compose run --rm api python manage.py makemigrations

shell:
	docker compose run --rm api python manage.py shell

fmt:
	docker compose run --rm worker cargo fmt
