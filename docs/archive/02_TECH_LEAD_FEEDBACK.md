# 🔍 Tech Lead Feedback on Architecture Decisions
**Scrabble Learning Game (TileMania)**

---

## 📋 Document Overview
This document provides the Tech Lead's formal review and feedback on the Architecture Decision Record (ADR), including conditional approval and critical research requirements.

**Review Date:** 2025-10-08
**Reviewer:** Tech Lead / Scrabble Grandmaster
**Document Reviewed:** [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md)
**Status:** ⚠️ **CONDITIONAL APPROVAL**

---

## 🎯 Executive Summary

**Overall Assessment:** The ADR represents excellent architectural thinking and the wolges decision is strategically brilliant. However, **3 critical technical risks** must be validated in Sprint 1 before we can proceed with confidence.

**Approval Status:** ✅ **95% Aligned** with **5 conditions** for full sign-off

**Risk Level:** 🟡 **MEDIUM** - External dependency (wolges) needs validation

---

## ✅ What I Strongly Approve

### Strong Approvals

1. **wolges decision is BRILLIANT** ✅
   - Solves 3 critical blockers simultaneously
   - Industry-standard, battle-tested solution
   - Saves 4-6 weeks of development time
   - As fellow Scrabble grandmasters, we both know this is the right call

2. **Sparse SRS tracking** ✅
   - Perfect solution to memory bloat
   - 98% memory reduction is massive
   - Elegant and pragmatic

3. **IndexedDB over LocalStorage** ✅
   - Correct choice for WASM
   - 50MB+ quota vs 5-10MB
   - Future-proof

4. **Bevy 0.15 upgrade** ✅
   - Necessary and overdue
   - Better community support
   - Performance improvements

5. **TileBag architecture** ✅
   - Clean, simple, correct
   - Proper Scrabble tile distribution
   - Well-integrated with Bevy ECS

6. **Timeline adjustment to Week 13** ✅
   - Realistic and honest
   - Includes buffer for integration
   - Conservative estimate appreciated

---

## 🚨 CRITICAL CONCERNS

### **Concern #1: wolges KWG Format Availability**

**Severity:** 🔴 **CRITICAL - BLOCKS SPRINT 3**

**Problem:**
- ADR shows: `include_bytes!("../../assets/lexicons/CSW24.kwg")` ([ARCHITECTURE_DECISIONS.md:97](ARCHITECTURE_DECISIONS.md))
- **wolges uses KWG format, NOT plain text**
- CSW24 is typically distributed as **plain text** (.txt file)
- We need to **convert** CSW24.txt → CSW24.kwg before Sprint 3

**Questions:**
1. Do we have access to a pre-built CSW24.kwg file?
2. If not, how do we generate it?
3. Does wolges provide conversion tools?
4. What's the conversion process?

**Technical Details:**
From wolges crate documentation:
- KWG = "Kurnia Word Graph" (compressed DAWG format)
- Requires build tools: `wolges-make` or `wolges::kwg::build_kwg()`
- Plain text → KWG conversion is one-time build step

**Impact:**
- If we can't generate KWG file → **Sprint 3 blocked**
- No word validation → No Stage 1 gameplay
- Critical path item

**Request:**
Please add to **Sprint 1 tasks** (Week 1):

```markdown
### Week 1: Lexicon Preparation (CRITICAL)

#### Tasks
- [ ] **Acquire CSW24 source file**
  - Download CSW24.txt from official source
  - Verify word count (~280k words)
  - Document source and licensing

- [ ] **Generate CSW24.kwg file**
  - Install wolges-make CLI tool
  - Run conversion: `wolges-make build CSW24.txt -o CSW24.kwg`
  - Verify output file size (~5-8MB expected)
  - Test load time and memory usage

- [ ] **Validate KWG file**
  - Load in test program
  - Verify sample words (QI, ZYZZYVA, etc.)
  - Compare word count with source
  - Add to assets/lexicons/ directory

#### Deliverable
- ✅ CSW24.kwg file ready for Sprint 3 integration
- ✅ Conversion process documented
- ✅ Load time benchmarked
```

**Priority:** P0 - Must complete Week 1

---

### **Concern #2: wolges Board API for 9×9 Boards**

**Severity:** 🟡 **MEDIUM - AFFECTS STAGE 5 DESIGN**

