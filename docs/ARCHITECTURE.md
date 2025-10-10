# 🏗️ Technical Architecture Document
**Scrabble Learning Game (TileMania)**

---

## 📋 Document Overview
This document provides the technical architecture for the Scrabble Learning Game, designed for competitive Scrabble training for ages 7-12. The system is built using Rust and Bevy, with offline-first design and cross-platform deployment.

**Last Updated:** 2025-10-08
**Architecture Version:** 1.0
**Target Platforms:** Desktop (Windows/Mac/Linux), Web (WASM)

---

## 🎯 Architectural Principles

### Core Design Goals
1. **Offline-First**: No internet dependency for core gameplay
2. **Performance**: <16ms frame time (60fps minimum)
3. **Extensibility**: Easy addition of new game stages
4. **Cross-Platform**: Single codebase for desktop and web
5. **Data Integrity**: Local persistence with data validation

### Technical Constraints
- CSW24 lexicon (~280k words) must load in <2 seconds
- Word validation must complete in <5ms
- Game state transitions must feel instantaneous (<100ms)
- WASM build size target: <10MB compressed

---

## 🧱 System Architecture

### High-Level Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      Bevy Game Engine                        │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Game       │  │   Lexicon    │  │  Persistence │      │
│  │   States     │  │   System     │  │   Layer      │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         │                  │                  │              │
│  ┌──────▼──────────────────▼──────────────────▼───────┐    │
│  │              Core Systems (ECS)                     │    │
│  │  • Input  • Animation  • Scoring  • UI  • Audio    │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │           Game Stage Implementations                 │    │
│  │  Stage1  Stage2  Stage3  Stage3.5  Stage4  ...     │    │
│  └─────────────────────────────────────────────────────┘    │
│                                                               │
└─────────────────────────────────────────────────────────────┘
         │                    │                    │
    ┌────▼────┐         ┌────▼────┐         ┌────▼────┐
    │ Desktop │         │   Web   │         │  Mobile │
    │ (Native)│         │ (WASM)  │         │ (Future)│
    └─────────┘         └─────────┘         └─────────┘
```

---

## 🔧 Core Systems

### 1. Lexicon System

**Purpose:** Fast word validation and lexicon management

**Components:**
- `LexiconResource`: Bevy resource holding the active word dictionary
- `TrieNode`: Custom trie structure for O(n) word lookup
- `LexiconLoader`: Async loader for lexicon files
- `WordValidator`: Public API for word validation

**Data Structure:**
```rust
pub struct LexiconResource {
    trie: TrieNode,
    word_count: usize,
    lexicon_type: LexiconType, // CSW24, TWL, Custom
    metadata: LexiconMetadata,
}

pub enum LexiconType {
    CSW24,      // Collins Scrabble Words 2024 (default)
    TWL,        // Tournament Word List
    SOWPODS,    // British/International
    Custom(PathBuf),
}
```

**Performance Targets:**
- Load CSW24: <2 seconds on desktop, <4 seconds on WASM
- Validate word: <5ms (average), <10ms (99th percentile)
- Memory footprint: <50MB for full trie

**File Format:**
- Plain text, one word per line, UTF-8 encoded
- Optional metadata header: `# LEXICON: CSW24 VERSION: 2024`

---

### 2. Game State System

**Purpose:** Manage game flow and state transitions

**States:**
```rust
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    StageSelect,
    Stage1,      // 2-letter words
    Stage2,      // 3-4 letter construction
    Stage3,      // Hooks & extensions
    Stage3_5,    // Anagrams & Q-words
    Stage4,      // Rack training
    Stage5,      // Strategy board
    Stage6,      // Tournament mode
    Settings,
    Paused,
    GameOver,
}
```

**State Transition Rules:**
- Progressive unlock: stages unlock when previous stage achieves 70% mastery
- Settings accessible from any state
- Pause overlay available during all gameplay states

---

### 3. Persistence Layer

**Purpose:** Local storage of progress, settings, and custom lexicons

**Storage Format:** RON (Rusty Object Notation)

**Persisted Data:**
```rust
#[derive(Serialize, Deserialize)]
pub struct PlayerProfile {
    player_id: Uuid,
    name: String,
    created_at: DateTime<Utc>,

    // Progress tracking
    stage_progress: HashMap<GameStage, StageProgress>,
    total_words_learned: usize,
    total_play_time: Duration,

    // SRS data
    word_strength: HashMap<String, f32>,  // 0.0 to 1.0
    last_seen: HashMap<String, DateTime<Utc>>,

    // Settings
    lexicon_preference: LexiconType,
    custom_lexicon_path: Option<PathBuf>,
    audio_volume: f32,
    difficulty_modifier: f32,
}

#[derive(Serialize, Deserialize)]
pub struct StageProgress {
    unlocked: bool,
    mastery_percent: f32,
    high_score: u32,
    attempts: u32,
    last_played: DateTime<Utc>,
}
```

