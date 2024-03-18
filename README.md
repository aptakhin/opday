# drkpublish (Unstable)

DexExperience for deployment made easy.

```bash
dkrd --config tests/dkrdeliver.test.toml docker build -- --build-arg "BACKEND_TAG=0.0.1"
dkrd --config tests/dkrdeliver.test.toml docker deploy --env prod --build-arg "BACKEND_TAG=0.0.1"
```

This tool requires local installation of another tools like: `docker`, `ssh`, `scp`.

# TODO

* Multiple providers support (docker, databases, monitoring, S3 and other storages)
* Secrets handling
* Reverse-proxies configuration
* Easy locally on dev and for CD pipelines

Entities and ideas: https://miro.com/app/board/uXjVN3A6wi4=/?share_link_id=724804630324

# Preparing host

For Ubuntu docker please follow: https://docs.docker.com/engine/install/ubuntu/

# Dev

We use [pre-commit](https://pre-commit.com/).

```bash
pre-commit install
pre-commit
```

```bash
rustup component add clippy
cargo binstall cargo-watch
```

```bash
# Run tests with watch on change
make testw
```
