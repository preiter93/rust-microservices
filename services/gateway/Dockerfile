# Stage 1: Install workspace-cache tool
FROM rust:1.94-bookworm AS base
WORKDIR /app
RUN cargo install workspace-cache@0.1.1


# Stage 2: Generate minimal workspace with stub sources
FROM base AS planner
COPY . .
RUN workspace-cache deps --bin gateway --fast

# Stage 3: Build dependencies only (cached until Cargo.toml/Cargo.lock change)
FROM base AS deps
COPY --from=planner /app/.workspace-cache .
RUN cargo build --release

# Stage 4: Build the actual binary with real source code
FROM deps AS builder
RUN rm -rf auth/auth-api auth/auth-client pkg/setup gateway user/user-api user/user-client
COPY auth/auth-api auth/auth-api
COPY auth/auth-client auth/auth-client
COPY pkg/setup pkg/setup
COPY gateway gateway
COPY user/user-api user/user-api
COPY user/user-client user/user-client
RUN cargo clean --release -p auth-api -p auth-client -p setup -p gateway -p user-api -p user-client
RUN cargo build --release --bin gateway

# Stage 5: Minimal runtime image
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/gateway /usr/local/bin/gateway
ENTRYPOINT ["/usr/local/bin/gateway"]