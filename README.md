# nostr-user-discovery

NIP-05 に従って internet-identifier を nostr key (public key) に解決するコマンド。 <https://github.com/nostr-protocol/nips/blob/master/05.md>

## Installation

```console
$ # `cargo`
$ git clone https://github.com/bouzuya/nostr-user-discovery
$ cd nostr-user-discovery
$ cargo install --path .
$ nostr-user-discovery --help
Usage: nostr-user-discovery <QUERY>

Arguments:
  <QUERY>

Options:
  -h, --help  Print help

$ # `docker`
$ alias nostr-user-discovery='docker run ghcr.io/bouzuya/nostr-user-discovery:0.1.0'
$ nostr-user-discovery --help
...
```

## Usage

```console
$ nostr-user-discovery b@bouzuya.net
npub16ysnnfr4lge38jrp0ptt762wmjvr5zyupvvqyaa3xl8aahjhrxtqa3uadg
```
