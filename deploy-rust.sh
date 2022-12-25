#/bin/sh
# see doc https://gist.github.com/belst/ff36c5f3883f7bf9b06c379d0a7bed9e
source "$HOME/.cargo/env"
timestamp=$(date +"%Y-%m-%d-%H-%M-%S")
mkdir -p deployments-rust/$timestamp
cp bundle-rust.zip deployments-rust/$timestamp
cd deployments-rust/$timestamp
unzip bundle-rust.zip
#cargo install --path . 
rustup run nightly cargo build --release