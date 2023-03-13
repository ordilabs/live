
default:
  just --list

setup-once:
  just _install-tools
  just _download-punks
  cd docker && docker-compose build
  npm install
  @echo "Almost done: adding *.local domains to your /etc/hosts requires sudo"
  sudo just _setup-hosts
  @echo "Setup done.\n"
  @echo 'To open all relevant *.local domains run `just open`'
  @echo 'To reset the dev environment run `just down`.'
  @echo 'To start the dev environment run `just up`.'
  
up:  
  cd docker && docker-compose up

down:
  cd docker &&  docker-compose down
  -rm -r docker/data

watch:
    npm run build-css
    cargo leptos watch

watch-css:
    # todo gfi: combine with watch recipe  
    npm run watch-css


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
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_0.png.webp | shuf | head -n1`
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_1.png.webp | shuf | head -n1`
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_2.png.webp | shuf | head -n1`
  {{ORDA}} wallet inscribe `ls /tmp/punks/punk_3.png.webp | shuf | head -n1`
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
_install-tools:
  sudo apt install -y \
    imagemagick \
    webp
  
  cargo install --locked \
    sccache \
    cargo-leptos \

[macos]
_install-tools:
  which magick || brew install imagemagick
  which cwebp || brew install webp
  which sccache || brew install sccache 

  cargo install --locked \
    cargo-leptos \

_setup-hosts:
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
  cd docker && docker-compose exec {{CONTAINER}} /bin/bash

alias g := generate
generate NBLOCKS:
  cd docker && docker-compose exec fixtures just _generate {{NBLOCKS}}

alias f := faucet
faucet ADDRESS AMOUNT:
  cd docker && docker-compose exec fixtures just _faucet {{ADDRESS}} {{AMOUNT}}

_faucet ADDRESS AMOUNT:
  bitcoin-cli -rpcwallet=miner -named sendtoaddress fee_rate=1 address={{ADDRESS}} amount={{AMOUNT}}


alias p := inscribe-punk-1
inscribe-punk-1: # generate
  cd docker && docker-compose exec fixtures just _inscribe-punk 1

_inscribe-punk PUNK:
  seq 9999 | shuf | head -n1 | xargs -I{} {{ORDA}} wallet inscribe /tmp/punks/punk_{}.png.webp
  #just _generate 1


_download-punks:
    @echo Downloading, unpacking punks to '/tmp/punks/punk_*.webp'
    mkdir -p /tmp/punks
    cd /tmp && [ -f punks.png ] || curl -LO "https://github.com/larvalabs/cryptopunks/raw/master/punks.png"
    cd /tmp/punks && [ -f punk_0.png.webp ] || ( \
      convert ../punks.png -crop 100x100@ +repage +adjoin punk_%d.png && \
      ls *.png | xargs -n1 -I{} cwebp -lossless {} -o {}.webp && \
      ls *.png | grep -v webp | xargs rm )
    
