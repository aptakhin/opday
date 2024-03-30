https://github.com/acmesh-official/acme.sh

```bash
mkdir -p /etc/nginx/cert

curl https://get.acme.sh | sh -s email=...
acme.sh --issue -d opday.dev -w /etc/nginx/cert
```
