# Build the rust project, it uses a Debain OS
FROM rust:1.82.0 AS builder

WORKDIR /app

# Cache dependencies work around, copying over all files triggers a complete rebuuild as cargo.toml is new.
# Copying individually allows us only rebuild dependencies if they have changed
COPY Cargo.toml .
COPY Cargo.lock .

# Create a dummy file
RUN mkdir -p ./src/bin && echo 'fn main() { println!("hello"); }' > ./src/main.rs && echo 'fn main() {}' > ./src/bin/blog_util.rs
RUN cargo build --release
RUN rm -rf ./src

COPY . .

# Force update modification times to make cargo rebuild them
RUN touch -a -m ./src/main.rs
RUN touch -a -m ./src/bin/blog_util.rs

RUN cargo build --release

# Deploy the application
FROM ubuntu:24.04

WORKDIR /app
COPY /static /app/static
COPY --from=builder /app/target/release/website .
COPY /posts /app/posts

EXPOSE 8080

ENTRYPOINT [ "./website" ]