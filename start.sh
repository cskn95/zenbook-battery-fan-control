#!/bin/bash

# Load Rust environment variables (if they exist)
# Use '.' (dot command) for POSIX compatibility instead of 'source'
if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

# Check if Rust is installed
if ! command -v rustc &> /dev/null
then
    # If Rust is not found
    echo "Rust not found. Would you like to install it? (Y/N)"
    read -r answer

    # If the user answers 'Y' or 'y'
    if [[ "$answer" == "Y" || "$answer" == "y" ]]; then
        echo "Installing Rust..."
        # Download and install Rust via rustup, automatically confirming prompts
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        # Load environment variables immediately after installation using '.'
        if [ -f "$HOME/.cargo/env" ]; then
             . "$HOME/.cargo/env"
        fi
        echo "Rust has been installed! You may need to restart the terminal to use it."
    else
        # If the user does not want to install
        echo "Rust was not installed. Exiting..."
        exit 1 # Exit with an error code
    fi
else
    # If Rust is already installed
    echo "Rust is already installed."
fi

# --- Build Step ---
echo "Building the project with Cargo..." # Mesaj güncellendi
# Projeyi derle (release modunda)
cargo build --release

# Derlemenin başarılı olup olmadığını kontrol et
if [ $? -eq 0 ]; then
    echo "Build successful."

    # --- Installation Step ---
    source_bin="./target/release/zenbook-cli" # <<< Dosya adı düzeltildi
    local_bin_dir="$HOME/.local/bin"          # Kullanıcı için standart binary dizini
    dest_bin="$local_bin_dir/zenbook-cli"     # Hedef dosya yolu

    # Derlenen dosyanın var olup olmadığını kontrol et
    if [ -f "$source_bin" ]; then
        echo "Installing zenbook-cli to $local_bin_dir..."

        # Hedef dizini oluştur (varsa hata verme, yoksa oluştur)
        mkdir -p "$local_bin_dir"

        # Derlenmiş dosyayı .local/bin dizinine kopyala
        cp "$source_bin" "$dest_bin"

        # Kopyalamanın başarılı olup olmadığını kontrol et
        if [ $? -eq 0 ]; then
            echo "Successfully installed zenbook-cli to $dest_bin"
            echo ""
            echo "You should now be able to run 'zenbook-cli' commands directly from your terminal."
            echo "(e.g., 'zenbook-cli --help', 'zenbook-cli --battery optimal')"
            echo ""
            echo "If the command 'zenbook-cli' is not found:"
            echo "  1. Try opening a NEW terminal window/tab."
            echo "  2. If that doesn't work, ensure '$HOME/.local/bin' is in your PATH."
            echo "     You might need to log out and log back in, or add 'export PATH=\"\$HOME/.local/bin:\$PATH\"' to your ~/.bashrc or ~/.profile and run 'source ~/.bashrc'."

        else
            echo "Error: Failed to copy binary from $source_bin to $dest_bin."
            echo "Please check permissions."
            exit 1
        fi
    else
        echo "Error: Compiled binary not found at $source_bin."
        exit 1
    fi
    # --- End Installation Step ---

else
    echo "An error occurred during the build (cargo build --release failed)." # Hata mesajı detaylandırıldı
    exit 1
fi

echo ""
echo "Setup complete!" # Son mesaj güncellendi
exit 0