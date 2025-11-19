# 🚨 URGENT: CSW24.txt Copyright Issue

## ⚠️ Critical Problem

**CSW24.txt is currently tracked in your Git repository!**

This is a **copyright violation** - the CSW24 word list is copyrighted by HarperCollins and should NOT be distributed without a license.

---

## ✅ Immediate Actions Required

### Step 1: Remove from Git (But Keep Locally)

```bash
# Navigate to project
cd /home/user/tilemania

# Remove from git tracking (keeps local file)
git rm --cached CSW24.txt
git rm --cached assets/lexicons/CSW24.kwg

# Commit the removal
git commit -m "Remove copyrighted CSW24 lexicon files

CSW24 is copyrighted by HarperCollins. Users must provide
their own licensed word list. See LEGAL_CONSIDERATIONS.md"

# Push to remove from remote
git push origin <your-branch>
```

### Step 2: Update Documentation

Add to README.md and relevant docs:

```markdown
## ⚠️ Word List Not Included

Due to licensing restrictions, word lists are NOT included in this repository.

**To use TileMania, you must provide your own word list:**

### Option 1: Use Public Domain List
- Download ENABLE word list (public domain)
- Place in `assets/lexicons/ENABLE.txt`

### Option 2: License CSW24 (For competitive play)
- Contact: WESPA (wespa.org) or HarperCollins
- Obtain commercial license
- Place CSW24.txt in `assets/lexicons/`

### Option 3: Create Custom List
- Curate from public domain dictionaries
- Save as `assets/lexicons/custom.txt`

**File format:** Plain text, one word per line, uppercase
```

### Step 3: Update Code to Handle Missing Lexicon

```rust
// In src/lexicon/mod.rs
impl Lexicon {
    pub fn load_default() -> Result<Self, String> {
        // Try multiple lexicons in order of preference
        let paths = vec![
            "assets/lexicons/CSW24.txt",      // If user has license
            "assets/lexicons/ENABLE.txt",     // Public domain
            "assets/lexicons/custom.txt",     // User-provided
        ];

        for path in paths {
            if let Ok(lexicon) = Self::load_from_file(path) {
                info!("Loaded lexicon from {}", path);
                return Ok(lexicon);
            }
        }

        Err("No word list found. Please provide a lexicon file.".to_string())
    }
}
```

---

## 📋 Git History Cleanup (Optional but Recommended)

If you want to remove CSW24.txt from **entire git history**:

```bash
# ⚠️ WARNING: This rewrites history. Only do before public release!

# Using BFG Repo Cleaner (easiest)
# Download from: https://reclaimthegarden.github.io/bfg-repo-cleaner/
java -jar bfg.jar --delete-files CSW24.txt
git reflog expire --expire=now --all
git gc --prune=now --aggressive

# OR using git filter-branch (built-in)
git filter-branch --force --index-filter \
  'git rm --cached --ignore-unmatch CSW24.txt assets/lexicons/CSW24.kwg' \
  --prune-empty --tag-name-filter cat -- --all

# Force push (⚠️ only if repo is not public yet)
git push origin --force --all
```

**Note:** Only do full history cleanup if:
- Repository is private and you're the only contributor
- You haven't released publicly yet
- You understand the risks of force-pushing

---

## 🆓 Public Domain Alternatives

### ENABLE Word List (Recommended for Free Version)
- **Source:** Public domain
- **Size:** ~173,000 words
- **Download:** http://www.puzzlers.org/pub/wordlists/enable1.txt
- **License:** Public domain, free to use commercially
- **Use case:** Perfect for free/educational versions

### SCOWL (Spell Checker Oriented Word Lists)
- **Source:** http://wordlist.aspell.net/
- **License:** Mix of public domain and permissive licenses
- **Customizable:** Can select size/variety

### Wiktionary Word Lists
- **Source:** https://en.wiktionary.org/wiki/Wiktionary:Frequency_lists
- **License:** CC BY-SA (attribution required)
- **Quality:** Varies

---

## 💰 Commercial Licensing Options

### CSW24 (Collins Scrabble Words)
- **Contact:** WESPA (wespa.org) or HarperCollins
- **Cost:** Unknown (negotiate)
- **Benefits:** Official tournament word list
- **Best for:** Competitive/serious players

### TWL (Tournament Word List) - North America
- **Contact:** Hasbro / NASPA
- **Cost:** Unknown (negotiate)
- **Benefits:** Official North American list

---

## 🛡️ Legal Protection Checklist

- [ ] Remove CSW24.txt from git tracking
- [ ] Add to .gitignore (already done)
- [ ] Update README with "BYO lexicon" instructions
- [ ] Add error handling for missing lexicon
- [ ] Support multiple lexicon formats
- [ ] Document public domain alternatives
- [ ] Add license check in code (optional)
- [ ] Include sample ENABLE.txt in releases (public domain)

---

## 📝 Sample User Instructions (For Your Documentation)

```
# TileMania - Getting Started

## 1. Install Dependencies
[...]

## 2. Provide a Word List

TileMania requires a word list to function. Due to licensing restrictions,
we cannot include one in the repository.

### Quick Start (Free):
wget http://www.puzzlers.org/pub/wordlists/enable1.txt
mv enable1.txt assets/lexicons/ENABLE.txt

### For Competitive Play:
If you have a licensed copy of CSW24.txt, place it in:
assets/lexicons/CSW24.txt

## 3. Build & Run
cargo run --release
```

---

## ⚖️ Why This Matters

**Without proper licensing:**
- ❌ HarperCollins could issue DMCA takedown
- ❌ Lawsuit for copyright infringement
- ❌ Damages: Statutory damages up to $150,000 per work (in US)
- ❌ Reputational damage
- ❌ Platform bans (GitHub, app stores)

**With proper approach:**
- ✅ Legal safety
- ✅ Multiple word list support (flexibility)
- ✅ User choice (competitive vs. casual)
- ✅ Demonstrable good faith
- ✅ Scalable business model

---

## 🎯 Recommended Next Steps

1. **Immediate:** Remove CSW24.txt from git
2. **Today:** Implement multi-lexicon support
3. **This week:** Test with ENABLE.txt (public domain)
4. **Before launch:** Legal consultation
5. **Optional:** Negotiate CSW24 license for "Pro" version

---

**Status:** 🚨 Action Required IMMEDIATELY
**Priority:** CRITICAL
**Impact:** Legal risk / Product release blocker

---

Last Updated: 2025-11-19
