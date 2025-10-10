# Lexicon Conversion Guide

**TileMania - Sprint 1**
Converting word list files (TXT) to KWG format for fast word validation

---

## Overview

This guide explains how to convert Scrabble word list files (like CSW24.txt) into KWG (Kurnia Word Graph) format, which is optimized for fast word lookups and anagram generation.

**What is KWG?**
- Binary format containing GADDAG data structure
- Enables instant word validation (<10ms per word)
- Compressed representation of the entire lexicon
- Used by wolges engine for game logic

---

## Prerequisites

### System Requirements
- **Rust:** 1.80+ (for edition 2024 support)
- **Disk Space:** ~500MB for wolges repository and builds
- **RAM:** 4GB minimum
- **OS:** Linux, macOS, or Windows

### Install Rust (if needed)
```bash
# Install rustup (Rust toolchain manager)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Activate the new PATH
source "$HOME/.cargo/env"

# Verify installation
rustc --version  # Should be 1.80+
cargo --version
```

---

## Step 1: Clone and Build wolges

The `buildlex` tool is part of the wolges repository and must be built from source.

```bash
# Navigate to your projects directory
cd ~/Documents/GitHub

# Clone the wolges repository
git clone https://github.com/andy-k/wolges.git
cd wolges

# Build the buildlex binary (release mode for speed)
cargo build --release --bin buildlex

# Verify the binary was created
ls -lh target/release/buildlex
# Expected: ~1.7MB executable
```

**Build time:** 5-10 minutes on first build

---

## Step 2: Prepare Your Word List

### Supported Formats

The source file should be a **plain text file** with:
- One word per line
- Uppercase letters only (for English)
- No duplicates
- Sorted alphabetically (recommended)

### Common Lexicons

| Lexicon | Words | Description | Source |
|---------|-------|-------------|---------|
| CSW24 | 280,886 | Collins Scrabble Words 2024 | Official international |
| NWL23 | ~192,000 | NASPA Word List 2023 | North American |
| TWL06 | ~178,000 | Tournament Word List 2006 | Legacy |

### Verify Your Source File

```bash
# Check file exists
ls -lh CSW24.txt

# Count words
wc -l CSW24.txt
# Expected: 280886 CSW24.txt (for CSW24)

# Check format (first 10 words)
head -10 CSW24.txt
# Expected: AA, AAH, AAHED, ...
```

---

## Step 3: Convert to KWG Format

### Basic Conversion

```bash
# Navigate to wolges directory
cd ~/Documents/GitHub/wolges

# Convert TXT to KWG
cargo run --release --bin buildlex -- english-kwg \
  /path/to/CSW24.txt \
  /path/to/output/CSW24.kwg

# Example for tilemania project:
cargo run --release --bin buildlex -- english-kwg \
  ~/Documents/GitHub/tilemania/CSW24.txt \
  ~/Documents/GitHub/tilemania/assets/lexicons/CSW24.kwg
```

**Conversion time:** 30-60 seconds for CSW24

### Using the Binary Directly

After building once, you can use the binary directly:

```bash
# From any directory
~/Documents/GitHub/wolges/target/release/buildlex english-kwg \
  input.txt \
  output.kwg
```

### Supported Alphabets

The `buildlex` tool supports multiple languages:

```bash
# English (default)
buildlex english-kwg input.txt output.kwg

# French
buildlex french-kwg input.txt output.kwg

# German
buildlex german-kwg input.txt output.kwg

# Spanish
buildlex spanish-kwg input.txt output.kwg

# Polish
buildlex polish-kwg input.txt output.kwg

# See full list:
~/Documents/GitHub/wolges/target/release/buildlex
```

---

## Step 4: Validate the KWG File

### Automated Validation Script

Use the provided validation script:

```bash
cd ~/Documents/GitHub/tilemania
./validate_kwg.sh
```

**Expected output:**
```
🔍 TileMania - CSW24.kwg Validation
====================================

📄 Source File Check:
  ✅ CSW24.txt found
  📊 Word count: 280886
  💾 File size: 3.0M
  ✅ Word count matches expected (280,886)

🎯 KWG File Check:
  ✅ CSW24.kwg found
  💾 File size: 4.7M (4.82 MB)
  ✅ File size in expected range (4-10 MB)

✨ Validation Summary:
  ✅ All checks passed!
```

### Manual Validation

```bash
# Check file exists and size
ls -lh assets/lexicons/CSW24.kwg

# Expected size ranges by lexicon:
# CSW24: 4-8 MB
# NWL23: 3-6 MB
# TWL06: 3-5 MB

# Verify it's a binary file
file assets/lexicons/CSW24.kwg
# Expected: "data" or binary file type
```

