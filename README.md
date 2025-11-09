# Rust Nix Monorepo Demo

Cargo workspace management and Nix flakes for building multiple Rust services with shared dependencies.

## Structure
```
rust-nix-monorepo/
├── flake.nix              # Nix flake managing all builds
├── Cargo.toml             # Workspace root
├── services/
│   ├── web-service/       # HTTP service (port 3000)
│   ├── api-service/       # HTTP service (port 4000)
│   └── cli-tool/          # CLI client
└── shared/
    └── common/            # Shared library
```

## Features

- **Cargo workspace** with shared dependency versions
- **Nix flake** with individual service outputs
- **Shared library** used across all services
- **Service interaction** - CLI calls web service, web service fetches external data

## Building
```bash
# Build individual service
nix build .#web-service
nix build .#api-service
nix build .#cli-tool

# Build all services
nix build

# Enter development shell
nix develop
```

## Running
```bash
# Start web service
./result/bin/web-service &

# Start API service  
./result/bin/api-service &

# Run CLI (fetches from web service)
./result/bin/cli-tool
```

## Development
```bash
# Enter Nix shell with Rust toolchain
nix develop

# Run services with cargo
cargo run -p web-service
cargo run -p api-service
cargo run -p cli-tool

# Run tests
cargo test
```

## Technical Highlights

1. **Workspace Dependencies** - Centralized version management in root `Cargo.toml`
2. **Nix Flake Outputs** - Each service builds independently via `nix build .#<service>`
3. **Shared Library Pattern** - Common types and logic in `shared/common` crate
4. **Self-Referential Demo** - Web service fetches this repo's metadata from Codeberg API

## Demonstrated Skills

- Monorepo structure and management
- Nix flakes for reproducible builds
- Cargo workspace patterns
- Service composition and communication
- Infrastructure as Code principles

---

Built in 5 hours while learning Nix.
