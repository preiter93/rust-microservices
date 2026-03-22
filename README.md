# rust-svelte-setup

This project explores creating a standard setup for a microservice backend using Rust. The focus is on backend architecture, simple CRUD operations (no event-driven architecture), with an emphasis on simplicity, type safety, and testability.

## Architecture

### Overview

This is a base setup for a Rust-based microservice backend.

### Services

The backend consists of Rust microservices. Client requests always reach the `gateway` service, where they are authenticated and forwarded to the respective microservice. The gateway exposes a RESTful HTTP server (`axum`). Within the backend, communication happens via `gRPC` (`tonic`). Each microservice has its own protobuf file defining its service and models.

#### Microservice Structure

Each microservice focuses on simple CRUD operations and uses a straightforward structure. The architecture decouples the database/repository layer from the service logic. If complexity grows, responsibilities can be further split (e.g., add a dedicated service layer for domain logic).

A typical microservice (see [`dummy`](./services/dummy)) has the following structure:
- `main.rs` – setup (environment variables, database connection) and service startup
- `lib.rs` – exposes service boundaries (such as `proto.rs`) for other microservices
- `handler.rs` – implements gRPC endpoints and service logic
  - Each endpoint typically gets its own file (e.g., `get_entity.rs`)
- `database/` – database/repository layer for CRUD operations
- `proto.rs` – generated code from protobuf definitions (not checked into git)
- `utils.rs` – shared methods between endpoints, models, etc.
- `error.rs` – error types for endpoints and database operations
- `client.rs` – gRPC client implementation + service mocks (auto-generated)

