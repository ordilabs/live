
default:
  just --list

# faster, become the default for crates.io in the 1.70.0
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL := "sparse"
export DOCKER_BUILDKIT := "0"

install:
  just _install-tools
  just _download-punks
  rustup override set nightly
  rustup target add wasm32-unknown-unknown
  #cd docker && docker compose build
  npm install
  @echo "Almost done: adding *.local domains to your /etc/hosts requires sudo"
  just _install-hosts
  @echo "Install done.\n"
  @echo "Relevant next commands:"
  @echo '`just open` all relevant *.local domains'
  @echo '`just clean-services` resets the micro-services (AND ALL THEIR DATA!)'
  @echo '`just run-services` starts all micro-sevices inside docker'
  @echo '`just watch` auto recompile rust, css code. refresh frontend'

[macos]
run-services:
  cd docker && docker compose up

[linux]
run-services:
  # if someone has a better solution, be my guest
  sed 's/GATEWAY_IPV4/'` ip addr show eth0 | grep inet | awk '{print $2}' | cut -d/ -f1 | head -n1`'/g' \
    docker/docker-compose.monkey-patch-linux.yml | tee tmp/docker-compose.linux.yml
  cd docker && docker compose -f docker-compose.yml -f ../tmp/docker-compose.linux.yml up 

clean-services:
  cd docker && docker compose down -v

watch:
  just watch-leptos

watch-leptos:
  cargo leptos watch -v --hot-reload

build-release:
  npx tailwindcss \
    -c ./src/style/tailwind.config.js \
    -i ./src/style/input.css \
    -o ./target/style/output.css \
    --minify
  cargo leptos build --release

run-tunnel:
  cd docker && docker compose run --rm cftunnel 2>&1 | grep "|"

_tunnel:
  cd docker && docker compose run --rm cftunnel


ORDA := "ord -r --wallet alice --rpc-url bitcoin-core:18443/wallet/alice"
_fixtures:
  -[ -f .fixtures ] && sleep infinity
  -bitcoin-cli -named createwallet wallet_name=miner load_on_startup=true
  bitcoin-cli -rpcwallet=miner getnewaddress | tee miner.txt && just _generate 101
  -{{ORDA}} wallet restore "pipe minimum luggage modify solid sail another twenty say pioneer cloud scrub"
  [ -f alice.txt ] || {{ORDA}} wallet receive | jq -r .address | tee alice.txt
  bitcoin-cli -rpcwallet=miner -named sendtoaddress fee_rate=1 address=`cat alice.txt` amount=1
  bitcoin-cli -rpcwallet=miner -named sendtoaddress fee_rate=1 address=`cat alice.txt` amount=2
  bitcoin-cli -rpcwallet=miner -named sendtoaddress fee_rate=1 address=`cat alice.txt` amount=3
  bitcoin-cli -rpcwallet=miner -named sendtoaddress fee_rate=1 address=`cat alice.txt` amount=4
  just _generate 1
  {{ORDA}} wallet balance
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_0.webp | shuf | head -n1`
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_1.webp | shuf | head -n1`
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_2.webp | shuf | head -n1`
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_3.webp | shuf | head -n1`
  just _generate 1 
  touch .fixtures
  sleep infinity

_generate BLOCKS:
  bitcoin-cli -rpcwallet=miner -named generatetoaddress nblocks={{BLOCKS}} address=`cat miner.txt`

[macos]
open:
  open \
    http://live-ol.local \
    http://ord-ol.local \
    http://mempool-ol.local \

[linux]
open:
  xdg-open http://live-ol.local
  xdg-open http://ord-ol.local
  xdg-open http://mempool-ol.local

[linux]
_install-tools:
  sudo apt install -y \
    build-essential \
    imagemagick \
    webp \
    pkg-config \
    libssl-dev \

  # TODO Switch back to stable `cargo-leptos` as soon as `hot-reload` is supported there 
  cargo install --locked \
    --git https://github.com/akesson/cargo-leptos \
    cargo-leptos \
  

[macos]
_install-tools:
  which magick || brew install imagemagick
  which cwebp || brew install webp
  which sccache || brew install sccache 

  cargo install --locked --git https://github.com/akesson/cargo-leptos cargo-leptos

_install-hosts:
  just _add-host-once mempool-ol.local
  just _add-host-once ord-ol.local
  just _add-host-once live-ol.local

_add-host-once HOST:
  just _add-ip-host-once 127.0.0.1 {{HOST}}
  just _add-ip-host-once ::1 {{HOST}}

_add-ip-host-once IP HOST:
  grep "{{IP}} {{HOST}}" /etc/hosts || echo "{{IP}} {{HOST}}" | sudo tee -a /etc/hosts

alias e := enter
enter CONTAINER:
  cd docker && docker compose exec {{CONTAINER}} /bin/bash

alias g := generate
generate NBLOCKS:
  cd docker && docker compose exec fixtures just _generate {{NBLOCKS}}

alias f := faucet
faucet ADDRESS AMOUNT:
  cd docker && docker compose exec fixtures just _faucet {{ADDRESS}} {{AMOUNT}}

_faucet ADDRESS AMOUNT:
  bitcoin-cli -rpcwallet=miner -named sendtoaddress fee_rate=1 address={{ADDRESS}} amount={{AMOUNT}}


alias p := inscribe-punk-1
inscribe-punk-1: # generate
  cd docker && docker compose exec fixtures just _inscribe-punk 1

_inscribe-punk PUNK:
  seq 9999 | shuf | head -n1 | xargs -I{} {{ORDA}} wallet inscribe /tmp/punks/punk_{}.webp
  #just _generate 1


_download-punks:
    @echo Downloading, unpacking punks to 'tmp/punks/punk_*.webp'
    mkdir -p tmp/punks
    cd tmp && [ -f punks.png ] || curl -LO "https://github.com/larvalabs/cryptopunks/raw/master/punks.png"
    cd tmp/punks && [ -f punk_0.webp ] || ( \
      convert ../punks.png -crop 100x100@ +repage +adjoin punk_%d.png && \
      seq 0 1 9999 | xargs -n1 -P 10 -I{} cwebp -lossless punk_{}.png -o punk_{}.webp \
    )
    -cd tmp/punks && rm punk_*.png*
