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

### Streaming Pipeline

[`tau-radio`](https://github.com/tau-org/tau-radio) runs on your local
machine, and captures sound from the audio device on your system. The defaults
are `BlackHole 2ch` on macOS, and `pipewire` on Linux, though these can be
overwritten by the config or in the CLI arguments.

The captured audio is then streamed to the
[`tau-tower`](https://github.com/tau-org/tau-tower), which should run on a
remote server. This server application exposes a audio media stream that can be
consumed by many clients, as a web radio. 

Alongside `tau-tower` should run an instance of [`Asciinema`](https://github.com/asciinema/asciinema) 
which can use the live audio stream as background to a live terminal stream, by
setting the broadcast endpoint URL as a media source in the streams settings.

```
https://example.com:8002/tau.ogg
```
 
For this to work, the Asciinema origin must be added to `cors_allow_list` in `tower.toml`:
 
```toml
cors_allow_list = ["https://example.com:4000"]
```

| your computer |  | remote server |  |  |
| - | - | - | - | - |
| (audio capture) | (opt. tls/ssl) | (proxy) | (broadcaster) | (opt. stream host) |
| tau-radio -> | internet -> | Caddy  -> | tau-tower  -> | Asciinema |



> For TLS termination and reverse proxy setup, see [Proxy Setup (Caddy)](docs/proxy-setup.md).

## Funding

This project is funded through [NGI Zero Core](https://nlnet.nl/core), a fund established by [NLnet](https://nlnet.nl) with financial support from the European Commission's [Next Generation Internet](https://ngi.eu) program. Learn more at the [NLnet project page](https://nlnet.nl/project/Tau).

[<img src="https://nlnet.nl/logo/banner.png" alt="NLnet foundation logo" width="20%" />](https://nlnet.nl)
[<img src="https://nlnet.nl/image/logos/NGI0_tag.svg" alt="NGI Zero Logo" width="20%" />](https://nlnet.nl/core)
