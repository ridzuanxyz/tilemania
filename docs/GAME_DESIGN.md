# 🎮 Game Design Document
**Scrabble Learning Game (TileMania)**

---

## 📋 Document Overview
This document defines the gameplay mechanics, user experience, and learning design for each stage of the Scrabble training game.

**Last Updated:** 2025-10-08
**Design Version:** 1.0
**Target Audience:** Children ages 7-12 aspiring to competitive Scrabble play

---

## 🎯 Design Philosophy

### Core Principles
1. **Fun First:** Learning should never feel like studying
2. **Instant Feedback:** Every action gets immediate visual/audio response
3. **Progressive Challenge:** Difficulty scales with player skill
4. **Intrinsic Motivation:** Progress unlocks, not paywalls
5. **Tournament-Ready Skills:** Every mechanic teaches real competitive patterns

### Learning Psychology
- **Flow State:** Difficulty matches skill level (challenge = skill + 10%)
- **Reward Schedule:** Variable ratio reinforcement (surprise bonuses)
- **Mastery Visibility:** Clear progress bars and achievement badges
- **Growth Mindset:** Mistakes show as "learning opportunities," not failures

---

## 🕹️ Core Game Loop

```
┌─────────────────────────────────────────────────┐
│                                                 │
│   Play Mini-Game → Get Feedback → Earn XP     │
│         ↓              ↓              ↓         │
│   Learn Pattern → Reinforce → Unlock Next     │
│         ↓              ↓              ↓         │
│   Increase Skill → Confidence → Tournament!    │
│                                                 │
└─────────────────────────────────────────────────┘
```

---

## 🏆 Progression System

### Mastery Tracking

Each word/pattern has a **Strength Score** (0.0 to 1.0):
- **0.0 - 0.3:** Novice (red badge)
- **0.4 - 0.6:** Learning (yellow badge)
- **0.7 - 0.9:** Proficient (green badge)
- **1.0:** Mastered (gold star)

### Stage Unlocking

| Stage | Unlock Requirement |
|-------|-------------------|
| 1     | Always unlocked |
| 2     | Stage 1: 70% mastery (89/127 words) |
| 3     | Stage 2: 70% mastery |
| 3.5   | Stage 3: 60% mastery |
| 4     | Stage 3.5: 60% mastery |
| 5     | Stage 4: 65% mastery |
| 6     | Stage 5: Win 3 games against Intermediate AI |

### XP & Leveling

**XP Sources:**
- Correct word: 10 XP × word length × multiplier
- Streak bonus: +50 XP per 10-streak
- Stage completion: 500 XP
- Daily login: 25 XP
- Perfect round: 200 XP bonus

**Player Levels:**
- Level 1-5: Beginner
- Level 6-10: Apprentice
- Level 11-20: Intermediate
- Level 21-30: Advanced
- Level 31+: Expert

---

## 🎨 Visual & UI Design

### Color Palette
- **Primary:** Bright teal (`#00D9FF`)
- **Secondary:** Sunny yellow (`#FFD700`)
- **Success:** Vibrant green (`#00FF7F`)
- **Error:** Soft red (`#FF6B6B`)
- **Background:** Clean white/light gray gradient
- **Text:** Dark charcoal (`#2C3E50`)

### Typography
- **Headers:** Bold, rounded sans-serif (e.g., Nunito Bold)
- **Body:** Clean sans-serif (e.g., Open Sans)
- **Game Letters:** Mono-spaced, high-contrast (e.g., JetBrains Mono)

### Mascot: "Lexi the Owl"
- Friendly cartoon owl character
- Appears in corners during gameplay
- Animated reactions:
  - **Correct answer:** Thumbs up, sparkle eyes
  - **Wrong answer:** Sympathetic head tilt
  - **Streak milestone:** Confetti celebration
  - **New unlock:** Excited jumping

### UI Layout

```
┌─────────────────────────────────────────────────┐
│  🏠 Menu    👤 Profile    ⚙️ Settings          │
├─────────────────────────────────────────────────┤
│                                                 │
│         [GAME AREA - DYNAMIC CONTENT]          │
│                                                 │
│                                                 │
├─────────────────────────────────────────────────┤
│  Score: 1250    Streak: 7🔥    ⏱️ 1:45         │
│  [█████████░░░░░░] 65% Mastery                 │
└─────────────────────────────────────────────────┘
       [Lexi mascot in corner]
```

