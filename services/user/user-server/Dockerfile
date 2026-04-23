# Stage 1: Install workspace-cache tool
FROM rust:1.94-bookworm AS base
WORKDIR /app
RUN cargo install workspace-cache@0.1.1


# Stage 2: Generate minimal workspace with stub sources
FROM base AS planner
COPY . .
RUN workspace-cache deps --bin user-server --fast

# Stage 3: Build dependencies only (cached until Cargo.toml/Cargo.lock change)
FROM base AS deps
COPY --from=planner /app/.workspace-cache .
RUN cargo build --release

# Stage 4: Build the actual binary with real source code
FROM deps AS builder
RUN rm -rf pkg/setup pkg/common pkg/database pkg/mock pkg/testutils user/user-api user/user-server
COPY pkg/setup pkg/setup
COPY pkg/common pkg/common
COPY pkg/database pkg/database
COPY pkg/mock pkg/mock
COPY pkg/testutils pkg/testutils
COPY user/user-api user/user-api
COPY user/user-server user/user-server
RUN cargo clean --release -p setup -p common -p database -p mock -p testutils -p user-api -p user-server
RUN cargo build --release --bin user-server

# Stage 5: Minimal runtime image
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/user-server /usr/local/bin/user-server
ENTRYPOINT ["/usr/local/bin/user-server"]