**File Locations:**
- Desktop: `~/.config/tilemania/` (Linux/Mac), `%APPDATA%/tilemania/` (Windows)
- WASM: LocalStorage API

---

### 4. Scoring System

**Purpose:** Calculate scores, track progress, implement SRS

**Base Scoring:**
```rust
pub struct ScoringEngine {
    base_points: HashMap<StageType, u32>,
    speed_multipliers: Vec<(Duration, f32)>,
    streak_bonus: StreakCalculator,
    srs_tracker: SpacedRepetitionSystem,
}
```

**Score Calculation:**
- Base points per correct word (varies by stage)
- Speed multiplier: faster answers = higher multiplier
- Streak bonus: consecutive correct answers
- Penalty for incorrect answers: -10% from current streak

**Spaced Repetition Algorithm:**
- SM-2 algorithm (SuperMemo 2)
- Words with low strength (<0.5) appear more frequently
- Mastery threshold: 70% of word set with strength >0.7

---

### 5. Input System

**Purpose:** Handle keyboard, mouse, and touch input across platforms

**Input Types:**
```rust
pub enum InputMode {
    Keyboard,        // Desktop typing
    Touch,           // Mobile tap/swipe
    MouseClick,      // Desktop clicking
}
```

**Key Handlers:**
- `handle_letter_input()`: Process typed letters
- `handle_word_submit()`: Enter/Return key or tap submit button
- `handle_backspace()`: Delete last character
- `handle_pause()`: ESC key or pause button

---

### 6. Animation & Visual Effects System

**Purpose:** Smooth animations and particle effects for engagement

**Libraries:**
- `bevy_tweening`: Easing animations (position, scale, color, rotation)
- `bevy_hanabi`: Particle effects (confetti, sparkles, explosions)

**Animation Presets:**
- `tile_fall()`: Gravity-based falling animation
- `tile_bounce()`: Elastic bounce on landing
- `word_correct()`: Green flash + confetti burst
- `word_incorrect()`: Red shake + fade
- `combo_effect()`: Growing multiplier animation

---

### 7. Audio System

**Purpose:** Sound effects and background music

**Library:** `bevy_kira_audio`

**Audio Assets:**
- `letter_tap.ogg`: Typing sound
- `word_correct.ogg`: Success chime
- `word_incorrect.ogg`: Buzzer sound
- `level_up.ogg`: Stage unlock fanfare
- `background_menu.ogg`: Menu music loop
- `background_gameplay.ogg`: Gameplay music loop

**Volume Control:**
- Master volume
- SFX volume (separate)
- Music volume (separate)

---

## 🎮 Game Stage Architecture

### Stage Plugin Pattern

Each stage is implemented as a Bevy plugin:

```rust
pub struct Stage1Plugin;

impl Plugin for Stage1Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Stage1), setup_stage1)
            .add_systems(Update, (
                spawn_falling_letters,
                handle_letter_input,
                validate_word_submission,
                update_score,
            ).run_if(in_state(GameState::Stage1)))
            .add_systems(OnExit(GameState::Stage1), cleanup_stage1);
    }
}
```

### Stage-Specific Systems

#### Stage 1: 2-Letter Words
- **Mechanic:** Letters fall from top, player types to match
- **Word Pool:** All 127 CSW24 2-letter words
- **Difficulty Progression:** Increase fall speed, add distractor letters
- **Mastery Metric:** 100/127 words with >0.7 strength

#### Stage 2: 3-4 Letter Construction
- **Mechanic:** Given 4-6 tiles, form valid words
- **Word Pool:** Common 3-4 letter words (~5000 words)
- **Difficulty Progression:** Fewer tiles, less common words
- **Mastery Metric:** 70% success rate over 50 attempts

#### Stage 3: Hooks & Extensions
- **Mechanic:** Given base word, add front/back hooks
- **Examples:** CARE → SCARE, CARE → CARES
- **Word Pool:** High-hookability words
- **Mastery Metric:** 70% hook identification rate

#### Stage 3.5: Anagrams & Q-Words
- **Mechanic A:** Anagram solver (RETINA → RETAIN, RATINE)
- **Mechanic B:** Q-without-U drill (QI, QOPH, QADI, QAID, QANAT, etc.)
- **Word Pool:** 6-7 letter anagram families, 20 Q-without-U words
- **Mastery Metric:** 60% anagram solve rate + 80% Q-word recall

#### Stage 4: Rack Training
- **Mechanic:** Given 7 tiles, find highest-scoring word
- **Scoring:** Scrabble tile values (A=1, Z=10, etc.)
- **Difficulty Progression:** Harder racks, bingo opportunities
- **Mastery Metric:** Average score >30 points per rack

