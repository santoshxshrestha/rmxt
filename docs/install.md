# Installation

## Quick Install

```bash
cargo install rmxt
```

## From Source

```bash
git clone https://github.com/santoshxshrestha/rmxt
cd rmxt
cargo build --release
sudo cp target/release/rmxt /usr/local/bin/
```

## Using Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/santoshxshrestha/rmxt/main/scripts/install.sh | bash
```

## Using Nix Flakes

You can use [Nix flakes](https://nixos.wiki/wiki/Flakes) to run or build `rmxt` without installing Rust or any dependencies manually. This is the recommended method for Nix users.

**Prerequisites:**
- [Nix package manager](https://nixos.org/download.html) (version 2.4+)
- Flakes enabled (see [NixOS Wiki: Flakes](https://nixos.wiki/wiki/Flakes#Installing_flakes))

### Run `rmxt` directly (no install)

```bash
nix run github:santoshxshrestha/rmxt
```

Or, if you have cloned the repo:

```bash
nix run .
```

This will build and run the latest `rmxt` binary in a temporary environment.

### Build the binary

```bash
nix build github:santoshxshrestha/rmxt
```

Or, from a local clone:

```bash
nix build
```

The compiled binary will be available at `./result/bin/rmxt`.

### Development shell

For contributors, you can enter a shell with all development dependencies:

```bash
nix develop
```

### Notes
- The provided flake currently supports `x86_64-linux`.
- For more details on Nix flakes, see the [NixOS Wiki](https://nixos.wiki/wiki/Flakes) or [Practical Nix Flakes](https://serokell.io/blog/practical-nix-flakes).

## Verification

```bash
rmxt --help
```

## Updating

```bash
cargo install rmxt  # Updates to latest version
```