---

## 📚 Stage-by-Stage Design

---

## 🎯 Stage 1: 2-Letter Words

### Learning Objective
Master all 127 CSW24 2-letter words with instant recall speed.

### Gameplay Mechanic

**Type:** Reaction-based typing game

**Flow:**
1. Two letters appear at top of screen
2. Letters fall slowly toward bottom
3. Player types the word if valid (or presses Space to skip)
4. Instant feedback: ✓ or ✗
5. Next pair appears immediately

**Visual Design:**
- Large, colorful letter tiles (Scrabble-style)
- Smooth gravity-based falling animation
- Bounce effect when landing
- Particle burst on correct answer

### Difficulty Progression

| Level | Fall Speed | Distractor Letters | Words per Round |
|-------|-----------|-------------------|-----------------|
| 1-10  | Slow (5s) | None | 10 |
| 11-20 | Medium (3s) | 1-2 invalid pairs | 15 |
| 21-30 | Fast (2s) | 3-4 invalid pairs | 20 |
| 31+   | Very Fast (1.5s) | 5+ invalid pairs | 25 |

### Scoring
- **Correct:** +10 XP
- **Wrong:** -5 XP (no negative total)
- **Streak:** +2 XP per streak level
- **Speed Bonus:** +5 XP if answered within 1 second

### Mastery Criteria
- 100/127 words with strength ≥0.7
- Average response time <2 seconds

### SRS Integration
- Words with low strength appear 2x more frequently
- Recently missed words reappear within 5 rounds

---

## 🔤 Stage 2: 3-4 Letter Construction

### Learning Objective
Build 3-4 letter words from given tiles, developing anagram instincts.

### Gameplay Mechanic

**Type:** Tile manipulation puzzle

**Flow:**
1. Player receives 5-6 letter tiles
2. Drag and drop (or type) to arrange valid words
3. Can form multiple words (bonus points)
4. Timer counts down (45 seconds)
5. Submit or auto-submit when time expires

**Visual Design:**
- Draggable Scrabble tiles
- Glow effect on selected tiles
- Snap-to-grid animation
- Rainbow border on multi-word submissions

### Difficulty Progression

| Level | Tiles Given | Time Limit | Valid Words Available |
|-------|------------|-----------|---------------------|
| 1-10  | 5 tiles | 60s | 3-5 words |
| 11-20 | 6 tiles | 45s | 5-8 words |
| 21+   | 6 tiles | 30s | 8+ words |

### Scoring
- **3-letter word:** 30 XP
- **4-letter word:** 50 XP
- **Multiple words:** +20 XP per additional word
- **Time bonus:** +1 XP per second remaining

### Mastery Criteria
- 70% success rate (valid word submitted) over 50 attempts
- Average 2+ words per round

---

## 🪝 Stage 3: Hooks & Extensions

### Learning Objective
Master front and back hooks to extend words during gameplay.

### Gameplay Mechanic

**Type:** Pattern recognition quiz

**Flow:**
1. Base word appears in center (e.g., "CARE")
2. Player types all valid hooks:
   - Front hooks: SCARE
   - Back hooks: CARES, CARED, CARER
3. Hints available (cost: -10 XP)
4. Time limit: 30 seconds
5. Reveal all answers at end

**Visual Design:**
- Base word in large bold text
- Hook letters slide in from left/right
- Green checkmark for correct
- Yellow highlight for missed (revealed)

### Word Selection
- High-hookability words (e.g., ARE, AT, AN, IT)
- Common 4-5 letter words with 3+ hooks
- Tournament-critical hooks (e.g., QADI → QADIS)

### Difficulty Progression

| Level | Hooks per Word | Obscure Words | Time Limit |
|-------|---------------|--------------|-----------|
| 1-10  | 1-3 hooks | 0% | 45s |
| 11-20 | 3-5 hooks | 20% | 30s |
| 21+   | 5+ hooks | 40% | 20s |

### Scoring
- **Each correct hook:** 20 XP
- **All hooks found:** +100 XP bonus
- **No hints used:** +50 XP bonus

