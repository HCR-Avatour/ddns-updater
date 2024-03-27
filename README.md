# DDNS Updater

This repository contains a small program that:
* Polls frequently to check the computer's **local** IP address
  * The network interface to get the address of is either specified manually or automatically selected based on route metrics, meaning e.g. Ethernet will be prioritized over WiFi
* Checks if the IP has changed since the last poll
* If it has, it resets the DuckDNS domain's target IP and sets it to the detected IP address using the DuckDNS API.

It supports both IPv4 and IPv6.

There is a service file to run the program as a daemon using systemd, and a Makefile to build the program and install it as a systemd service.

This project was used as part of the Avatour team's Human Centered Robotics project (Imperial College London, MEng EIE 4th year)
