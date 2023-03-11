# syntax=docker/dockerfile:1
FROM rust as builder

ARG ORD_ORIGIN=casey
ARG ORD_BRANCH=/refs/tags/0.5.1 # or set master

RUN mkdir -p /build && \
  cd /build && \
  curl -LO https://github.com/$ORD_ORIGIN/ord/archive/$ORD_BRANCH.tar.gz && \
  tar -xzf *.tar.gz --strip-components=1 && \
  cargo install --locked --path .

FROM debian:stable-slim
WORKDIR /root
COPY --from=builder /usr/local/cargo/bin/ord /usr/bin/ord