**Problem:**
- ADR shows: `WolgesBoard::new(9)` for Stage 5 ([ARCHITECTURE_DECISIONS.md:634](ARCHITECTURE_DECISIONS.md))
- **I reviewed wolges source code** - the `Board` struct is designed for **15×15 standard Scrabble boards**
- wolges may not support arbitrary board sizes like 9×9
- No API documentation found for custom board dimensions

**Questions:**
1. Does wolges actually support 9×9 boards?
2. If not, do we use 15×15 with restricted play area?
3. Or implement our own simplified board for Stage 5?

**Technical Details:**
```rust
// From wolges source (unconfirmed - needs verification)
pub struct Board {
    size: usize,  // May be hardcoded to 15
    tiles: [[Option<Tile>; 15]; 15],  // Fixed size array?
}
```

If wolges Board is fixed at 15×15, we cannot instantiate 9×9 boards.

**Impact:**
- Stage 5 design may need revision
- Either: Use full 15×15 board with simplified rules
- Or: Build custom SimpleBoard for Stage 5

**Pedagogical Consideration (As Scrabble Grandmaster):**
Using a 15×15 board with simpler rules might actually be **better** pedagogically than a 9×9 board:
- Kids learn real board layout early
- Premium square positions become familiar
- Seamless transition to Stage 6
- No need to "unlearn" 9×9 layout

**Proposed Alternative Design:**

```rust
// Stage 5: Full 15x15 board with training wheels
pub struct Stage5Board {
    wolges_board: WolgesBoard,  // Always 15x15
    allowed_vocabulary: HashSet<String>,  // Limit to 3-4 letter words
    hints_enabled: bool,
    suggest_moves: bool,
}

impl Stage5Board {
    pub fn new_training_mode() -> Self {
        Self {
            wolges_board: WolgesBoard::new_standard(),
            allowed_vocabulary: load_common_words(3, 4),  // Only 3-4 letter
            hints_enabled: true,
            suggest_moves: true,
        }
    }
}
```

**Benefits of 15×15 with restrictions:**
- Uses wolges without modification
- More realistic training
- Simpler codebase (no custom board)
- Progressive unlock of vocabulary, not board size

**Request:**
Please add to **Sprint 1 tasks**:

```markdown
- [ ] **Verify wolges board size support**
  - Check wolges API docs for Board::new(size)
  - Test instantiate 9×9 board in Rust
  - If not supported, pivot to 15×15 with vocabulary limits

- [ ] **Update Stage 5 design if needed**
  - Revise GAME_DESIGN.md if using 15×15
  - Update board layout documentation
  - Adjust difficulty progression
```

**Priority:** P1 - Must clarify before Sprint 6 (Stage 5 implementation)

---

### **Concern #3: wolges Move Generation API - Ownership & Lifetimes**

**Severity:** 🟡 **MEDIUM - INTEGRATION COMPLEXITY**

**Problem:**
- ADR pseudocode shows: `self.engine.generate_moves(board, rack)` ([ARCHITECTURE_DECISIONS.md:284](ARCHITECTURE_DECISIONS.md))
- **wolges has complex lifetime requirements** for move generation
- Board state needs to be borrowed mutably/immutably at different times
- This can conflict with Bevy's ECS ownership model

**Technical Details:**

Bevy ECS systems use strict borrowing rules:
```rust
fn ai_move_system(
    mut board: ResMut<GameBoard>,     // Mutable borrow of World
    rack: Res<AIRack>,                 // Immutable borrow
    mut engine: ResMut<WolgesEngine>,  // Another mutable borrow
) {
    // wolges needs &mut for some operations, & for others
    let moves = engine.generate_moves(&board, &rack);  // May conflict!
    //                                 ^      ^
    //                                 |      |
    // Bevy already has mutable borrow here, can't reborrow
}
```

**Rust Borrow Checker Issues:**
- Bevy systems can have multiple `ResMut<T>` OR multiple `Res<T>`
- But mixing mutable and immutable borrows can fail
- wolges API may require simultaneous access patterns that conflict

**Questions:**
1. Have you prototyped wolges integration with Bevy?
2. What are wolges's exact borrow requirements for move generation?
3. Do we need interior mutability patterns (Arc<Mutex<>>)?

**Potential Solutions:**

