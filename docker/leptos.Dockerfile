FROM rust:1.68

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN rustup override set nightly \
  && rustup target add wasm32-unknown-unknown \
  && cargo install cargo-leptos
