FROM rust:1.80-slim as builder
WORKDIR /src
# Copy manifest first for cached dependency build
COPY Cargo.toml ./
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release || true

# Now copy the real source and build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /src/target/release/worker /app/worker
CMD ["/app/worker"]
