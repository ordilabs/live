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
