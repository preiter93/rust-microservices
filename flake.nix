{
  description = "Rust microservices development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain

            # Build tools
            just
            pkg-config
            openssl

            # Frontend
            nodejs_20

            # Container tools
            docker-compose

            # Database tools (optional, for local postgres)
            postgresql

            # Protobuf
            protobuf
            buf
          ];

          shellHook = '''';

          # Environment variables
          RUST_BACKTRACE = "1";
          DATABASE_URL = "postgres://postgres:postgres@localhost:5432";
        };
      }
    );
}
