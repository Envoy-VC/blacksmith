# ----------- Base stage (cargo-chef) -----------
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# ----------- Planner stage -----------
FROM chef AS planner

COPY Cargo.toml Cargo.lock ./
COPY crates/uniswap-deployer/ crates/uniswap-deployer/
RUN cargo chef prepare --recipe-path recipe.json

# ----------- Cacher stage -----------
FROM chef AS cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ----------- Builder stage -----------
FROM chef AS builder

# Copy Application
COPY Cargo.toml Cargo.lock ./
COPY crates/uniswap-deployer/src crates/uniswap-deployer/src
COPY crates/uniswap-deployer/Cargo.toml crates/uniswap-deployer/

COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Build the uniswap-deployer binary
RUN cargo build --release --package uniswap_deployer --bin uniswap_deployer && strip target/release/uniswap_deployer

# ----------- Runtime stage -----------
FROM gcr.io/distroless/cc:nonroot AS runtime
WORKDIR /app

# Copy the CA certificates
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copy only the built binary
COPY --from=builder /app/target/release/uniswap_deployer /usr/local/bin/uniswap_deployer

# Run the app
ENTRYPOINT ["/usr/local/bin/uniswap_deployer"]
    