**Option A: Interior Mutability**
```rust
#[derive(Resource)]
pub struct WolgesEngine {
    inner: Arc<Mutex<wolges::Engine>>,  // Thread-safe interior mutability
}

impl WolgesEngine {
    pub fn generate_moves(&self, board: &Board, rack: &Rack) -> Vec<Move> {
        let engine = self.inner.lock().unwrap();
        engine.generate_moves(board, rack)
    }
}
```

**Option B: Split Resources**
```rust
// Separate immutable and mutable state
#[derive(Resource)]
pub struct WolgesLexicon {
    kwg: Arc<Kwg>,  // Immutable, shareable
}

#[derive(Resource)]
pub struct WolgesMoveGenerator {
    lexicon: Arc<Kwg>,
    cache: HashMap<BoardHash, Vec<Move>>,  // Mutable cache
}
```

**Option C: System Ordering**
```rust
// Split into multiple systems with explicit ordering
fn prepare_board_system(board: ResMut<Board>) { /* ... */ }
fn generate_moves_system(board: Res<Board>, engine: ResMut<Engine>) { /* ... */ }
fn evaluate_moves_system(moves: Res<Moves>) { /* ... */ }

app.add_systems(Update, (
    prepare_board_system,
    generate_moves_system.after(prepare_board_system),
    evaluate_moves_system.after(generate_moves_system),
));
```

**Impact:**
- May require refactoring of ADR pseudocode
- Could add complexity to ECS architecture
- Needs early validation to avoid Sprint 6-8 surprises

**Request:**
Please add to **Sprint 3 tasks**:

```markdown
- [ ] **Prototype wolges + Bevy integration**
  - Create minimal test: wolges move generation in Bevy system
  - Validate no lifetime/borrowing conflicts
  - Test ECS resource access patterns
  - Document ownership patterns that work

- [ ] **Document integration patterns**
  - Provide working code examples
  - Update ADR with correct API usage
  - Add troubleshooting guide for common issues
```

**Priority:** P1 - Must validate before Sprint 6 (Stage 5-6 AI implementation)

---

## 🔍 TECHNICAL CLARIFICATIONS NEEDED

### **Clarification #1: SRS Algorithm Accuracy**

**Severity:** 🟢 **LOW - NAMING/DOCUMENTATION**

**Issue:**
ADR shows ([ARCHITECTURE_DECISIONS.md:232](ARCHITECTURE_DECISIONS.md)):

```rust
if correct {
    progress.correct_count += 1;
    progress.consecutive_correct += 1;
    // SM-2 algorithm: increase strength
    progress.strength = (progress.strength + 0.1).min(1.0);
} else {
    progress.consecutive_correct = 0;
    // SM-2 algorithm: decrease strength
    progress.strength = (progress.strength - 0.2).max(0.0);
}
```

**Problem:**
- **This is NOT SM-2 algorithm** - it's a linear increment/decrement
- Real SM-2 uses **intervals, ease factors, and repetition counts**
- Comment is misleading

**What SM-2 Actually Is:**

SM-2 (SuperMemo 2) algorithm has these components:
- `easiness_factor` (EF): 1.3 - 2.5, affects interval growth
- `interval`: Days until next review
- `repetitions`: Successful reviews in a row
- Quality response: 0-5 scale

**True SM-2 Implementation:**
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct WordProgress {
    easiness_factor: f32,   // 1.3 - 2.5
    interval_days: u32,     // Days until next review
    repetitions: u32,       // Successful reviews in a row
    last_seen: DateTime<Utc>,
}

impl WordProgress {
    pub fn update_sm2(&mut self, quality: u8) {
        // quality: 0=complete blackout, 5=perfect recall

        // Update easiness factor
        self.easiness_factor = (self.easiness_factor
            + (0.1 - (5 - quality as f32) * (0.08 + (5 - quality as f32) * 0.02)))
            .max(1.3);

        // Update interval
        if quality < 3 {
            // Failed recall - reset
            self.repetitions = 0;
            self.interval_days = 1;
        } else {
            // Successful recall
            self.repetitions += 1;
            self.interval_days = match self.repetitions {
                1 => 1,
                2 => 6,
                _ => (self.interval_days as f32 * self.easiness_factor).round() as u32,
            };
        }

        self.last_seen = Utc::now();
    }

