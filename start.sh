#!/bin/bash
if ! command -v rustc &> /dev/null
then
    echo "Rust not found. Would you like to install it? (Y/N)"
    read -r answer

    if [[ "$answer" == "Y" || "$answer" == "y" ]]; then
        echo "Installing Rust..."
        echo "Rust has been installed! You may need to restart the terminal to use it."
    else
        echo "Rust was not installed. Exiting..."
    fi
else
    echo "Rust is already installed."
fi

cargo build --release

