.PHONY: build
build:
	cargo build --release

.PHONY: install
install:
    install --owner root --group root --mode 0744 target/release/dyndns /usr/bin/ddnsd
    install --owner root --group root --mode 0644 ddnsd.service /etc/systemd/system/ddnsd.service