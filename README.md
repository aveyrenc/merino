```
                     _
 _ __ ___   ___ _ __(_)_ __   ___
| '_ ` _ \ / _ \ '__| | '_ \ / _ \
| | | | | |  __/ |  | | | | | (_) |
|_| |_| |_|\___|_|  |_|_| |_|\___/
```

**A `SOCKS5` Proxy server written in Rust**

[![Crates.io](https://img.shields.io/crates/v/merino.svg)](https://crates.io/crates/merino)
[![stego](https://docs.rs/merino/badge.svg)](https://docs.rs/merino)
[![License](https://img.shields.io/crates/l/pbr.svg)](https://github.com/aveyrenc/merino/blob/master/LICENSE.md)
[![Build Status](https://github.com/aveyrenc/merino/actions/workflows/cross_build_and_release.yml/badge.svg)](https://github.com/aveyrenc/merino/actions/workflows/cross_build_and_release.yml)
[![dependency status](https://deps.rs/repo/github/aveyrenc/merino/status.svg)](https://deps.rs/repo/github/aveyrenc/merino)

## 🎁 Features

- Written in **100% Safe Rust**
- Multi-threaded connection handler
- Lightweight (Less than 0.6% CPU usage while surfing the web/streaming YouTube)
- Standalone binary (no system dependencies)
- `1+ Gb/second` connection speeds (**On Gigabit LAN network over ethernet. Results may vary!**)
- Tunable logging (by flags or `RUST_LOG` environmental variable)
- `SOCKS5` Compatible Authentication methods:
  - `NoAuth`
  - Username & Password
  - `GSSAPI` Coming Soon!

## 📦 Installation & 🏃 Usage

### Installation

```bash
cargo install merino
```

OR

```bash
git clone https://github.com/aveyrenc/merino
cd merino
cargo install --path .
```

OR

```bash
docker image pull ghcr.io/aveyrenc/merino:latest
```

### Usage

```bash
# Start a SOCKS5 Proxy server listening on port 1080 without authentication
merino --no-auth

# Use username/password authentication and read users from users.csv
merino --users users.csv

# Display a help menu
merino --help
```

OR

```bash
docker container run --pull=always --name=merino -p=8001:8001 ghcr.io/aveyrenc/merino:latest --no-auth --port=8001
```

# 🚥 Roadmap

- [x] IPV6 Support
- [ ] `SOCKS5` Authentication Methods
  - [x] `NOAUTH`
  - [x] `USERPASS`
  - [ ] `GSSAPI` Coming Soon!
- [ ] Custom plugin/middleware support
- [ ] `SOCKS5` Commands
  - [x] `CONNECT`
  - [ ] `BIND`
  - [ ] `ASSOCIATE`
- [ ] Benchmarks & Unit tests
- [ ] [Actix](https://github.com/actix-rs/actix) based backend
- [ ] `SOCKS4`/`SOCKS4a` Support
