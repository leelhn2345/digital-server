down:
	docker compose --profile prod down

dev:
	docker compose --profile dev up --build

up:
	docker compose --profile prod up -d

prep:
	cargo sqlx prepare --workspace
