#!/bin/bash
if ! command -v rustc &> /dev/null
then
    echo "Rust not found. Would you like to install it? (Y/N)"
    read -r answer

    if [[ "$answer" == "Y" || "$answer" == "y" ]]; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        source $HOME/.cargo/env
        echo "Rust has been installed! You may need to restart the terminal to use it."
    else
        echo "Rust was not installed. Exiting..."
        exit 1
    fi
else
    echo "Rust is already installed."
fi

echo "Installing dependencies..."
cargo build --release

echo "Running the program..."
./target/release/zenbook-battery-fan-control