#### Stage 5: Strategy Board
- **Mechanic:** Play against AI on simplified board (9x9)
- **Focus:** Premium squares (TW, DW, TL, DL), board control
- **AI Difficulty:** Beginner → Intermediate → Advanced
- **Mastery Metric:** Win 60% against intermediate AI

#### Stage 6: Tournament Mode
- **Mechanic:** Full 15x15 board, timed games, blank tiles
- **Rules:** Standard Scrabble rules, 25-minute game clock
- **AI Difficulty:** Advanced tournament-level play
- **Mastery Metric:** Win 40% against advanced AI

---

## 🤖 AI Opponent System (Stages 5-6)

### AI Architecture

```rust
pub struct ScrabbleAI {
    difficulty: AIDifficulty,
    lexicon: Arc<LexiconResource>,
    move_generator: MoveGenerator,
    evaluator: PositionEvaluator,
}

pub enum AIDifficulty {
    Beginner,       // Random valid moves
    Intermediate,   // Greedy scoring
    Advanced,       // Minimax with board control
}
```

### Move Generation Strategy

**Beginner AI:**
- Find all valid moves
- Pick randomly from top 50%

**Intermediate AI:**
- Find all valid moves
- Sort by score
- Pick from top 10%

**Advanced AI:**
- Find all valid moves
- Evaluate board control
- Consider leave quality
- Apply minimax lookahead (depth 2)

---

## 🔒 Data Flow & Security

### Offline-First Data Flow

```
User Action → Input System → Game Logic → State Update
                                   ↓
                          Persistence Layer
                                   ↓
                          Local Storage (RON)
```

### Custom Lexicon Loading

```
User selects file → File validation → Parse → Build trie
                                                  ↓
                                    Cache to LocalStorage
                                                  ↓
                                    Set as active lexicon
```

**Validation Rules:**
- File size <50MB
- Valid UTF-8 encoding
- Max word length: 15 characters
- Min word length: 2 characters
- Alphanumeric only (A-Z)

---

## 📦 Build & Deployment

### Cargo Features

```toml
[features]
default = ["native"]
native = ["bevy/default"]
web = ["bevy/webgl2"]
dev = ["bevy/dynamic_linking"]
```

### Build Commands

**Desktop (Native):**
```bash
cargo build --release --features native
```

**Web (WASM):**
```bash
cargo build --release --target wasm32-unknown-unknown --features web
wasm-bindgen --out-dir ./web/pkg --target web ./target/wasm32-unknown-unknown/release/tilemania.wasm
```

### Asset Bundling

- **Desktop:** Embed assets at compile time using `include_bytes!`
- **Web:** Bundle assets with WASM, serve via CDN
- **Lexicon:** Compress with gzip, decompress at runtime

---

## 📊 Performance Monitoring

### Key Metrics

- **Frame Time:** <16ms (60fps target)
- **Word Validation:** <5ms per lookup
- **State Transition:** <100ms perceived latency
- **Memory Usage:** <200MB RAM (desktop), <150MB (WASM)
- **WASM Bundle Size:** <10MB compressed

### Profiling Tools

- Bevy's built-in diagnostics plugin
- Chrome DevTools (WASM profiling)
- `cargo flamegraph` (native profiling)

---

## 🧪 Testing Strategy

### Unit Tests
- Lexicon loading and validation
- Word validation correctness
- Scoring calculations
- SRS algorithm correctness

### Integration Tests
- State transitions
- Save/load functionality
- Custom lexicon loading

### Performance Tests
- Lexicon load time
- Word validation benchmarks
- Frame time consistency

### User Acceptance Tests
- Playtest each stage
- Input responsiveness
- Visual feedback clarity

---

## 🔮 Future Architecture Considerations

### Phase 2+ Enhancements

1. **Online Multiplayer**
   - WebSocket server for real-time matches
   - Matchmaking system
   - Replay system

2. **Teacher Dashboard**
   - Web portal for progress tracking
   - Class management
   - Custom word list creation

3. **Mobile Native**
   - iOS/Android builds
   - Touch-optimized UI
   - Offline sync

4. **Advanced Analytics**
   - Word frequency analysis
   - Learning curve visualization
   - Personalized recommendations

---

## 📚 References

- [Bevy Engine Documentation](https://bevyengine.org/)
- [CSW24 Lexicon Specification](https://scrabble.org.au/words/csw24)
- [SM-2 Spaced Repetition Algorithm](https://www.supermemo.com/en/archives1990-2015/english/ol/sm2)
- [Trie Data Structure](https://en.wikipedia.org/wiki/Trie)

---

**Document Status:** ✅ Complete
**Next Review Date:** 2025-11-08
**Maintained By:** Architecture Team
