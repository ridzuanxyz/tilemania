# Font Assets

This directory contains font files for TileMania.

---

## 🔤 Required Fonts

All fonts should be in **TrueType Font (.ttf)** format.

### Font Files Needed

| File | Purpose | Style | Size Range |
|------|---------|-------|------------|
| `primary.ttf` | Main UI text, menus | Sans-serif, friendly | 12-48pt |
| `title.ttf` | Game title, stage headers | Bold, playful | 24-72pt |
| `score.ttf` | Score displays, timers | Monospace, clear | 16-36pt |
| `tile.ttf` | Letter tiles (A-Z) | Bold, high contrast | 18-48pt |

---

## 📋 Font Specifications

### Primary Font (UI Text)
- **Style:** Clean sans-serif
- **Weight:** Regular (400)
- **Character Set:** A-Z, a-z, 0-9, basic punctuation
- **Features:** Good readability at small sizes
- **Age Group:** Kid-friendly (7-12 years)
- **Examples:** Nunito, Fredoka, Comic Neue, Quicksand

### Title Font (Headers)
- **Style:** Bold, playful
- **Weight:** Bold (700)
- **Character Set:** A-Z, 0-9
- **Features:** Attention-grabbing but readable
- **Examples:** Baloo 2, Bangers, Lilita One

### Score Font (Numbers)
- **Style:** Monospace
- **Weight:** Medium (500)
- **Character Set:** 0-9, +, -, ×
- **Features:** Equal width, clear digits
- **Examples:** JetBrains Mono, IBM Plex Mono, Roboto Mono

### Tile Font (Game Letters)
- **Style:** High contrast, bold
- **Weight:** Bold (700)
- **Character Set:** A-Z (uppercase)
- **Features:** Clear at all sizes, distinct letterforms
- **Examples:** Poppins Bold, Montserrat Bold, Open Sans Bold

---

## 🎨 Design Guidelines

**Readability:**
- Clear distinction between similar letters (I/l, O/0, etc.)
- Good kerning and spacing
- High contrast against background colors

**Age-Appropriate:**
- Not too "childish" (avoid Comic Sans)
- Not too formal (avoid Times New Roman)
- Friendly, modern, accessible

**Technical:**
- Unicode support
- Hinting for screen display
- Web-safe fallbacks available

---

## 📝 Placeholder Status

**Current Status:** 🚧 Placeholders/Empty

The game currently uses Bevy's default font. Custom fonts will enhance the visual polish.

**To add fonts:**
1. Download/license .ttf font files
2. Place in this directory with exact filenames
3. Ensure fonts are properly licensed for commercial use
4. Test readability at various sizes in-game

---

## 🔗 Font Resources

**Free/Open Source Fonts:**
- [Google Fonts](https://fonts.google.com/) - Free, commercial-use fonts
- [Font Squirrel](https://www.fontsquirrel.com/) - Curated free fonts
- [DaFont](https://www.dafont.com/) - Free fonts (check licenses)
- [1001 Free Fonts](https://www.1001freefonts.com/) - Large collection

**Recommended Free Fonts for Games:**
- **Primary:** Nunito, Quicksand, Poppins (Regular)
- **Title:** Baloo 2, Fredoka One, Lilita One
- **Score:** JetBrains Mono, Roboto Mono
- **Tile:** Poppins Bold, Montserrat Bold

**Premium/Commercial:**
- [MyFonts](https://www.myfonts.com/)
- [FontShop](https://www.fontshop.com/)
- [Adobe Fonts](https://fonts.adobe.com/) (with Creative Cloud)

---

## ⚖️ Licensing Notes

**Important:** Verify font licenses before use!

- ✅ **Free for commercial use:** Google Fonts, SIL Open Font License
- ⚠️ **Personal use only:** Many DaFont fonts (not suitable for TileMania)
- ❌ **Requires purchase:** Most premium fonts

**For educational/non-commercial builds:**
- More lenient licensing options available
- Still verify "Educational Use" is permitted

---

**For detailed specifications, see [ASSET_SPECIFICATIONS.md](../../ASSET_SPECIFICATIONS.md)**

*Last Updated: 2025-11-19*
