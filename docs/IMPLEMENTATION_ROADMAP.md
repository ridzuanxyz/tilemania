# 🚀 Implementation Roadmap
**Scrabble Learning Game (TileMania)**

---

## 📋 Document Overview
This roadmap provides a detailed sprint-by-sprint implementation plan for the development team, from project setup through MVP launch.

**Last Updated:** 2025-10-08
**Roadmap Version:** 1.0
**Target MVP Date:** Week 12

---

## 🎯 MVP Scope

### MVP Features (Phase 1)
- ✅ **Stage 1:** 2-letter word falling game (fully playable)
- ✅ **Core Systems:** Lexicon, scoring, persistence, SRS
- ✅ **UI/UX:** Main menu, stage select, settings, mascot
- ✅ **Audio/Visual:** Sound effects, animations, particles
- ✅ **Platform:** Desktop (Windows/Mac/Linux) + Web (WASM)

### Post-MVP Features (Phase 2+)
- ⏳ Stages 2-6 (incremental releases)
- ⏳ Advanced analytics dashboard
- ⏳ Multiplayer mode
- ⏳ Mobile native builds
- ⏳ Teacher portal

---

## 📅 12-Week Development Timeline

```
Sprint 1-2:  Foundation & Setup
Sprint 3-4:  Lexicon & Core Systems
Sprint 5-6:  Stage 1 Gameplay
Sprint 7-8:  UI/UX & Mascot
Sprint 9-10: Audio, Polish & Web Build
Sprint 11:   Testing & Bug Fixes
Sprint 12:   Launch Prep & Documentation
```

---

## 🏗️ Sprint 1-2: Foundation & Setup
**Duration:** 2 weeks
**Goal:** Project skeleton, tooling, and base architecture

### Week 1: Project Initialization

#### Tasks
- [ ] **Initialize Rust/Bevy project**
  - Create Cargo workspace
  - Add Bevy 0.12+ as dependency
  - Configure features (native + web)
  - Set up folder structure (`src/`, `assets/`, `docs/`)

