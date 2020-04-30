# rtwod2

A Rust implementation of twod.

## Usage

```terminal
rtwod2
```

## Configuration

Per default `rtwod2` will try to read its configuration from
`$XDG_CONFIG_HOME/rtwod2/rtwod2.toml`. The `-c/--config` flag can be used to
provide a different path.

Example config:

```toml
[twodns]
url = "https://api.twodns.de/hosts/example.dd-dns.de"
user = "username@example.com"
token = "token"
timeout = 5

[update]
# mode = "round_robin"
mode = "random"
interval = 60
timeout = 5
urls = [ "https://icanhazip.com", "https://ipinfo.io/ip" ]
```