See also: [Master hexagonal architecture in Rust](https://www.howtocodeit.com/articles/master-hexagonal-architecture-rust)

#### Microservice Boundaries (`lib.rs`)

Microservices need access to the API layer of other microservices—specifically the proto-generated client and request/response messages. This can be solved by:
1. Compiling protos in a common `proto` library and including it in each microservice, or
2. Compiling protos as part of each service and exposing them via `lib.rs`

This setup uses the second approach. It avoids introducing a shared `proto` library, and each service can define which parts of the proto to expose. Note: `lib.rs` should only expose what's needed by other services—typically just the full or partial `proto.rs`.

#### Database

This project uses `tokio-postgres` for database access. `sqlx` with compiled SQL statements was tried but caused more problems than it solved. Plain SQL with good unit testing is the way to go. Connection pooling is handled by `deadpool-postgres`.

#### Shared Dependencies (Workspace)

Microservices share many dependencies (tonic, prost, tokio, serde, etc.), which can lead to version drift between services. The solution is to put all microservices in a `workspace` and define shared dependencies as workspace dependencies.

### Deployment

#### Docker Builds with workspace-cache

The `Dockerfile` for each microservice is auto-generated using [`workspace-cache`](https://github.com/preiter93/workspace-cache), a tool built specifically for this purpose. It analyzes workspace dependencies and generates optimal Dockerfiles that include only the microservice itself and its actual dependencies.

This approach uses a two-stage build:
1. Compile all external dependencies (which change rarely)
2. Compile the microservice's actual binary

This separation allows Docker to cache the dependency layer, making rebuilds much faster when only service code changes. Unlike `cargo-chef`, `workspace-cache` is designed specifically for workspaces and generates minimal, optimized Dockerfiles automatically.

All backend microservices are deployed together with Docker Compose.

#### Alternative Docker Strategy

Currently, binaries are built within the Docker build process. For Rust images this can be slow. Significant effort has gone into optimal caching, but if a central dependency changes, it can still be painful.

An alternative is building binaries outside Docker and copying them into a minimal image (e.g., `scratch` or `alpine`). This is arguably more scalable—but there's something elegant about building everything within Docker.

### Authentication

Authentication is hand-rolled using information from [lucia](https://lucia-auth.com/) and implements OAuth login with Google and GitHub.

**⚠️ This is not production-grade security. Do not use this for production apps!**

### Protos

Backend communication uses `gRPC`. Proto files are compiled into both Rust and TypeScript code, allowing the backend to share request/response models with the frontend.

### Routing

**Traefik** serves as a reverse proxy to route requests to the backend or frontend. Setup is straightforward.

### Testing

#### Unit Tests

Unit tests use [`rstest`](https://github.com/la10736/rstest) for table-driven testing, making it easy to cover multiple scenarios.

#### Database Tests

Database tests use [`testcontainers`](https://docs.rs/testcontainers/latest/testcontainers/) to spin up a real Postgres database.

#### Integration Tests

Integration tests also use `testcontainers` to spin up all required services. These tests live in [`services/gateway/tests`](./services/gateway/tests) and verify interactions between microservices in a realistic environment.

### Tracing

**OpenTelemetry** instruments and collects traces. Traces are sent to **Jaeger** by default, but this can be swapped with any OpenTelemetry-compatible backend.

#### Inter-Service Tracing

Traces propagate between microservices:
- **Sending:** Interceptors inject/extract context and add a `trace_id`
- **Receiving:** Middleware picks up the context and records the `trace_id`

#### Further Reading

- [Logging basics](https://heikoseeberger.de/2023-07-29-dist-tracing-1/)
- [Tracing in a single service](https://heikoseeberger.de/2023-08-18-dist-tracing-2/)
- [Inter-service tracing](https://heikoseeberger.de/2023-08-28-dist-tracing-3/)

## How to Run

1. Copy `.env.example` to `.env` and adjust as needed

2. Generate code and Dockerfiles:
   ```sh
   just generate
   ```

3. Build and deploy the backend:
   ```sh
   just build-services
   just deploy
   ```

4. Run the app locally (in the `app` directory):
   ```sh
   npm run dev -- --open
   ```
   
   Or build and deploy the app:
   ```sh
   just build-app
   just deploy-app
   ```

This may not work flawlessly out of the box. There might be manual steps required. Feel free to open an issue if you run into problems.

## How Does It Compare to Go?

**TL;DR:** For large software projects, Go remains a solid choice for the majority of services, but Rust is worth considering for performance-critical parts (see: [How Grab rewrote their counter service in Rust](https://engineering.grab.com/counter-service-how-we-rewrote-it-in-rust)).

### Why Rust

- **Type safety** – In Go it's easy to forget passing values to structs. Who creates explicit constructors for everything?
- **Performance** – Blazingly fast. Does it matter for an app with 1 user? Not really. But it's nice to know Rust is the right tool when it matters.
- **No nil pointer exceptions** – In Go it's too easy to get a nil pointer exception and crash a service. Accessing a nested proto struct without checking the parent for nil? Boom.
- **Compile-time features** – For example, using Rust's features to put shared test utilities in a service. In Go, sharing test utilities without polluting the public API isn't straightforward.
- **Error handling** – Go's verbosity is fine, but Rust's approach feels nicer. With `anyhow` and `thiserror`, the ecosystem is better too.
- **No garbage collection** – One less thing to worry about.

### The Downsides

- **Compile time** – Rebuilding a full service from scratch in Docker on a Mac can take up to 10 minutes. Want to parallelize across 10 microservices? Memory gets killed. Significant effort has gone into caching optimization with `workspace-cache` and auto-generated Dockerfiles, but Go just wins here.
- **Table testing** – A bit cumbersome in Rust. rstest is great, but it's macro-based, which can break formatting in editors.
- **No gRPC gateway** – Surprisingly, Rust doesn't have a good gRPC gateway. Maybe tonic will add one? ([Issue #332](https://github.com/hyperium/tonic/issues/332))
- **HTTP/gRPC middleware** – Writing gRPC middleware in Rust takes time. Much easier in Go, but once the tower way clicks, it's actually fun.
- **Onboarding** – Go onboarding is easy. Rust requires explaining generics, lifetimes, and async traits. What's that `Pin` thing again?
- **Test harness** – Go's test harness is much simpler for pre/post setup. For database tests, spinning up one Postgres container for all tests and destroying it afterward is straightforward in Go. In Rust, this requires workarounds: [testcontainers-rs issue #707](https://github.com/testcontainers/testcontainers-rs/issues/707).

## Where Is It Used?

A backend with a similar setup powers [runaround.world](https://runaround.world), a personal website for tracking running data. Feel free to try it—but it's early stage and only supports Polar and Strava data at the moment.

It works really well. Rust + Postgres delivers the expected performance, and in practice there's no need to optimize beyond writing sane Rust code. Don't worry about a few `.clone()` calls here and there. The type safety Rust provides means issues after compilation are rare. When they do occur, tracing helps track them down quickly.

## Similar Projects

A few similar projects that provided inspiration:

- [rusve](https://github.com/mpiorowski/rusve)
- [rust-microservice-template](https://github.com/nkz-soft/rust-microservice-template)
- [rust-simple-event-driven-microservices](https://github.com/Jamesmallon1/rust-simple-event-driven-microservices)