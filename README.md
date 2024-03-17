# drkpublish

Services deployment made easy for people.



Pipeline helpers cli
Easy locally
Easy in ci-cd

Secrets.
Port
Remapping
Nginx under balancer.
Volumes management.

Entities and ideas : https://miro.com/app/board/uXjVN3A6wi4=/?share_link_id=724804630324


```bash
dkrd init

dkrd local showrun
> docker compose up -d --build -e ...
dkrd build --config tests/dkrdeliver.test.toml backend --build-arg "BACKEND_TAG=0.0.1"
dkrd deploy --config tests/dkrdeliver.test.toml backend db --env dev --build-arg "BACKEND_TAG=0.0.1"
```

# Preparing host

For Ubuntu docker please follow: https://docs.docker.com/engine/install/ubuntu/


# Dev

We use [pre-commit](https://pre-commit.com/).

```bash
pre-commit install
```

https://crates.io/crates/cargo-watch

```bash
# Run tests with watch on change
make testw
```
