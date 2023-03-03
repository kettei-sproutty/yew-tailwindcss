# echo "Install Rust, Cargo, Trunk and Build"

curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"

whereis rustup
rustup target add wasm32-unknown-unknown

cargo install trunk wasm-bindgen-cli
if [ ! -f "dist/tailwind.css" ]; then
  pnpm cross-env NODE_ENV=production postcss src/styles/main.css -o "dist/tailwind.css" --minify
fi


trunk build --release
