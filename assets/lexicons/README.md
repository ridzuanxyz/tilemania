# Lexicons Directory

This directory contains word lists for TileMania.

## ⚠️ Important: Lexicon Required

TileMania requires a word list to function. Due to licensing restrictions, some lexicons cannot be distributed with the source code.

---

## 📥 Getting a Word List

### Option 1: ENABLE (Public Domain) ✅ **RECOMMENDED**

**Status:** Included (sample subset - full list needed)

The ENABLE (Enhanced North American Benchmark LExicon) is **public domain** and free to use.

**Download Full ENABLE:**
```bash
curl -o assets/lexicons/ENABLE.txt http://www.puzzlers.org/pub/wordlists/enable1.txt
```

**Or visit:** http://www.puzzlers.org/pub/wordlists/enable1.txt

**Specifications:**
- **Words:** ~173,000
- **License:** Public domain (free for commercial use)
- **Quality:** High-quality word list for word games
- **Format:** One word per line, lowercase

---

### Option 2: CSW24 (Licensed) 🔒

**Status:** Not included (requires license)

CSW24 (Collins Scrabble Words 2024) is the official competitive word list.

**To Use CSW24:**
1. Purchase/license CSW24 from:
   - WESPA: https://www.wespa.org/
   - HarperCollins Publishers
2. Save as: `assets/lexicons/CSW24.txt`
3. Format: One word per line, uppercase

**Cost:** Contact WESPA for licensing terms

**When to use:**
- Competitive/tournament mode
- Serious players
- Official word validation

---

### Option 3: TWL (Licensed) 🔒

**Status:** Not included (requires license)

TWL (Tournament Word List) is the North American official list.

**To Use TWL:**
1. License from Hasbro/NASPA
2. Save as: `assets/lexicons/TWL.txt`

---

### Option 4: Custom Word List 📝

**Status:** User-provided

You can provide your own custom word list.

**Requirements:**
- One word per line
- Text file format (.txt)
- Encoding: UTF-8
- Case: Any (will be normalized to uppercase)

**Save as:** `assets/lexicons/custom.txt`

**Example:**
```
AA
AB
WORD
EXAMPLE
```

---

## 🔄 Auto-Detection

TileMania automatically tries to load lexicons in this priority order:

1. `CSW24.txt` (if you have license)
2. `ENABLE.txt` (public domain)
3. `TWL.txt` (if available)
4. `custom.txt` (your own list)

The first found lexicon will be loaded.

---

## 📊 Lexicon Comparison

| Lexicon | Words | License | Cost | Best For |
|---------|-------|---------|------|----------|
| **ENABLE** | ~173K | Public domain | **FREE** | ✅ General use, educational |
| **CSW24** | ~280K | Licensed | $$$ | Tournament, competitive |
| **TWL** | ~187K | Licensed | $$$ | North American tournaments |
| **Custom** | Varies | Your choice | FREE | Specialized/educational |

---

## ⚖️ Legal Information

**Can I use CSW24 without a license?**
❌ No. CSW24 is copyrighted by HarperCollins. Using without a license is copyright infringement.

**Can I distribute ENABLE?**
✅ Yes. ENABLE is public domain and free to distribute.

**Can I create my own word list?**
✅ Yes. You can curate from public domain sources.

See `LEGAL_CONSIDERATIONS.md` in the project root for more details.

---

## 🚀 Quick Start

**Just want to play?** Use ENABLE (public domain):

```bash
# Download ENABLE
curl -o assets/lexicons/ENABLE.txt http://www.puzzlers.org/pub/wordlists/enable1.txt

# Run the game
cargo run --release
```

---

## 📝 File Format

All lexicons should follow this format:

```
# Plain text file
# One word per line
# Any case (normalized internally)
# UTF-8 encoding
# No special characters

AA
AB
AARDVARK
...
ZYMURGY
```

**Notes:**
- Empty lines are ignored
- Whitespace is trimmed
- Case doesn't matter (converted to uppercase internally)
- Words can be 2-15 letters long (game limitation)

---

## 🔍 Verification

To verify your lexicon loaded correctly:

```bash
cargo run --release 2>&1 | grep "Loaded"
```

Expected output:
```
✓ Loaded ENABLE lexicon with 173528 words from assets/lexicons/ENABLE.txt
```

---

## 🆘 Troubleshooting

**Error: "No word list found!"**
- Make sure you have at least one lexicon file in this directory
- Check filename matches exactly (case-sensitive on Linux)
- Verify file format (one word per line, text file)

**Game seems to reject valid words:**
- Check which lexicon loaded (see output at startup)
- ENABLE is American English focused
- Some obscure words may not be in all lists

**Words with special characters don't work:**
- Remove accents/diacritics
- Only A-Z letters supported
- Apostrophes not supported

---

**Last Updated:** 2025-11-19
