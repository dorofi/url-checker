# Installing Tauri Dependencies on Linux

## Quick Install (Ubuntu/Debian/Pop!_OS)

Run this command to install all required dependencies:

```bash
sudo apt-get update && sudo apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    pkg-config
```

Or use the install script:

```bash
cd gui
./install-deps.sh
```

## What These Packages Do

- **libwebkit2gtk-4.0-dev** - WebKit engine for Tauri (provides javascriptcoregtk-4.0)
- **libsoup2.4-dev** - HTTP library (usually comes with webkit2gtk)
- **build-essential** - Compiler tools (gcc, make, etc.)
- **libgtk-3-dev** - GTK3 GUI toolkit
- **libayatana-appindicator3-dev** - System tray support
- **librsvg2-dev** - SVG rendering
- **pkg-config** - Package configuration tool

## After Installation

Once dependencies are installed, you can run:

```bash
cd gui
npm run tauri:dev
```

The first build will take several minutes as it compiles Rust dependencies.

