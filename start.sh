#!/bin/bash

# Color codes
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
RESET='\033[0m'  # Reset to default color

# Load Rust environment variables (if they exist)
if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null
then
    echo -e "${RED}Rust not found.${RESET} ${YELLOW}Would you like to install it? (Y/N)${RESET}"
    read -r answer

    if [[ "$answer" == "Y" || "$answer" == "y" ]]; then
        echo -e "${CYAN}Installing Rust...${RESET}"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        if [ -f "$HOME/.cargo/env" ]; then
             . "$HOME/.cargo/env"
        fi
        echo -e "${GREEN}Rust successfully installed!${RESET} ${MAGENTA}You may need to restart the terminal to use it.${RESET}"
    else
        echo -e "${RED}Rust was not installed. Exiting...${RESET}"
        exit 1
    fi
else
    echo -e "${GREEN}Rust is already installed.${RESET}"
fi

# --- Build Step ---
echo -e "${BLUE}Building the project with Cargo...${RESET}"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}Build successful.${RESET}"

    # --- Installation Step ---
    source_bin="./target/release/zenbook-cli"
    local_bin_dir="$HOME/.local/bin"
    dest_bin="$local_bin_dir/zenbook-cli"

    if [ -f "$source_bin" ]; then
        echo -e "${CYAN}Installing zenbook-cli to ${MAGENTA}$local_bin_dir${CYAN}...${RESET}"
        mkdir -p "$local_bin_dir"
        cp "$source_bin" "$dest_bin"

        if [ $? -eq 0 ]; then
            echo -e "${GREEN}Successfully installed zenbook-cli to ${MAGENTA}$dest_bin${GREEN}.${RESET}"
            echo -e "${YELLOW}You should now be able to run 'zenbook-cli' commands directly from your terminal.${RESET}"
            echo -e "${CYAN}(Example: 'zenbook-cli --help', 'zenbook-cli --battery optimal')${RESET}"
        else
            echo -e "${RED}Error: Failed to copy binary. Please check permissions.${RESET}"
            exit 1
        fi
    else
        echo -e "${RED}Error: Compiled binary not found.${RESET}"
        exit 1
    fi

else
    echo -e "${RED}Error: 'cargo build --release' failed.${RESET}"
    exit 1
fi

echo -e "${GREEN}Setup complete!${RESET}"
exit 0
