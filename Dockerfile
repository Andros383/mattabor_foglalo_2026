# Stage 1: Build Frontend (WASM)
FROM rust:1.98-slim AS frontend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
COPY . .
WORKDIR /app/crates/frontend
RUN trunk build --release

# Stage 2: Build Backend
FROM rust:1.98-slim AS backend-builder
WORKDIR /app
COPY . .
COPY --from=frontend-builder /app/crates/frontend/dist /app/dist
RUN cargo build --release -p backend

# Stage 3: Minimal Runtime
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=backend-builder /app/target/release/backend /app/backend
COPY --from=frontend-builder /app/crates/frontend/dist /app/dist

ENV RUST_LOG=info
EXPOSE 3000
CMD ["./backend"]
