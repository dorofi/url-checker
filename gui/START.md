# Quick Start Guide

## Step 1: Install System Dependencies (Linux)

**Ubuntu/Debian/Pop!_OS:**
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
./install-deps.sh
```

## Step 2: Free Port 1420 (if needed)

If you get "Port 1420 is already in use" error:
```bash
./kill-port.sh
```

## Step 3: Run the Application

```bash
npm run tauri:dev
```

## What to Expect

1. Vite dev server will start on port 1420
2. Rust dependencies will compile (first time takes 5-10 minutes)
3. Tauri window will open automatically
4. You can now use the GUI application!

## Troubleshooting

**Port 1420 in use:**
```bash
./kill-port.sh
```

**Missing system libraries:**
Install dependencies (see Step 1)

**Icons error:**
Icons are already created in `src-tauri/icons/`

**First build is slow:**
This is normal - Rust is compiling many dependencies. Subsequent builds are much faster.

