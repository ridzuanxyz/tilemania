# Lexicons Directory

This directory contains word lists for TileMania.

## ⚠️ Important: Lexicon Required

TileMania requires a word list to function. Due to licensing restrictions, some lexicons cannot be distributed with the source code.

---

## 📥 Getting a Word List

### Option 1: TML - TileMania Lexicon (Included) ✅ **RECOMMENDED**

**Status:** ✅ **Included** (167,737 words)

**TML (TileMania Lexicon)** is a filtered version of RE-Enable containing only words that also exist in CSW24. This avoids US/UK spelling confusion (color vs colour).

**Specifications:**
- **Words:** 167,737 (97.3% of RE-Enable)
- **License:** Proprietary (included with TileMania)
- **Source:** RE-Enable filtered to CSW24 vocabulary
- **Purpose:** International English words, avoids regional variants
- **Format:** One word per line, lowercase
- **Location:** `assets/lexicons/TML.txt` ✅ **Already included!**

**Why use this:**
- Avoids confusing kids with US/UK spelling differences
- International English vocabulary (same as competitive play)
- Curated for educational purposes
- Included with TileMania

---

### Option 2: RE-ENABLE (Included) ✅

**Status:** ✅ **Included** (172,400 words)

**RE-ENABLE** is a modern, public domain recreation of the ENABLE word list.

**Download from source:**
```bash
curl -L -o assets/lexicons/RE-ENABLE.txt https://raw.githubusercontent.com/JakesMD/Re-Enable/main/re-enable.txt
```

**Or visit:** https://github.com/JakesMD/Re-Enable

**Specifications:**
- **Words:** 172,400
- **License:** Public domain (free for commercial use)
- **Source:** Modern recreation of ENABLE
- **Format:** One word per line, lowercase
- **Location:** `assets/lexicons/RE-ENABLE.txt` ✅ **Already included!**

**Includes:**
- US English spelling variants (color, honor, etc.)
- Some words NOT in CSW24 (regional variations)

---

### Option 3: ENABLE (Public Domain)

**Status:** Included (sample subset - full list needed)

The original ENABLE (Enhanced North American Benchmark LExicon) is **public domain** and free to use.

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

### Option 4: CSW24 (Licensed) 🔒

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

### Option 5: Custom Word List 📝

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
2. `TML.txt` (TileMania Lexicon - recommended)
3. `RE-ENABLE.txt` (public domain)
4. `ENABLE.txt` (public domain)
5. `custom.txt` (your own list)

The first found lexicon will be loaded.

---

## 📊 Lexicon Comparison

| Lexicon | Words | License | Cost | Best For |
|---------|-------|---------|------|----------|
| **TML** | 167K | Proprietary | **Included** | ✅ **Recommended** - International words, no US/UK confusion |
| **RE-ENABLE** | 172K | Public domain | **FREE** | General use with US variants |
| **ENABLE** | ~173K | Public domain | **FREE** | General use, educational |
| **CSW24** | ~280K | Licensed | $$$ | Tournament, competitive |
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
