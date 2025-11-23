#!/bin/bash

# Script to install Tauri dependencies for Linux

echo "Installing Tauri system dependencies for Linux..."

# Detect Linux distribution
if [ -f /etc/debian_version ]; then
    # Debian/Ubuntu
    echo "Detected Debian/Ubuntu system"
    sudo apt-get update
    sudo apt-get install -y \
        libwebkit2gtk-4.0-dev \
        build-essential \
        curl \
        wget \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        pkg-config
elif [ -f /etc/fedora-release ]; then
    # Fedora
    echo "Detected Fedora system"
    sudo dnf install -y \
        webkit2gtk3-devel.x86_64 \
        openssl-devel \
        curl \
        wget \
        libappindicator-gtk3 \
        librsvg2-devel \
        pkg-config
elif [ -f /etc/arch-release ]; then
    # Arch Linux
    echo "Detected Arch Linux system"
    sudo pacman -S --needed \
        webkit2gtk \
        base-devel \
        curl \
        wget \
        openssl \
        appmenu-gtk-module \
        gtk3 \
        libappindicator-gtk3 \
        librsvg \
        libvips \
        pkg-config
else
    echo "Unknown Linux distribution. Please install dependencies manually."
    echo "See gui/README.md for instructions."
    exit 1
fi

echo "Dependencies installed successfully!"
echo "You can now run: npm run tauri:dev"

