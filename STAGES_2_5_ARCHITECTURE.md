# 🎮 Stages 2-5: Complete Architecture & Implementation Plan

**Status:** Architecture Complete, Ready for Full Implementation
**Date:** 2025-11-18

---

## 📋 Overview

This document outlines the complete architecture for Stages 2-5, following the proven patterns from Stage 1 (2,136 lines, 100% complete).

Each stage follows the same module structure:
- `mod.rs` - Plugin, configuration, resources
- `components.rs` - ECS components
- `systems.rs` - Core gameplay logic
- `difficulty.rs` - 5 difficulty levels
- `visuals.rs` - Visual feedback & animations
- `ui.rs` - UI screens (start, HUD, results)
- `pause.rs` - Pause menu
- `audio.rs` - Audio event hooks

---

## 🎯 Stage 2: Tile Matching (3-4 Letter Words)

### Status: ✅ Core Systems Implemented

**Gameplay:** Match-3 style grid (8×8), swap tiles to form 3-4 letter words

**What's Implemented:**
- ✅ mod.rs (125 lines) - Plugin & configuration
- ✅ components.rs (95 lines) - Grid tiles, animations
- ✅ difficulty.rs (95 lines) - 5 levels (180s → 60s, target 500 → 2000)
- ✅ systems.rs (410 lines) - Grid, selection, swapping, matching, cascading

**What's Remaining (~800 lines):**
- 🔲 visuals.rs (250 lines) - Match animations, particle effects
- 🔲 ui.rs (350 lines) - Start screen, HUD, results
- 🔲 pause.rs (100 lines) - Pause menu (reuse Stage 1 pattern)
- 🔲 audio.rs (100 lines) - Audio hooks (reuse Stage 1 pattern)

**Key Systems:**
```rust
// Core Gameplay
- spawn_grid() - Creates 8×8 grid
- handle_tile_selection() - Click to select
- handle_tile_swap() - Swap adjacent tiles
- find_word_matches() - Find horizontal/vertical 3-4 letter words
- clear_matched_words() - Remove matches with animation
- cascade_tiles() - Tiles fall to fill gaps
- spawn_new_tiles() - Fill empty spaces at top

// Win Condition
- Reach target score before time/moves run out
- D1: 500 points in 180s
- D5: 2000 points in 60s with 30 moves limit
```

**Estimated Total:** ~1,500 lines for complete Stage 2

---

## 🎲 Stage 3: Classic Board (Full Scrabble, 2-15 Letter Words)

### Status: 🟡 Foundation Exists (Sprint 4), Needs Integration

**Gameplay:** Traditional 15×15 word tile board with AI opponent

**Existing Code (from Sprint 4):**
- ✅ 15×15 board system
- ✅ Tile placement mechanics
- ✅ AI opponent (Easy/Medium/Hard)
- ✅ tile scoring with premium squares

**What Needs Implementation (~1,000 lines):**
```
src/stage3/
├── mod.rs (150 lines) - Plugin, integrate existing board
├── components.rs (80 lines) - Board cells, rack, AI state
├── systems.rs (300 lines) - Turn logic, AI moves, validation
├── difficulty.rs (80 lines) - 5 AI difficulty levels
├── visuals.rs (150 lines) - Tile placement animations
├── ui.rs (150 lines) - Board UI, rack display, score
├── pause.rs (50 lines) - Pause menu
└── audio.rs (40 lines) - Audio hooks
```

**Key Features:**
- 7-tile rack per player
- Premium squares (DW, TW, DL, TL)
- Turn-based gameplay
- AI opponent with 5 difficulty levels:
  - D1: Random valid moves
  - D2: Prioritizes longer words
  - D3: Uses premium squares strategically
  - D4: Blocks player opportunities
  - D5: Near-optimal play (wolges integration)
- First to 100/150/200/250/300 points wins (D1-D5)

**Estimated Total:** ~1,000 lines

---

## ⚡ Stage 4: Speed Challenge (Rapid Word Formation)

### Status: 🔴 Not Started, Design Complete

**Gameplay:** Form as many words as possible from 7-tile rack in 60-90 seconds

**Complete Architecture (~1,200 lines):**
```
src/stage4/
├── mod.rs (120 lines) - Plugin, config, resources
├── components.rs (60 lines) - Tile rack, word input
├── systems.rs (350 lines) - Word submission, rack refresh, timer
├── difficulty.rs (90 lines) - 5 levels (90s → 45s, target varies)
├── visuals.rs (220 lines) - Speed effects, panic mode animations
├── ui.rs (250 lines) - Timer, rack, word input, streak display
├── pause.rs (60 lines) - Pause menu
└── audio.rs (50 lines) - Fast-paced audio events
```

