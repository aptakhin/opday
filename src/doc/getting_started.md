# Getting Started Guide

Make docker compose for your backend or nginx:

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

Make `opday.toml` config:

```bash
cat > opday.toml << EOL
path = "."
EOL
```

Let's call build now:

```bash
opday docker build
```

Okay, just nginx is not so interesting to build.

For more examples please take a look into `<repo-root>/tests` folder.
