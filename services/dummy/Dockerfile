# Stage 1: Install workspace-cache tool
FROM rust:1.94-bookworm AS base
WORKDIR /app
RUN cargo install workspace-cache@0.1.0-alpha.1


# Stage 2: Generate minimal workspace with stub sources
FROM base AS planner
COPY . .
RUN workspace-cache deps --bin dummy --fast

# Stage 3: Build dependencies only (cached until Cargo.toml/Cargo.lock change)
FROM base AS deps
COPY --from=planner /app/.workspace-cache .
RUN cargo build --release

# Stage 4: Build the actual binary with real source code
FROM deps AS builder
RUN rm -rf pkg/common/src pkg/database/src pkg/setup/src pkg/mock/src pkg/testutils/src dummy/src
COPY pkg/common pkg/common
COPY pkg/database pkg/database
COPY pkg/setup pkg/setup
COPY pkg/mock pkg/mock
COPY pkg/testutils pkg/testutils
COPY dummy dummy
RUN cargo clean --release -p common -p database -p setup -p mock -p testutils -p dummy
RUN cargo build --release --bin dummy

# Stage 5: Minimal runtime image
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/dummy /usr/local/bin/dummy
ENTRYPOINT ["/usr/local/bin/dummy"]