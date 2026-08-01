# Try Tau locally

Run `tau-tower` and `tau-radio` on the same machine to confirm they work before
deploying to a server. This uses a direct connection, with no domain, TLS, or
Caddy.

Install both programs first, following the
[`tau-tower`](https://github.com/tau-org/tau-tower#install-tau-tower) and
[`tau-radio`](https://github.com/tau-org/tau-radio#install-tau-radio) READMEs.

Create the `tau-tower` config at `$HOME/.config/tau/tower.toml`:

```toml
username = "<source-username>"
password = "<source-password>"

listen_port = 8000
broadcast_port = 8001
broadcast_endpoint = "tau.ogg"

cors_allow_list = ["*"]
```

Start it in one terminal:

```bash
tau-tower
```

Create the `tau-radio` config at `$HOME/.config/tau/config.toml`, using the same
username and password and an input device on this machine (e.g `BlackHole 2ch` on
macOS, `pipewire` on Linux):

```toml
username = "<source-username>"
password = "<source-password>"

url = "127.0.0.1"
upstream_port = 8000

audio_interface = "<audio-device>"
tls = false
```

Start it in a second terminal:

```bash
tau-radio
```

Open `http://127.0.0.1:8001/tau.ogg` in a browser or media player. If you hear
audio, both programs work. you are ready to
[deploy to a server](self-hosting.md).
