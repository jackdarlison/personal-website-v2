# Build the rust project, it uses a Debain OS
FROM rust:1.82.0 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Deploy the application
FROM ubuntu:24.04

WORKDIR /app
COPY /static /app/static
COPY --from=builder /app/target/release/website .

EXPOSE 8080

ENTRYPOINT [ "./website" ]