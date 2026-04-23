# Stage 1: Install workspace-cache tool
FROM rust:1.94-bookworm AS base
WORKDIR /app
RUN cargo install workspace-cache@0.1.1


# Stage 2: Generate minimal workspace with stub sources
FROM base AS planner
COPY . .
RUN workspace-cache deps --bin auth-server --fast

# Stage 3: Build dependencies only (cached until Cargo.toml/Cargo.lock change)
FROM base AS deps
COPY --from=planner /app/.workspace-cache .
RUN cargo build --release

# Stage 4: Build the actual binary with real source code
FROM deps AS builder
RUN rm -rf auth/auth-api pkg/setup auth/auth-server pkg/common pkg/database pkg/oauth pkg/mock pkg/testutils
COPY auth/auth-api auth/auth-api
COPY pkg/setup pkg/setup
COPY auth/auth-server auth/auth-server
COPY pkg/common pkg/common
COPY pkg/database pkg/database
COPY pkg/oauth pkg/oauth
COPY pkg/mock pkg/mock
COPY pkg/testutils pkg/testutils
RUN cargo clean --release -p auth-api -p setup -p auth-server -p common -p database -p oauth -p mock -p testutils
RUN cargo build --release --bin auth-server

# Stage 5: Minimal runtime image
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/auth-server /usr/local/bin/auth-server
ENTRYPOINT ["/usr/local/bin/auth-server"]