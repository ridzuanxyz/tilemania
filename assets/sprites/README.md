# Sprite Assets

This directory contains visual sprite assets for TileMania.

---

## 🎨 Required Sprites

All sprites should be in **PNG** format with transparency support.

---

## 📂 Sprite Categories

### 1. UI Elements

| File | Size | Description |
|------|------|-------------|
| `button_normal.png` | 200×60px | Standard button |
| `button_hover.png` | 200×60px | Button hover state |
| `button_pressed.png` | 200×60px | Button pressed state |
| `background_main.png` | 1920×1080px | Main menu background |
| `logo.png` | 512×512px | TileMania logo |
| `icon.png` | 256×256px | Application icon |

### 2. Tile Sprites (Stage 1, 2, 4)

| File | Size | Description |
|------|------|-------------|
| `tile_blank.png` | 64×64px | Empty tile |
| `tile_A.png` - `tile_Z.png` | 64×64px | Letter tiles (26 files) |
| `tile_selected.png` | 64×64px | Selected tile highlight |
| `tile_valid.png` | 64×64px | Valid word tile highlight (green) |
| `tile_invalid.png` | 64×64px | Invalid word tile highlight (red) |

**Point Value Overlays (optional):**
- Small number in corner showing tile point value
- 1pt, 2pt, 3pt, 4pt, 5pt, 8pt, 10pt variants

### 3. Board Elements (Stage 3)

| File | Size | Description |
|------|------|-------------|
| `board_cell.png` | 48×48px | Empty board cell |
| `board_center.png` | 48×48px | Center star cell |
| `board_DL.png` | 48×48px | Double Letter (light blue) |
| `board_TL.png` | 48×48px | Triple Letter (dark blue) |
| `board_DW.png` | 48×48px | Double Word (light red) |
| `board_TW.png` | 48×48px | Triple Word (dark red) |

### 4. Power-Ups (Stage 1)

| File | Size | Description |
|------|------|-------------|
| `powerup_slowmo.png` | 64×64px | Slow Motion icon (clock) |
| `powerup_bomb.png` | 64×64px | Bomb icon |
| `powerup_shuffle.png` | 64×64px | Shuffle icon (arrows) |
| `powerup_time.png` | 64×64px | Extra Time icon (hourglass) |
| `powerup_active.png` | 64×64px | Active power-up glow/border |

### 5. Particle Effects

| File | Size | Description |
|------|------|-------------|
| `particle_star.png` | 32×32px | Star particle (combos) |
| `particle_sparkle.png` | 16×16px | Sparkle particle (matches) |
| `particle_explosion.png` | 64×64px | Explosion sprite sheet (4×4 frames) |
| `particle_confetti.png` | 128×128px | Confetti sprite sheet (8 frames) |

### 6. AI Avatars (Stage 5)

| File | Size | Description |
|------|------|-------------|
| `ai_novice.png` | 128×128px | Novice Bot avatar |
| `ai_wizard.png` | 128×128px | Word Wizard avatar |
| `ai_professor.png` | 128×128px | The Professor avatar |
| `ai_speed.png` | 128×128px | Speed Demon avatar |
| `ai_defensive.png` | 128×128px | Defensive Dan avatar |
| `ai_combo.png` | 128×128px | Combo King avatar |
| `ai_champion.png` | 128×128px | The Champion avatar |

### 7. Icons & Indicators

| File | Size | Description |
|------|------|-------------|
| `icon_timer.png` | 32×32px | Timer/clock icon |
| `icon_score.png` | 32×32px | Score/trophy icon |
| `icon_moves.png` | 32×32px | Moves remaining icon |
| `icon_streak.png` | 32×32px | Streak/fire icon |
| `icon_difficulty.png` | 32×32px | Difficulty level icon |
| `icon_pause.png` | 48×48px | Pause button icon |
| `icon_sound_on.png` | 32×32px | Sound enabled |
| `icon_sound_off.png` | 32×32px | Sound muted |

---

## 🎨 Art Style Guidelines

**Overall Style:**
- Clean, modern, flat design
- Age-appropriate (7-12 years)
- High contrast for readability
- Consistent color palette across all sprites

