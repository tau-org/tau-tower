# Put tau-tower behind Caddy

Caddy is a reverse proxy that terminates HTTPS and forwards to `tau-tower` on the
same server, which gives you a browser-trusted certificate with no manual
certificate steps. This is part of the [self-hosting guide](self-hosting.md).
Before doing the following, `tau-tower` should be configured and the firewall open.

Install Caddy from the [Caddy install guide](https://caddyserver.com/docs/install),
then edit the Caddyfile (usually `/etc/caddy/Caddyfile` on packaged Linux
installs):

```Caddyfile
<your-audio-domain>:8001 {
    reverse_proxy 127.0.0.1:6000
}

<your-audio-domain> {
    reverse_proxy 127.0.0.1:6001
}
```

The first block forwards the `tau-radio` source connection on `8001` to
`listen_port`; the second serves listeners over HTTPS from `broadcast_port`. Keep
`127.0.0.1` literal, since Caddy and `tau-tower` run on the same machine. Replace
both domain values, then validate and reload:

```bash
sudo caddy validate --config /etc/caddy/Caddyfile
sudo systemctl reload caddy
```

Port `8001` carries the encrypted source connection from `tau-radio`, not listener
traffic. Listeners use the plain HTTPS domain at
`https://<your-audio-domain>/tau.ogg`.

For advanced configuration, see the official
[Caddy documentation](https://caddyserver.com/docs/).

## Broadcast without TLS (optional)

The setup above assumes `tau-radio` sends with `tls = true`. If you cannot enable
TLS on the source stream, match the source port with no domain so Caddy does not
try to obtain a certificate for it, and keep the listener block on HTTPS:

```Caddyfile
:8001 {
    reverse_proxy 127.0.0.1:6000
}

<your-audio-domain> {
    reverse_proxy 127.0.0.1:6001
}
```

In the `tau-radio` config, set `tls = false` and set `url` to the server's IP
address. Because the source stream is now unencrypted, restrict inbound port
`8001` in the server firewall to your broadcaster's public IP, so only you can
send to it.
