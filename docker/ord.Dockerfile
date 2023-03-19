# syntax=docker/dockerfile:1
FROM rust as builder

# todo gfi: install ord from release if available for this arch
ARG ORD_ORIGIN=casey
ARG ORD_BRANCH=/refs/tags/0.5.1 # or set master

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN mkdir -p /build && \
  cd /build && \
  curl -LO https://github.com/$ORD_ORIGIN/ord/archive/$ORD_BRANCH.tar.gz && \
  tar -xzf *.tar.gz --strip-components=1 && \
  cargo install --locked --path .

RUN apt-get update && apt-get install -y \
  curl \
  jq \
  ""

# todo gfi: install just from release if available
RUN cargo install just
#can be done in one step with ADD --chmod-755 with buildkit
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/81b1373f17855a4dc21156cfe1694c31d7d1792e/wait-for-it.sh /usr/bin/wait-for-it.sh
RUN chmod +x /usr/bin/wait-for-it.sh

# todo gfi: make this multi-arch
ADD https://bitcoincore.org/bin/bitcoin-core-24.0.1/bitcoin-24.0.1-aarch64-linux-gnu.tar.gz /
RUN tar -xvf bitcoin-24.0.1-*.tar.gz && mv bitcoin-*/bin/* /usr/local/bin/


FROM debian:stable-slim
WORKDIR /root
# comprehensive list of all binaries/scripts below, for easy reference in /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/just /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/ord /usr/local/bin/
COPY --from=builder /usr/local/bin/bitcoin-cli /usr/local/bin/
COPY --from=builder /usr/local/bin/bitcoind /usr/local/bin/
COPY --from=builder /usr/bin/jq /usr/local/bin/
COPY --from=builder /usr/bin/wait-for-it.sh /usr/local/bin/

RUN apt-get update && apt-get install -y \
  curl \
  jq \
  && rm -rf /var/lib/apt/lists/* 
