# syntax=docker/dockerfile:1
FROM amd64/ubuntu:22.04
#FROM rust/debian as builder

# todo gfi: install ord from release if available for this arch
#ARG ORD_ORIGIN=casey
#ARG ORD_BRANCH=/refs/tags/0.5.1 # or set master

#ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
# RUN mkdir -p /build && \
#   cd /build && \
#   curl -LO https://github.com/$ORD_ORIGIN/ord/archive/$ORD_BRANCH.tar.gz && \
#   tar -xzf *.tar.gz --strip-components=1 && \
#   cargo install --locked --path .

RUN apt-get update && apt-get install -y \
  curl \
  ca-certificates \
  gpg \
  jq \
  lsb-release \
  ""

ADD https://github.com/casey/ord/releases/download/0.5.1/ord-0.5.1-x86_64-unknown-linux-gnu.tar.gz /tmp/
RUN cd /tmp && tar -xvf ord-*.tar.gz && mv ord /usr/local/bin/

ADD https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub /tmp/prebuilt-mpr.pub
RUN cat /tmp/prebuilt-mpr.pub \
  | gpg --dearmor \
  | tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null \
  && echo "deb [arch=amd64 signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" \
  | tee /etc/apt/sources.list.d/prebuilt-mpr.list

RUN apt-get update && apt-get install -y \
  just \
  "" \
  && rm -rf /var/lib/apt/lists/* \
  && rm -rf /tmp/*


#can be done in one step with ADD --chmod-755 with buildkit
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/81b1373f17855a4dc21156cfe1694c31d7d1792e/wait-for-it.sh /usr/local/bin/wait-for-it.sh
RUN chmod +x /usr/local/bin/wait-for-it.sh

# todo gfi: make this multi-arch
ADD https://bitcoincore.org/bin/bitcoin-core-24.0.1/bitcoin-24.0.1-x86_64-linux-gnu.tar.gz /
RUN tar -xvf bitcoin-24.0.1-*.tar.gz && mv bitcoin-*/bin/* /usr/local/bin/


#FROM amd64/ubuntu:22.04
WORKDIR /root
# comprehensive list of all binaries/scripts below, for easy reference in /usr/local/bin/
#COPY --from=builder /usr/local/cargo/bin/just /usr/local/bin/
#COPY --from=builder /usr/local/cargo/bin/ord /usr/local/bin/
# COPY --from=builder /usr/local/bin/ord /usr/local/bin/
# COPY --from=builder /usr/local/bin/bitcoin-cli /usr/local/bin/
# COPY --from=builder /usr/local/bin/bitcoind /usr/local/bin/
# COPY --from=builder /usr/bin/jq /usr/local/bin/
# COPY --from=builder /usr/bin/wait-for-it.sh /usr/local/bin/

# RUN apt-get update && apt-get install -y \
#   curl \
#   jq \
#   && rm -rf /var/lib/apt/lists/* 