### Mastery Criteria
- 70% of words with all hooks identified
- Average 4+ hooks per word

---

## 🔄 Stage 3.5: Anagrams & Q-Words

### Learning Objective
Recognize anagram families and master Q-without-U words.

### Gameplay Mechanic A: Anagram Solver

**Type:** Word unscrambling race

**Flow:**
1. 6-7 scrambled letters appear
2. Player types all valid anagrams (minimum 4 letters)
3. Timer: 60 seconds
4. Hints show letter patterns (e.g., "_ E _ A I N")

**Visual Design:**
- Scrambled tiles shake/rotate
- Countdown timer pulses when <10s
- Anagram list reveals progressively

**Example Anagram Families:**
- RETINA, RETAIN, RATINE
- ALERTS, ALTERS, ARTELS, ESTRAL, LASTER, SALTER, SLATER, STALER, STELAR
- SEARCH, ARCHES, CASHER, CHASER, ESCHAR, RACHES

### Gameplay Mechanic B: Q-Without-U Drill

**Type:** Flashcard-style rapid recall

**Flow:**
1. Definition or image appears (e.g., "Chinese life force")
2. Player types the Q-word (QI)
3. 5 seconds per word
4. 20-word rotation

**Q-Word List (CSW24):**
- QI, QOPH, QADI, QAID, QANAT, QAWWAL
- QINTAR, QINDAR, QWERTY, TRANQ
- And 10 more...

### Scoring
- **Anagram found:** 50 XP × word length
- **All anagrams:** +200 XP bonus
- **Q-word correct:** 30 XP
- **Perfect Q-drill:** +150 XP

### Mastery Criteria
- 60% of anagram families fully solved
- 80% accuracy on Q-word drills

---

## 🎒 Stage 4: Rack Training

### Learning Objective
Maximize points from any 7-tile rack, learning tile value and leave management.

### Gameplay Mechanic

**Type:** Best-word puzzle

**Flow:**
1. Player receives 7 random tiles (standard Scrabble distribution)
2. Find the highest-scoring word
3. Optional: AI shows top 3 best words after submission
4. Scoring uses standard Scrabble tile values

**Visual Design:**
- Rack display at bottom (classic Scrabble rack)
- Tile values shown on each tile
- Scoreboard shows current best score
- "Bingo!" animation if 7-letter word used

### Tile Values (Standard Scrabble)
```
A=1  B=3  C=3  D=2  E=1  F=4  G=2  H=4  I=1  J=8
K=5  L=1  M=3  N=1  O=1  P=3  Q=10 R=1  S=1  T=1
U=1  V=4  W=4  X=8  Y=4  Z=10  Blank=0
```

### Difficulty Progression

| Level | Rack Quality | Bingo Probability | Time Limit |
|-------|-------------|------------------|-----------|
| 1-10  | Vowel-rich | 10% | 90s |
| 11-20 | Balanced | 30% | 60s |
| 21+   | Challenging | 50% | 45s |

### Scoring
- **Word score:** Actual Scrabble point value
- **Bingo bonus:** +50 XP (if 7-letter word)
- **Within 90% of optimal:** +50 XP bonus
- **Leave quality bonus:** +20 XP for good leave (A, E, I, R, S, T remaining)

### Mastery Criteria
- Average score ≥30 points per rack
- Find 80%+ of available bingos

---

## ♟️ Stage 5: Strategy Board

### Learning Objective
Apply positional strategy on a Scrabble board with premium squares.

### Gameplay Mechanic

**Type:** Turn-based Scrabble match (simplified board)

**Flow:**
1. 9x9 board with premium squares
2. Player vs AI opponent (Beginner/Intermediate/Advanced)
3. Standard Scrabble rules, 7-tile racks
4. Timer per turn (3 minutes)
5. Game ends when tiles exhausted

**Visual Design:**
- Colorful board with TW (red), DW (pink), TL (blue), DL (light blue)
- Animated tile placement
- Score popup on each play
- Move suggestion available (cost: 1 hint token)

