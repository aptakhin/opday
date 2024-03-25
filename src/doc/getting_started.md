# Getting Started Guide

Make docker compose for your backend or nginx:

```bash
cat > docker-compose.yaml << EOL
version: '3.7'
services:
  nginx:
    image: nginx:latest
    ports:
    - "80:80"
EOL
```

Make `opday.toml` config:

```bash
cat > opday.toml << EOL
path = "."

[environments]
registry = "registry.digitalocean.com"
registry_auth_config = ".secrets/docker-config.json"
registry_export_auth_config = "/root/.docker/config.json"

export_path = "/root/test-01"
docker_compose_overrides = []
EOL
```

Let's call build now:

```bash
opday docker build --build-arg "BACKEND_TAG=0.0.1"
```
