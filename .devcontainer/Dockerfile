FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive

# set time zone
RUN apt update && \
    apt install -y tzdata && \
    ln -sf /usr/share/zoneinfo/Asia/Tokyo /etc/localtime

# install utilities
RUN apt update && \
    apt install -y curl gcc xz-utils udev git

# clean cache
RUN apt clean && \
    rm -rf /var/lib/apt/lists/*

# change user
USER ubuntu

# instal Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# install probe-rs
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh

# change Rust version
ENV PATH=/home/ubuntu/.cargo/bin:$PATH

RUN rustup toolchain install 1.99.1 && \
    rustup default 1.99.1 && \
    rustup toolchain uninstall stable