### Board Layout (9x9)
```
  0 1 2 3 4 5 6 7 8
0 T _ _ D _ _ _ _ T
1 _ D _ _ _ _ D _ _
2 _ _ D _ _ D _ _ _
3 D _ _ d _ d _ _ D
4 _ _ _ _ ★ _ _ _ _
5 D _ _ d _ d _ _ D
6 _ _ D _ _ D _ _ _
7 _ D _ _ _ _ D _ _
8 T _ _ D _ _ _ _ T

Legend:
T = Triple Word (3x)
D = Double Word (2x)
d = Double Letter (2x)
★ = Start square
```

### AI Personalities

**Beginner AI:**
- Plays first valid word found
- Ignores premium squares
- Random tile placement

**Intermediate AI:**
- Seeks highest-scoring play
- Uses premium squares
- Basic rack management

**Advanced AI:**
- Plans 2 turns ahead
- Controls board space
- Strategic blocking

### Scoring
- Standard Scrabble scoring with premium multipliers
- Bingo: +50 points
- Endgame bonus: opponent's remaining tile values

### Mastery Criteria
- Win 60% of games against Intermediate AI
- Average game score ≥200 points

---

## 🏆 Stage 6: Tournament Mode

### Learning Objective
Simulate real tournament conditions with full Scrabble rules.

### Gameplay Mechanic

**Type:** Full 15×15 Scrabble match

**Flow:**
1. Standard 15×15 board with all premium squares
2. Player vs Advanced AI
3. 25-minute game clock (chess clock style)
4. Blank tiles included (2 per game)
5. Challenge system: flag invalid words

**Visual Design:**
- Professional Scrabble board appearance
- Digital clock display
- Bag counter (tiles remaining)
- Move history panel
- Post-game analysis dashboard

### Advanced Features

**Blank Tiles:**
- Can represent any letter
- Zero point value
- Strategic placement training

**Challenge System:**
- Player can challenge AI's word
- AI can challenge player's word
- Wrong challenge = lose turn

**Time Management:**
- Clock counts down per turn
- Time pressure increases difficulty
- Overtime penalty: -10 points per minute

### Post-Game Analysis

**Metrics Shown:**
- Move-by-move replay
- Missed bingo opportunities
- Leave quality ratings
- Board control heatmap
- Optimal move suggestions

### Scoring
- Standard tournament scoring
- Rating points (Elo-style): +/- 10-30 per game
- Achievements for milestones (e.g., "First 100pt play")

### Mastery Criteria
- Win 40% of games against Advanced AI
- Average game score ≥300 points
- 70% challenge accuracy

---

## 🎁 Reward System

### Streaks
- **7-day login streak:** 200 XP bonus
- **Gameplay streak:** +5% XP per consecutive correct answer (max 50%)

### Badges & Achievements

| Badge | Requirement | XP Reward |
|-------|-----------|----------|
| 🔤 Word Wizard | Learn 100 words | 500 XP |
| 🪝 Hook Master | 50 perfect hook rounds | 300 XP |
| 🎯 Bingo King | Score 10 bingos | 400 XP |
| 🔥 7-Day Streak | 7 consecutive days | 200 XP |
| 🧠 Genius | Complete Stage 6 | 1000 XP |
| 📚 Lexicon Lord | 1000 words mastered | 1500 XP |

### Cosmetic Unlocks
- Tile themes (wood, marble, neon)
- Mascot outfits (wizard hat, crown, graduation cap)
- Background themes (space, forest, ocean)
- Sound packs (retro, futuristic, nature)

---

## ⚙️ Settings & Accessibility

### Gameplay Settings
- **Difficulty modifier:** -20% to +20% (affects XP and mastery)
- **SRS aggressiveness:** Low / Medium / High
- **Timer visibility:** Show / Hide
- **Hint availability:** Unlimited / Limited / Disabled

### Accessibility Options
- **Text size:** Small / Medium / Large / Extra Large
- **High contrast mode:** On / Off
- **Colorblind palette:** Deuteranopia / Protanopia / Tritanopia
- **Screen reader support:** Describe game state
- **Reduced motion:** Disable particles and complex animations

### Audio Settings
- **Master volume:** 0-100%
- **SFX volume:** 0-100%
- **Music volume:** 0-100%
- **Mute all:** Toggle

---

## 📊 Player Progress Dashboard

### Main Dashboard View

