# Quick Start Guide

## ⚠️ IMPORTANT: How to Run the GUI

The GUI application **MUST** be run through Tauri, not in a regular browser!

### Step 1: Install Dependencies

```bash
cd gui
npm install
```

### Step 2: Run the Application

```bash
npm run tauri:dev
```

**DO NOT** use `npm run dev` - that will only start a web server without Tauri!

### What Happens:

1. `npm run tauri:dev` starts the Vite dev server
2. Tauri compiles the Rust backend
3. A desktop window opens with the GUI
4. You can now use the application!

### Troubleshooting

**Error: "Tauri is required. Run: npm run tauri:dev"**

This means you're running the app in a browser instead of Tauri. Make sure you:
1. Use `npm run tauri:dev` (not `npm run dev`)
2. Wait for the Tauri window to open
3. Don't open `http://localhost:1420` in a browser

**Error: "tauri: not found"**

Install Tauri CLI:
```bash
cargo install tauri-cli
```

**First run is slow**

The first time you run `npm run tauri:dev`, it will download and compile many Rust dependencies. This can take 5-10 minutes. Subsequent runs will be much faster.

