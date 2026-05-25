# Setup Guide

## Prerequisites

- [Rust](https://rustup.rs) (install with `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)

## Install

```bash
# Clone the repo
git clone <repo-url>
cd awk_interpreter

# Build and install the binary
cargo install --path rust

# Add cargo bin to PATH (if not already done)
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
source ~/.cargo/env
```

## Usage

```bash
awk_rust '{print}' tests/test1.txt
awk_rust '{print $1}' tests/test1.txt
awk_rust '{print $1, $2}' tests/test1.txt
awk_rust '{print $NF}' tests/test1.txt
awk_rust '{print NR, NF, $0}' tests/test1.txt
awk_rust '{print "hello", $1}' tests/test1.txt
```

## Reinstall after changes

```bash
cargo install --path rust
```
