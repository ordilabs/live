# Ordi Labs Live

Ordinals mempool viewer. View inscriptions before they're inscribed!

## Setup

### macos Prerequisites

1. [XCode](https://developer.apple.com/support/xcode/)
2. [Homebrew](https://brew.sh/)
3. [Docker](https://docker.com/)
4. [Rust](https://rustup.rs/)
5. [Just](https://just.systems/)
`brew install just`

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