**Key Systems:**
```rust
// Core Gameplay
- spawn_tile_rack() - 7 random tiles
- handle_word_input() - Type or click tiles
- validate_and_score() - Check word, award points
- refresh_rack() - New tiles after valid word
- update_speed_timer() - Countdown with warnings

// Panic Mode (last 15 seconds)
- Screen shake
- Red vignette
- Faster music
- Bonus multiplier (1.5x)

// Scoring
- Base: tile values
- Length bonus: +10 per letter over 3
- Speed bonus: Based on time remaining
- Streak multiplier: 1x → 3x for consecutive words
```

**Difficulty Levels:**
| Level | Time | Target Score | Tile Difficulty |
|-------|------|--------------|-----------------|
| D1 | 90s | 200 | Easy (common letters) |
| D2 | 75s | 300 | Normal |
| D3 | 60s | 400 | Normal |
| D4 | 50s | 500 | Hard (rare letters) |
| D5 | 45s | 600 | Very Hard |

**Estimated Total:** ~1,200 lines

---

## 🏆 Stage 5: AI Competitions (Competitive Play)

### Status: 🔴 Not Started, Design Complete

**Gameplay:** Competition bracket vs AI opponents, best-of-3 matches

**Complete Architecture (~1,500 lines):**
```
src/stage5/
├── mod.rs (140 lines) - Plugin, tournament state machine
├── components.rs (100 lines) - Bracket, AI profiles, match state
├── systems.rs (450 lines) - Tournament logic, AI play, match scoring
├── difficulty.rs (100 lines) - 5 tournament difficulties
├── visuals.rs (250 lines) - Bracket display, match animations
├── ui.rs (350 lines) - Bracket UI, match screen, victory screens
├── pause.rs (60 lines) - Pause menu
└── audio.rs (50 lines) - Tournament audio (crowd, victory)
```

**Tournament Structure:**
```
8-Player Single Elimination
├── Quarter-finals (4 matches)
├── Semi-finals (2 matches)
├── Finals (1 match)
└── Championship Match
```

**AI Opponents:**
```rust
pub struct AIOpponent {
    name: &'static str,
    difficulty: AIDifficulty,
    personality: AIPersonality, // Aggressive, Defensive, Balanced
    portrait: String, // Character icon
}

// D1: All Easy opponents
// D2: Easy → Medium progression
// D3: Medium → Hard progression
// D4: Hard → Expert progression
// D5: All Expert + final "Lexicon Master"
```

**Match Format:**
- Best-of-3 games on classic 15×15 board
- Each game to 50/75/100 points (depends on difficulty)
- Winner advances in bracket
- Lose = tournament over, retry from start

**Key Systems:**
```rust
// Tournament Management
- init_tournament_bracket() - 8 AI opponents
- advance_to_next_match() - Bracket progression
- play_tournament_match() - Best-of-3 on classic board
- determine_match_winner() - 2/3 games wins

// AI Personalities
- Aggressive: Prioritizes high-scoring moves
- Defensive: Blocks player opportunities
- Balanced: Mix of both
- Lexicon Master: Near-perfect play (final boss)

// Rewards
- Unlock cosmetics per tournament win
- Trophy collection
- Achievements ("Undefeated Champion", "Comeback King")
```

**Difficulty Levels:**
| Level | Opponents | Game Points | Match Format |
|-------|-----------|-------------|--------------|
| D1 | All Easy | 50 pts | Best of 3 |
| D2 | Easy → Medium | 60 pts | Best of 3 |
| D3 | Medium → Hard | 75 pts | Best of 3 |
| D4 | Hard → Expert | 90 pts | Best of 3 |
| D5 | Expert + Master | 100 pts | Best of 3 |

**Estimated Total:** ~1,500 lines

---

## 📊 Complete MVP Code Estimate

### Current Status:
- **Stage 1**: 2,136 lines ✅ (100% complete)
- **Core Systems**: 335 lines ✅ (lexicon + scoring)

### Remaining Work:
- **Stage 2**: ~1,500 lines (60% done, ~600 remaining)
- **Stage 3**: ~1,000 lines (foundation exists, needs integration)
- **Stage 4**: ~1,200 lines (full implementation)
- **Stage 5**: ~1,500 lines (full implementation)

### Grand Total Estimate:
```
Current:    2,500 lines ✅
Stage 2-5:  5,200 lines 🔲
━━━━━━━━━━━━━━━━━━━━━━━
Total MVP:  7,700 lines
```

