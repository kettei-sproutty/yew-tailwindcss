# echo "Install Rust, Cargo, Trunk and Build"

curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"

whereis rustup
rustup target add wasm32-unknown-unknown

yum install wget -y

wget -qO- https://github.com/thedodd/trunk/releases/download/0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

whereis trunk

trunk build --release