    pub fn is_due_for_review(&self) -> bool {
        let elapsed = Utc::now() - self.last_seen;
        elapsed.num_days() as u32 >= self.interval_days
    }
}
```

**As a Scrabble Grandmaster Teaching Kids:**

For **fast-paced gameplay** (Stage 1-3), true SM-2 is **overkill**:
- Kids play multiple rounds per session
- Need immediate reinforcement, not spaced days
- Simpler = better for game flow

Your simplified approach is **fine for MVP**, but:

**Request:**
Choose one approach and update ADR:

**Option A: Keep Simplified (RECOMMENDED for MVP)**
```rust
// RECOMMENDED: Simple strength tracking for fast gameplay
#[derive(Serialize, Deserialize, Clone)]
pub struct WordProgress {
    strength: f32,              // 0.0-1.0 simple score
    last_seen: DateTime<Utc>,
    attempts: u32,
    correct_count: u32,
}

impl WordProgress {
    pub fn record_attempt(&mut self, correct: bool) {
        self.attempts += 1;
        self.last_seen = Utc::now();

        if correct {
            self.correct_count += 1;
            // Increase strength, but with diminishing returns
            let learning_rate = 0.1 * (1.0 - self.strength);
            self.strength = (self.strength + learning_rate).min(1.0);
        } else {
            // Decrease strength on failure
            self.strength = (self.strength - 0.2).max(0.0);
        }
    }
}
```

**Option B: Implement True SM-2 (More Complex)**
- Use code above
- Add `is_due_for_review()` logic
- Schedule reviews across sessions
- Better for long-term retention

**My Recommendation:**
**Option A for MVP**, upgrade to SM-2 in Phase 2 if needed.

**Action Required:**
Update ADR comment from "SM-2 algorithm" to "Simplified strength tracking" or implement true SM-2.

**Priority:** P2 - Clarify before Sprint 4 implementation

---

### **Clarification #2: wolges WASM Compatibility**

**Severity:** 🔴 **CRITICAL - WASM DEPLOYMENT RISK**

**Question:**
- Has wolges been compiled and tested on WASM?
- Does it use any unsupported APIs?
- Are there known WASM issues?

**Why This Matters:**

Not all Rust crates compile to WASM cleanly. Potential blockers:

**Common WASM Issues:**
1. **Multi-threading** - WASM doesn't support threads (yet)
2. **Filesystem access** - Needs wasm-bindgen polyfills
3. **Unsafe code** - Platform-specific assumptions may fail
4. **Large dependencies** - May bloat bundle size
5. **Compilation errors** - Missing features for wasm32 target

**Specific Concerns with wolges:**

Looking at wolges crate metadata:
- Uses `std::collections` - ✅ OK
- May use `std::fs` for file loading - ⚠️ Needs polyfill
- Likely uses unsafe code for performance - ⚠️ Needs testing
- Unknown WASM compile status - 🚨 Risk

**If wolges doesn't compile to WASM:**
- **Entire architecture fails**
- Need Plan B: Custom trie implementation
- 4-6 week delay

**Request:**
Add to **Sprint 1 tasks** (Week 1 - HIGHEST PRIORITY):

```markdown
### Week 1: wolges WASM Validation (CRITICAL PATH)

#### Tasks
- [ ] **Compile wolges to WASM**
  ```bash
  cargo build --target wasm32-unknown-unknown --release
  ```
  - Document any compilation errors
  - Check for unsupported features
  - Verify no threading dependencies

- [ ] **Test wolges in browser**
  - Create minimal HTML test page
  - Load CSW24.kwg in browser
  - Test word validation (100 random words)
  - Measure performance (validation time)

- [ ] **Benchmark WASM vs Native**
  - Word validation: Target <10ms per word
  - KWG load time: Target <2s
  - Memory usage: Target <20MB
  - Move generation (if testable): Target <500ms

- [ ] **Document WASM limitations**
  - Any features that don't work
  - Performance characteristics
  - Workarounds or polyfills needed
  - Bundle size impact

#### Deliverables
- ✅ wolges confirmed WASM-compatible
- ✅ Performance benchmarks within targets
- ✅ OR Plan B activated if incompatible

