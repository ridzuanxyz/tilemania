# Audio Requirements for TileMania

## Overview

This document lists all required audio assets for TileMania, based on the `AudioEvent` system defined in `src/stage1/audio.rs`.

**Current Status**: Audio hooks are implemented but audio files are not yet added to the project.

**Audio System**: Uses Bevy's audio system (bevy_kira_audio integration planned)

---

## 📁 Required Audio Files

All audio files should be placed in: `assets/audio/`

### Directory Structure
```
assets/
└── audio/
    ├── sfx/          # Sound effects
    │   ├── ui/       # UI sounds
    │   ├── gameplay/ # Gameplay sounds
    │   ├── powerups/ # Power-up sounds
    │   └── score/    # Score/combo sounds
    └── music/        # Background music
```

---

## 🎵 Music Tracks (2 tracks)

### Background Music

| File Name | Event | Description | Mood | Length |
|-----------|-------|-------------|------|--------|
| `main_theme.ogg` | `MusicTrack::MainTheme` | Main menu and normal gameplay | Calm, upbeat, word-puzzle vibe | 2-3 min loop |
| `intense_mode.ogg` | `MusicTrack::IntenseMode` | Plays when time < 30 seconds | Tense, urgent, faster tempo | 30-60 sec loop |

**Suggested Style**:
- Casual puzzle game aesthetic
- Light electronic or acoustic
- Not overwhelming - players need to concentrate
- Seamless looping

---

## 🔊 Sound Effects (21 effects)

### 1. UI Sounds (2 effects)

| File Name | Event | When It Plays | Suggested Sound |
|-----------|-------|---------------|-----------------|
| `button_click.ogg` | `AudioEvent::ButtonClick` | Any button pressed | Soft click, tap, or pop |
| `button_hover.ogg` | `AudioEvent::ButtonHover` | Mouse hovers over button | Subtle whoosh or chirp |

**Design Notes**:
- Keep UI sounds short (<200ms)
- Lower volume than gameplay sounds
- Should feel responsive, not intrusive

---

### 2. Gameplay Sounds (6 effects)

| File Name | Event | When It Plays | Suggested Sound |
|-----------|-------|---------------|-----------------|
| `tile_select.ogg` | `AudioEvent::TileSelect` | Player selects a tile | Light tap, click, or ping |
| `tile_deselect.ogg` | `AudioEvent::TileDeselect` | Player deselects a tile | Softer version of select sound |
| `word_valid.ogg` | `AudioEvent::WordValid` | Valid word submitted | Positive chime, success tone |
| `word_invalid.ogg` | `AudioEvent::WordInvalid` | Invalid word attempted | Buzzer, error tone (not harsh) |
| `tile_spawn.ogg` | `AudioEvent::TileSpawn` | New tile appears on board | Gentle pop, materialization |
| `tile_despawn.ogg` | `AudioEvent::TileDespawn` | Tile removed from board | Soft poof, fade out |

**Design Notes**:
- Tile sounds should be satisfying and tactile
- Valid/invalid should be clearly distinguishable
- Spawn/despawn should feel smooth, not jarring

---

### 3. Power-Up Sounds (5 effects + 1 generic)

| File Name | Event | When It Plays | Suggested Sound |
|-----------|-------|---------------|-----------------|
| `powerup_collect.ogg` | `AudioEvent::PowerUpCollect` | Player collects any power-up | Coin collect, item pickup |
| `powerup_slowmotion.ogg` | `AudioEvent::PowerUpActivate(SlowMotion)` | Slow motion activated | Time warp, slowdown effect |
| `powerup_bomb.ogg` | `AudioEvent::PowerUpActivate(Bomb)` | Bomb power-up used | Explosion (cartoony, not violent) |
| `powerup_shuffle.ogg` | `AudioEvent::PowerUpActivate(Shuffle)` | Board shuffle activated | Whoosh, cards shuffling |
| `powerup_extratime.ogg` | `AudioEvent::PowerUpActivate(ExtraTime)` | Extra time granted | Clock winding, time extension |

**Design Notes**:
- Collect sound should be rewarding
- Each power-up should have distinct character
- Bomb shouldn't be too harsh (it's a puzzle game)

---

### 4. Score & Combo Sounds (3 effects)

| File Name | Event | When It Plays | Suggested Sound |
|-----------|-------|---------------|-----------------|
| `score_popup.ogg` | `AudioEvent::ScorePopup(points)` | Points earned displayed | Ding, register, point gain |
| `combo_increase.ogg` | `AudioEvent::ComboIncrease(level)` | Combo level increases | Ascending tone, build-up |
| `combo_break.ogg` | `AudioEvent::ComboBreak` | Combo chain broken | Descending tone, loss (subtle) |

**Design Notes**:
- Could have variations based on point value:
  - Small scores (10-50): Light ding
  - Medium scores (50-100): Brighter chime
  - Large scores (100+): Triumphant sound
- Combo sounds should escalate with combo level
- Combo break shouldn't be punishing, just informative

---

### 5. Game State Sounds (4 effects)

