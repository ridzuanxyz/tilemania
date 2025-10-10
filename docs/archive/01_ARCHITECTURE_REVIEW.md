# 🔍 Architecture Review - Critical Concerns & Questions
**Scrabble Learning Game (TileMania)**

---

## 📋 Document Overview
This document provides a critical technical review of the TileMania architecture from the perspective of a senior technical lead and Scrabble grandmaster.

**Review Date:** 2025-10-08
**Reviewer:** Senior Technical Lead / Scrabble Grandmaster
**Documents Reviewed:**
- [ARCHITECTURE.md](ARCHITECTURE.md)
- [GAME_DESIGN.md](GAME_DESIGN.md)
- [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md)

---

## 🚨 Critical Technical Issues

### 1. **Trie Memory Footprint - WASM Constraint Conflict**

**Problem:**
- Architecture doc claims <50MB for full trie ([ARCHITECTURE.md:98](ARCHITECTURE.md))
- WASM bundle target is <10MB compressed ([ARCHITECTURE.md:28](ARCHITECTURE.md))
- CSW24 has ~280k words

**Analysis:**
- How will a 50MB trie fit in a 10MB WASM bundle?
- Even with gzip (3-4x compression), 50MB → ~15MB compressed
- On WASM, you need to load **both** the WASM binary AND decompress the lexicon in-browser

**Impact:** 🔴 **CRITICAL** - Blocks WASM deployment

**Recommendation:**
1. **Option A:** Relax WASM bundle target to ~20MB compressed
2. **Option B:** Implement **progressive loading**: Ship with 2-letter + common words (~5k), lazy-load full lexicon on-demand
3. **Option C:** Use more memory-efficient structure (DAWG/GADDAG) - typical compression to 5-10MB

**Priority:** P0 - Must resolve in Sprint 3

---

### 2. **Word Validation Performance - Unrealistic on WASM**

**Problem:**
- Target: <5ms word validation ([ARCHITECTURE.md:97](ARCHITECTURE.md))
- WASM runs 2-5x slower than native Rust
- With 280k words in trie, worst-case lookup is O(n) where n=word length (15 chars max)

**Analysis:**
- Have you benchmarked trie lookup in WASM?
- 5ms on native might be 10-25ms on WASM (especially mobile browsers)
- This could cause input lag in Stage 1 (falling letters game)
- User experience degrades if validation feels sluggish

**Impact:** 🟡 **HIGH** - Affects gameplay feel

**Recommendation:**
- Add WASM benchmarking to Sprint 3 deliverables
- If too slow, consider:
  - Bloom filter pre-check (false positive OK, eliminates 99% of invalid words instantly)
  - Prefix caching for common patterns
  - Web Workers for async validation (non-blocking)

**Priority:** P1 - Test in Sprint 3, contingency plan ready

---

### 3. **SRS Data Structure - Scaling Problem**

**Problem:**
- `word_strength: HashMap<String, f32>` for **every word** ([ARCHITECTURE.md:157](ARCHITECTURE.md))
- CSW24 = 280k words × (String + f32 + DateTime) ≈ **20-30MB RAM just for SRS data**
- Save file bloat: RON serialization adds 50-100% overhead
- Kids might learn 1000-5000 words max over months of play

**Analysis:**
- Why store SRS data for **all 280k words** the player has never seen?
- This bloats save files and memory usage unnecessarily
- 95% of CSW24 words will never be encountered by typical player
- Wastes LocalStorage quota on WASM

**Impact:** 🟡 **MEDIUM** - Performance and storage inefficiency

**Recommendation:**
```rust
// BETTER APPROACH
pub struct PlayerProfile {
    // Only track words player has encountered
    word_strength: HashMap<String, WordProgress>,  // sparse, grows organically
}

#[derive(Serialize, Deserialize)]
pub struct WordProgress {
    strength: f32,
    last_seen: DateTime<Utc>,
    attempts: u32,
    correct_count: u32,
}
```

- Lazy-initialize on first exposure
- Expect typical save file: 1000-5000 entries vs 280k
- Reduces memory by 98%+

**Priority:** P1 - Implement in Sprint 4

---

### 4. **AI Move Generation - Computationally Expensive**

**Problem:**
- Advanced AI does "minimax lookahead depth 2" ([ARCHITECTURE.md:373](ARCHITECTURE.md))
- On a 15×15 board with 7-tile rack, possible moves = **thousands per position**
- Minimax depth-2 = evaluate (thousands of moves) × (hundreds of responses) = **millions of positions**
- Even on native desktop, this is 5-30 seconds per move for strong players