#### Escalation
If wolges does NOT work on WASM:
- **Escalate immediately to Senior Architect**
- Activate Plan B: Custom trie implementation
- Reassess 13-week timeline
```

**Priority:** P0 - **MUST COMPLETE WEEK 1 DAY 1-2**

**Decision Gate:**
- If wolges WASM works → Proceed with architecture ✅
- If wolges WASM fails → Emergency architecture meeting 🚨

---

## 💡 MINOR SUGGESTIONS (Non-Blocking)

### **Suggestion #1: Add Leave Quality Heuristic Implementation**

**Context:**
ADR shows: `self.evaluate_leave(rack.after_move(mv))` but no implementation ([ARCHITECTURE_DECISIONS.md:310](ARCHITECTURE_DECISIONS.md))

**As a Scrabble Grandmaster**, here's a battle-tested heuristic:

```rust
impl ScrabbleAI {
    /// Evaluate quality of remaining tiles (leave) after a move
    /// Returns score: higher is better
    fn evaluate_leave(&self, leave: &[char]) -> i32 {
        // Tiles you want to keep
        const GOOD_TILES: &str = "AEINRST";  // "RETAINS" - most flexible

        // Tiles you want to play
        const BAD_TILES: &str = "QVJ";       // Hard to use without specific letters

        let mut score = 0;

        // Score each tile
        for tile in leave {
            if GOOD_TILES.contains(*tile) {
                score += 3;  // Good leave - flexible for next turn
            } else if BAD_TILES.contains(*tile) {
                score -= 5;  // Bad leave - hard to play next turn
            }
            // Other tiles: neutral (score += 0)
        }

        // Bonus for balanced vowel/consonant ratio
        let vowels = leave.iter().filter(|c| "AEIOU".contains(*c)).count();
        let consonants = leave.len() - vowels;

        // Ideal: 2-3 vowels, 4-5 consonants (for 7-tile rack)
        match vowels {
            2..=3 if consonants >= 4 => score += 5,  // Excellent balance
            1 | 4 => {},                             // Acceptable
            _ => score -= 3,                          // Poor balance
        }

        // Penalty for keeping duplicate tiles (reduces flexibility)
        let unique_tiles: std::collections::HashSet<_> = leave.iter().collect();
        if unique_tiles.len() < leave.len() {
            score -= 2 * (leave.len() - unique_tiles.len()) as i32;
        }

        score
    }
}
```

**Why This Works:**
- RETAINS tiles form the most words in English
- Q without U, V, J are notoriously difficult
- Balanced vowel ratio prevents "stuck" racks
- Duplicates reduce options

**Recommendation:**
Add this implementation to ADR or create separate `AI_HEURISTICS.md` document.

---

### **Suggestion #2: Add Tile Values Constant**

**Context:**
TileBag code is good ([ARCHITECTURE_DECISIONS.md:333-393](ARCHITECTURE_DECISIONS.md)), but we need tile point values for Stage 4 (Rack Training).

**Proposed Addition:**

```rust
/// Standard Scrabble tile point values
pub const TILE_VALUES: [(char, u8); 27] = [
    ('A', 1), ('B', 3), ('C', 3), ('D', 2), ('E', 1),
    ('F', 4), ('G', 2), ('H', 4), ('I', 1), ('J', 8),
    ('K', 5), ('L', 1), ('M', 3), ('N', 1), ('O', 1),
    ('P', 3), ('Q', 10), ('R', 1), ('S', 1), ('T', 1),
    ('U', 1), ('V', 4), ('W', 4), ('X', 8), ('Y', 4),
    ('Z', 10), ('*', 0),  // Blank tile
];

/// Get point value for a tile
pub fn tile_value(c: char) -> u8 {
    TILE_VALUES.iter()
        .find(|(tile, _)| *tile == c.to_ascii_uppercase())
        .map(|(_, val)| *val)
        .unwrap_or(0)
}

/// Calculate total value of a rack
pub fn rack_value(tiles: &[char]) -> u32 {
    tiles.iter().map(|&c| tile_value(c) as u32).sum()
}
```

**Usage in Stage 4:**
```rust
fn calculate_word_score(word: &str, tiles_used: &[char]) -> u32 {
    let base_score: u32 = tiles_used.iter()
        .map(|&c| tile_value(c) as u32)
        .sum();

    // Add bingo bonus if all 7 tiles used
    if tiles_used.len() == 7 {
        base_score + 50
    } else {
        base_score
    }
}
```

**Recommendation:**
Add to Issue #5 (Tile Bag System) section of ADR.

---

### **Suggestion #3: Specify CSW24 Source and Licensing**

**Legal Concern:**
CSW24 is **copyrighted by Collins** and licensed by WESPA (World English Scrabble Players Association).

**Questions:**
1. Where are we obtaining CSW24.txt?
   - Official purchase: https://scrabble.org.au/words/csw24 (~$50-100)
   - GitHub mirrors: May violate licensing
   - Alternative: Use TWL (free for non-commercial use)

2. What's our licensing status?
   - Commercial use: Requires license from Collins
   - Educational/non-commercial: May be acceptable under fair use
   - Research/personal: Generally OK

**For MVP:**
Since this is an **educational project** training future Scrabble champions, likely covered under:
- Fair use (educational purpose)
- Non-commercial use
- Research and development

**Recommendation:**

Add to ADR:

```markdown
## ⚖️ Licensing & Legal

