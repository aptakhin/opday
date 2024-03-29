# Getting Started Guide

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

Okay, just nginx is not so interesting to build.

And for further progress we need some remote virtual machine.

For host preparing, please follow:
* For Ubuntu: https://docs.docker.com/engine/install/ubuntu/

Let's create configuration `opday.toml` in current directory.

```bash
cat > opday.toml << EOL
[environments]
registry = "registry.digitalocean.com"

[environments.prod]
export_path = "/root/test-01"
EOL
```

## Login

Login command authentificates machines to use private container registry.

It creates or uses existing `docker-config.json`, pushes it to remote machines and run `docker login` remotely.

Some container registries like `registry.digitalocean.com` gives you to download `docker-config.json` file you can use with `-f` parameter.

```bash
opday docker login -f docker-config.json
```

For other options with username and password:

```bash
opday docker login --username REGISTRY_USERNAME --password-stdin
```

Then paste the registry password, press Enter and close input (Usually it's `Ctrl+D` in terminals).

## Build

Build images locally.

## Push

Pushes images to container registry.

## Deploy

Deploys containers on remote machines.

## Summary

For more examples please take a look into `<repo-root>/tests` folder.
