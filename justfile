create-db:
    sqlx database create

drop-db:
    echo "y" | sqlx database drop

# re-create the database and run pending migrations
reset-db:
    echo "y" | sqlx database reset

# Run database migrations
migrate:
    sqlx migrate run

# create the database and run migrations
setup-db:
    sqlx database setup

run:
    #cargo watch -q -c -w src/ -x run
    cargo watch -q -c -w src/ -w templates/ -x run

test:
    cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

generate-css:
    # pnpm dlx tailwindcss -i styles/tailwind.css -o public/css/main.css --watch
    bunx tailwindcss -i styles/tailwind.css -o public/css/main.css --watch