### CSW24 Word List

**Source:** [Specify source - official purchase or alternative]

**Licensing:**
- CSW24 is copyrighted by HarperCollins Publishers
- Licensed by WESPA for tournament use
- Our use: Educational, non-commercial MVP
- Legal assessment: Fair use for educational training tool

**For Production Launch:**
- [ ] Obtain commercial license from Collins (if distributing widely)
- [ ] OR use alternative free lexicon (TWL, OSPD)
- [ ] OR restrict to personal/educational use only

**Alternative Lexicons (if needed):**
- **TWL (Tournament Word List):** Free for non-commercial use
- **OSPD (Official Scrabble Players Dictionary):** More restrictive
- **Enable6k:** Free, but less comprehensive

**Action:** Document lexicon source and confirm legal status before public launch.
```

**Priority:** P2 - Clarify before public MVP release

---

## 📊 Updated Risk Assessment

| Risk | Original | Post-ADR | After Sprint 1 Validation |
|------|---------|----------|---------------------------|
| WASM Bundle Size | 🔴 CRITICAL | 🟢 LOW | 🟢 LOW (if wolges WASM works) |
| AI Performance | 🔴 CRITICAL | 🟢 LOW | 🟢 LOW (wolges proven) |
| Tile Bag Missing | 🔴 CRITICAL | 🟢 LOW | 🟢 LOW (clear plan) |
| GADDAG Missing | 🔴 CRITICAL | 🟢 LOW | 🟢 LOW (wolges includes) |
| **NEW: wolges WASM Compatibility** | - | 🔴 **CRITICAL** | 🟢 LOW (after validation) |
| **NEW: KWG File Generation** | - | 🟡 **HIGH** | 🟢 LOW (after Sprint 1) |
| **NEW: wolges+Bevy Integration** | - | 🟡 **MEDIUM** | 🟢 LOW (after Sprint 3 prototype) |
| Validation Performance | 🟡 HIGH | 🟡 MEDIUM | 🟢 LOW (after benchmarks) |
| SRS Bloat | 🟡 MEDIUM | 🟢 LOW | 🟢 LOW |
| Storage Limits | 🟡 MEDIUM | 🟢 LOW | 🟢 LOW |
| Bevy Version | 🟡 MEDIUM | 🟢 LOW | 🟢 LOW |

**New Risks Introduced by wolges Dependency:**
- ⚠️ External dependency validation
- ⚠️ WASM compatibility unknown
- ⚠️ API integration with Bevy ECS

**Risk Mitigation Plan:**
All new risks resolved by Sprint 1 validation tasks.

---

## 🚀 Updated Timeline with Validation Gates

### Original: 12 weeks → ADR: 13 weeks → With Validation: 13-14 weeks

```
Week 1: Project Init + wolges Validation (CRITICAL GATE)
  ├─ Sprint 1 tasks
  └─ NEW: wolges WASM validation
         ├─ Compile to WASM ✅/❌
         ├─ Generate CSW24.kwg ✅/❌
         └─ DECISION GATE: Proceed or Pivot?

Week 2: Core Architecture
  ├─ Sprint 1 tasks (continued)
  └─ Baseline Bevy app

Week 3: Lexicon System + wolges Integration
  ├─ Integrate wolges
  ├─ WolgesAdapter implementation
  └─ Benchmark performance

Week 4: Scoring & Persistence
  ├─ Sparse SRS tracking
  ├─ TileBag + Rack
  └─ IndexedDB (WASM)

Week 5-6: Stage 1 Gameplay
  └─ (No changes)

