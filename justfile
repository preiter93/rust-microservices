set dotenv-load := true

default:
    @just --list

# Build all backend services
[group: "build"]
build-services:
  docker compose -f services/docker-compose.yml build

# Builds a single backend service
[group: "build"]
build-service target:
  docker compose -f services/docker-compose.yml build {{ target }}

# Build backend services synchronously
[group: "build"]
build-services-sync:
  docker compose -f services/docker-compose.yml build gateway
  docker compose -f services/docker-compose.yml build user
  docker compose -f services/docker-compose.yml build auth

# Build the app
[group: "build"]
build-app:
  docker compose -f app/docker-compose.yml build

# Deploy infrastructure (network, DB, Traefik, Jaeger)
[group: "deploy"]
deploy-infrastructure:
  docker network create shared_network 2>/dev/null || true
  echo "Starting DB..."
  docker compose -f infrastructure/db/docker-compose.yml up -d
  echo "Starting Traefik..."
  docker compose -f infrastructure/traefik/docker-compose.yml up -d
  echo "Starting Jaeger..."
  docker compose -f infrastructure/jaeger/docker-compose.yml up -d
  echo "Infrastructure deployed!"

# Undeploy infrastructure (Jaeger, Traefik, DB, network)
[group: "deploy"]
undeploy-infrastructure:
  echo "Stopping Jaeger..."
  docker compose -f infrastructure/jaeger/docker-compose.yml down
  echo "Stopping Traefik..."
  docker compose -f infrastructure/traefik/docker-compose.yml down
  echo "Stopping DB..."
  docker compose -f infrastructure/db/docker-compose.yml down
  echo "Infrastructure undeployed!"

# Deploy all backend services
[group: "deploy"]
deploy-services:
  docker compose -f services/docker-compose.yml up -d

# Undeploy all backend services
[group: "deploy"]
undeploy-services:
  docker compose -f services/docker-compose.yml down

# Deploy the app
[group: "deploy"]
deploy-app:
  docker compose -f app/docker-compose.yml up -d

# Undeploy the app
[group: "deploy"]
undeploy-app:
  docker compose -f app/docker-compose.yml down

# Undeploys the full system with volumes (DB, services, Jaeger, Traefik)
[group: "deploy"]
nuke:
  @echo "⚠️WARNING: This will REMOVE ALL Docker volumes for the system."
  @echo "This includes database data and is NOT reversible."

  @printf "Type 'yes' to continue: "
  @read confirm && [ "$confirm" = "yes" ]

  @echo "Stopping Jaeger..."
  docker compose -f infrastructure/jaeger/docker-compose.yml down --volumes

  @echo "Stopping Traefik..."
  docker compose -f infrastructure/traefik/docker-compose.yml down --volumes

  @echo "Stopping services..."
  docker compose -f services/docker-compose.yml down --volumes

  @echo "Stopping DB..."
  docker compose -f infrastructure/db/docker-compose.yml down --volumes

  @echo "Undeployment complete!"

# Generate rust protobuf files
[working-directory: 'services']
[group: "generate"]
generate-protos-rs:
  #!/usr/bin/env sh
  set -e
  for d in */; do
    if [ -f "$d"/justfile ] && [ -n "$(find "$d" -name '*.proto' -print -quit)" ]; then
      echo "🧬 Generating protos in $d"
      just -f "$d"/justfile generate-protos
    fi
  done

# Generate Dockerfiles for all services
[working-directory: 'services']
[group: "generate"]
generate-dockerfile:
  #!/usr/bin/env sh
  set -e
  for d in */; do
    if [ -f "$d"/justfile ] && [ "$d" != "pkg/" ]; then
      echo "🐳 Generating dockerfiles in $d"
      just -f "$d"/justfile generate-dockerfile
    fi
  done

# Generate typescript protobuf files
[working-directory: 'app']
[group: "generate"]
generate-protos-ts:
  @just -f ./justfile generate-protos

# Generate all protobuf files
[group: "generate"]
generate-protos: generate-protos-rs generate-protos-ts

# Generate protobuf files and Dockerfiles
[group: "generate"]
generate: generate-protos-ts generate-protos-rs generate-dockerfile
