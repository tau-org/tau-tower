# Configuration reference

The two Tau programs read TOML config files. `username` and `password` must
match between them. These credentials are not a security boundary. They only pair
a `tau-radio` client with its `tau-tower` server, so avoid reusing a sensitive
password here.

## tau-tower: `$HOME/.config/tau/tower.toml` on the server

- `username`: source username. It must match the `tau-radio` config.
- `password`: source password. It must match the `tau-radio` config.
- `listen_port`: local port where `tau-tower` receives audio from `tau-radio`.
- `broadcast_port`: local port where `tau-tower` serves listener audio.
- `broadcast_endpoint`: listener path. With `tau.ogg`, listeners open `/tau.ogg`.
- `cors_allow_list`: browser origins allowed to fetch the audio stream from
  another website.

## tau-radio: `$HOME/.config/tau/config.toml` on the broadcaster machine

- `username`: source username. It must match `tower.toml`.
- `password`: source password. It must match `tower.toml`.
- `url`: server domain or IP address only. Do not include `https://` or
  `/tau.ogg`.
- `upstream_port`: source port where `tau-radio` sends audio.
- `audio_interface`: exact audio input device name on the broadcaster machine.
- `tls`: set `true` when connecting through Caddy with HTTPS. Set `false` for a
  direct local or private HTTP test.
- `file`: optional. Base filename for the local recording. Leave it out to use
  `tau_<timestamp>.ogg`. See
  [`tau-radio` recording](https://github.com/tau-org/tau-radio#record-the-stream-locally).