Week 7-8: UI/UX & Mascot
  ├─ NEW: Prototype wolges + Bevy integration (Stage 5-6 prep)
  └─ (Other tasks unchanged)

Week 9-10: Audio & Web Build
  └─ (No changes)

Week 11: Testing
  └─ (No changes)

Week 12-13: Polish & Launch
  └─ (No changes)
```

**Key Change:**
- Week 1 now includes **critical validation gate**
- If wolges WASM fails → Emergency pivot meeting
- Week 7-8 includes **integration prototype** for de-risking Stage 5-6

**Total Timeline: 13 weeks (optimistic) to 14 weeks (conservative)**

---

## 📝 Required ADR Updates

Please update [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md) with:

### **Section to Add: Sprint 1 Research & Validation**

```markdown
## 🔬 Sprint 1 Research & Validation (CRITICAL)

**MUST complete before Sprint 3 integration**

These tasks validate the feasibility of the wolges-based architecture. If any task fails, we escalate and pivot to Plan B.

### Week 1: wolges WASM Compatibility (P0)

**Goal:** Confirm wolges compiles and runs on WASM

#### Tasks
- [ ] **Compile wolges to wasm32-unknown-unknown**
  ```bash
  cargo build --target wasm32-unknown-unknown --release --package wolges
  ```
  - Document compilation errors (if any)
  - Check for missing features or dependencies
  - Verify build completes successfully

- [ ] **Test wolges in browser**
  - Create minimal HTML + WASM test page
  - Load test: wolges.kwg file
  - Validate 100 random words
  - Measure performance vs native

- [ ] **Benchmark WASM performance**
  - Word validation time: Target <10ms per word
  - KWG load time: Target <2-4 seconds
  - Memory usage: Target <20MB
  - Bundle size: Target <15MB total

#### Deliverables
- ✅ wolges confirmed WASM-compatible
- ✅ Performance benchmarks within targets
- ✅ Proof-of-concept browser demo

#### Escalation Path
If wolges WASM compatibility fails:
1. Emergency meeting with Senior Architect
2. Activate Plan B: Custom trie + simplified AI
3. Reassess 13-week timeline (likely +4-6 weeks)

---

### Week 1: CSW24 KWG File Generation (P0)

**Goal:** Generate CSW24.kwg file for lexicon system

#### Tasks
- [ ] **Acquire CSW24 source**
  - Download CSW24.txt (plain text format)
  - Verify word count (~280,000 words)
  - Document source and licensing status

- [ ] **Install wolges-make tool**
  ```bash
  cargo install wolges-make
  ```

- [ ] **Convert CSW24.txt to KWG format**
  ```bash
  wolges-make build CSW24.txt --output CSW24.kwg --alphabet english
  ```

- [ ] **Validate KWG file**
  - Load in test program
  - Check file size (~5-8MB expected)
  - Verify sample words (QI, RETINA, ZYZZYVA, etc.)
  - Compare word count with source

- [ ] **Add to repository**
  - Place in `assets/lexicons/CSW24.kwg`
  - Document conversion process
  - Add to .gitignore if large (use LFS)

#### Deliverables
- ✅ CSW24.kwg file ready for integration
- ✅ Conversion process documented
- ✅ File integrity validated

---

### Week 1: Board Size Compatibility (P1)

**Goal:** Confirm wolges supports 9×9 boards OR pivot to 15×15

#### Tasks
- [ ] **Review wolges Board API**
  - Check documentation for custom board sizes
  - Look for `Board::new(size)` or similar
  - Review source code if needed

- [ ] **Test 9×9 board instantiation**
  ```rust
  let board = wolges::Board::new(9); // Does this work?
  ```

- [ ] **Decision: 9×9 or 15×15?**
  - If 9×9 supported: Proceed with original design ✅
  - If NOT supported: Update Stage 5 to use 15×15 with vocabulary limits

#### Deliverables
- ✅ Board size strategy confirmed
- ✅ GAME_DESIGN.md updated if needed
- ✅ Stage 5 design finalized

---

### Sprint 3: wolges + Bevy ECS Integration (P1)

**Goal:** Validate no ownership/lifetime conflicts

#### Tasks
- [ ] **Create integration prototype**
  - Minimal Bevy app
  - wolges move generation in Bevy system
  - Test ECS resource borrowing patterns