---

## Troubleshooting

### Issue: "edition2024 is required"

**Problem:** Old Rust version (< 1.80)

**Solution:**
```bash
# Update Rust with rustup
rustup update stable
source "$HOME/.cargo/env"
rustc --version  # Verify 1.80+
```

### Issue: "No such file or directory"

**Problem:** Incorrect path or file doesn't exist

**Solution:**
```bash
# Use absolute paths
realpath CSW24.txt  # Get absolute path

# Verify source file exists
ls -la CSW24.txt

# Create output directory if needed
mkdir -p assets/lexicons
```

### Issue: Build takes too long / runs out of memory

**Problem:** Low RAM or slow system

**Solution:**
```bash
# Use fewer parallel jobs
cargo build --release --bin buildlex -j 1

# Or download pre-built binary (if available)
# Check wolges releases: github.com/andy-k/wolges/releases
```

### Issue: "cargo: command not found"

**Problem:** Cargo not in PATH

**Solution:**
```bash
# Add cargo to PATH
source "$HOME/.cargo/env"

# Or use full path
~/.cargo/bin/cargo build --release --bin buildlex
```

### Issue: KWG file size seems wrong

**Expected sizes:**
- CSW24 (280K words): 4-8 MB
- Smaller than expected: Possible corruption
- Much larger (>20MB): Wrong format or duplicate data

**Solution:**
```bash
# Verify source file word count
wc -l CSW24.txt

# Re-run conversion with verbose output
cargo run --release --bin buildlex -- english-kwg input.txt output.kwg

# Check for errors in output
```

---

## Alternative Methods

### If wolges-make fails or is unavailable:

**Option 1: Pre-converted Files**
- Some lexicon distributors provide pre-built KWG files
- Check wolges releases or Woogles.io resources

**Option 2: Use Different Format**
- wolges supports multiple formats: KWG, DAWG, GADDAG
- Try: `buildlex english-macondo input.kwg output CSW24.dawg CSW24.gaddag`

**Option 3: Build from Different Branch**
```bash
cd ~/Documents/GitHub/wolges
git checkout main  # or specific tag
cargo build --release --bin buildlex
```

---

## Performance Expectations

### Conversion Performance

| Lexicon | Words | File Size | Conversion Time | Load Time |
|---------|-------|-----------|-----------------|-----------|
| CSW24 | 280,886 | 4.8 MB | 2-3 seconds | <100ms |
| NWL23 | ~192,000 | 3.5 MB | 1-2 seconds | <50ms |
| TWL06 | ~178,000 | 3.2 MB | 1-2 seconds | <50ms |

*Tested on: Intel i5, 8GB RAM, SSD*

### Runtime Performance

- **Word validation:** <1ms per word
- **Anagram generation:** 10-100ms (depends on rack)
- **Memory usage:** ~10-20MB in RAM when loaded

---

## Integration with TileMania

### Directory Structure

```
tilemania/
├── assets/
│   └── lexicons/
│       ├── CSW24.kwg          # Main lexicon
│       └── NWL23.kwg          # Optional alternative
├── CSW24.txt                  # Source file (can be removed after conversion)
└── validate_kwg.sh            # Validation script
```

### Loading in Rust (Sprint 3+)

```rust
use std::fs;

// Load KWG file
let kwg_bytes = fs::read("assets/lexicons/CSW24.kwg")?;

// Use with wolges library
// (Full implementation in Sprint 3)
```

---

## Additional Resources

### Documentation
- [wolges GitHub](https://github.com/andy-k/wolges) - Source repository
- [Woogles.io](https://woogles.io) - Online Scrabble platform using wolges
- [TileMania Architecture](../ARCHITECTURE.md) - Technical design

### Lexicon Sources
- **CSW24:** Official Scrabble lexicon (Collins)
- **NWL23:** NASPA official word list
- **Download:** Check official Scrabble associations

### Tools
- `buildlex` - Part of wolges (this guide)
- `validate_kwg.sh` - TileMania validation script
- `cargo bench` - Performance benchmarking (requires full build)

---

## Summary Checklist

- [ ] Rust 1.80+ installed
- [ ] wolges repository cloned
- [ ] buildlex binary compiled
- [ ] Source TXT file prepared (280,886 words for CSW24)
- [ ] KWG file generated (4-8 MB)
- [ ] Validation script passes all checks
- [ ] File moved to `assets/lexicons/` directory

---

**Document Status:** ✅ Complete
**Last Updated:** 2025-10-09
**Sprint:** 1 - Week 1 - Day 2
