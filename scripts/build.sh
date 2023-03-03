# echo "Install Rust, Cargo, Trunk and Build"

curl https://sh.rustup.rs -sSf | sh -s -- -y && rustup target add wasm32-unknown-unknown && cargo install --locked trunk && trunk build --release
