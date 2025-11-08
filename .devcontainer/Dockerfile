FROM ubuntu:24.04

ENV DEBIAN_FRONTEND=noninteractive

# change time zone
RUN apt update && \
    apt install -y tzdata && \
    ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

# instal Rust
RUN apt update && \
    apt install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH

# install probe-rs
RUN apt update && \
    apt install -y gcc xz-utils && \
    curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

# change Rust version
RUN rustup toolchain install 1.90 && \
    rustup default 1.90 && \
    rustup toolchain uninstall stable

# install utilities
RUN apt update && \
    apt install -y udev git

# clean cache
RUN apt clean && \
    rm -rf /var/lib/apt/lists/*
