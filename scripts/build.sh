echo "Install Rust, Cargo, Trunk and Build"
amazon-linux-extras install rust1 && rustup target add wasm32-unknown-unknown && cargo install --locked trunk && trunk build --release
