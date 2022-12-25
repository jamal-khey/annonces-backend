#/bin/sh
sudo apt-get update
sudo apt install build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup override set nightly
# if application stop building , run  rustup update && cargo update