```
┌──────────────────────────────────────────────────┐
│  👤 Player: Emma                   Level: 12 ⭐   │
│  📈 Total XP: 4,250 / 5,000 (85% to next level)  │
├──────────────────────────────────────────────────┤
│                                                   │
│  📚 Words Mastered: 342 / 127,000+               │
│  🎮 Stages Completed: 3 / 6                      │
│  🔥 Current Streak: 5 days                       │
│  ⏱️ Total Play Time: 12h 34m                     │
│                                                   │
│  [Stage Progress Bars]                           │
│  Stage 1: ████████████████ 100% ✓               │
│  Stage 2: █████████████░░░  85% 🔓              │
│  Stage 3: ████████░░░░░░░░  55% 🔓              │
│  Stage 4: ░░░░░░░░░░░░░░░░   0% 🔒              │
│                                                   │
│  [Recent Achievements]                           │
│  🔤 Word Wizard - Unlocked 2 days ago            │
│  🪝 Hook Novice - Unlocked 5 days ago            │
│                                                   │
└──────────────────────────────────────────────────┘
```

### Detailed Stats (Drill-Down)

**Stage-Specific Stats:**
- High score
- Attempts
- Win rate (Stages 5-6)
- Average score
- Time spent

**Word Stats:**
- Strongest words (top 20)
- Weakest words (bottom 20)
- Recently learned
- Due for review (SRS)

---

## 🎯 First-Time User Experience (FTUE)

### Onboarding Flow

**Step 1: Welcome Screen**
- Mascot introduction: "Hi! I'm Lexi the Owl!"
- Game overview (30 seconds)
- "Let's get started!" button

**Step 2: Name & Profile**
- Enter player name
- Optional: Age range (7-9, 10-12, 13+)
- Difficulty auto-adjusts based on age

**Step 3: Tutorial (Stage 1 Preview)**
- 5 sample 2-letter words with guidance
- Lexi explains: "Type the word before it falls!"
- First correct word triggers celebration

**Step 4: Lexicon Selection**
- "We use CSW24 (international words). Want to change it?"
- Option to load custom lexicon (skippable)

**Step 5: Jump In!**
- Unlock Stage 1
- Show progress bar
- Begin gameplay

---

## 🔄 Retention Mechanics

### Daily Challenges
- Special word sets (e.g., "Animal words day")
- 2x XP for 1 hour
- Exclusive badge rewards

### Weekly Tournaments
- Leaderboard competition (local device only for MVP)
- Top 10% get special badge
- Resets every Monday

### Comeback Bonuses
- Missed 3+ days? "Welcome back! Here's 100 XP!"
- SRS temporarily easier after long breaks

---

## 📐 Design Constraints

### Performance
- 60fps minimum on low-end devices
- <2s load time for stage transitions
- Smooth animations (no jank)

### Usability
- Touch targets ≥44×44 pixels (mobile)
- Keyboard shortcuts for all actions
- <3 taps to reach any feature

### Content
- All words verified against CSW24
- No inappropriate words in UI text
- Age-appropriate mascot design

---

## 🧪 Playtesting Checklist

### Key Questions
- [ ] Is the game fun without a Scrabble background?
- [ ] Do players understand the progression system?
- [ ] Are difficulty spikes frustrating or motivating?
- [ ] Does the mascot feel helpful or annoying?
- [ ] Are load times acceptable?
- [ ] Do players return after Day 1?

### Success Metrics
- **Session length:** >10 minutes average
- **Retention:** >40% return next day
- **Stage completion:** >60% complete Stage 1
- **Satisfaction:** >4.0/5.0 rating

---

## 🚀 Future Expansion Ideas

### New Game Modes
- **Speed Mode:** Faster timers, bigger rewards
- **Zen Mode:** No timers, pure learning
- **Puzzle Mode:** Daily word puzzles
- **Multiplayer Mode:** Race against friends

### Advanced Features
- **Voice Input:** Speak words instead of typing
- **AR Mode:** Place tiles in physical space
- **Custom Challenges:** Teachers create assignments
- **Social Sharing:** Share achievements

---

**Document Status:** ✅ Complete
**Next Review Date:** 2025-11-08
**Maintained By:** Game Design Team
