# echo "Install Rust, Cargo, Trunk and Build"

yum install rust cargo && source $HOME/.cargo/env && rustup target add wasm32-unknown-unknown && cargo install --locked trunk && trunk build --release
