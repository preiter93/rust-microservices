# Stage 1: Install workspace-cache tool
FROM rust:1.94-bookworm AS base
WORKDIR /app
RUN cargo install workspace-cache@0.1.0


# Stage 2: Generate minimal workspace with stub sources
FROM base AS planner
COPY . .
RUN workspace-cache deps --bin auth --fast

# Stage 3: Build dependencies only (cached until Cargo.toml/Cargo.lock change)
FROM base AS deps
COPY --from=planner /app/.workspace-cache .
RUN cargo build --release

# Stage 4: Build the actual binary with real source code
FROM deps AS builder
RUN rm -rf auth/src pkg/common/src pkg/database/src pkg/oauth/src pkg/setup/src pkg/mock/src pkg/testutils/src
COPY auth auth
COPY pkg/common pkg/common
COPY pkg/database pkg/database
COPY pkg/oauth pkg/oauth
COPY pkg/setup pkg/setup
COPY pkg/mock pkg/mock
COPY pkg/testutils pkg/testutils
RUN cargo clean --release -p auth -p common -p database -p oauth -p setup -p mock -p testutils
RUN cargo build --release --bin auth

# Stage 5: Minimal runtime image
FROM debian:bookworm-slim AS runtime
COPY --from=builder /app/target/release/auth /usr/local/bin/auth
ENTRYPOINT ["/usr/local/bin/auth"]