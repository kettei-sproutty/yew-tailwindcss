#!/bin/bash

whereis rustup
which rustup
whereis cargo
which cargo
whereis trunk
which trunk

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

# Check if Trunk is already installed
if command -v trunk >/dev/null 2>&1; then
  echo "Trunk is already installed"
else
  # Install Trunk
  cargo install --locked trunk
fi

# Build WebAssembly
trunk build --release

# Build CSS
if [ ! -f "dist/tailwind.css" ]; then
  pnpm cross-env NODE_ENV=production postcss src/styles/main.css -o "dist/tailwind.css" --minify
fi

