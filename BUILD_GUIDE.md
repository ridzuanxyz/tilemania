# TileMania Build Guide

**Version:** 1.0
**Date:** November 19, 2025
**Platforms:** Windows, macOS, Linux, Web (WASM)

Complete step-by-step guide for building and running TileMania from source.

---

## 📋 Table of Contents

1. [Prerequisites](#prerequisites)
2. [Quick Start](#quick-start)
3. [Platform-Specific Instructions](#platform-specific-instructions)
4. [Troubleshooting](#troubleshooting)
5. [Build Configurations](#build-configurations)
6. [WASM/Web Build](#wasmweb-build)

---

## Prerequisites

### Required Software

| Software | Minimum Version | Purpose |
|----------|----------------|---------|
| Rust | 1.70+ | Programming language |
| Cargo | (comes with Rust) | Build tool & package manager |
| Git | 2.0+ | Version control |

### Optional Software

| Software | Purpose |
|----------|---------|
| VS Code | Code editor (with rust-analyzer extension) |
| RustRover | JetBrains IDE for Rust |
| wasm-pack | For web builds |

---

## Quick Start

### 1. Install Rust

**All Platforms:**
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, then restart your terminal

# Verify installation
rustc --version  # Should show 1.70 or higher
cargo --version
```

**Windows Alternative:**
- Download from [rust-lang.org](https://www.rust-lang.org/tools/install)
- Run the installer
- Restart your terminal

### 2. Clone Repository

```bash
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania
```

### 3. Build & Run

```bash
# Development build (faster compilation, slower runtime)
cargo run

# Release build (slower compilation, faster runtime - RECOMMENDED)
cargo run --release
```

**First build will take 5-10 minutes** (downloads dependencies, compiles everything)

Subsequent builds are much faster (30 seconds - 2 minutes).

---

## Platform-Specific Instructions

### 🐧 Linux

#### Ubuntu / Debian

```bash
# Install system dependencies
sudo apt-get update
sudo apt-get install -y \
    libasound2-dev \
    libudev-dev \
    pkg-config \
    build-essential

# Clone and build
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania
cargo run --release
```

#### Fedora / RHEL

```bash
# Install dependencies
sudo dnf install -y \
    alsa-lib-devel \
    systemd-devel \
    pkg-config \
    gcc

# Clone and build
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania
cargo run --release
```

#### Arch Linux

```bash
# Install dependencies
sudo pacman -S alsa-lib systemd

# Clone and build
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania
cargo run --release
```

---

### 🍎 macOS

```bash
# No additional dependencies needed!
# Rust is all you need

# Clone and build
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania
cargo run --release
```

**Note:** First time you run, macOS may ask for permission to access:
- Keyboard input
- Display recording (for rendering)

Allow these in System Preferences → Security & Privacy.

---

### 🪟 Windows

#### Option 1: PowerShell (Recommended)

```powershell
# No additional dependencies needed!

# Clone and build
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania
cargo run --release
```

#### Option 2: WSL2 (Windows Subsystem for Linux)

Follow the Linux instructions above within WSL2.

**Visual Studio Build Tools (if needed):**
- Download from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/)
- Install "Desktop development with C++"
- Restart terminal

---

## Troubleshooting

### Common Issues

#### 1. "ALSA lib not found" (Linux)

**Error:**
```
error: failed to run custom build command for `alsa-sys`
```

**Solution:**
```bash
sudo apt-get install libasound2-dev
```

#### 2. "udev not found" (Linux)

**Error:**
```
error: failed to run custom build command for `libudev-sys`
```

**Solution:**
```bash
sudo apt-get install libudev-dev
```

#### 3. "pkg-config not found" (Linux)

**Error:**
```
error: failed to run custom build command
note: pkg-config was not found
```

**Solution:**
```bash
sudo apt-get install pkg-config
```

#### 4. Slow Compilation

**Problem:** First build takes forever

**Solutions:**
```bash
# Use faster linker (Linux/macOS)
cargo install -f cargo-binutils
rustup component add llvm-tools-preview

# Add to ~/.cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

# Or use mold (even faster)
sudo apt-get install mold  # Linux
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

#### 5. "Cannot find -lasound" (Linux)

**Error:**
```
/usr/bin/ld: cannot find -lasound
```

**Solution:**
```bash
# Install ALSA development libraries
sudo apt-get install libasound2-dev

# If still failing, check pkg-config
pkg-config --libs alsa
```

#### 6. Out of Disk Space

**Problem:** Build fails due to disk space

**Solution:**
```bash
# Clean build artifacts
cargo clean

# Remove old Rust toolchains
rustup toolchain list
rustup toolchain remove <old-version>

# Clean cargo cache
rm -rf ~/.cargo/registry
```

#### 7. Assets Not Found (Runtime)

**Error:**
```
Failed to load asset: fonts/FiraSans-Bold.ttf
```

**Solution:**
```bash
# Ensure you're running from project root
cd /path/to/tilemania
cargo run --release

# OR create placeholder assets (see ASSET_SPECIFICATIONS.md)
```

---

## Build Configurations

### Development Build (Debug)

```bash
# Fast compilation, slow runtime, includes debug symbols
cargo build

# Run development build
cargo run

# Run with environment variables
RUST_LOG=info cargo run

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo run
```

**When to use:**
- Active development
- Debugging issues
- Quick iteration

### Release Build (Optimized)

```bash
# Slow compilation, fast runtime, optimized
cargo build --release

# Run release build
cargo run --release

# Release binary location
./target/release/tilemania  # Linux/macOS
.\target\release\tilemania.exe  # Windows
```

**When to use:**
- Playtesting
- Distribution
- Performance testing

### Build Options

```bash
# Check for errors without building
cargo check

# Build with all optimizations (slower but faster runtime)
cargo build --release

# Build for specific target
cargo build --target x86_64-pc-windows-gnu

# Build with features
cargo build --release --features native

# Build everything (all targets, all features)
cargo build --all --all-features
```

---

## WASM/Web Build

### Prerequisites for Web Build

```bash
# Install wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
cargo install wasm-bindgen-cli

# Install basic-http-server (for testing)
cargo install basic-http-server
```

### Build for Web

```bash
# Build WASM binary
cargo build --release --target wasm32-unknown-unknown

# Generate JavaScript bindings
wasm-bindgen --out-dir ./web \
    --target web \
    ./target/wasm32-unknown-unknown/release/tilemania.wasm

# Create HTML file (web/index.html)
cat > web/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>TileMania</title>
    <style>
        body { margin: 0; padding: 0; background: #1a1a1a; }
        canvas { width: 100%; height: 100vh; display: block; }
    </style>
</head>
<body>
    <canvas id="bevy"></canvas>
    <script type="module">
        import init from './tilemania.js';
        init();
    </script>
</body>
</html>
EOF

# Serve locally
basic-http-server web
```

Visit: `http://127.0.0.1:4000`

### WASM Optimization

```bash
# Install wasm-opt
cargo install wasm-opt

# Optimize WASM binary
wasm-opt -Oz -o web/tilemania_opt.wasm web/tilemania_bg.wasm

# Check size
ls -lh web/*.wasm
```

---

## Testing

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test lexicon

# Run tests with output
cargo test -- --nocapture

# Run tests for specific stage
cargo test stage1::
cargo test stage3::difficulty

# Run doc tests
cargo test --doc
```

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench lexicon_bench
```

---

## Continuous Integration

### GitHub Actions (Example)

```yaml
# .github/workflows/build.yml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]

    steps:
    - uses: actions/checkout@v2

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable

    - name: Install Linux deps
      if: matrix.os == 'ubuntu-latest'
      run: |
        sudo apt-get update
        sudo apt-get install -y libasound2-dev libudev-dev

    - name: Build
      run: cargo build --release

    - name: Test
      run: cargo test
```

---

## Performance Profiling

### CPU Profiling (Linux)

```bash
# Install perf
sudo apt-get install linux-tools-generic

# Build with debug symbols
cargo build --release

# Profile
perf record --call-graph dwarf ./target/release/tilemania

# View report
perf report
```

### Memory Profiling

```bash
# Install valgrind
sudo apt-get install valgrind

# Run with memory profiling
valgrind --tool=massif ./target/release/tilemania

# View report
ms_print massif.out.*
```

---

## Clean Build

```bash
# Remove all build artifacts
cargo clean

# Remove Cargo.lock (use with caution)
rm Cargo.lock

# Full clean rebuild
cargo clean && cargo build --release
```

---

## Distribution

### Create Distributable Package

**Linux:**
```bash
cargo build --release
cp target/release/tilemania dist/
cp -r assets/ dist/
tar -czf tilemania-linux.tar.gz dist/
```

**Windows:**
```powershell
cargo build --release
Copy-Item target\release\tilemania.exe dist\
Copy-Item -Recurse assets\ dist\
Compress-Archive -Path dist\* -DestinationPath tilemania-windows.zip
```

**macOS:**
```bash
cargo build --release
mkdir -p TileMania.app/Contents/MacOS
cp target/release/tilemania TileMania.app/Contents/MacOS/
cp -r assets/ TileMania.app/Contents/Resources/
```

---

## Build Times Reference

Approximate build times on typical hardware:

| Build Type | First Build | Incremental | Hardware |
|------------|-------------|-------------|----------|
| Debug | 5-8 min | 30-60 sec | Intel i5, 8GB RAM |
| Release | 8-12 min | 1-2 min | Intel i5, 8GB RAM |
| Debug | 3-5 min | 15-30 sec | AMD Ryzen 7, 16GB RAM |
| Release | 5-8 min | 30-90 sec | AMD Ryzen 7, 16GB RAM |

---

## Build Size Reference

Approximate binary sizes:

| Platform | Debug | Release | Release (stripped) |
|----------|-------|---------|-------------------|
| Linux | ~80 MB | ~20 MB | ~15 MB |
| Windows | ~90 MB | ~25 MB | ~20 MB |
| macOS | ~85 MB | ~22 MB | ~18 MB |
| WASM | N/A | ~8 MB | ~6 MB (optimized) |

---

## Getting Help

**Build Issues:**
1. Check this guide's [Troubleshooting](#troubleshooting) section
2. Search GitHub Issues: https://github.com/ridzuanxyz/tilemania/issues
3. Check Bevy documentation: https://bevyengine.org/learn/
4. Rust community: https://users.rust-lang.org/

**Reporting Build Problems:**
Include:
- Operating system and version
- Rust version (`rustc --version`)
- Complete error message
- Output of `cargo --version`

---

**Last Updated:** November 19, 2025
**Status:** Tested on Linux (Ubuntu 22.04), macOS (Sonoma), Windows 11
**Next Update:** After WASM testing complete
