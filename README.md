# 🧠 TileMania - Scrabble Learning Game

**A modern, gamified platform to train future Scrabble champions (Ages 7-12)**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/bevy-0.15-blue.svg)](https://bevyengine.org/)
[![Status](https://img.shields.io/badge/status-Sprint%201-green.svg)](SPRINT_1_KICKOFF.md)
[![License](https://img.shields.io/badge/license-Educational-yellow.svg)](LICENSE)

---

## 🎯 Project Overview

TileMania transforms Scrabble learning from rote memorization into a **rewarding, playful, and strategic journey**. Through 6 progressive stages, kids master everything from 2-letter words to tournament-level strategy.

**Key Features:**
- 🎮 6 progressive learning stages (2-letter words → Tournament play)
- 🌐 Offline-first design (no internet required)
- 🦀 Built with Rust + Bevy (fast, cross-platform)
- 📚 CSW24 lexicon (280,886 words)
- 🤖 AI opponents (Beginner → Advanced)
- 🏆 Tournament-ready training

---

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.70+
cargo --version
```

### Build & Run

```bash
# Clone repository
git clone <repository-url>
cd tilemania

# Build and run (native)
cargo run --release --features native

# Or for web (WASM)
cargo build --release --target wasm32-unknown-unknown --features web
```

---

## 📚 Documentation

### For Developers

| Document | Purpose |
|----------|---------|
| [Executive Summary](Executive%20Summary.md) | Project overview and vision |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Technical architecture |
| [docs/GAME_DESIGN.md](docs/GAME_DESIGN.md) | Gameplay mechanics & UX |
| [docs/IMPLEMENTATION_ROADMAP.md](docs/IMPLEMENTATION_ROADMAP.md) | Sprint-by-sprint plan |
| [docs/README.md](docs/README.md) | Documentation navigation guide |

### Current Sprint

- 📅 **Sprint 1** (Week 1-2): Foundation & Validation
- 📖 See [SPRINT_1_KICKOFF.md](SPRINT_1_KICKOFF.md) for details
- 📊 Track progress: [SPRINT_1_TRACKER.md](SPRINT_1_TRACKER.md)

---

## 🎮 Learning Stages

| Stage | Focus | Mechanic |
|-------|-------|----------|
| 1 | 2-letter words | Falling letters game |
| 2 | 3-4 letter words | Tile construction |
| 3 | Hooks & extensions | Pattern recognition |
| 3.5 | Anagrams & Q-words | Word families |
| 4 | Rack training | Best-word puzzles |
| 5 | Strategy board | 15×15 board with AI |
| 6 | Tournament mode | Full competition rules |

---

## 🛠️ Tech Stack

- **Language:** Rust 🦀
- **Engine:** Bevy 0.15 (ECS, 2D rendering, UI)
- **Lexicon:** CSW24 (Collins Scrabble Words 2024)
- **Scrabble Engine:** wolges (GADDAG-based move generation)
- **Animation:** bevy_tweening
- **Particles:** bevy_hanabi
- **Audio:** bevy_kira_audio
- **Platforms:** Desktop (Win/Mac/Linux) + Web (WASM)

---

## 📦 Project Structure

```
tilemania/
├── src/
│   ├── main.rs                    # App entry point
│   ├── states/                    # Game states (menu, gameplay, etc.)
│   ├── systems/                   # Core systems (input, scoring, etc.)
│   ├── ui/                        # UI components
│   ├── lexicon/                   # Word validation engine
│   ├── game_stages/               # Stage 1-6 implementations
│   └── ai/                        # AI opponent logic
│
├── assets/
│   ├── lexicons/
│   │   └── CSW24.kwg              # 280k words (binary format)
│   ├── fonts/                     # Typography
│   ├── sounds/                    # SFX & music
│   ├── sprites/                   # Lexi the Owl mascot
│   └── animations/                # Visual effects
│
├── docs/                          # Documentation
│   ├── README.md                  # Docs navigation
│   ├── ARCHITECTURE.md            # Technical specs
│   ├── GAME_DESIGN.md             # Gameplay design
│   ├── IMPLEMENTATION_ROADMAP.md  # Sprint plan
│   └── archive/                   # Historical decisions
│
├── tests/                         # Unit & integration tests
├── benches/                       # Performance benchmarks
│
├── CSW24.txt                      # Source lexicon (280,886 words)
├── Executive Summary.md           # Project overview
├── SPRINT_1_KICKOFF.md            # Current sprint plan
└── SPRINT_1_TRACKER.md            # Progress tracking
```

---

## 🎯 Development Status

### Sprint 1 (Current) - Weeks 1-2

**Week 1: Foundation & Validation**
- [x] Architecture approved (see [docs/archive/](docs/archive/))
- [ ] Convert CSW24.txt → CSW24.kwg
- [ ] Validate wolges WASM compatibility
- [ ] Initialize Bevy 0.15 project
- [ ] Decision gate (Friday)

**Week 2: Core Architecture**
- [ ] Plugin architecture
- [ ] State management
- [ ] Asset pipeline
- [ ] Input system

**Next:** Sprint 2-3 → Lexicon & Scoring Systems

---

## 🏆 Key Architectural Decisions

### Why wolges?
- Industry-standard Scrabble engine
- GADDAG for fast move generation
- Memory-efficient (5-8MB for full CSW24)
- Battle-tested in production apps

### Why 15×15 board for Stage 5?
- Kids learn real board layout early
- Premium square positions become familiar
- No "unlearning" required for Stage 6
- Vocabulary-limited training mode

### Why CSW24?
- International standard (WESPA)
- Most comprehensive word list (280k words)
- Tournament-ready training

See [docs/ARCHITECTURE_DECISIONS.md](docs/ARCHITECTURE_DECISIONS.md) for full ADR.

---

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run benchmarks
cargo bench

# Run specific test
cargo test lexicon_test

# WASM tests (requires wasm-pack)
wasm-pack test --headless --firefox
```

---

## 📈 Performance Targets

| Metric | Target | Status |
|--------|--------|--------|
| Frame rate | 60fps | TBD (Sprint 1 Week 2) |
| Word validation | <5ms | TBD (Sprint 3) |
| Lexicon load time | <2s | TBD (Sprint 1 Day 2) |
| WASM bundle size | <15MB | TBD (Sprint 1 Day 4) |
| Memory usage | <200MB | TBD (Sprint 3) |

---

## 🤝 Contributing

### For Team Members

1. Read [docs/README.md](docs/README.md) for documentation guide
2. Check [SPRINT_1_TRACKER.md](SPRINT_1_TRACKER.md) for current tasks
3. Follow daily standup schedule (10 AM)
4. Post end-of-day status (5 PM)

### Code Style

- Follow Rust conventions (rustfmt)
- Use descriptive variable names
- Document public APIs
- Write tests for core logic

---

## 📞 Team

- **Senior Architect:** Architecture & system design
- **Tech Lead:** Sprint planning & code review
- **Lead Developer:** Implementation & testing
- **Game Designer:** UX & gameplay mechanics

---

## 📜 License

Educational use only (MVP phase). See [docs/ARCHITECTURE_DECISIONS.md](docs/ARCHITECTURE_DECISIONS.md) for CSW24 licensing details.

For commercial use, will require:
- HarperCollins license for CSW24
- OR switch to TWL/OSPD
- OR educational-only distribution

---

## 🎓 Learning Resources

### Scrabble Strategy
- [NASPA](https://www.scrabbleplayers.org/) - North American Scrabble Players Association
- [WESPA](https://www.wespa.org/) - World English Scrabble Players Association
- [CSW24](https://scrabble.org.au/words/csw24) - Official word list

### Technical Resources
- [Bevy Book](https://bevyengine.org/learn/book/introduction/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [wolges Crate](https://crates.io/crates/wolges)
- [GADDAG Algorithm](https://en.wikipedia.org/wiki/GADDAG)

---

## 🚀 Roadmap

### Phase 1: MVP (13 weeks)
- ✅ Week 1-2: Foundation
- 🔄 Week 3-4: Lexicon & Core Systems
- ⏳ Week 5-6: Stage 1 Gameplay
- ⏳ Week 7-8: UI/UX & Mascot
- ⏳ Week 9-10: Audio & Web Build
- ⏳ Week 11: Testing
- ⏳ Week 12-13: Launch

### Phase 2: Full Stages (Weeks 14-24)
- Stage 2: 3-4 letter construction
- Stage 3: Hooks & extensions
- Stage 3.5: Anagrams & Q-words
- Stage 4: Rack training
- Stage 5: Strategy board
- Stage 6: Tournament mode

### Phase 3: Advanced Features
- Online multiplayer
- Teacher dashboard
- Mobile apps (iOS/Android)
- Advanced analytics
- Custom word lists

---

## 📊 Project Metrics

- **Lines of Code:** TBD (Sprint 1+)
- **Test Coverage:** Target >70%
- **Word Count:** 280,886 (CSW24)
- **Supported Platforms:** Desktop + Web (MVP)
- **Target Age:** 7-12 years
- **Development Timeline:** 13 weeks MVP

---

## 🎉 Acknowledgments

- Collins Scrabble Words (CSW24) - HarperCollins Publishers
- wolges crate - Andy Kurnia
- Bevy Engine - Bevy Foundation
- Scrabble grandmasters worldwide for pedagogical insights

---

## 📞 Contact

For questions about:
- **Architecture:** Senior Architect
- **Sprint tasks:** Tech Lead
- **Implementation:** Lead Developer
- **Gameplay:** Game Designer

---

**Status:** 🟢 Sprint 1 Active
**Last Updated:** 2025-10-09
**Next Milestone:** Decision Gate (Friday 2025-10-13)

---

*"Training the next generation of Scrabble champions, one word at a time."* 🏆
