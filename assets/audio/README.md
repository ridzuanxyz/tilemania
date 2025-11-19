# Audio Assets

This directory contains all audio assets for TileMania.

## Directory Structure

```
audio/
├── music/          # Background music tracks (10 files)
└── sfx/            # Sound effects (60+ files)
```

---

## 🎵 Required Music Files (10 tracks)

All music files should be in **OGG Vorbis** format.

### Specifications
- Format: `.ogg` (OGG Vorbis)
- Sample Rate: 44.1 kHz
- Bit Rate: 128-192 kbps
- Channels: Stereo
- Looping: Seamless loop points
- Volume: Normalized to -6dB peak

### Music Tracks

| File | Duration | BPM | Mood | Usage |
|------|----------|-----|------|-------|
| `music/main_theme.ogg` | 2-3 min | 120-140 | Upbeat, friendly | Main menu, splash |
| `music/stage1_gameplay.ogg` | 2-3 min (loop) | 140-160 | Energetic, arcade | Stage 1 normal play |
| `music/stage1_intense.ogg` | 1-2 min (loop) | 160-180 | Urgent, exciting | Stage 1 final 15s |
| `music/stage2_puzzle.ogg` | 2-3 min (loop) | 100-120 | Thoughtful, casual | Stage 2 gameplay |
| `music/stage3_classic.ogg` | 3-4 min (loop) | 80-100 | Contemplative | Stage 3 normal |
| `music/stage3_endgame.ogg` | 2-3 min (loop) | 90-110 | Focused, strategic | Stage 3 <20 tiles |
| `music/stage4_speed.ogg` | 2-3 min (loop) | 150-170 | Fast, intense | Stage 4 normal |
| `music/stage4_panic.ogg` | 1-2 min (loop) | 180-200 | Frantic, urgent | Stage 4 panic mode |
| `music/stage5_tournament.ogg` | 3-4 min (loop) | 110-130 | Epic, competitive | Stage 5 early rounds |
| `music/victory_fanfare.ogg` | 5-10 sec | N/A | Triumphant | Win screens |

---

## 🔊 Required Sound Effects (60+ files)

All SFX should be in **OGG Vorbis** format, short duration.

### Common SFX (All Stages)
- `sfx/button_click.ogg` - UI button press (<0.2s)
- `sfx/pause.ogg` - Pause menu open/close (<0.3s)

### Stage 1: Falling Letters (12 files)
- `sfx/stage1_tile_select.ogg` - Tile clicked
- `sfx/stage1_word_valid.ogg` - Valid word formed (0.5-0.8s)
- `sfx/stage1_word_invalid.ogg` - Invalid word attempt (0.3-0.5s)
- `sfx/stage1_combo_increase.ogg` - Combo +1 (0.3s)
- `sfx/stage1_combo_break.ogg` - Combo resets (0.5s)
- `sfx/stage1_powerup_collect.ogg` - Power-up collected (0.5s)
- `sfx/stage1_powerup_slowmo.ogg` - Slow Motion activated (1.0s)
- `sfx/stage1_powerup_bomb.ogg` - Bomb explosion (0.8s)
- `sfx/stage1_powerup_shuffle.ogg` - Shuffle tiles (0.5s)
- `sfx/stage1_powerup_time.ogg` - Extra time granted (0.5s)
- `sfx/stage1_warning.ogg` - 15 seconds remaining (1.0s)
- `sfx/stage1_complete.ogg` - Stage victory (2.0s)

### Stage 2: Tile Matching (10 files)
- `sfx/stage2_tile_swap.ogg` - Tiles swapping (0.3s)
- `sfx/stage2_match.ogg` - Word matched (0.5s)
- `sfx/stage2_cascade.ogg` - Cascade trigger (0.4s)
- `sfx/stage2_no_moves.ogg` - No valid moves (0.6s)
- `sfx/stage2_3letter.ogg` - 3-letter word formed (0.4s)
- `sfx/stage2_4letter.ogg` - 4-letter word formed (0.5s)
- `sfx/stage2_5letter.ogg` - 5+ letter word formed (0.7s)
- `sfx/stage2_target_reached.ogg` - Target score achieved (1.5s)
- `sfx/stage2_moves_low.ogg` - <5 moves remaining (0.5s)
- `sfx/stage2_complete.ogg` - Stage victory (2.0s)

