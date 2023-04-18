# Ordi Labs Live

Ordinals mempool viewer. View inscriptions before they're inscribed!

## Setup

We use Docker to containerize all micro-services and you `just` need to run a few recipes (see below) to get your devbox `up` and running. When your systems prerequisites are already met, usually in under 15 minutes.

### prerequisites

- [Rust ^1.68](https://rustup.rs/)
- [nodejs ^18.15](https://nodejs.org/en)
- [Typescript](https://www.typescriptlang.org/)
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

### developing

All micro-services are managed with 2 simple commands

```bash
just run-services
# just clean-services
```

Once they are running, you can start developing

```bash
just watch # for changes, recompile rust/css, refresh frontend
```

### additional commands

Once up and running you can perform dev tasks

```bash
just open # all .local domains in a browser

# generate(mine) 1 block
just g 1 

# inscribe a punk
just p

# create temporary tunnel to expose your .local on the internet
just run-tunnel
```

see more commands with `just -l`

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