**Color Palette Suggestions:**
- **Primary:** Bright blue (#4A90E2)
- **Secondary:** Warm orange (#F5A623)
- **Success:** Green (#7ED321)
- **Error:** Red (#D0021B)
- **Background:** Light gray (#F8F8F8)
- **Text:** Dark gray (#333333)

**Premium Square Colors (Stage 3):**
- Double Letter (DL): Light Blue (#B3E5FC)
- Triple Letter (TL): Dark Blue (#2196F3)
- Double Word (DW): Light Red (#FFCDD2)
- Triple Word (TW): Dark Red (#F44336)
- Center Star: Gold (#FFD700)

**Tile Design:**
- Rounded corners (4-6px radius)
- Subtle shadow for depth
- Clear letter in center
- Point value in corner (small, 10-12pt)
- Consistent letter sizing

---

## 📐 Technical Specifications

**Format:**
- PNG with alpha transparency
- 32-bit color depth
- No compression artifacts

**Resolution:**
- Designed for 1920×1080 display
- Scalable for different resolutions
- Crisp edges (avoid anti-aliasing blur)

**Sprite Sheets (for animations):**
- Equal-sized frames in grid layout
- Power-of-two dimensions when possible
- Horizontal layout preferred

**File Naming:**
- Lowercase
- Underscores for spaces
- Descriptive names
- Consistent prefixes (tile_, powerup_, etc.)

---

## 🖼️ Animation Sprite Sheets

For animated elements, use sprite sheets:

**Explosion (bomb power-up):**
- Frame size: 64×64px
- Total size: 256×256px (4×4 grid)
- 16 frames total
- Frame duration: 50ms (20fps)

**Confetti (victory):**
- Frame size: 128×128px
- Total size: 1024×128px (8×1 layout)
- 8 frames total
- Frame duration: 100ms (10fps)

**Sparkle (word matches):**
- Frame size: 32×32px
- Total size: 128×32px (4×1 layout)
- 4 frames total
- Frame duration: 75ms (~13fps)

---

## 📝 Placeholder Status

**Current Status:** 🚧 Placeholders/Empty

The game currently uses Bevy's default shapes and colors. Sprite assets will dramatically improve visual appeal.

**To add sprites:**
1. Create/commission sprites according to specifications
2. Place PNG files in this directory
3. Ensure filenames match exactly as listed
4. Test in-game to verify proper rendering
5. Adjust sizes if needed based on screen resolution

---

## 🔗 Art Resources

**Free/Open Source Graphics:**
- [OpenGameArt](https://opengameart.org/) - CC0/CC-BY game art
- [Kenney](https://kenney.nl/) - Free game assets
- [Itch.io Game Assets](https://itch.io/game-assets/free) - Community assets
- [Game Icons](https://game-icons.net/) - Free SVG icons

**Design Tools:**
- [Figma](https://www.figma.com/) - UI/UX design (free tier)
- [GIMP](https://www.gimp.org/) - Free image editor
- [Inkscape](https://inkscape.org/) - Free vector graphics
- [Aseprite](https://www.aseprite.org/) - Pixel art & sprites ($)
- [Piskel](https://www.piskelapp.com/) - Free online sprite editor

**Commission Artists:**
- [Fiverr](https://www.fiverr.com/) - Freelance game artists
- [Upwork](https://www.upwork.com/) - Professional designers
- [ArtStation](https://www.artstation.com/) - Portfolio platform
- [Behance](https://www.behance.net/) - Creative portfolios

**AI Generation (Quick Prototypes):**
- [DALL-E](https://openai.com/dall-e-2) - AI image generation
- [Midjourney](https://www.midjourney.com/) - AI art
- [Stable Diffusion](https://stability.ai/) - Open source AI
- Note: AI art may need manual touch-ups for game use

---

## ⚖️ Licensing Notes

**Important:** Verify asset licenses before use!

- ✅ **Free for commercial use:** CC0, CC-BY, Kenney assets
- ⚠️ **Attribution required:** CC-BY (credit the artist)
- ⚠️ **Personal use only:** Some free assets (not suitable for commercial TileMania)
- ❌ **Requires purchase:** Most premium asset packs

**For educational/non-commercial builds:**
- More lenient licensing options available
- Still verify "Educational Use" is permitted

---

## 🎯 Priority Sprites (Start Here)

If creating assets incrementally, prioritize these:

**Phase 1 (Core Gameplay):**
1. Letter tiles (A-Z) - 26 files
2. Board cells (Stage 3) - 7 files
3. Basic UI buttons - 3 files

**Phase 2 (Polish):**
4. Power-up icons - 4 files
5. Particle effects - 3 files
6. Icons & indicators - 8 files

**Phase 3 (Extras):**
7. AI avatars - 7 files
8. Backgrounds - 1 file
9. Logo & branding - 2 files

---

**For detailed specifications, see [ASSET_SPECIFICATIONS.md](../../ASSET_SPECIFICATIONS.md)**

*Last Updated: 2025-11-19*
