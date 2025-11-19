# Asset Specifications for TileMania

**Version:** 1.0
**Date:** November 19, 2025
**Status:** Production-ready code awaiting assets

This document provides detailed specifications for all audio, visual, and font assets needed to complete TileMania.

---

## 📁 Asset Directory Structure

```
assets/
├── fonts/
│   ├── FiraSans-Bold.ttf
│   └── FiraSans-Medium.ttf
│
├── audio/
│   ├── music/
│   │   ├── main_theme.ogg
│   │   ├── stage1_gameplay.ogg
│   │   ├── stage1_intense.ogg
│   │   ├── stage2_puzzle.ogg
│   │   ├── stage3_classic.ogg
│   │   ├── stage3_endgame.ogg
│   │   ├── stage4_speed.ogg
│   │   ├── stage4_panic.ogg
│   │   ├── stage5_tournament.ogg
│   │   └── victory_fanfare.ogg
│   │
│   └── sfx/
│       ├── common/
│       │   ├── button_click.ogg
│       │   └── pause.ogg
│       ├── stage1/
│       ├── stage2/
│       ├── stage3/
│       ├── stage4/
│       └── stage5/
│
├── sprites/
│   ├── tiles/
│   ├── ui/
│   ├── particles/
│   └── backgrounds/
│
└── lexicons/
    └── CSW24.txt (already included)
```

---

## 🎵 Audio Assets

### Background Music (10 tracks, .ogg format)

| File | Duration | BPM | Mood | Usage |
|------|----------|-----|------|-------|
| `main_theme.ogg` | 2-3 min | 120-140 | Upbeat, friendly | Main menu, splash |
| `stage1_gameplay.ogg` | 2-3 min (loop) | 140-160 | Energetic, arcade | Stage 1 normal play |
| `stage1_intense.ogg` | 1-2 min (loop) | 160-180 | Urgent, exciting | Stage 1 final 15s |
| `stage2_puzzle.ogg` | 2-3 min (loop) | 100-120 | Thoughtful, casual | Stage 2 gameplay |
| `stage3_classic.ogg` | 3-4 min (loop) | 80-100 | Contemplative | Stage 3 normal |
| `stage3_endgame.ogg` | 2-3 min (loop) | 90-110 | Focused, strategic | Stage 3 <20 tiles |
| `stage4_speed.ogg` | 2-3 min (loop) | 150-170 | Fast, intense | Stage 4 normal |
| `stage4_panic.ogg` | 1-2 min (loop) | 180-200 | Frantic, urgent | Stage 4 panic mode |
| `stage5_tournament.ogg` | 3-4 min (loop) | 110-130 | Epic, competitive | Stage 5 early rounds |
| `victory_fanfare.ogg` | 5-10 sec | N/A | Triumphant | Win screens |

**Specifications:**
- Format: OGG Vorbis
- Sample Rate: 44.1 kHz
- Bit Rate: 128-192 kbps
- Channels: Stereo
- Looping: Seamless loop points
- Volume: Normalized to -6dB peak

---

### Sound Effects (60+ files, .ogg format)

#### Common SFX (2 files)

| File | Duration | Description |
|------|----------|-------------|
| `button_click.ogg` | <0.2s | UI button press sound |
| `pause.ogg` | <0.3s | Pause menu open/close |

#### Stage 1: Falling Letters (12 files)

| File | Duration | Description | Trigger |
|------|----------|-------------|---------|
| `tile_select.ogg` | <0.2s | Click/selection sound | Tile clicked |
| `word_valid.ogg` | 0.5-0.8s | Success chime | Valid word formed |
| `word_invalid.ogg` | 0.3-0.5s | Error buzz | Invalid word attempt |
| `combo_increase.ogg` | 0.3s | Pitch increases with combo | Combo +1 |
| `combo_break.ogg` | 0.5s | Descending tone | Combo resets |
| `powerup_collect.ogg` | 0.5s | Sparkle/collect sound | Power-up picked up |
| `powerup_slowmo.ogg` | 1.0s | Time-stretch effect | Slow Motion activated |
| `powerup_bomb.ogg` | 0.8s | Explosion sound | Bomb activated |
| `powerup_shuffle.ogg` | 0.6s | Swirl/shuffle sound | Shuffle activated |
| `powerup_time.ogg` | 0.5s | Clock tick sound | Extra Time activated |
| `game_start.ogg` | 1.0s | Ready, go! | Stage begins |
| `game_over.ogg` | 1.5s | Game end sound | Time expires |

**Specifications:**
- Format: OGG Vorbis
- Sample Rate: 44.1 kHz
- Bit Rate: 96-128 kbps
- Channels: Mono
- Volume: Normalized to -12dB peak

#### Stage 2: Tile Matching (12 files)

