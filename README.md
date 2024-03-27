# DDNS Updater

This repository contains a small program, written in Rust, that:
* Polls frequently (every 500ms) to check the host's **local** IP address
  * The target network interface is either specified manually or automatically selected based on route metrics (e.g. Ethernet will be prioritized over WiFi)
* If the IP has changed since the last poll, it resets the DuckDNS domain's target IP and sets it to the detected IP address using the DuckDNS API.
* Therefore, the DuckDNS domain will always point to the host.

## Features
* Supports both IPv4 and IPv6
* IPv6 addresses are filtered, the first global (`2xxx:`) address is used.
* Supports being run as a daemon using systemd
* Configured using flags, you can modify the configuration by changing the flags in the systemd service file.

## Building / Running

This is designed to run under Linux. It should compile for any architecture that Rust supports but it was tested on Ubuntu `aarch64`.

Build requirements:
* Latest Rust toolchain (`cargo` and `rustc` available)
* OpenSSL development headers (e.g. `apt install libssl-dev`)

Build: `cargo build --release` (produces `target/release/dyndns`)

Run: `./dyndns --domain mydomain --token INSERT_DUCKDNS_TOKEN_HERE --interface wlan0` (For `mydomain.duckdns.org`. If `--interface` is not specified, an interface will automatically be selected)

Install as systemd service: `sudo make install`

## About

This project was written by [Harry Phillips](https://github.com/harryjph).

This project was used as part of the Avatour team's Human Centered Robotics project (Imperial College London, MEng EIE 4th year).
