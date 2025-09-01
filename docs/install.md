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

## Verification

```bash
rmxt --help
```

## Updating

```bash
cargo install rmxt  # Updates to latest version
```