**Analysis:**
- How do you expect this to run in real-time (3-minute turn timer in Stage 6)?
- WASM will be even slower (2-5x)
- No mention of move ordering, alpha-beta pruning, or iterative deepening
- Tournament-level AI (Quackle, Maven) uses GADDAG + sophisticated heuristics, not brute-force minimax

**Impact:** 🔴 **CRITICAL** - Stage 6 AI may be unimplementable as described

**Recommendation:**
1. **For MVP (Stages 5-6):** Use greedy AI with heuristics:
   - Evaluate immediate score
   - Leave quality (keep AEINRST)
   - Board control (block premium squares)
   - No lookahead

2. **For Post-MVP:** If minimax desired:
   - Iterative deepening with time limit
   - Alpha-beta pruning
   - Transposition table
   - Move ordering (try high-scoring moves first)

3. **Best approach:** Use existing Scrabble engine library (see Issue #6)

**Priority:** P0 - Clarify AI scope in Sprint 1-2

---

### 5. **Missing: Tile Distribution & Bag Management**

**Critical Omission:**
- Stages 4-6 require **standard Scrabble tile distribution**:
  - A×9, B×2, C×2, D×4, E×12, F×2, G×3, H×2, I×9, J×1, K×1, L×4, M×2, N×6, O×8, P×2, Q×1, R×6, S×4, T×6, U×4, V×2, W×2, X×1, Y×2, Z×1, Blank×2
  - Total: 100 tiles per game
- Architecture doc has **no mention** of tile bag system
- No bag tracking, no draw mechanics, no endgame tile counting

**Analysis:**
- How is Stage 4 (rack training) generating "standard Scrabble distribution" racks?
- How do Stages 5-6 track remaining tiles in bag?
- How does AI infer unseen tiles (critical for advanced play)?
- Is there a `TileBag` resource?

**Impact:** 🔴 **CRITICAL** - Blocks Stages 4-6 implementation

**Recommendation:**
Add to architecture:

```rust
pub struct TileBag {
    remaining: HashMap<char, u8>,  // tiles left in bag
    drawn: Vec<char>,              // tiles drawn (for tracking)
}

impl TileBag {
    pub fn new_standard() -> Self {
        // CSW24 standard distribution
        let mut remaining = HashMap::new();
        remaining.insert('A', 9);
        remaining.insert('B', 2);
        // ... etc
        remaining.insert('*', 2);  // blanks
        Self { remaining, drawn: vec![] }
    }

    pub fn draw(&mut self, count: usize) -> Vec<char> { /* ... */ }
    pub fn peek_remaining(&self) -> &HashMap<char, u8> { /* ... */ }
}
```

**Priority:** P0 - Add to Sprint 4 architecture

---

### 6. **DAWG/GADDAG Missing for Competitive Play**

**As a Scrabble Grandmaster:**
- Stages 5-6 AI needs to **generate all valid plays** from rack + board state
- This requires traversing the board and finding "cross-sets" (perpendicular words formed)
- Standard trie is **highly inefficient** for this task

**Technical Problem:**
- With trie: Must check every board position × every rack permutation × validate cross-words
- Complexity: O(board_size × rack_permutations × validation_time) = **exponential**
- For 15×15 board, 7-tile rack: ~225 positions × 5040 permutations × trie lookups = **millions of operations**

**Industry Standard:**
- **GADDAG** (Directed Acyclic Word Graph) - used by all competitive Scrabble engines
- Enables O(board_size × avg_word_length) move generation
- Used by Quackle, Maven, Elise, etc.
- **100-1000x faster** than naive trie approach

**Analysis:**
- How will Advanced AI find moves fast enough without GADDAG?
- Simple trie makes Stage 5-6 AI implementation **extremely difficult** or impossibly slow
- Even Intermediate AI (find all moves, pick best) requires fast generation

**Impact:** 🔴 **CRITICAL** - Stage 5-6 AI may be unusably slow

**Recommendation:**
Choose one approach:

1. **Option A: Use existing library** (RECOMMENDED)
   - `wolges` Rust crate - full Scrabble engine with GADDAG
   - Proven, tested, maintained
   - Saves months of development

2. **Option B: Implement GADDAG** (HIGH EFFORT)
   - Add to Sprint 3-4
   - Requires specialized knowledge
   - 2-4 weeks of dedicated dev time

3. **Option C: Simplify Stages 5-6** (FALLBACK)
   - Player-only mode (no AI move generation)
   - AI only validates player moves
   - Defer real AI to Phase 2

**Priority:** P0 - Decide in Sprint 1

---

### 7. **LocalStorage Limits on WASM**

**Problem:**
- WASM persistence uses LocalStorage API ([ARCHITECTURE.md:179](ARCHITECTURE.md))
- Browser LocalStorage limit: **5-10MB per domain** (browser-dependent)
- PlayerProfile with full SRS data + game history could exceed this
- No error handling mentioned for quota exceeded

**Analysis:**
- What happens when save file exceeds LocalStorage quota?
- Will game crash? Silently fail? Corrupt data?
- Heavy users (complete all 6 stages) will hit limits

**Impact:** 🟡 **MEDIUM** - Data loss risk for power users

**Recommendation:**
1. **Primary fix:** Use **IndexedDB** instead of LocalStorage
   - Limits: 50MB-1GB (browser-dependent, usually ~50% of available disk)
   - Better for binary data
   - Async API (non-blocking)

2. **Fallback:** Compress save data with gzip/brotli (3-5x reduction)

3. **Safety:** Implement quota checking:
```rust
// Pseudo-code
fn save_profile(profile: &PlayerProfile) -> Result<(), SaveError> {
    let data = ron::to_string(profile)?;
    if data.len() > QUOTA_LIMIT {
        return Err(SaveError::QuotaExceeded);
    }
    // ... save
}
```

**Priority:** P1 - Implement in Sprint 4

---

### 8. **Bevy Version Compatibility**

**Problem:**
- Roadmap specifies Bevy 0.12+ ([IMPLEMENTATION_ROADMAP.md:56](IMPLEMENTATION_ROADMAP.md))
- Current stable Bevy is **0.15.0** (as of January 2025)
- Bevy 0.12 released August 2023 - **16+ months outdated**
- Bevy has breaking changes between major versions (0.12→0.13→0.14→0.15)

**Analysis:**
- Why target 0.12?
- Plugin compatibility:
  - `bevy_tweening` 0.9 (roadmap) → current 0.11+
  - `bevy_hanabi` 0.9 (roadmap) → current 0.14+
  - `bevy_kira_audio` 0.18 (roadmap) → current 0.20+
- Using outdated version means:
  - Missing performance improvements
  - Missing bug fixes
  - Harder to find support/docs
  - Plugin version mismatches

**Impact:** 🟡 **MEDIUM** - Technical debt from day 1

**Recommendation:**
- Update dependencies to Bevy 0.15+ and latest plugin versions:
```toml
[dependencies]
bevy = "0.15"
bevy_tweening = "0.11"
bevy_hanabi = "0.14"
bevy_kira_audio = "0.20"
```
- Test plugin compatibility in Sprint 1
- Budget 1-2 days for migration issues

**Priority:** P1 - Fix in Sprint 1

---

## 🧠 Scrabble-Specific Design Issues

### 9. **Q-Word List Incomplete**

**Problem:**
- Stage 3.5 mentions "20 Q-without-U words" ([ARCHITECTURE.md:316](ARCHITECTURE.md))
- **CSW24 actually has 33 Q-without-U words**

**Complete CSW24 Q-without-U List:**
- 2-letter: QI
- 3-letter: QAT
- 4-letter: QADI, QAID, QATS, QOPH
- 5-letter: QAIDS, QADIS, QANAT, QOPHS, QORMA, NIQAB, TRANQ
- 6-letter: QABALA, QANATS, QAWWAL, QIGONG, QINDAR, QINTAR, QORMAS, QWERTY, NIQABS
- 7-letter: QAWWALI, QINDARS, QINTARS, QWERTYS, SHEQELS
- 8+ letter: QABALAH, QABALAHS, QAWWALIS, SHEQALIM, FIQH, etc.

**Impact:** 🟢 **LOW** - Pedagogical completeness

**Recommendation:**
- Update Stage 3.5 to include all 33 words
- Group by length for progressive difficulty
- Critical for tournament readiness

**Priority:** P2 - Update in Sprint 6 (Stage 3.5 design)

---

### 10. **Missing: Common 3-Letter Word Selection Criteria**

**Problem:**
- Stage 2 targets "~5000 words" for 3-4 letter construction ([ARCHITECTURE.md:303](ARCHITECTURE.md))
- CSW24 3-4 letter words ≈ **8000-10000 words**
- No guidance on which subset to use for "common" words
- CSW24 includes obscure words (e.g., "CWMS", "PHPHT", "HRYVNA")

**Analysis:**
- How to select 5000 "common" words from CSW24?
- CSW24 has no frequency data (it's a tournament word list, not a frequency corpus)
- Kids should learn common words first (CAT, DOG, RUN) before obscure ones (QAT, PHO, CWMS)

**Impact:** 🟡 **MEDIUM** - Learning curve quality

**Recommendation:**
Choose one approach:

1. **Option A: Use frequency corpus** (RECOMMENDED)
   - Cross-reference CSW24 with COCA (Corpus of Contemporary American English)
   - Or Moby word frequency list
   - Select top 5000 by frequency

2. **Option B: Start length-based**
   - Stage 2A: All 3-letter words (~1300 words)
   - Stage 2B: Common 4-letter words (top 3000-4000)
   - More digestible progression

3. **Option C: Use tournament study lists**
   - NASPA "3-letter words to know"
   - Pre-curated for Scrabble players

**Priority:** P2 - Define in Sprint 5 (Stage 2 design)

---

### 11. **Missing: Board Position Encoding (Stages 5-6)**

**Problem:**
- Stages 5-6 require tracking board state (placed tiles)
- No data structure specified for board representation
- Need efficient query for:
  - "What's at position (x, y)?"
  - "What are the cross-words at position (x, y)?"
  - "What are valid placement positions?"

**Impact:** 🟡 **MEDIUM** - Implementation ambiguity

**Recommendation:**
Add to architecture:

```rust
pub struct Board {
    size: usize,  // 9 for Stage 5, 15 for Stage 6
    tiles: Vec<Vec<Option<Tile>>>,  // 2D grid
    premium_squares: HashMap<(usize, usize), PremiumType>,
}

#[derive(Copy, Clone)]
pub enum PremiumType {
    TripleWord,
    DoubleWord,
    TripleLetter,
    DoubleLetter,
}

pub struct Tile {
    letter: char,
    is_blank: bool,  // for blank tiles
}
```

**Priority:** P1 - Add in Sprint 4 architecture

---

## 💡 Questions for Product Owner / Tech Lead

### Strategic Questions

1. **WASM bundle size vs lexicon size:**
   - Accept 20MB target or implement progressive loading?
   - What's acceptable for target users (schools, home)?

2. **GADDAG requirement:**
   - Are you planning to implement GADDAG or use existing library (wolges)?
   - Or simplify AI stages to avoid need?

3. **Performance targets:**
   - Are the 5ms validation and 60fps targets based on actual WASM benchmarks?
   - Or aspirational goals?

4. **Team experience:**
   - Does your team have prior Rust/Bevy/WASM experience?
   - 12 weeks is tight for learning + building

5. **Scrabble engine:**
   - Any plans to use existing libraries (wolges, etc.)?
   - Or build complete engine from scratch?

6. **Mobile web priority:**
   - Is mobile web (WASM on phone) a hard requirement for MVP?
   - Very challenging for performance - consider desktop-first?

### Technical Questions

7. **Tile bag implementation:**
   - Who owns this component? Sprint 4 team?

8. **AI complexity:**
   - Is "Advanced AI" truly needed for MVP (Stage 6)?
   - Or can we ship with Intermediate AI and defer Advanced to Phase 2?

9. **Testing strategy:**
   - How to unit test SRS algorithm correctness?
   - Automated playtesting bots?

10. **Asset pipeline:**
    - Who creates mascot sprites (Week 8)?
    - In-house or commissioned?

---

## ✅ What's Well-Designed

### Excellent Decisions

1. **Progressive stage unlock** - Perfect pedagogical flow from 2-letter → tournament
2. **SRS integration** - Critical for long-term word retention
3. **Offline-first architecture** - Smart for classroom/school environments
4. **Plugin architecture** - Clean Bevy ECS pattern, easy to extend
5. **CSW24 choice** - International standard, future-proof
6. **12-week MVP scope** - Realistic timeline (if issues above addressed)
7. **Stage 3.5 (Anagrams + Q-words)** - Grandmaster-level insight, critical gap filled
8. **Lexi the Owl mascot** - Engaging, age-appropriate UX
9. **RON for persistence** - Human-readable, debug-friendly
10. **Cross-platform targeting** - Rust+Bevy is correct tech choice

### Strong Game Design

- Hook training (Stage 3) - **Critical** for competitive play
- Rack management (Stage 4) - Often overlooked, excellent inclusion
- 9×9 board (Stage 5) - Smart simplification before full 15×15
- Post-game analysis (Stage 6) - Accelerates learning
- Blank tile strategy - Tournament essential
- Challenge system - Mirrors real tournament rules

---

## 📊 Risk Assessment Summary

| Issue | Severity | Impact | Timeline | Mitigation Cost |
|-------|----------|--------|----------|-----------------|
| #1 Trie/WASM size | 🔴 CRITICAL | Blocks WASM | Sprint 3 | 1-2 weeks |
| #2 Validation perf | 🟡 HIGH | UX degradation | Sprint 3 | 3-5 days |
| #3 SRS bloat | 🟡 MEDIUM | Memory waste | Sprint 4 | 2-3 days |
| #4 AI complexity | 🔴 CRITICAL | Blocks Stage 6 | Sprint 1 | 2-4 weeks |
| #5 Tile bag missing | 🔴 CRITICAL | Blocks Stage 4-6 | Sprint 4 | 1 week |
| #6 No GADDAG | 🔴 CRITICAL | Blocks Stage 5-6 | Sprint 1 | 3-4 weeks |
| #7 LocalStorage limits | 🟡 MEDIUM | Data loss risk | Sprint 4 | 3-5 days |
| #8 Bevy version | 🟡 MEDIUM | Tech debt | Sprint 1 | 1-2 days |
| #9 Q-word list | 🟢 LOW | Completeness | Sprint 6 | 1 day |
| #10 Word selection | 🟡 MEDIUM | Learning curve | Sprint 5 | 2-3 days |
| #11 Board encoding | 🟡 MEDIUM | Implementation | Sprint 4 | 2-3 days |

**Total Mitigation Cost:** 6-10 weeks of engineering time

**Critical Path Risks:** Issues #1, #4, #5, #6 could delay MVP by 4-8 weeks if not addressed proactively.

---

## 🎯 Recommended Actions

### Sprint 1 Actions (Week 1-2)

1. ✅ **Update Bevy to 0.15** - Low effort, high value
2. ✅ **Decide on GADDAG strategy** - Use wolges or simplify AI?
3. ✅ **Prototype WASM lexicon loading** - Validate <20MB bundle feasible
4. ✅ **Define AI scope** - Clarify Advanced AI requirements

### Sprint 3 Actions (Week 3)

5. ✅ **Benchmark trie performance on WASM** - Validate 5ms target
6. ✅ **Implement progressive lexicon loading** - If bundle size blocked
7. ✅ **Add GADDAG or integrate wolges** - If building Stages 5-6 AI

### Sprint 4 Actions (Week 4)

8. ✅ **Implement sparse SRS tracking** - Only store encountered words
9. ✅ **Add TileBag architecture** - Required for Stage 4+
10. ✅ **Switch to IndexedDB** - Replace LocalStorage for WASM

### Before Launch (Week 11)

11. ✅ **Load test WASM bundle** - Verify mobile browser performance
12. ✅ **Validate Q-word list** - Ensure all 33 CSW24 Q-words present

---

## 📈 Overall Assessment

**Architecture Score:** 7.5/10

**Strengths:**
- ✅ Solid pedagogical progression
- ✅ Appropriate tech stack (Rust + Bevy)
- ✅ Clean ECS architecture
- ✅ Excellent game design fundamentals

**Critical Gaps:**
- ❌ WASM performance not validated
- ❌ AI implementation underspecified
- ❌ Missing core Scrabble engine components (GADDAG, TileBag)
- ❌ Memory/storage optimization needed

**Recommendation:**
This is an **ambitious but achievable** project with a focused team. However, 6 critical technical risks (#1, #4, #5, #6, #2, #7) could derail the 12-week timeline if not addressed in Sprint 1-4.

**Action Required:**
- Address P0 issues in Sprint 1-2
- Create contingency plan for WASM performance
- Consider extending MVP timeline to 14-16 weeks for safety margin
- Prototype high-risk components (lexicon loading, AI move generation) early

---

## 🚦 Go/No-Go Decision

**Current Status:** 🟡 **CONDITIONAL GO**

**Conditions for GREEN:**
1. ✅ GADDAG strategy decided (use wolges or simplify)
2. ✅ WASM lexicon loading prototype successful (<20MB)
3. ✅ Team has Rust/Bevy experience or 2-week ramp-up time added
4. ✅ TileBag architecture added
5. ✅ AI scope clarified (defer Advanced AI if needed)

**If conditions met:** Project has 80% chance of successful MVP in 12-14 weeks

**If conditions unmet:** High risk of 4-8 week delay or scope reduction

---

**Document Status:** ✅ Complete
**Next Review Date:** Sprint 1 completion (Week 2)
**Action Items Owner:** Tech Lead + Product Owner

---

## 📞 Follow-Up Required

Please schedule architecture review meeting to address:
1. GADDAG strategy decision
2. WASM performance validation plan
3. MVP scope adjustment (if needed)
4. Sprint 1-2 task additions

**Recommended Attendees:**
- Tech Lead
- Senior Rust Engineer
- Game Designer
- Product Owner

Let's build this right! 🏆
