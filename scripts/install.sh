#!/bin/bash

pnpm install

if test -f "$HOME/.cargo/env"; then
  . "$HOME/.cargo/env"
fi

# Check if Rust is already installed
if command -v rustup >/dev/null 2>&1; then
  echo "Rust is already installed"
else
  # Install Rust and Cargo
  curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable
  source "$HOME/.cargo/env"
fi

rustup target add wasm32-unknown-unknown

cargo xtask dist
