# opday (Unstable, not-working and use on your own risk)

Dex(Ops)Experience for deployments and other ops operations made easy.

Make docker compose for default nginx:

```bash
cat > docker-compose.yaml << EOL
version: "3.7"
services:
  nginx:
    image: nginx:latest
    ports:
    - "80:80"
EOL
```

Let's call build now:

```bash
opday docker build
```

This tool requires local installation of another tools and their availability in the shell: `docker`, `ssh`, `scp`.

# The idea

This tool prioritizes simplicity and imperativeness over scaling and decorativeness. It provides a simple yet flexible way to maintain basic Ops operations and automates its own infrastructure. There are plenty amount of `deploy.sh` scripts we might write manually or tools like [umputun/spot](https://github.com/umputun/spot), [Kamal](https://kamal-deploy.org/) or Ansible. Usage of some of them might be overkill, but also I don't want to force people to learn one more DSL for this obvious deployment stuff. This time might be spent on learning better new programming language, Kubernetes or speaking language :) Hence, the main features are based on `docker` and `docker compose`.

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