| File | Duration | Description | Trigger |
|------|----------|-------------|---------|
| `tile_select.ogg` | <0.2s | Selection click | Tile selected |
| `tile_swap.ogg` | 0.3s | Swap/slide sound | Valid swap |
| `invalid_swap.ogg` | 0.3s | Error sound | Invalid swap |
| `word_match.ogg` | 0.6s | Match sound (pitch varies) | Word matched |
| `cascade.ogg` | 0.5s | Falling sound | Tiles cascading |
| `tile_spawn.ogg` | 0.2s | Pop-in sound | New tile appears |
| `combo_up.ogg` | 0.3s | Ascending tone | Combo increased |
| `combo_break.ogg` | 0.4s | Descending tone | Combo broken |
| `level_complete.ogg` | 1.5s | Victory sound | Target reached |
| `game_over.ogg` | 1.0s | Defeat sound | Time/moves out |
| `button_click.ogg` | <0.2s | UI click | Button pressed |
| `pause.ogg` | <0.3s | Pause toggle | ESC pressed |

#### Stage 3: Classic Board (12 files)

| File | Duration | Description | Trigger |
|------|----------|-------------|---------|
| `tile_place.ogg` | 0.3s | Clack sound | Tile placed on board |
| `tile_pickup.ogg` | 0.2s | Click sound | Tile selected from rack |
| `word_submit.ogg` | 0.5s | Submit sound | Move submitted |
| `word_valid.ogg` | 0.8s | Approval chime | Valid word accepted |
| `word_invalid.ogg` | 0.6s | Rejection buzz | Invalid word |
| `turn_change.ogg` | 0.4s | Transition sound | Turn switches |
| `ai_thinking.ogg` | 2.0s (loop) | Contemplative tone | AI calculating |
| `ai_plays.ogg` | 0.6s | Placement sound | AI makes move |
| `high_score.ogg` | 1.0s | Impressive sound | Move scores 50+ |
| `game_over.ogg` | 1.5s | End game sound | Game completes |
| `button_click.ogg` | <0.2s | UI click | Button pressed |
| `pause.ogg` | <0.3s | Pause toggle | ESC pressed |

#### Stage 4: Speed Challenge (12 files)

| File | Duration | Description | Trigger |
|------|----------|-------------|---------|
| `tile_select.ogg` | <0.2s | Quick click | Tile selected |
| `word_submit.ogg` | 0.3s | Fast submit sound | Word submitted |
| `word_valid.ogg` | 0.5s | Success ping | Valid word |
| `word_invalid.ogg` | 0.3s | Quick buzz | Invalid word |
| `streak_increase.ogg` | 0.3s | Pitch rises with streak | Streak +1 |
| `streak_break.ogg` | 0.5s | Streak lost sound | Invalid word breaks streak |
| `high_streak.ogg` | 0.8s | Epic sound | Streak >= 10 |
| `panic_mode.ogg` | 1.0s | Alarm sound | Time < threshold |
| `time_warning.ogg` | 0.5s | Warning beep | Time < 30s |
| `game_over.ogg` | 1.0s | Buzzer sound | Time expires |
| `button_click.ogg` | <0.2s | UI click | Button pressed |
| `pause.ogg` | <0.3s | Pause toggle | ESC pressed |

#### Stage 5: AI Tournaments (12 files)

| File | Duration | Description | Trigger |
|------|----------|-------------|---------|
| `match_start.ogg` | 1.5s | Introduction fanfare | Match begins |
| `game_won.ogg` | 1.0s | Victory cheer | Player wins game |
| `game_lost.ogg` | 0.8s | Defeat sound | Player loses game |
| `round_advance.ogg` | 2.0s | Progression fanfare | Advance to next round |
| `tournament_victory.ogg` | 3.0s | Championship fanfare | Win tournament |
| `tournament_defeat.ogg` | 2.0s | Elimination sound | Lose match |
| `opponent_intro.ogg` | 1.0s | Introduction sound | Opponent revealed |
| `crowd_cheer.ogg` | 1.5s | Crowd cheering | Good move |
| `crowd_aww.ogg` | 1.0s | Crowd disappointment | Bad move |
| `bracket_update.ogg` | 0.5s | Update sound | Bracket changes |
| `button_click.ogg` | <0.2s | UI click | Button pressed |
| `pause.ogg` | <0.3s | Pause toggle | ESC pressed |

**Total Audio Files:** ~70 files

---

## 🎨 Visual Assets

### Fonts (2 files, .ttf format)

| File | Usage | Sizes Needed |
|------|-------|--------------|
| `FiraSans-Bold.ttf` | Titles, scores, buttons | 16, 20, 24, 28, 32, 36, 56, 64 |
| `FiraSans-Medium.ttf` | Labels, body text | 14, 16, 18, 20, 22, 24 |

