[![Compiles on macOS (Intel / Silicon), Ubuntu and NixOS](https://github.com/tau-org/tau-tower/actions/workflows/rust.yml/badge.svg?event=pull_request)](https://github.com/tau-org/tau-tower/actions/workflows/rust.yml)

# tau-tower

`tau-tower` is a server that broadcasts a
[`tau-radio`](https://github.com/tau-org/tau-radio) audio stream to listeners on
the web. It receives live audio from `tau-radio` running on your machine and
serves it over one URL that listeners can open in any browser or media player.
The stream is Ogg Opus and audio only.

Pair `tau-tower` with [Asciinema](https://asciinema.org) to broadcast a terminal
session with live audio.

Tau is two programs:

- **`tau-radio`** is the broadcaster client. It runs on your machine, captures
  an audio device, and sends the stream out.
- **`tau-tower`** is the server. It runs on a server, receives that stream, and
  serves it to many listeners at once.

## Ports

`tau-tower` uses two ports: `listen_port` receives audio from `tau-radio`, and
`broadcast_port` serves the listener stream. If you put `tau-tower` behind
Caddy, Caddy exposes the public ports and forwards to these `tau-tower` ports.
See the [self-hosting guide](docs/self-hosting.md) for the full setup.

## Install tau-tower

Install the build tools that `tau-tower` needs:

- [`Rust and Cargo`](https://www.rust-lang.org/tools/install)
- Git
- A C compiler available as `cc`
- Make
- Perl

Then install `tau-tower`:

```bash
cargo install --git https://github.com/tau-org/tau-tower
```

On first run, `tau-tower` looks for a config file. It uses
`$XDG_CONFIG_HOME/tau/tower.toml` when `XDG_CONFIG_HOME` is set,
`$HOME/.config/tau/tower.toml` when `HOME` is set, and `tower.toml` in the
current directory as a fallback.

If no config file exists, `tau-tower` prompts you to create one. To start over
later, run `tau-tower --reset-config` and answer the prompts again.

Values inside angle brackets are placeholders. Replace the whole placeholder,
including the brackets, before saving the config file.

```toml
username = "<source-username>"
password = "<source-password>"

listen_port = 8000
broadcast_port = 8001
broadcast_endpoint = "tau.ogg"

cors_allow_list = ["*"]
```

<!-- [![asciicast](https://asciinema.org/a/JqdeXeILf0lALG34pZzAarmih.svg)](https://asciinema.org/a/JqdeXeILf0lALG34pZzAarmih) -->

You can override config values with CLI arguments:

```bash
tau-tower \
  --listen-port <listen-port> \
  --broadcast-port <broadcast-port> \
  --cors-allow-list "*"
```

## Next steps

Use the dedicated guides for setup details:

- [Try Tau locally](docs/local-test.md) before deploying to a server.
- [Self-host Tau on a VPS](docs/self-hosting.md) for a public audio stream.
- [Put tau-tower behind Caddy](docs/proxy-setup.md) for HTTPS and proxy setup.
- [Add a self-hosted Asciinema terminal stream](docs/asciinema_setup.md) after
  the audio stream works.
- [Configuration reference](docs/config-reference.md) for every config field.

## Funding

This project is funded through [NGI Zero Core](https://nlnet.nl/core), a fund established by [NLnet](https://nlnet.nl) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at the [NLnet project page](https://nlnet.nl/project/Tau).

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)
[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)
