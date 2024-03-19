# drkpublish (Unstable, not-working and use on your own risk)

Dex(Ops)Experience for deployments and other ops operations made easy.

```bash
cat > opday.toml << EOL
path = "tests/01_trivial-backend-no-storage"

[environments]
registry = "registry.digitalocean.com"
registry_auth_config = ".secrets/docker-config.json"
registry_export_auth_config = "/root/.docker/config.json"

[environments.prod]
hosts = [
    "root@46.101.98.131",
]

export_path = "/root/test-01"
docker_compose_overrides = []
EOL

opday docker build-push-deploy --build-arg "BACKEND_TAG=0.0.1"
```

This tool requires local installation of another tools like: `docker`, `ssh`, `scp`.

# The idea

This tool prioritizes simplicity and imperativeness over scaling and decorativeness. It provides a simple yet flexible way to maintain basic Ops operations and automates its own infrastructure. The main features are based on `docker` and `docker compose`.

* Allocation of resources on virtual private and dedicated servers
* Multiple providers support (docker, databases, monitoring, S3, and other storages)
* Reverse-proxies configuration
* Monitoring with metrics and alerts
* Logs collection tools
* Backups and checking for backups infra
* Secrets management
* Support of existing APIs
* Migration tools to lambdas or Kubernetes

Scope of applying this tool:
* Product is not Kubernetes ready. Kubernetes for monolithic or citadel-like applications brings more complexity than solving problems in the early stages.
* Less than 100 virtual machines for service. More hosts have become an issue for straightforward push architecture.
* Up to 100 daily releases
* Base-level infra with trivial sharding and replication for storage and databases. Everything above might be more suitable for custom or cloud-managed services.

# Preparing host

For Ubuntu docker please follow: https://docs.docker.com/engine/install/ubuntu/

# Dev

We use [pre-commit](https://pre-commit.com/).

```bash
pre-commit install
pre-commit
```

Linters and tools:

```bash
rustup component add clippy
cargo binstall cargo-watch
```

Random notes:

```bash
# I have some questions to global rust and git hooks work together.
# So we link cargo to repository root to have the same code running
# with `make`, CI and git hooks.
ln -s `which cargo`

# Support RUST_LOG environment
RUST_LOG=debug cargo run

# Run tests with watch on change
make testw
```
