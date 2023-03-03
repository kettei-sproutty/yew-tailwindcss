# echo "Install Rust, Cargo, Trunk and Build"

curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"

whereis rustup
rustup target wasm32-unknown-unknown

whereis rust
whereis cargo
