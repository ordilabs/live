# Ordi Labs Live

Ordinals mempool viewer. View inscriptions before they're inscribed!

## Setup

We use Docker to containerize all micro-services and you `just` need to run a few recipes (see below) to get your devbox `up` and running. When your systems prerequisites are already met, usually in under 15 minutes.

### prerequisites

- [Rust ^1.68](https://rustup.rs/)
- [nodejs ^18.15](https://nodejs.org/en)
- [just](https://just.systems/man/en/)
- git
- fast internet, decent cpu

| macos                                       | linux  | windows |
| ------------------------------------------- | ------ | ------- |
| [Docker for Mac](https://www.docker.com/)   | Docker | TODO    |
| [homebrew](https://brew.sh/)                |        |         |
| [XCode](https://developer.apple.com/xcode/) |        |         |

### first installation

```bash
git clone https://github.com/ordilabs/live live--ordilabs
cd live--ordilabs
just install
```

### inscription watching

Run a local instance of Ordi Live after starting bitcoin core:

```bash
just watch
```

Open a browser `http:://127.0.0.1:3000` to new inscriptions hitting your mempool in real time

### developing

All micro-services are managed with 2 simple commands

```bash
just run-services
# just clean-services
```

Once they are running, you can start developing.

```bash
cp .env.sample .env # and uncomment the relevant lines
just watch # for changes, recompile rust/css, refresh frontend
```

### additional commands

Once up and running you can perform dev tasks

```bash
just open # all .local domains in a browser

# inscribe-1-punk (into the mempool) then mine-1-block
just i1p m1b


# create temporary tunnel to expose your .local on the internet
just run-tunnel
```

see more commands with `just -l`

### run `live` through Tor

`socat` can be used as a relay between the standard web and a hidden service on the Tor network. 

- Add to `.env`
```bash
CORE_ADDRESS=127.0.0.1
CORE_PORT=8332

# Replace {onion-address} + {onion-port} with your data
TOR_ADDRESS={onion-address}.onion
TOR_PORT={onion-port}
```

- Then you have two options:

   - Option A: Using Docker and `socator`

   ```bash
   just socator
   ```

   - Option B: Using [`socat`](https://linux.die.net/man/1/socat) command (currently tested on Linux only). Note: `socat` needs to be install on your machine.

   ```bash
   just socat
   ```


#### known issues

##### Linux

*Issue by running `just run-services` command:*

```bash
Error response from daemon: invalid IP address in add-host: ""
error: Recipe `run-services` failed on line 34 with exit code 1
```

*Quick fix (manually):*

- In `just` file override `run-services` as follow

```bash
[linux]
run-services:
  # if someone has a better solution, be my guest
  cd docker && docker compose -f docker-compose.yml -f docker-compose.monkey-patch-linux.yml up 
```

- Get you local IP address (on Ubuntu `Settings` -> `Network` -> `Details` -> `IPv4 Address`) and replace `GATEWAY_IPV4` with that IP in `docker/docker-compose.monkey-patch-linux.yml`

```bash
version: "3.7"

services:

  nginx-proxy:
    extra_hosts:
      # Replace {IP-ADDRESS} with your local IP address
      # e.g. "host.docker.internal.:192.168.1.212" 
      - "host.docker.internal.:{IP-ADDRESS}"
```
