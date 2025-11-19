# 🎮 TileMania User Guide

**Welcome to TileMania!** This guide will help you get started and master all 5 stages of the game.

---

## 📋 Table of Contents

1. [Installation](#installation)
2. [Launching the Game](#launching-the-game)
3. [Main Menu](#main-menu)
4. [Stage 1: Falling Letters](#stage-1-falling-letters)
5. [Stage 2: Tile Matching](#stage-2-tile-matching)
6. [Stage 3: Classic Board](#stage-3-classic-board)
7. [Stage 4: Speed Challenge](#stage-4-speed-challenge)
8. [Stage 5: AI Competitions](#stage-5-ai-competitions)
9. [Keyboard Controls](#keyboard-controls)
10. [Settings](#settings)
11. [FAQ](#faq)

---

## Installation

### Prerequisites

**What you need:**
- A computer running Windows, macOS, or Linux
- About 500 MB of free disk space
- Internet connection (for initial download only)

### Quick Installation

**Option 1: Download Pre-built Binary (Coming Soon)**
1. Visit the [Releases page](https://github.com/ridzuanxyz/tilemania/releases)
2. Download the version for your platform
3. Extract the zip file
4. Run `tilemania.exe` (Windows) or `tilemania` (Mac/Linux)

**Option 2: Build from Source**

If you have Rust installed:

```bash
# 1. Clone the repository
git clone https://github.com/ridzuanxyz/tilemania.git
cd tilemania

# 2. Build and run
cargo run --release
```

**For detailed build instructions, see [BUILD_GUIDE.md](BUILD_GUIDE.md)**

---

## Launching the Game

### Desktop (Native)

**Windows:**
- Double-click `tilemania.exe`

**macOS:**
- Double-click `tilemania.app`
- If you see a security warning, right-click → Open

**Linux:**
```bash
./tilemania
```

### Web Browser (WASM)

If running the web version:
1. Open your browser
2. Navigate to the hosted URL
3. The game loads automatically

---

## Main Menu

When you start TileMania, you'll see the main menu with these options:

```
┌─────────────────────────────┐
│      🧠 TILEMANIA          │
│                             │
│   [Stage 1] Falling Letters │
│   [Stage 2] Tile Matching   │
│   [Stage 3] Classic Board   │
│   [Stage 4] Speed Challenge │
│   [Stage 5] AI Competitions │
│   [Settings]                │
│   [Quit]                    │
└─────────────────────────────┘
```

**Navigation:**
- **Arrow Keys (↑/↓)**: Select a stage
- **Enter**: Start selected stage
- **Escape**: Exit game

**Tip:** Start with Stage 1 if you're new to the game!

---

## Stage 1: Falling Letters

**Goal:** Form valid 2-letter words by catching falling letters

### How to Play

1. **Start the Stage**
   - Select difficulty (Beginner → Expert)
   - Press Enter to begin

2. **Gameplay**
   - Letters fall from the top of the screen
   - Type the first letter, then the second letter
   - If they form a valid 2-letter word, you score points!
   - Invalid words: no points, combo breaks

3. **Scoring**
   - Valid word: +10 points × combo multiplier
   - Combo multiplier: 1x → 2x → 3x (consecutive valid words)
   - Time bonus: +50 points for finishing early

### Power-Ups

| Power-Up | Effect | How to Use |
|----------|--------|------------|
| ⏱️ **Slow Motion** | Slows falling speed by 50% for 10s | Press `1` |
| 💣 **Bomb** | Clears screen of letters | Press `2` |
| 🔀 **Shuffle** | Randomizes all letters | Press `3` |
| ⏰ **Extra Time** | Adds 15 seconds to timer | Press `4` |

**Earning Power-Ups:** Automatically granted after streaks of 5+ valid words

### Difficulty Levels

| Difficulty | Time Limit | Fall Speed | Goal |
|------------|------------|------------|------|
| Beginner | 90 seconds | Slow | 10 words |
| Easy | 75 seconds | Medium-slow | 15 words |
| Medium | 60 seconds | Medium | 20 words |
| Hard | 50 seconds | Fast | 25 words |
| Expert | 45 seconds | Very fast | 30 words |

### Tips & Strategy

✅ **Learn common 2-letter words:**
- AA, AB, AD, AE, AG, AH, AI, AL, AM, AN, AR, AS, AT, AW, AX, AY
- BA, BE, BI, BO, BY
- (127 valid 2-letter words in total)

✅ **Maintain combos:** Avoid guessing invalid words

✅ **Save power-ups:** Use Slow Motion when overwhelmed

✅ **Stay calm:** Speed comes with practice

### Controls

- **Type letters**: A-Z keyboard
- **Power-ups**: Number keys 1-4
- **Pause**: ESC
- **Restart**: R (when paused)

---

## Stage 2: Tile Matching

**Goal:** Form valid words on an 8×8 grid using Match-3 mechanics

### How to Play

1. **Grid Layout**
   - 8×8 grid filled with letter tiles
   - Each tile has a letter A-Z

2. **Gameplay**
   - Click/select a tile to start
   - Click adjacent tile (horizontal/vertical) to swap
   - Form valid 3+ letter words horizontally or vertically
   - Matched words disappear, tiles fall down (cascade)
   - New tiles appear from top

3. **Scoring**
   - 3-letter word: 30 points
   - 4-letter word: 80 points
   - 5+ letter word: 150+ points
   - Cascade bonus: +10 points per cascade level

### Word Formation Rules

✅ **Valid:**
- Horizontal words (left to right)
- Vertical words (top to bottom)
- Minimum 3 letters

❌ **Invalid:**
- Diagonal words
- Backwards words
- 2-letter words (not counted)

### Difficulty Levels

| Difficulty | Target Score | Moves Limit | Time |
|------------|--------------|-------------|------|
| Beginner | 500 | Unlimited | Unlimited |
| Easy | 1,000 | 50 moves | 5 minutes |
| Medium | 2,000 | 40 moves | 4 minutes |
| Hard | 3,500 | 30 moves | 3 minutes |
| Expert | 5,000 | 25 moves | 2 minutes |

### Tips & Strategy

✅ **Look for 4-letter words:** More points than 3-letter

✅ **Plan cascades:** Set up chain reactions

✅ **Use common patterns:**
- -ING endings
- -ED endings
- Common prefixes (UN-, RE-)

✅ **Check both directions:** Horizontal AND vertical

✅ **Don't rush:** Think before swapping

### Controls

- **Mouse Click**: Select and swap tiles
- **Arrow Keys**: Navigate grid (alternative)
- **Enter**: Swap selected tiles
- **Pause**: ESC

---

## Stage 3: Classic Board

**Goal:** Play on a 15×15 word tile board against AI opponent

### How to Play

1. **Board Setup**
   - 15×15 grid with premium squares
   - You start with 7 tiles in your rack
   - AI opponent also has 7 tiles

2. **Gameplay**
   - Your turn: Place tiles on the board to form words
   - Words must connect to existing words
   - Click board squares to place tiles
   - Click "Submit" to score your word
   - AI takes its turn
   - First to reach target score wins (or highest score when bag empties)

3. **Scoring**
   - Each letter has a point value (A=1, Z=10, etc.)
   - Premium squares multiply your score:
     - **DL** (Double Letter): 2× letter value
     - **TL** (Triple Letter): 3× letter value
     - **DW** (Double Word): 2× word value
     - **TW** (Triple Word): 3× word value
   - Bonus: +50 points for using all 7 tiles (bingo)

### Premium Squares Layout

```
    TW . . DL . . . TW . . . DL . . TW
    .  DW . . . TL . . . TL . . . DW .
    .  . DW . . . DL . DL . . . DW . .
    DL . . DW . . . DL . . . DW . . DL
    .  . . . DW . . . . . DW . . . .
    .  TL . . . TL . . . TL . . . TL .
    .  . DL . . . DL . DL . . . DL . .
    TW . . DL . . . DW . . . DL . . TW
    (Center square is start position)
```

### Tile Point Values

| Letters | Points |
|---------|--------|
| A, E, I, O, U, L, N, S, T, R | 1 |
| D, G | 2 |
| B, C, M, P | 3 |
| F, H, V, W, Y | 4 |
| K | 5 |
| J, X | 8 |
| Q, Z | 10 |

### AI Difficulty Levels

| Difficulty | AI Strategy | Avg Word Score |
|------------|-------------|----------------|
| Beginner | Random words | 10-20 points |
| Easy | Short words | 20-40 points |
| Medium | Balanced play | 30-60 points |
| Hard | Strategic blocking | 50-100 points |
| Expert | Optimal moves | 80-150 points |

### Tips & Strategy

✅ **Use premium squares:** Aim for TW (Triple Word) with high-value letters

✅ **Save high-value tiles:** Q, Z, X, J for premium squares

✅ **Form parallel words:** Create multiple words in one turn

✅ **Block opponent:** Cover premium squares before AI uses them

✅ **Balance rack:** Keep vowels and consonants balanced

✅ **Learn 2-letter words:** Essential for tight spaces

### Controls

- **Mouse Click**: Select tiles and board positions
- **Drag & Drop**: Place tiles on board
- **Backspace**: Return tile to rack
- **Enter/Submit**: Play word
- **Shuffle**: Rearrange rack tiles
- **Pass**: Skip turn (lose points)
- **Exchange**: Swap tiles (lose turn)
- **Pause**: ESC

---

## Stage 4: Speed Challenge

**Goal:** Form as many valid words as possible under time pressure

### How to Play

1. **Setup**
   - You get a 7-tile rack
   - Timer starts immediately
   - No board—just form words from your rack

2. **Gameplay**
   - Drag tiles to form a valid word
   - Click "Submit" to score
   - Rack instantly refills with new tiles
   - Keep forming words until time runs out

3. **Scoring**
   - Word length × base points × streak multiplier
   - 3-letter: 10 points
   - 4-letter: 20 points
   - 5-letter: 40 points
   - 6-letter: 80 points
   - 7-letter: 150 points + Bonus

### Streak Multiplier

| Consecutive Words | Multiplier |
|-------------------|------------|
| 1-4 words | 1.0x |
| 5-9 words | 1.05x |
| 10-14 words | 1.10x |
| 15-19 words | 1.15x |
| 20-24 words | 1.20x |
| 25+ words | 1.25x |

**Streak breaks if:** You submit an invalid word or take >10 seconds

### Difficulty Levels

| Difficulty | Time Limit | Target Score |
|------------|------------|--------------|
| Beginner | 120 seconds | 500 points |
| Easy | 100 seconds | 800 points |
| Medium | 80 seconds | 1,200 points |
| Hard | 60 seconds | 1,800 points |
| Expert | 45 seconds | 2,500 points |

### Panic Mode

**When time drops below 15 seconds:**
- Screen flashes red
- Urgent music plays
- Time bonus for each word: +5 seconds

### Tips & Strategy

✅ **Think fast:** Don't overthink, submit quickly

✅ **Start with 3-letter words:** Build confidence and streak

✅ **Look for common patterns:**
- -ING
- -ED
- -ER, -EST

✅ **Use all tiles:** 7-letter words give massive bonus

✅ **Maintain streak:** Consistency > single high-scoring word

✅ **Panic mode:** Stay calm, focus on simple words

### Controls

- **Mouse**: Drag tiles to form words
- **Enter/Submit**: Play word
- **Backspace**: Clear current word
- **Shuffle**: Rearrange rack
- **Pause**: ESC (timer pauses)

---

## Stage 5: AI Competitions

**Goal:** Win an 8-player single-elimination bracket competition

### How to Play

1. **Competition Format**
   - You + 7 AI opponents
   - Single-elimination bracket
   - Best-of-3 match format
   - Quarterfinals → Semifinals → Finals

2. **Match Play**
   - Each match is played on classic 15×15 board
   - Best-of-3 games (first to win 2 games advances)
   - Same rules as Stage 3 (Classic Board)

3. **Progression**
   - **Round 1 (Quarterfinals):** 8 players → 4 winners
   - **Round 2 (Semifinals):** 4 players → 2 winners
   - **Round 3 (Finals):** 2 players → 1 champion

### AI Opponents

| Opponent | Personality | Strategy | Difficulty |
|----------|-------------|----------|------------|
| **Novice Bot** | Cautious | Short safe words | Easy |
| **Word Wizard** | Aggressive | High-scoring moves | Medium |
| **The Professor** | Balanced | Strategic placement | Medium |
| **Speed Demon** | Fast | Quick plays | Hard |
| **Defensive Dan** | Blocking | Denies premium squares | Hard |
| **Combo King** | Cascading | Multi-word combos | Expert |
| **The Champion** | Optimal | Perfect play | Expert |

### Competition Bracket

```
Round 1 (Quarters)    Round 2 (Semis)      Finals
┌─────────────┐
│ YOU         │───┐
│ Novice Bot  │   │
└─────────────┘   ├─────┐
┌─────────────┐   │     │
│ Word Wizard │───┘     │
│ Professor   │         ├─────┐
└─────────────┘         │     │
┌─────────────┐   ┌─────┘     │
│ Speed Demon │───┤           ├──→ CHAMPION
│ Defensive D.│   │           │
└─────────────┘   ├─────┐     │
┌─────────────┐   │     │     │
│ Combo King  │───┘     ├─────┘
│ Champion    │         │
└─────────────┘   ┌─────┘
```

### Scoring & Victory

**Each Game:**
- First to 200 points OR
- Highest score when tile bag empties

**Match Victory:**
- First to win 2 games

**Competition Victory:**
- Win all 3 rounds (Quarters, Semis, Finals)

### Tips & Strategy

✅ **Study opponents:** Each AI has patterns

✅ **Early advantage:** Win Round 1 decisively for momentum

✅ **Adapt strategy:**
- vs. Aggressive: Block premium squares
- vs. Defensive: Create parallel words
- vs. Fast: Take time to plan optimal moves

✅ **Finals preparation:** Save best plays for championship

✅ **Manage pressure:** Stay calm in best-of-3

### Rewards

🏆 **Champion:** Unlock achievement, victory celebration

🥈 **Runner-up:** Unlock "Almost There" achievement

📊 **Statistics:** View your competition stats and AI opponent records

### Controls

Same as Stage 3 (Classic Board):
- **Mouse**: Tile placement
- **Submit**: Play word
- **Shuffle/Pass/Exchange**: Rack management
- **Pause**: ESC

---

## Keyboard Controls

### Universal Controls (All Stages)

| Key | Action |
|-----|--------|
| **ESC** | Pause game / Return to menu |
| **Enter** | Confirm / Submit |
| **Arrow Keys** | Navigate menus |
| **R** | Restart (when paused) |
| **M** | Mute/Unmute audio |

### Stage-Specific Controls

**Stage 1 (Falling Letters):**
- `A-Z`: Type letters to form words
- `1-4`: Activate power-ups

**Stage 2 (Tile Matching):**
- Mouse click: Select/swap tiles
- Arrow keys: Navigate grid

**Stage 3 (Classic Board):**
- Mouse: Select tiles and board positions
- `Backspace`: Return tile to rack
- `Shuffle`: Rearrange rack

**Stage 4 (Speed Challenge):**
- Mouse: Drag tiles
- `Enter`: Submit word
- `Backspace`: Clear word

**Stage 5 (AI Competitions):**
- Same as Stage 3

---

## Settings

Access settings from the main menu or by pressing `ESC` during gameplay.

### Available Settings

**Audio:**
- Master Volume: 0-100%
- Music Volume: 0-100%
- Sound Effects Volume: 0-100%
- Mute All: Toggle

**Graphics:**
- Fullscreen: On/Off
- Resolution: 1920×1080, 1280×720, 800×600
- VSync: On/Off

**Gameplay:**
- Show Hints: On/Off
- Auto-Submit: On/Off (Stage 4)
- Difficulty Default: Beginner → Expert

**Lexicon:**
- Current: TML (167,737 words)
- Alternative: RE-ENABLE (172,400 words)
- Custom: Load your own word list

**Controls:**
- Rebind keys (Coming Soon)

---

## FAQ

### General Questions

**Q: What age group is this game for?**
A: Ages 7-12 primarily, but enjoyable for all ages learning vocabulary.

**Q: Do I need internet to play?**
A: No! TileMania works 100% offline after installation.

**Q: How many words are in the game?**
A: 167,737 words in the TML (TileMania Lexicon), all public domain.

**Q: Can I use my own word list?**
A: Yes! See `assets/lexicons/README.md` for instructions.

---

### Gameplay Questions

**Q: Why was my word rejected?**
A: The word might not be in the TML lexicon. TML uses international English and avoids regional variants.

**Q: How do I unlock all stages?**
A: All stages are unlocked from the start! Play in any order.

**Q: Can I play multiplayer?**
A: Not yet. Multiplayer is planned for a future update.

**Q: What happens if I lose?**
A: You can retry immediately! No penalties, unlimited attempts.

---

### Technical Questions

**Q: The game won't start. What should I do?**
A: Check that:
- You have the lexicon files (`TML.txt` in `assets/lexicons/`)
- Your system meets minimum requirements
- See [BUILD_GUIDE.md](BUILD_GUIDE.md) for platform-specific help

**Q: The game is laggy. How can I improve performance?**
A: Try:
- Lowering resolution in Settings
- Disabling VSync
- Running in windowed mode
- Building with `--release` flag if running from source

**Q: Where are my save files?**
A: Currently, progress is not saved between sessions. Save system coming in v2.0.

**Q: How do I report a bug?**
A: Use the [GitHub Issues](https://github.com/ridzuanxyz/tilemania/issues) page with the bug report template.

---

### Word List Questions

**Q: Why don't you use the official word list?**
A: Official word lists (like CSW24) require expensive licenses. TML is public domain and free for everyone.

**Q: What's the difference between TML and RE-ENABLE?**
A:
- **TML**: 167,737 words, filters out regional US spellings
- **RE-ENABLE**: 172,400 words, includes US variants (color/colour)

**Q: Can I switch to CSW24 if I have a license?**
A: Yes! Place `CSW24.txt` in `assets/lexicons/` and the game will use it automatically.

---

## Tips for Parents & Teachers

### Educational Use

TileMania is designed to help children:
- ✅ Build vocabulary through play
- ✅ Improve spelling recognition
- ✅ Develop strategic thinking
- ✅ Practice under time pressure (optional)

### Classroom Setup

**Recommended progression:**
1. Stage 1 (2-3 weeks): Master 2-letter words
2. Stage 2 (3-4 weeks): Build 3-4 letter vocabulary
3. Stage 3 (ongoing): Strategic gameplay
4. Stages 4-5: Advanced challenge

**Multi-student use:**
- Take turns on one computer, OR
- Install on multiple computers (free public domain lexicon)
- Track progress manually (save system coming soon)

### Safety & Screen Time

- Game has no ads, in-app purchases, or online features
- No chat or social features
- Recommended: 20-30 minute sessions
- Encourage breaks between stages

---

## Need More Help?

**Documentation:**
- [README.md](README.md) - Project overview
- [BUILD_GUIDE.md](BUILD_GUIDE.md) - Installation & building
- [CONTRIBUTING.md](CONTRIBUTING.md) - How to contribute

**Support:**
- GitHub Issues: https://github.com/ridzuanxyz/tilemania/issues
- Read the docs: See `docs/` folder

**Community:**
- Join discussions on GitHub
- Share your high scores and strategies!

---

**Happy word building!** 🎮✨

*Last Updated: 2025-11-19*