- [ ] **Test ownership patterns**
  ```rust
  fn ai_system(
      board: Res<GameBoard>,
      rack: Res<Rack>,
      mut engine: ResMut<WolgesEngine>,
  ) {
      let moves = engine.generate_moves(&board, &rack);
      // Does this compile? ✅ or ❌
  }
  ```

- [ ] **Document working patterns**
  - Update ADR with correct API usage
  - Provide code examples
  - Add troubleshooting guide

#### Deliverables
- ✅ Integration prototype working
- ✅ No lifetime conflicts confirmed
- ✅ Documentation updated

---

## ✅ Success Criteria (Sprint 1)

**Week 1 completion requires:**
1. ✅ wolges compiles to WASM successfully
2. ✅ CSW24.kwg file generated and validated
3. ✅ Board size strategy decided (9×9 or 15×15)
4. ✅ WASM performance benchmarks within targets
5. ✅ No showstopper issues found

**If all criteria met → Full approval to proceed** ✅

**If any criteria failed → Escalate immediately** 🚨
```

---

## 🎯 MY CONDITIONAL APPROVAL

### Approval Status

**Status:** ⚠️ **CONDITIONAL APPROVAL**

**Conditions for Full Sign-Off:**

1. ✅ **Sprint 1 Week 1:** wolges WASM compatibility validated
2. ✅ **Sprint 1 Week 1:** CSW24.kwg file generated successfully
3. ✅ **Sprint 1 Week 1:** Board size strategy confirmed (9×9 or 15×15)
4. ✅ **Sprint 3:** wolges + Bevy integration prototype working
5. ✅ **Sprint 4:** SRS algorithm clarified (simplified vs true SM-2)

### Confidence Level

**If all 5 conditions met:**
- ✅ **95% confidence** in 13-week delivery
- ✅ **Low risk** of major surprises
- ✅ **Ready to build** with solid foundation

**If any condition fails:**
- 🚨 **Immediate escalation**
- 🚨 **Emergency architecture review**
- 🚨 **Plan B activation** (custom trie, extended timeline)

---

## ✍️ FORMAL SIGN-OFF

**Tech Lead:** Andy Chen

**Status:** ⚠️ **CONDITIONAL APPROVAL**

**Signature:** [Digital signature pending completion of Sprint 1 validation tasks]

**Date:** 2025-10-08

**Conditions:** See above (5 items)

**Expected Full Approval Date:** End of Sprint 1 (Week 2)

---

## 📞 Next Steps

### Immediate Actions (This Week)

1. **Senior Architect:** Review this feedback
2. **Senior Architect:** Update ADR with Sprint 1 validation section
3. **Both:** Schedule 30-minute alignment call
4. **Both:** Agree on Sprint 1 validation plan

### Week 1 Actions (Sprint 1)

1. **Dev Team:** Execute Sprint 1 validation tasks
2. **Tech Lead:** Monitor validation progress daily
3. **Senior Architect:** Prepare Plan B contingency
4. **Both:** Week 1 Friday - Go/No-Go decision meeting

### Decision Gate (End of Week 1)

**Friday Week 1 - Architecture Decision Meeting**

**Agenda:**
1. Review wolges WASM validation results
2. Review CSW24.kwg generation
3. Review board size decision
4. **Decision:** Proceed with architecture OR Activate Plan B

**Attendees:**
- Senior Architect (required)
- Tech Lead (required)
- Product Owner (required)
- Lead Developer (optional)

**Duration:** 1 hour

**Outcome:** Formal approval OR pivot decision

---

## 💬 Personal Note to Senior Architect

Hey boss,

I absolutely love the wolges decision—technically brilliant and strategically sound. As fellow Scrabble grandmasters, we both know wolges is the industry standard and the right foundation.

My concerns are purely about **de-risking the integration**. External dependencies can surprise us, and I want to validate early (Sprint 1) so we don't get blocked in Sprint 3.

The validation tasks I'm proposing add **2-3 days to Sprint 1**, but they could save us **4-6 weeks** if we discover issues early. Worth it.

Once we confirm wolges plays nice with Bevy + WASM, I'm 100% on board and ready to ship. 🚀

Let's schedule that alignment call and knock out Sprint 1 validation together.

**Ready to build champions!** 🏆

— Andy (Tech Lead / Scrabble Grandmaster)

---

**Document Status:** ✅ Complete
**Next Review:** End of Sprint 1 (Week 2)
**Action Required:** Senior Architect response + ADR update