---

## 🎯 Implementation Priority Order

### Already Done:
1. ✅ **Stage 1** - Complete (2,136 lines)
2. ✅ **Stage 2 Core** - Partial (625 lines)

### Recommended Order:
3. **Complete Stage 2** (~900 lines remaining)
   - Finish visuals, UI, pause, audio
   - Test match-3 mechanics work

4. **Stage 3** (~1,000 lines)
   - Integrate existing board code
   - Add tournament mode wrapper
   - 5 AI difficulty levels

5. **Stage 4** (~1,200 lines)
   - Speed challenge mechanics
   - Panic mode
   - Streak system

6. **Stage 5** (~1,500 lines)
   - Competition bracket
   - AI personalities
   - Championship match

---

## 🏗️ Shared Systems (Already Built)

These systems work across ALL stages:

✅ **Lexicon** (`src/lexicon/mod.rs`)
- 167,737 words loaded
- O(1) validation
- Length filtering (2-15 letters)

✅ **Scoring** (`src/scoring/mod.rs`)
- tile values
- Time bonuses
- Combo multipliers
- Reusable for all stages

✅ **Visual Patterns** (from Stage 1)
- Selection highlighting
- Validation animations
- Score popups
- Particle effects
- All patterns reusable

✅ **UI Patterns** (from Stage 1)
- Start screen template
- HUD template
- Pause menu template
- Results screen template
- All adaptable to new stages

---

## 🎨 Design Consistency

All stages follow same patterns:

**Module Structure:**
```
src/stageX/
├── mod.rs          - Plugin & config
├── components.rs   - ECS components
├── systems.rs      - Gameplay logic
├── difficulty.rs   - 5 difficulty levels
├── visuals.rs      - Visual feedback
├── ui.rs           - UI screens
├── pause.rs        - Pause menu
└── audio.rs        - Audio hooks
```

**State Flow:**
```
GameBoard (Start Screen)
    ↓
StageXPlaying (Gameplay)
    ↓ (ESC)
StageXPaused (Pause Menu)
    ↓ (ESC / Resume)
StageXPlaying
    ↓ (Win/Lose)
Results (Results Screen)
```

**UI Components:**
- Score display (top-left)
- Timer display (top-center)
- Objective display (top-right)
- Current action (bottom-center)
- Pause menu (ESC key)

---

## 🚀 Next Steps

### To Complete ALL Stages:

**Option A: Sequential (Safest)**
1. Finish Stage 2 (900 lines)
2. Implement Stage 3 (1,000 lines)
3. Implement Stage 4 (1,200 lines)
4. Implement Stage 5 (1,500 lines)
**Total:** 4,600 additional lines

**Option B: Parallel Modules**
1. Complete all visuals.rs files (4 × 250 = 1,000 lines)
2. Complete all ui.rs files (4 × 350 = 1,400 lines)
3. Complete all systems.rs files (remaining ~2,200 lines)
**Total:** Same 4,600 lines, different order

**Option C: Core First**
1. All core systems (mod.rs, components.rs, systems.rs) first
2. Then all polish (visuals, UI, pause, audio) at end
**Total:** Same 4,600 lines

---

## ✅ Quality Standards

All stages must meet Stage 1 quality bar:

**Code Quality:**
- ✅ Clean architecture (ECS patterns)
- ✅ Modular design (reusable components)
- ✅ Documented functions
- ✅ Error handling
- ✅ Performance optimized

**Gameplay Quality:**
- ✅ Clear visual feedback
- ✅ Responsive controls
- ✅ Balanced difficulty progression
- ✅ Win/lose conditions work
- ✅ Audio event hooks ready

**UI Quality:**
- ✅ Professional appearance
- ✅ Clear information hierarchy
- ✅ Button hover/click states
- ✅ Responsive layouts
- ✅ Consistent styling

---

## 📝 Summary

**Architecture Status:** ✅ Complete for all 5 stages
**Implementation Status:**
- Stage 1: ✅ 100% (2,136 lines)
- Stage 2: 🟡 60% (625/1,500 lines)
- Stage 3: 🔴 0% (design complete)
- Stage 4: 🔴 0% (design complete)
- Stage 5: 🔴 0% (design complete)

**Remaining Work:** ~4,600 lines of implementation
**Estimated Time:** With current velocity, ~6-8 hours of coding

**The path to 100% MVP is clear and achievable!** 🚀

---

**Last Updated:** 2025-11-18
**Status:** Ready for continued implementation