| File Name | Event | When It Plays | Suggested Sound |
|-----------|-------|---------------|-----------------|
| `game_start.ogg` | `AudioEvent::GameStart` | Game/level begins | Positive fanfare, short jingle |
| `game_pause.ogg` | `AudioEvent::GamePause` | Game paused | Soft pause tone |
| `game_resume.ogg` | `AudioEvent::GameResume` | Game resumed | Unpause, continue tone |
| `game_over.ogg` | `AudioEvent::GameOver` | Game ends | Victory or defeat jingle (2-3 sec) |
| `time_warning.ogg` | `AudioEvent::TimeWarning` | <10 seconds remaining | Clock ticking, urgent beep |

**Design Notes**:
- Start/over sounds can be longer (1-3 seconds)
- Pause/resume should be immediate and clear
- Time warning should repeat or loop until timer expires

---

## 🎨 Audio Style Guide

### Overall Aesthetic
- **Genre**: Casual puzzle game
- **Mood**: Friendly, encouraging, mentally stimulating
- **References**: Similar to Wordle, Scrabble GO, Words With Friends

### Technical Specifications

**Format**: `.ogg` (Ogg Vorbis)
- Better compression than .wav
- Native support in Bevy
- Good quality-to-size ratio

**Sample Rate**: 44.1 kHz
**Bit Depth**: 16-bit
**Channels**: Mono (for SFX), Stereo (for music)

**File Size Guidelines**:
- UI sounds: <50 KB each
- Gameplay sounds: 50-100 KB each
- Power-up sounds: 100-200 KB each
- Music tracks: 2-5 MB each (compressed)

### Volume Guidelines

Recommended relative volumes (0.0 - 1.0):
- **Background Music**: 0.3 - 0.5 (subtle, not dominant)
- **UI Sounds**: 0.4 - 0.6 (clear but not loud)
- **Gameplay Sounds**: 0.6 - 0.8 (primary focus)
- **Power-ups**: 0.7 - 0.9 (exciting moments)
- **Score/Combo**: 0.5 - 0.7 (rewarding feedback)
- **Game State**: 0.6 - 0.8 (important notifications)

---

## 🎤 Recommended Audio Sources

### Free Sound Effects
1. **Freesound.org** - https://freesound.org/
   - Creative Commons licensed
   - Search terms: "button", "click", "ding", "chime", "pop"

2. **Zapsplat** - https://www.zapsplat.com/
   - Free for indie games
   - Good UI sound collection

3. **Kenney.nl** - https://kenney.nl/assets?q=audio
   - Public domain game assets
   - Interface sounds pack

### Free Music
1. **Incompetech** - https://incompetech.com/music/
   - Kevin MacLeod's royalty-free music
   - Good for puzzle game loops

2. **OpenGameArt** - https://opengameart.org/
   - Community-contributed music
   - Various licenses (check each)

3. **FreePD** - https://freepd.com/
   - Public domain music

### Paid Options (Budget-Friendly)
1. **AudioJungle** ($5-20 per track)
2. **Pond5** ($10-50 per pack)
3. **Epidemic Sound** ($15/month subscription)

---

## 🛠️ Implementation Status

### Current State
- ✅ Audio event system defined (`AudioEvent` enum)
- ✅ Audio hooks implemented in code
- ❌ Audio files not yet added
- ❌ bevy_kira_audio integration pending
- ❌ Volume controls not implemented

### Integration Steps

1. **Add audio files**:
   ```bash
   mkdir -p assets/audio/{sfx/{ui,gameplay,powerups,score},music}
   # Add .ogg files to appropriate directories
   ```

2. **Update Cargo.toml**:
   ```toml
   [dependencies]
   bevy_kira_audio = "0.21"  # Check for Bevy 0.15 compatible version
   ```

3. **Load audio assets** in `src/plugins/assets.rs`

4. **Implement playback** in `src/stage1/audio.rs`

5. **Add volume controls** in settings menu (Sprint 1, Week 3)

---

## 📝 Quick Start Checklist

For MVP audio implementation:

**Priority 1 - Must Have** (core gameplay):
- [ ] `button_click.ogg` - UI feedback
- [ ] `tile_select.ogg` - Core interaction
- [ ] `word_valid.ogg` - Success feedback
- [ ] `word_invalid.ogg` - Error feedback
- [ ] `main_theme.ogg` - Background music

**Priority 2 - Should Have** (enhanced experience):
- [ ] `score_popup.ogg` - Score feedback
- [ ] `combo_increase.ogg` - Combo feedback
- [ ] `game_start.ogg` - Level start
- [ ] `game_over.ogg` - Level end
- [ ] `time_warning.ogg` - Urgency

**Priority 3 - Nice to Have** (polish):
- [ ] All remaining SFX
- [ ] `intense_mode.ogg` music
- [ ] Volume controls in settings
- [ ] Audio mixing/ducking

---

## 🎯 Next Steps

1. **Choose audio sources** (free or paid)
2. **Download/create Priority 1 sounds** (5 files)
3. **Test audio in game** - verify timing and volume
4. **Iterate based on playtesting feedback**
5. **Add remaining sounds** in order of priority
6. **Implement volume controls** in settings

---

## 📚 References

- Audio event definitions: `src/stage1/audio.rs`
- Audio system implementation: `src/stage1/audio.rs` (lines 71-160)
- Bevy audio docs: https://bevyengine.org/learn/book/getting-started/plugins/#audio
- Bevy Kira Audio: https://github.com/NiklasEi/bevy_kira_audio

---

**Last Updated**: 2025-11-20
**Status**: Audio system designed, awaiting asset creation