**Source:** [FiraSans on Google Fonts](https://fonts.google.com/specimen/Fira+Sans)
**License:** SIL Open Font License

---

### Sprite Assets (Optional - Can start with colored rectangles)

#### Tile Sprites
- **Size:** 64×64 pixels
- **Format:** PNG with transparency
- **Variants:** 26 letters (A-Z)
- **States:** Normal, Selected, Matched, Invalid
- **Style:** Clean, readable, child-friendly

#### UI Elements
- **Buttons:** 200×60 pixels (normal, hover, pressed states)
- **Backgrounds:** 1280×720 pixels (stage-specific themes)
- **Icons:** 32×32 pixels (power-ups, settings, etc.)

#### Particle Textures
- **Size:** 8×8 to 16×16 pixels
- **Format:** PNG with transparency
- **Types:** Sparkle, burst, glow
- **Colors:** White (tinted programmatically)

---

## 🎯 Asset Priority Tiers

### Tier 1: CRITICAL (Game won't run without these)
- ✅ `CSW24.txt` (Already included)
- ❌ `FiraSans-Bold.ttf`
- ❌ `FiraSans-Medium.ttf`

**Workaround:** Use system default fonts temporarily

### Tier 2: HIGH (Game runs but has no audio)
- All background music tracks (10 files)
- Core SFX: button_click, pause (2 files)
- Stage 1 essential SFX (6 files)

### Tier 3: MEDIUM (Enhanced gameplay)
- All remaining stage SFX (50+ files)
- Sprite textures (can use colored rectangles initially)

### Tier 4: LOW (Polish)
- Particle textures
- Background images
- UI element graphics
- Mascot graphics

---

## 🛠️ Asset Creation Guidelines

### Audio Production

**Tools Recommended:**
- DAW: Audacity (free), Reaper, FL Studio
- Synthesis: Vital (free), Serum
- SFX Libraries: Freesound.org, Zapsplat.com

**Quality Standards:**
- All audio normalized to prevent clipping
- Consistent volume levels per category
- Seamless loops for background music
- No pops or clicks at start/end

### Visual Asset Creation

**Tools Recommended:**
- Vector: Inkscape (free), Adobe Illustrator
- Raster: GIMP (free), Photoshop, Aseprite
- Fonts: Google Fonts (free, open source)

**Style Guide:**
- **Color Palette:** Friendly, high contrast for readability
- **Typography:** Clean, sans-serif, child-appropriate
- **Theme:** Educational but fun, not too juvenile

---

## 📥 Placeholder Assets (Quick Start)

For immediate testing, you can use:

### Placeholder Fonts
```bash
# Download FiraSans from Google Fonts
wget https://github.com/google/fonts/raw/main/ofl/firasans/FiraSans-Bold.ttf
wget https://github.com/google/fonts/raw/main/ofl/firasans/FiraSans-Medium.ttf

# Place in assets/fonts/
mkdir -p assets/fonts
mv FiraSans-*.ttf assets/fonts/
```

### Placeholder Audio (Silent files)
```bash
# Generate silent OGG files for testing
for file in button_click.ogg pause.ogg; do
    ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 0.1 -c:a libvorbis assets/audio/sfx/common/$file
done
```

### Placeholder Sprites
- Use colored rectangles generated in code
- Bevy can render basic shapes without external assets

---

## 📊 Asset Checklist

### Fonts
- [ ] FiraSans-Bold.ttf
- [ ] FiraSans-Medium.ttf

### Music (10 tracks)
- [ ] main_theme.ogg
- [ ] stage1_gameplay.ogg
- [ ] stage1_intense.ogg
- [ ] stage2_puzzle.ogg
- [ ] stage3_classic.ogg
- [ ] stage3_endgame.ogg
- [ ] stage4_speed.ogg
- [ ] stage4_panic.ogg
- [ ] stage5_tournament.ogg
- [ ] victory_fanfare.ogg

### Common SFX (2 files)
- [ ] button_click.ogg
- [ ] pause.ogg

### Stage 1 SFX (12 files)
- [ ] All Stage 1 sounds listed above

### Stage 2 SFX (12 files)
- [ ] All Stage 2 sounds listed above

### Stage 3 SFX (12 files)
- [ ] All Stage 3 sounds listed above

### Stage 4 SFX (12 files)
- [ ] All Stage 4 sounds listed above

### Stage 5 SFX (12 files)
- [ ] All Stage 5 sounds listed above

**Total: ~72 asset files needed**

---

## 💡 Quick Start Recommendation

**Minimum Viable Asset Set (Can run the game):**
1. Download FiraSans fonts (2 files) - 5 minutes
2. Generate silent placeholder audio - 10 minutes
3. Test compilation with placeholders - 5 minutes

**Total time to playable:** ~20 minutes

Then add real audio/visuals incrementally while playtesting!

---

**Last Updated:** November 19, 2025
**Status:** Awaiting asset creation
**Contact:** Check README.md for project contacts
