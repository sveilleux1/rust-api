FROM ubuntu:14.04

ENV RUST_VERSION=nightly-2016-04-09

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
      build-essential \
      ca-certificates \
      curl \
      git \
      libssl-dev \
      nginx \
      supervisor && \
      curl -sO https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
      tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
      ./rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs

RUN DEBIAN_FRONTEND=noninteractive apt-get remove --purge -y curl && \
    DEBIAN_FRONTEND=noninteractive apt-get autoremove -y && \
    DEBIAN_FRONTEND=noninteractive apt-get clean all && \
    rm -rf \
      rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
      rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz \
      /var/lib/apt/lists/* \
      /tmp/* \
      /var/tmp/*

RUN mkdir /source && \
  rm -rf /etc/nginx/sites-enabled/* && \
  rm -rf /etc/nginx/sites-available/*

ADD docker/nginx.conf /etc/nginx/sites-enabled/default
ADD docker/supervisord.conf /etc/supervisor/supervisord.conf

COPY ./Cargo.lock /source
COPY ./Cargo.toml /source
COPY ./portal /source

WORKDIR /source
RUN cargo build
