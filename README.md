# rtwod2

**NOTE: At the time of writing it's not possible to even register for a new TwoDNS account anymore. Since I haven't been using it for quite a while, and there are better alternatives around, rtwod2 won't be updated anymore.**

A Rust implementation of [twod] - a [TwoDNS] client.

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
mode = "random"  # Can be either of: "random", "round_robin"
interval = 60
timeout = 5
urls = [ "https://icanhazip.com", "https://ipinfo.io/ip" ]
```


[twod]: https://www.github.com/t4k1t/twod
[TwoDNS]: https://www.twodns.de
