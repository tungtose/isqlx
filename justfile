dev:
  cargo watch -x 'run'

export DATABASE_URL := "sqlite:test.db"

create-db:
  sqlx db create

create-migrate name:
  sqlx migrate add {{name}}

migrate:
  sqlx migrate run

init:
  just create-db
  just migrate

clean:
  rm -rf *.db*
