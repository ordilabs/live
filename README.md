# Ordi Labs Live

Ordinals mempool viewer. View inscriptions before they're inscribed!

## Setup

We use Docker to containerize all microservices and you `just` need to run a few recipies (see below) to get your devbox `up` and running. When your systems prerquisites are already met, usually in under 15 minutes.

### prerequisites

#### all

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

### first setup

```bash
git clone https://github.com/ordilabs/live live--ordilabs
cd live--ordilabs
just setup-once
```

### dev environment

Run each command in a seperate terminal

```bash
# start up dev environment
just up 

# reset
## just down

# watch code for changes, recompile, refresh frontend
just watch

# open all .local domains in a brwoser
just open
```

### just commands

Once up and running you can perform dev tasks

```bash
# create temporary tunnel to expose your .local on the internet
just tunnel

# mine 1 block
just generate 1

# inscribe a punk
just punk-1
```

see more commands with `just -l`
