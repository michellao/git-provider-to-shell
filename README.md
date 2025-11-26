# Parsing Webhook from major Git provider

The app will start a shell script on the current path.

It just use webhook from GitHub and Gitlab. The software create an HTTP server with the uri /webhook where webhook will calling.

Maybe in the future I'll do 2 endpoints for each webhook.

## Secret Token

Manually generate a secret with

```sh
cat << EOL > .env
GH_WEBHOOK_SECRET=$(openssl rand -base64 32)
GL_WEBHOOK_SECRET=$(openssl rand -base64 32)
EOL
```

Then go to the repository settings > Webhook

Copy and paste webhook secret to secret token

## GitHub

To start HTTP server for GitHub

```sh
git-provider-to-shell github
```

## GitLab

To start HTTP server for GitLab

```sh
git-provider-to-shell gitlab
```

## TLS HTTP server for both Git server

```sh
git-provider-to-shell --private-key ./path/to/private.pem --fullchain-key ./path/to/fullchain.pem both
```
