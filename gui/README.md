# URL Checker GUI - Desktop Application

Professional desktop GUI application for URL Checker built with Tauri.

## ⚠️ Important: How to Run

**You MUST run this application through Tauri, not just with `npm run dev`!**

The correct command is:
```bash
npm run tauri:dev
```

Running `npm run dev` alone will show an error because Tauri API is not available in a regular browser.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (v16 or later)
- [npm](https://www.npmjs.com/) or [yarn](https://yarnpkg.com/)

### Linux System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install -y webkit2gtk3-devel.x86_64 \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3 \
    librsvg2-devel
```

**Arch Linux:**
```bash
sudo pacman -S webkit2gtk \
    base-devel \
    curl \
    wget \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips
```

## Installation

1. Navigate to the GUI directory:
```bash
cd gui
```

2. Install dependencies:
```bash
npm install
```

3. Install Tauri CLI (if not already installed):
```bash
cargo install tauri-cli
```

## Development

**Run the application in development mode (CORRECT WAY):**

```bash
npm run tauri:dev
```

This will:
- Start the Vite dev server
- Launch the Tauri desktop application
- Enable hot-reload for development

**DO NOT use `npm run dev` alone** - it will only start the web server without Tauri, causing errors.

## Building

Build the application for production:

```bash
npm run tauri:build
```

This will create platform-specific installers in `gui/src-tauri/target/release/`.

## Features

- Beautiful, modern UI
- Real-time URL checking
- Detailed statistics dashboard
- Configurable concurrency and timeout
- Results table with color-coded status
- Cross-platform (Windows, macOS, Linux)

## Troubleshooting

### Error: `window.__TAURI_IPC__ is not a function`

This means you're running the app without Tauri. Use `npm run tauri:dev` instead of `npm run dev`.

### Tauri CLI not found

Install it with:
```bash
cargo install tauri-cli
```

### Build errors

**Missing system libraries (Linux):**

If you see errors like:
- `javascriptcoregtk-4.0 not found`
- `libsoup-2.4 not found`

Install the required system dependencies (see Prerequisites section above).

**For Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

Make sure you have all prerequisites installed:
- Rust toolchain
- Node.js and npm
- Platform-specific build tools (see [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))
