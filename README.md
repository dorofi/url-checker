# 🌐 URL Checker - Professional Web Status Monitor

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)](https://github.com/dorofi/url-checker)

A high-performance, asynchronous URL monitoring tool written in Rust. Check hundreds of URLs simultaneously with beautiful terminal output, detailed statistics, and comprehensive reporting.

## ✨ Features

- ⚡ **Lightning Fast** - Concurrent async requests with configurable concurrency
- 🎨 **Beautiful Output** - Color-coded terminal interface with progress bars and tables
- 📊 **Detailed Statistics** - Response times, success rates, data transfer metrics
- 📁 **Multiple Export Formats** - CSV and JSON reports with timestamps and metadata
- 🖥️ **Desktop GUI Application** - Beautiful cross-platform desktop app built with Tauri
- 🌐 **Web Landing Page** - Professional HTML page explaining the project
- 🔧 **Highly Configurable** - Customizable timeouts, concurrency, and output formats
- 🖥️ **Cross-Platform** - Works on Linux, Windows, and macOS
- 🎯 **Professional CLI** - Clean command-line interface with helpful options

## 🚀 Quick Start

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later
- Internet connection

### Installation

```bash
# Clone the repository
git clone https://github.com/dorofi/url-checker.git
cd url-checker

# Build the project
cargo build --release
```

### Basic Usage

1. **Create a URLs file** (`urls.txt`):
   ```
   https://google.com
   https://github.com
   https://rust-lang.org
   https://example.com
   ```

2. **Run the checker**:
   ```bash
   cargo run --release
   ```

3. **View results**:
   - Terminal output with color-coded results
   - CSV report saved to `report.csv`

## 📖 Usage Examples

### Basic Check
```bash
cargo run --release
```

### Custom Input/Output Files
```bash
cargo run --release -- -i my-urls.txt -o my-report.csv
```

### High Concurrency (50 simultaneous requests)
```bash
cargo run --release -- -c 50
```

### Custom Timeout (30 seconds)
```bash
cargo run --release -- -t 30
```

### All Options Combined
```bash
cargo run --release -- -i urls.txt -o report.csv -c 50 -t 30
```

### Export to JSON
```bash
cargo run --release -- -i urls.txt -o report.json --format json
```

## 🎯 Command Line Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--input` | `-i` | `urls.txt` | Input file with URLs (one per line) |
| `--output` | `-o` | `report.csv` | Output file path (CSV or JSON) |
| `--format` | `-f` | `csv` | Export format: `csv` or `json` |
| `--concurrency` | `-c` | `20` | Number of concurrent requests |
| `--timeout` | `-t` | `10` | Request timeout in seconds |

## 📊 Output Format

### Terminal Output

The tool provides a professional terminal interface with:
- **Color-coded status codes**: 
  - 🟢 Green (2xx) - Success
  - 🟡 Yellow (3xx) - Redirect
  - 🔴 Red (4xx/5xx) - Errors
- **Progress bar** with percentage and ETA
- **Detailed statistics** including:
  - Total URLs checked
  - Success/failure rates
  - Average, min, and max response times
  - Total data transferred

### CSV Report

The CSV report includes:
- URL
- HTTP Status Code
- Status Reason
- Response Time (ms)
- Response Size (bytes)
- Timestamp (UTC)

## 🖥️ Desktop GUI Application

A beautiful cross-platform desktop application is available in the `gui/` directory!

### What is Tauri and How It Works?

**Tauri** is a framework for building desktop applications using web technologies (HTML, CSS, JavaScript) with a Rust backend. Here's how it works:

1. **Frontend (Web UI)**: HTML/CSS/JavaScript interface (like a website)
2. **Backend (Rust)**: Handles the actual URL checking logic
3. **Bridge**: Tauri provides secure communication between frontend and backend
4. **Native Window**: The app runs in a native desktop window (not a browser)

**Why not just a website?**
- Desktop apps have better performance
- Can access system resources securely
- No need for a web server
- Better user experience with native window controls

### GUI Features
- Modern, intuitive interface
- Real-time URL checking with live updates
- Visual statistics dashboard
- Color-coded status indicators
- Configurable settings panel
- Results table with sorting

### Running the Desktop GUI Application

#### Prerequisites

**1. Install System Dependencies (Linux - Ubuntu/Debian/Pop!_OS):**
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

**For other Linux distributions, see `gui/INSTALL_LINUX.md`**

**2. Install Tauri CLI:**
```bash
cargo install tauri-cli
```

#### Installation Steps

1. **Navigate to the GUI directory:**
```bash
cd gui
```

2. **Install Node.js dependencies:**
```bash
npm install
```

3. **Run in development mode:**
```bash
npm run tauri:dev
```

**Important:** Use `npm run tauri:dev`, NOT `npm run dev`! The latter only starts a web server without Tauri backend.

#### What Happens When You Run `npm run tauri:dev`:

1. **Vite dev server starts** on `http://localhost:1420` (serves the frontend)
2. **Rust backend compiles** (first time takes 5-10 minutes)
3. **Tauri window opens** automatically with the GUI
4. **Hot-reload enabled** - changes to code update automatically

#### Building for Production

```bash
cd gui
npm run tauri:build
```

This creates platform-specific installers:
- **Linux**: `.deb`, `.AppImage`, or `.rpm`
- **Windows**: `.msi` installer
- **macOS**: `.dmg` or `.app`

#### Troubleshooting

**Port 1420 already in use:**
```bash
cd gui
./kill-port.sh
```

**Missing system libraries:**
Install dependencies (see Prerequisites above)

**"Tauri is required" error:**
Make sure you're using `npm run tauri:dev`, not `npm run dev`

**First build is slow:**
Normal! Rust compiles many dependencies. Subsequent builds are much faster.

See `gui/README.md` and `gui/START.md` for more detailed instructions.

## 🏗️ Architecture

- **Async Runtime**: Tokio for high-performance async I/O
- **HTTP Client**: Reqwest with rustls (no OpenSSL dependency)
- **Concurrency**: Futures stream with configurable buffer
- **Error Handling**: Comprehensive error reporting with context
- **GUI Framework**: Tauri for lightweight, secure desktop apps

## 🛠️ Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
cargo test
```

## 📝 Project Structure

```
url-checker/
├── src/
│   └── main.rs          # Main CLI application logic
├── gui/                 # Desktop GUI application
│   ├── src/             # Frontend (HTML, CSS, JavaScript)
│   │   ├── main.js      # Frontend logic
│   │   └── styles.css   # Styling
│   ├── src-tauri/       # Tauri backend (Rust)
│   │   ├── src/
│   │   │   └── main.rs  # Rust backend with URL checking logic
│   │   ├── icons/       # Application icons
│   │   └── Cargo.toml   # Rust dependencies
│   ├── index.html       # GUI application HTML
│   ├── package.json     # Node.js dependencies
│   ├── vite.config.js   # Vite configuration
│   ├── install-deps.sh  # Linux dependencies installer
│   └── kill-port.sh     # Port cleanup script
├── Cargo.toml           # CLI dependencies and metadata
├── README.md            # This file (main documentation)
├── index.html           # Web landing page (informational)
├── LICENSE              # MIT License
└── urls.txt             # Example URLs file
```

## 🔍 Understanding the Architecture

### Command Line Interface (CLI)
- **Location**: `src/main.rs`
- **Technology**: Pure Rust
- **Usage**: Run directly from terminal
- **Output**: Terminal with colors + CSV/JSON files

### Desktop GUI Application
- **Frontend**: `gui/src/` (HTML, CSS, JavaScript)
- **Backend**: `gui/src-tauri/src/main.rs` (Rust)
- **Framework**: Tauri (combines web UI with Rust backend)
- **How it works**:
  1. Frontend (JavaScript) sends requests to Rust backend via Tauri API
  2. Rust backend performs URL checking (same logic as CLI)
  3. Results sent back to frontend for display
  4. Runs in native desktop window (not browser)

### Web Landing Page
- **Location**: `index.html` (root directory)
- **Purpose**: Informational/documentation page
- **Note**: This is NOT a working application - it's just documentation
- **Why it doesn't work**: The actual URL checking requires Rust backend, which cannot run in a browser

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**dorofi**

- GitHub: [@dorofi](https://github.com/dorofi)
- Email: dorofii.2005@gmail.com

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [Tokio](https://tokio.rs/) for async runtime
- Uses [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP client
- Uses [Clap](https://github.com/clap-rs/clap) for CLI parsing

## 📈 Future Enhancements

- [x] GUI desktop application
- [x] JSON export format
- [ ] Real-time monitoring mode
- [ ] Email notifications
- [ ] Web dashboard
- [ ] Docker containerization
- [ ] CI/CD integration examples
- [ ] Scheduled checks
- [ ] History tracking
- [ ] Alert system

---

⭐ If you find this project useful, please consider giving it a star!
