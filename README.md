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

---

To install:
```bash
$ cargo install --git https://github.com/tau-org/tau-tower
```

The first time using the tool, it will search your system for a config file. 
It looks for it in the path 
```bash
$HOME/.config/tau/tower.toml # on macOS
```
or 
```bash
$XDG_CONFIG_HOME/tau/tower.toml # on Linux
```

If there is no config file located there, you will be prompted to create one. 

```toml
# username and password are NOT secure, they only
# link a tauradio and tautower service together
username = "username" 
password = "emanresu" 

# Sets the listening port, to which the source stream is transmitted
listen_port = 8000      

# Sets the broadcast port, from which the stream will be accessable
broadcast-port = 8001       

# Sets the server http endpoint - http://localhost:8001/tau.ogg
broadcast-endpoint = "tau.ogg"       

# Optional: 
# Sets which other sites are able to rebroadcast the stream
# "*" allowes all, adding "http://localhost:4000" to list is redundant
cors_allow_list = ["*", "http://localhost:4000"]
```

<!-- [![asciicast](https://asciinema.org/a/JqdeXeILf0lALG34pZzAarmih.svg)](https://asciinema.org/a/JqdeXeILf0lALG34pZzAarmih) -->

If you want to temporarily overwrite the config, you are able to pass arguments.

```bash
# Ex: Uses temporary credentials, and disables the local recording. 
$ tau-tower \
  --listen-port <listen-port> \
  --broadcast-port <broadcast-port> \
  --cors-allow-list "*"
```

### Dependencies

**On Linux** (using apt):
```bash
$ sudo apt update
$ sudo apt install build-essential
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
