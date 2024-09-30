# Stage 1: Build
FROM rust:latest AS builder
WORKDIR /usr/src/melange-api
COPY . .
RUN cargo build --release -p melange-api

# Stage 2: Runtime
FROM gcr.io/distroless/cc-debian12

WORKDIR /app
COPY --from=builder /usr/src/melange-api/target/release/melange-api .
EXPOSE 8080
CMD ["./melange-api"]

