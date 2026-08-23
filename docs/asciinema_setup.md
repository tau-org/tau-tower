# Add a self-hosted Asciinema terminal stream

Asciinema streams your terminal, and Tau supplies the audio, so an Asciinema page
can play your session with the Tau audio attached. Set this up after the audio
deploy in the [self-hosting guide](self-hosting.md) is working.

You run a self-hosted Asciinema server on the same VPS as `tau-tower`. It listens
on `127.0.0.1:4000`, and Caddy publishes it at `<your-asciinema-domain>`, so no
new firewall ports are needed. For production concerns like email, storage, and
upgrades, follow the
[Asciinema server self-hosting guide](https://docs.asciinema.org/manual/server/self-hosting/).

## Step 1: Add the Asciinema DNS record

Create an A record for `<your-asciinema-domain>` pointing at the server's IP
address, then confirm it resolves:

```bash
dig +short <your-asciinema-domain>
```

## Step 2: Install Docker and start Asciinema

Install Docker Engine and Docker Compose.
Then create a directory for the deployment:

```bash
sudo mkdir -p /opt/asciinema
cd /opt/asciinema
```

Generate a secret key into an `.env` file, which the Compose file reads:

```bash
SECRET_KEY_BASE=$(tr -dc A-Za-z0-9 </dev/urandom | head -c 64; echo)
echo "SECRET_KEY_BASE=$SECRET_KEY_BASE" | sudo tee .env
```

Create `/opt/asciinema/docker-compose.yml`, replacing `<your-asciinema-domain>`.
You can swap `latest` for a pinned tag from the
[Asciinema server releases](https://github.com/asciinema/asciinema-server/releases):

```yaml
services:
  asciinema:
    image: ghcr.io/asciinema/asciinema-server:latest
    ports:
      - "127.0.0.1:4000:4000"
    environment:
      - SECRET_KEY_BASE=${SECRET_KEY_BASE}
      - URL_HOST=<your-asciinema-domain>
      - URL_SCHEME=https
    volumes:
      - asciinema_data:/var/lib/asciinema
    depends_on:
      postgres:
        condition: service_healthy

  postgres:
    image: docker.io/library/postgres:14
    environment:
      - POSTGRES_HOST_AUTH_METHOD=trust
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 2s
      timeout: 5s
      retries: 10

volumes:
  asciinema_data:
  postgres_data:
```

Start the stack:

```bash
sudo docker compose up -d
```

## Step 3: Publish Asciinema through Caddy

Add this block to `/etc/caddy/Caddyfile`, below the Tau audio blocks, replacing
`<your-asciinema-domain>`:

```Caddyfile
<your-asciinema-domain> {
    reverse_proxy 127.0.0.1:4000
}
```

Validate and reload Caddy, then open `https://<your-asciinema-domain>` to confirm
the site loads:

```bash
sudo caddy validate --config /etc/caddy/Caddyfile
sudo systemctl reload caddy
```

Now that the Asciinema domain is live, you can restrict `tau-tower`'s
`cors_allow_list` in `tower.toml` to `["https://<your-asciinema-domain>"]` and
restart `tau-tower`, so only the Asciinema site can embed the audio stream.

## Step 4: Log in to your Asciinema site

Asciinema logs you in with an emailed link. If you have not configured SMTP, pull
the link from the server logs instead. Submit your email on the login page, then
run:

```bash
cd /opt/asciinema
sudo docker compose logs asciinema | grep 'url from email' | tail -n 5
```

Open the newest URL and finish signing up in the browser.

## Step 5: Authenticate the Asciinema CLI

On the broadcaster machine, install the Asciinema CLI from the
[installation guide](https://docs.asciinema.org/manual/cli/installation/), then
authenticate it against your server:

```bash
asciinema auth --server-url https://<your-asciinema-domain>
```

Open the `/connect/...` URL it prints, entering the full
`https://<your-asciinema-domain>` if prompted.

## Step 6: Stream a terminal session with Tau audio

With `tau-tower` and `tau-radio` running and the audio URL playing, start the
stream:

```bash
asciinema stream -r \
  --server-url https://<your-asciinema-domain> \
  --audio-url https://<your-audio-domain>/tau.ogg
```

Asciinema opens a recorded shell, published at
`https://<your-asciinema-domain>/s/<stream-id>`. Open that URL to watch, and the
page plays the attached audio too. End the stream with `exit` or `Ctrl+D`.