- [ ] **Version control setup**
  - Initialize git repository
  - Create `.gitignore` (target/, assets cache)
  - Set up branching strategy (main, develop, feature/*)
  - Add README with build instructions

- [ ] **CI/CD pipeline**
  - GitHub Actions for Rust builds
  - Automated testing on PR
  - WASM build validation
  - Artifact storage

- [ ] **Development environment**
  - Install Rust toolchain
  - Install wasm32 target
  - Install wasm-bindgen
  - IDE setup (VS Code + rust-analyzer)

#### Deliverables
- ✅ Empty Bevy app that runs at 60fps
- ✅ CI/CD passing green
- ✅ Build instructions documented

---

### Week 2: Core Architecture Setup

#### Tasks
- [ ] **Bevy plugin architecture**
  - Create plugin structure for modularity
  - Implement state management system
  - Add diagnostics plugin for FPS monitoring

- [ ] **Asset pipeline**
  - Create asset loader system
  - Add placeholder sprites/fonts
  - Test asset hot-reloading (dev mode)

- [ ] **State scaffolding**
  - Implement `GameState` enum (MainMenu, Stage1, etc.)
  - Add state transition systems
  - Create placeholder screens for each state

- [ ] **Input system foundation**
  - Keyboard input handler
  - Mouse/touch input abstraction
  - Input event bus

#### Deliverables
- ✅ State machine navigable via keyboard
- ✅ Placeholder screens for all states
- ✅ Asset loading working

---

## 🔤 Sprint 3-4: Lexicon & Core Systems
**Duration:** 2 weeks
**Goal:** Word validation, scoring, persistence

### Week 3: Lexicon System

#### Tasks
- [ ] **Acquire CSW24 lexicon**
  - Download official CSW24 word list
  - Format as plain text (one word per line)
  - Validate word count (~280k words)
  - Add to `assets/lexicons/CSW24.txt`

- [ ] **Trie data structure**
  - Implement `TrieNode` struct
  - Build trie from word list
  - Add `is_valid_word()` method
  - Optimize memory layout

- [ ] **Lexicon resource**
  - Create `LexiconResource` Bevy resource
  - Implement async loader
  - Add loading screen with progress bar
  - Cache trie in memory

- [ ] **Performance testing**
  - Benchmark trie build time (<2s target)
  - Benchmark word lookup (<5ms target)
  - Profile memory usage (<50MB target)

#### Deliverables
- ✅ CSW24 loads successfully
- ✅ Word validation working (unit tested)
- ✅ Performance benchmarks passing

---

### Week 4: Scoring & Persistence

#### Tasks
- [ ] **Scoring engine**
  - Create `ScoringEngine` struct
  - Implement base scoring logic
  - Add speed multipliers
  - Add streak bonuses
  - Unit tests for edge cases

- [ ] **SRS system**
  - Implement SM-2 algorithm
  - Create `WordStrength` tracker
  - Add spaced repetition scheduler
  - Test with sample word set

- [ ] **Persistence layer**
  - Create `PlayerProfile` struct (serializable)
  - Implement save/load using RON format
  - Cross-platform file paths (use `dirs` crate)
  - Auto-save on state changes

- [ ] **Custom lexicon loader**
  - File picker dialog
  - Lexicon validation (format, size)
  - Replace active lexicon
  - Persist custom lexicon preference

#### Deliverables
- ✅ Scoring system functional
- ✅ Progress saves and loads correctly
- ✅ Custom lexicon loading works

---

## 🎮 Sprint 5-6: Stage 1 Gameplay
**Duration:** 2 weeks
**Goal:** Fully playable Stage 1 with 2-letter words

### Week 5: Core Gameplay Mechanics

#### Tasks
- [ ] **2-letter word pool**
  - Extract all 127 CSW24 2-letter words
  - Store in separate asset file
  - Implement word selection algorithm (SRS-weighted)

- [ ] **Falling letter system**
  - Spawn letter entities at top of screen
  - Apply gravity physics (Bevy Transform)
  - Animate falling with `bevy_tweening`
  - Despawn on reaching bottom

- [ ] **Input handling**
  - Capture keyboard input
  - Build current word buffer
  - Submit on Enter key
  - Clear on Backspace

- [ ] **Word validation flow**
  - Check buffer against lexicon
  - Provide instant feedback (✓/✗)
  - Update score on correct answer
  - Update SRS on incorrect answer

#### Deliverables
- ✅ Letters fall and respond to gravity
- ✅ Player can type words
- ✅ Validation works correctly

---

### Week 6: Difficulty & Progression

#### Tasks
- [ ] **Difficulty scaling**
  - Implement progressive fall speed
  - Add distractor (invalid) letters
  - Increase words per round
  - Save difficulty level per player

- [ ] **Round system**
  - Define round structure (10-25 words)
  - Track round progress
  - Show round completion screen
  - Calculate round score and XP

- [ ] **Mastery tracking**
  - Track strength per word
  - Display mastery percentage
  - Unlock Stage 2 at 70% mastery
  - Show progress bar on UI

- [ ] **Game over / completion**
  - Detect round end
  - Show results screen
  - Return to stage select
  - Persist progress

#### Deliverables
- ✅ Stage 1 playable end-to-end
- ✅ Difficulty increases appropriately
- ✅ Mastery tracking functional

---

## 🎨 Sprint 7-8: UI/UX & Mascot
**Duration:** 2 weeks
**Goal:** Polished UI, mascot animations, user experience

### Week 7: UI Implementation

#### Tasks
- [ ] **Main menu**
  - Title screen with logo
  - Play button → stage select
  - Settings button → settings screen
  - Quit button (desktop only)

- [ ] **Stage select screen**
  - Grid of stage cards (1-6)
  - Lock icons for unavailable stages
  - Progress bars on each card
  - High score display

- [ ] **In-game UI**
  - Score display (top-left)
  - Streak counter (top-center)
  - Timer (top-right)
  - Current word buffer (center-bottom)
  - Mastery progress bar (bottom)

- [ ] **Settings screen**
  - Volume sliders (master, SFX, music)
  - Lexicon selector dropdown
  - Difficulty modifier slider
  - Accessibility toggles

- [ ] **Results screen**
  - Score summary
  - XP earned
  - Mastery change
  - "Play Again" / "Back to Menu" buttons

#### Deliverables
- ✅ All UI screens implemented
- ✅ Navigation flow working
- ✅ UI responsive to different resolutions

---

### Week 8: Mascot & Visual Polish

#### Tasks
- [ ] **Mascot design**
  - Create/commission Lexi the Owl sprite
  - Design 5 animation states:
    - Idle
    - Correct answer (thumbs up)
    - Wrong answer (sympathetic)
    - Streak milestone (celebration)
    - New unlock (excited)

- [ ] **Mascot integration**
  - Add mascot sprite to corner
  - Implement animation state machine
  - Trigger animations on game events
  - Add speech bubbles (optional)

- [ ] **Visual feedback**
  - Tile bounce animation on landing
  - Green flash on correct answer
  - Red shake on wrong answer
  - Confetti particles on streak milestone

- [ ] **Typography & color**
  - Apply consistent color palette
  - Add custom fonts (headers, body, letters)
  - Ensure high contrast for readability

#### Deliverables
- ✅ Mascot fully animated
- ✅ Visual feedback feels responsive
- ✅ Professional UI appearance

---

## 🔊 Sprint 9-10: Audio, Polish & Web Build
**Duration:** 2 weeks
**Goal:** Audio integration, final polish, WASM deployment

### Week 9: Audio Implementation

#### Tasks
- [ ] **Audio asset creation**
  - Source/create sound effects:
    - Letter tap (typing sound)
    - Word correct (success chime)
    - Word incorrect (soft buzzer)
    - Streak milestone (fanfare)
    - Level up (celebration)
  - Source/create background music:
    - Menu theme (calm, welcoming)
    - Gameplay theme (upbeat, focus)

- [ ] **Audio system**
  - Integrate `bevy_kira_audio`
  - Implement `AudioManager` resource
  - Add volume controls (master, SFX, music)
  - Persist audio settings

- [ ] **Audio triggers**
  - Play sound on every game event
  - Crossfade music on state transitions
  - Add audio preview in settings

#### Deliverables
- ✅ All sound effects playing
- ✅ Background music looping
- ✅ Volume controls working

---

### Week 10: Polish & Web Build

#### Tasks
- [ ] **Game polish**
  - Smooth all animations (easing curves)
  - Add particle effects (bevy_hanabi)
  - Improve loading screens
  - Add tooltips/help text

- [ ] **WASM build**
  - Configure `wasm32-unknown-unknown` target
  - Use wasm-bindgen for JS interop
  - Optimize WASM size (opt-level, LTO)
  - Test in Chrome, Firefox, Safari

- [ ] **Web deployment**
  - Create HTML wrapper page
  - Add loading progress bar
  - Host on GitHub Pages / Netlify
  - Test on mobile browsers

- [ ] **Performance optimization**
  - Profile frame time
  - Reduce draw calls (sprite batching)
  - Optimize trie memory layout
  - Compress assets (gzip)

#### Deliverables
- ✅ Game feels polished
- ✅ WASM build functional
- ✅ Web deployment live

---

## 🧪 Sprint 11: Testing & Bug Fixes
**Duration:** 1 week
**Goal:** Comprehensive testing, bug squashing

### Week 11: QA & Testing

#### Tasks
- [ ] **Unit testing**
  - Lexicon validation tests
  - Scoring calculation tests
  - SRS algorithm tests
  - Save/load tests

- [ ] **Integration testing**
  - State transition tests
  - End-to-end gameplay test
  - Custom lexicon loading test

- [ ] **Manual playtesting**
  - Playtest Stage 1 (5+ full sessions)
  - Test all UI flows
  - Test on different devices
  - Note all bugs and issues

- [ ] **Bug fixing**
  - Triage bugs by severity
  - Fix critical bugs (crashes, data loss)
  - Fix high-priority bugs (broken gameplay)
  - Defer low-priority bugs to post-MVP

- [ ] **Performance testing**
  - Confirm 60fps on low-end hardware
  - Test WASM performance on mobile
  - Check memory leaks (long sessions)

#### Deliverables
- ✅ All critical bugs fixed
- ✅ Test coverage >70%
- ✅ Performance targets met

---

## 📦 Sprint 12: Launch Prep & Documentation
**Duration:** 1 week
**Goal:** Final polish, documentation, launch

### Week 12: Launch Preparation

#### Tasks
- [ ] **Documentation**
  - Write player-facing README
  - Create quick start guide
  - Document keyboard shortcuts
  - Write FAQ

- [ ] **Build artifacts**
  - Create installers (Windows EXE, Mac DMG, Linux AppImage)
  - Generate WASM bundle
  - Test all builds on clean machines

- [ ] **Marketing materials**
  - Create screenshots
  - Record gameplay video (30s trailer)
  - Write launch announcement
  - Prepare social media posts

- [ ] **Launch checklist**
  - [ ] All builds tested
  - [ ] Documentation complete
  - [ ] Web deployment live
  - [ ] GitHub release created
  - [ ] Announcement published

#### Deliverables
- ✅ MVP launched
- ✅ All platforms available
- ✅ Documentation published

---

## 🔧 Development Tools & Stack

### Required Tools
- **Rust:** 1.70+ (stable)
- **Cargo:** Latest
- **Bevy:** 0.12+
- **wasm-pack:** For WASM builds
- **wasm-bindgen-cli:** For JS interop

### Recommended IDE Setup
- **VS Code** with extensions:
  - rust-analyzer
  - CodeLLDB (debugging)
  - Even Better TOML
  - Error Lens

### Dependencies (Cargo.toml)
```toml
[dependencies]
bevy = "0.12"
bevy_tweening = "0.9"
bevy_hanabi = "0.9"
bevy_kira_audio = "0.18"
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"
dirs = "5.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
criterion = "0.5"  # Benchmarking
```

---

## 👥 Team Roles & Responsibilities

### Core Team (Recommended)
- **Tech Lead / Senior Rust Engineer** (1)
  - Architecture decisions
  - Code review
  - Performance optimization

- **Game Developer** (1-2)
  - Implement gameplay systems
  - Stage mechanics
  - AI opponent

- **UI/UX Designer** (1)
  - Design all screens
  - Create mascot artwork
  - Visual polish

- **Sound Designer** (0.5)
  - Create/source SFX
  - Create/source music
  - Mix audio

- **QA Tester** (0.5)
  - Manual playtesting
  - Bug reporting
  - Performance testing

---

## 📊 Success Metrics (MVP)

### Technical Metrics
- [ ] **Performance:** 60fps on Intel i5/8GB RAM
- [ ] **Load time:** <5s from launch to menu
- [ ] **WASM size:** <10MB compressed
- [ ] **Test coverage:** >70%
- [ ] **Critical bugs:** 0

### User Metrics (Week 1 post-launch)
- [ ] **Downloads:** 100+ (target)
- [ ] **Session length:** >10 minutes average
- [ ] **Retention (D1):** >40%
- [ ] **Stage 1 completion:** >60%
- [ ] **User rating:** >4.0/5.0

---

## 🚧 Risk Management

### High-Risk Areas

**Risk 1: Lexicon file size slows WASM load**
- **Mitigation:** Compress with gzip, lazy-load in background
- **Contingency:** Use smaller starter lexicon, load full set later

**Risk 2: SRS algorithm too complex for kids**
- **Mitigation:** Extensive playtesting, adjust parameters
- **Contingency:** Add "Zen mode" without SRS

**Risk 3: WASM performance issues on mobile**
- **Mitigation:** Profile early, optimize hot paths
- **Contingency:** Desktop-first launch, mobile later

**Risk 4: Scope creep (adding stages 2-6 to MVP)**
- **Mitigation:** Strict MVP scope, defer to Phase 2
- **Contingency:** Cut features, focus on Stage 1 quality

---

## 🔄 Post-MVP Roadmap (Phase 2)

### Stage 2-6 Rollout (Weeks 13-24)
- **Week 13-15:** Stage 2 (3-4 letter construction)
- **Week 16-18:** Stage 3 (hooks & extensions)
- **Week 19-20:** Stage 3.5 (anagrams & Q-words)
- **Week 21-22:** Stage 4 (rack training)
- **Week 23-24:** Stage 5 (strategy board)

### Advanced Features (Weeks 25+)
- **Multiplayer mode:** Online head-to-head matches
- **Teacher portal:** Progress tracking dashboard
- **Mobile native:** iOS/Android builds
- **Advanced analytics:** Learning curve analysis
- **Content packs:** Regional lexicons, themed word sets

---

## 📞 Communication & Reporting

### Daily Standups (15 min)
- What I did yesterday
- What I'm doing today
- Any blockers

### Weekly Demos (Friday)
- Show completed features
- Get feedback
- Adjust plan if needed

### Sprint Reviews (Every 2 weeks)
- Review sprint goals
- Demo deliverables
- Retrospective (what went well, what to improve)

### Reporting Channels
- **Slack/Discord:** Real-time chat
- **GitHub Projects:** Task tracking
- **GitHub Issues:** Bug tracking
- **Google Docs:** Design documents

---

## 🎯 Launch Day Checklist

### Pre-Launch (T-1 week)
- [ ] All builds tested on target platforms
- [ ] Critical bugs fixed
- [ ] Documentation complete
- [ ] Marketing materials ready

### Launch Day (T=0)
- [ ] Create GitHub release with binaries
- [ ] Deploy WASM to web host
- [ ] Publish announcement post
- [ ] Share on social media
- [ ] Monitor for bug reports

### Post-Launch (T+1 week)
- [ ] Respond to user feedback
- [ ] Hotfix critical bugs
- [ ] Collect analytics
- [ ] Plan Phase 2 priorities

---

## 📚 Reference Links

- [Bevy Engine Docs](https://bevyengine.org/learn/)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
- [CSW24 Official Site](https://scrabble.org.au/words/csw24)
- [SM-2 Algorithm](https://www.supermemo.com/en/archives1990-2015/english/ol/sm2)
- [Game Design Patterns](https://gameprogrammingpatterns.com/)

---

**Document Status:** ✅ Complete
**Next Review Date:** Sprint 3 (Week 3)
**Maintained By:** Tech Lead

---

## 🎉 Final Notes

This roadmap is **ambitious but achievable** with a focused team. The key to success is:

1. **Stay disciplined** on MVP scope (Stage 1 only!)
2. **Playtest early** and iterate on feedback
3. **Ship fast**, improve later
4. **Celebrate milestones** (each sprint completion is a win!)

Let's build an amazing learning game that trains the next generation of Scrabble champions! 🏆
