# drkpublish

Pipeline helpers cli
Easy locally
Easy in ci-cd


Secrets.
Port
Remapping
Nginx under balancer.
Volumes management.

Entities: https://miro.com/app/board/uXjVN3A6wi4=/?share_link_id=724804630324


```bash
dkrd init

dkrd local showrun
> docker compose up -d --build -e ...
dkrd build backend --tag "0.0.1"
dkrd deploy backend db --env dev --tag "0.0.1"
```


# Preparing host

For Ubuntu docker please follow: https://docs.docker.com/engine/install/ubuntu/