### Stage 3: Classic Board (15 files)
- `sfx/stage3_tile_place.ogg` - Tile placed on board (0.2s)
- `sfx/stage3_tile_return.ogg` - Tile returned to rack (0.2s)
- `sfx/stage3_word_submit.ogg` - Word submitted (0.4s)
- `sfx/stage3_word_score.ogg` - Points awarded (0.6s)
- `sfx/stage3_double_letter.ogg` - DL square used (0.3s)
- `sfx/stage3_triple_letter.ogg` - TL square used (0.4s)
- `sfx/stage3_double_word.ogg` - DW square used (0.5s)
- `sfx/stage3_triple_word.ogg` - TW square used (0.7s)
- `sfx/stage3_bingo.ogg` - 7 tiles used (1.5s, celebratory)
- `sfx/stage3_ai_turn.ogg` - AI's turn start (0.3s)
- `sfx/stage3_ai_thinking.ogg` - AI thinking (loop, 0.5s segments)
- `sfx/stage3_ai_word.ogg` - AI plays word (0.5s)
- `sfx/stage3_bag_empty.ogg` - Tile bag empty (0.8s)
- `sfx/stage3_victory.ogg` - You win (2.5s)
- `sfx/stage3_defeat.ogg` - AI wins (2.0s)

### Stage 4: Speed Challenge (8 files)
- `sfx/stage4_word_quick.ogg` - Fast word submission (0.3s)
- `sfx/stage4_streak_up.ogg` - Streak increased (0.3s, pitch increases)
- `sfx/stage4_streak_break.ogg` - Streak broken (0.5s, descending)
- `sfx/stage4_rack_refresh.ogg` - Rack refilled (0.2s)
- `sfx/stage4_panic_start.ogg` - Panic mode activated (1.0s)
- `sfx/stage4_time_bonus.ogg` - Time added (0.4s)
- `sfx/stage4_complete.ogg` - Target reached (2.0s)
- `sfx/stage4_timeout.ogg` - Time's up (1.5s)

### Stage 5: AI Competitions (12 files)
- `sfx/stage5_match_start.ogg` - Match begins (1.0s)
- `sfx/stage5_round_start.ogg` - Round begins (0.8s)
- `sfx/stage5_game_won.ogg` - You win a game (1.5s)
- `sfx/stage5_game_lost.ogg` - AI wins a game (1.2s)
- `sfx/stage5_match_won.ogg` - You win the match (2.5s)
- `sfx/stage5_match_lost.ogg` - AI wins the match (2.0s)
- `sfx/stage5_bracket_advance.ogg` - Advance to next round (1.5s)
- `sfx/stage5_semifinals.ogg` - Semifinals announcement (1.5s)
- `sfx/stage5_finals.ogg` - Finals announcement (2.0s)
- `sfx/stage5_champion.ogg` - Tournament victory (3.0s, epic)
- `sfx/stage5_eliminated.ogg` - Eliminated from tournament (2.5s)
- `sfx/stage5_crowd_cheer.ogg` - Crowd reaction (1.0s, loop)

---

## 🎨 Audio Style Guidelines

**Music:**
- Age-appropriate (7-12 years)
- Non-distracting during gameplay
- Clear mood changes between stages
- Seamless loops (no silence at loop points)
- Royalty-free or properly licensed

**Sound Effects:**
- Clear, distinct sounds
- Not too loud or jarring
- Positive reinforcement for correct actions
- Gentle error sounds (not harsh)
- Short duration (< 2 seconds for most)

---

## 📝 Placeholder Status

**Current Status:** 🚧 Placeholders/Empty

The game will run without audio files, but audio events are implemented and ready.

**To add audio:**
1. Create/license audio files according to specs above
2. Place files in appropriate directories
3. Ensure filenames match exactly as listed
4. Test in-game to verify proper triggering

---

## 🔗 Resources

**Free/Royalty-Free Audio Sources:**
- [OpenGameArt](https://opengameart.org/) - CC0/CC-BY audio
- [Freesound](https://freesound.org/) - Creative Commons sounds
- [Incompetech](https://incompetech.com/) - Royalty-free music
- [ZapSplat](https://www.zapsplat.com/) - Free sound effects

**Commercial/Premium:**
- [AudioJungle](https://audiojungle.net/)
- [Pond5](https://www.pond5.com/)
- Commission custom audio from freelance composers

---

**For detailed specifications, see [ASSET_SPECIFICATIONS.md](../../ASSET_SPECIFICATIONS.md)**

*Last Updated: 2025-11-19*
