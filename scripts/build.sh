# echo "Install Rust, Cargo, Trunk and Build"

yum install sudo && sudo curl https://sh.rustup.rs -sSf | sh -s -- -y && sudo rustup target add wasm32-unknown-unknown && sudo cargo install --locked trunk && trunk build --release
