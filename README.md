# Rust Nix Monorepo Demo

[![Nix Build and Test](https://github.com/querret/rust-nix-monorepo/actions/workflows/nix-build.yml/badge.svg)](https://github.com/querret/rust-nix-monorepo/actions/workflows/nix-build.yml)

Demonstrates Cargo workspace management and Nix flakes for building multiple Rust services with shared dependencies.

## Live Demo

Automatically built and deployed on every push to main.

- Web Interface: http://54.90.165.134:8080/
- Health Check: http://54.90.165.134:8080/api/health
- Repository Info API: http://54.90.165.134:8080/api/repo
- API Service Status: http://54.90.165.134:8090/status

## Tech Stack

| Area                          | Technology                 |
| ----------------------------- | -------------------------- |
| Language                      | Rust                       |
| Build System                  | Nix Flakes                 |
| CI/CD                         | GitHub Actions             |
| Hosting                       | AWS EC2 (Ubuntu)           |
| Process Management            | systemd                    |
| Package/Dependency Management | Cargo + Nix                |
| Web Framework                 | Axum                       |
| HTTP Client                   | Reqwest                    |
| Deployment                    | SSH + systemctl automation |

## CI/CD Pipeline

| Phase      | Description                               |
| :--------- | :---------------------------------------- |
| **Build**  | All services built using Nix flakes       |
| **Test**   | Test suite execution via Cargo            |
| **Deploy** | Automatic deployment to AWS EC2 on `main` |

Each commit to main triggers:

- Nix builds for all services
- Test suite execution
- Automatic production deployment on the main branch

All builds use Nix flakes for complete reproducibility across local, CI, and production environments.

## Project Structure
``` bash
rust-nix-monorepo/
├── flake.nix              # Nix flake managing all builds
├── Cargo.toml             # Workspace root
├── services/
│   ├── web-service/       # HTTP service (port 8080)
│   ├── api-service/       # HTTP service (port 8090)
│   └── cli-tool/          # CLI client
└── shared/
    └── common/            # Shared library
```

## Features

- Cargo workspace with shared dependencies
- Nix flake outputs for each service
- Shared library used across all services
- Service interaction:
    - CLI calls the web service
    - Web service fetches repository metadata from the GitHub API

## Building
```bash
# Build individual service from within the service folder
nix build .#web-service
nix build .#api-service
nix build .#cli-tool

# Build all services
nix build

# Enter development shell
nix develop
```

## Running

All services are deployed and managed by systemd on the target host.

Each service runs as dedicated unit:
| Service     | Systemd Unit                   | Port |
| ----------- | ------------------------------ | ---- |
| Web Service | `web-service-monorepo.service` | 8080 |
| API Service | `api-service-monorepo.service` | 8090 |
| CLI Tool    | `cli-tool-monorepo.service`    | —    |

### Start, Stop, and Restart Services
``` bash
# Start all services
sudo systemctl start web-service-monorepo
sudo systemctl start api-service-monorepo
sudo systemctl start cli-tool-monorepo

# Restart a specific service
sudo systemctl restart web-service-monorepo

# Stop a service
sudo systemctl stop api-service-monorepo
```

### Check Status and Logs for Services
``` bash
# Check status
sudo systemctl status web-service-monorepo

# View logs
sudo journalctl -u web-service-monorepo -f
```

Each unit is automatically restarted on failure and included in the CI/CD deployment process.

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
- Continuous integration and deployment
- Infrastructure as Code principles

---

Built over a weekend while learning Nix to demonstrate end-to-end reproducibility, multi-service coordination, and automated deployment.
