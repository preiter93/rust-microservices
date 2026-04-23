# Stage 1: Install workspace-cache tool
FROM rust:1.94-bookworm AS base
WORKDIR /app
RUN cargo install workspace-cache@0.1.1


# Stage 2: Generate minimal workspace with stub sources
FROM base AS planner
COPY . .
RUN workspace-cache deps --bin dummy-server --fast

# Stage 3: Build dependencies only (cached until Cargo.toml/Cargo.lock change)
FROM base AS deps
COPY --from=planner /app/.workspace-cache .
RUN cargo build --release

# Stage 4: Build the actual binary with real source code
FROM deps AS builder
RUN rm -rf pkg/setup pkg/common pkg/database pkg/mock pkg/testutils dummy/dummy-api dummy/dummy-server
COPY pkg/setup pkg/setup
COPY pkg/common pkg/common
COPY pkg/database pkg/database
COPY pkg/mock pkg/mock
COPY pkg/testutils pkg/testutils
COPY dummy/dummy-api dummy/dummy-api
COPY dummy/dummy-server dummy/dummy-server
RUN cargo clean --release -p setup -p common -p database -p mock -p testutils -p dummy-api -p dummy-server
RUN cargo build --release --bin dummy-server

# Stage 5: Minimal runtime image
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/dummy-server /usr/local/bin/dummy-server
ENTRYPOINT ["/usr/local/bin/dummy-server"]