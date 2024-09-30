set dotenv-load

# Start dev server
dev:
    @cargo watch -q -i cli -i '*.md' -- just _run

# Internal helper to allow 'just dev' to reload .env changes
_run:
    @source {{source_directory()}}/.env && \
    cargo run --package melange-api

webdev:
    cd web && pnpm dev


docker-api:
   docker build --tag melange-api .

docker-web:
   docker build --tag melange-web web

docker-spice:
   docker build -f Dockerfile.spice --tag melange-spice .

