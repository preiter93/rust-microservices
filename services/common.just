set dotenv-load

root := parent_directory(parent_directory(justfile_directory()))

PROTO_GEN_RS_BINARY := root / "tools/proto-gen-rs/proto-gen-rs"
DOCKER_GEN_BINARY    := root / "tools/docker-gen/docker-gen"

[no-cd]
generate-protos:
  @{{PROTO_GEN_RS_BINARY}}

[no-cd]
generate-dockerfile:
  @{{DOCKER_GEN_BINARY}}

generate: generate-protos generate-dockerfile
