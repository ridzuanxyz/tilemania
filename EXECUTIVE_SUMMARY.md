# 🧠 Scrabble Learning Game – Executive Summary
*A modern, gamified approach to train future Scrabble champions (Ages 7–12)*

## 🎯 Objective
This project aims to create an **interactive, game-based learning platform** that teaches children fundamental Scrabble skills through fun, fast-paced mini-games — progressing from basic 2-letter words to advanced board strategy.

The game is designed to **train instinct**, not just memorization, enabling kids to **think like competitive players** from a young age.

---

## 🕹️ Core Gameplay Loop
1. **Letters appear or fall** on the screen (2-letter focus initially).  
2. Players **type or tap** to match or complete valid Scrabble words.  
3. Instant feedback: ✨ animation, sound, points.  
4. Difficulty increases with speed, distractors, and complexity.  
5. Rewards and streak bonuses reinforce learning.

---

## 📚 Learning Outcomes
- **Stage 1**: Instant recall of 2-letter words (all 127 valid CSW24 2-letter words).
- **Stage 2**: Word construction with 3–4 letter playable words.
- **Stage 3**: Hook and extension mastery.
- **Stage 3.5**: Anagram recognition and Q-without-U word training.
- **Stage 4**: Rack management and quick word generation.
- **Stage 5**: Positional strategy with simplified Scrabble boards (premium squares, bingo awareness).
- **Stage 6**: Tournament simulation with timed games, endgame analysis, and blank tile strategy.

---

## 🧠 Pedagogical Approach
- **Repetition through play**: builds reflexive word recognition.
- **Spaced repetition system (SRS)**: algorithmically reinforces weak words and patterns.
- **Progressive difficulty**: aligns with player growth and skill level.
- **Immediate reward feedback loop**: keeps children motivated.
- **Visual hooks**: animations, mascots, confetti, sound effects.
- **Competitive fundamentals**: tournament-essential patterns (Q-words, anagrams, blanks).
- **Offline-first design**: no internet required, works in classrooms and on-the-go.

---

## 🧩 Game Structure & Expansion

| Stage | Focus                       | Core Mechanic                         | Learning Goal                                 |
|-------|------------------------------|------------------------------------------|--------------------------------------------|
| 1     | 2-letter words               | Falling/tapping letters                 | Fast recognition of all 127 CSW24 2-letter words |
| 2     | 3–4 letter construction     | Tile drop + swap                        | Word building instincts                    |
| 3     | Hooks & extensions           | Front/back hook challenge               | Word expansion skills (CARE→SCARE, CARE→CARES) |
| 3.5   | Anagrams & Q-words          | Anagram solver + Q-without-U drills     | Pattern recognition (RETINA/RETAIN) + QI/QOPH/QADI |
| 4     | Rack training               | Timed rack-solving mini-game           | Maximizing rack value + leave management   |
| 5     | Strategy board              | Simplified Scrabble match vs AI        | Premium squares, bingo setup, board control |
| 6     | Tournament mode             | Real match simulation + analysis       | Competitive play + blank tile strategy     |

---

## 🎨 Visual & UX Direction
- **Duolingo-inspired UI** — clean, colorful, friendly.  
- **Mascot-assisted feedback** — animations, voice reactions, badges.  
- **Minimalist menus** with progressive unlocks.  
- **Particle effects & tween animations** for high engagement.  
- Cross-platform support (Web, Desktop, Mobile).

---

## 🛠️ Technology Stack
- **Language:** Rust 🦀
- **Engine:** [Bevy](https://bevyengine.org/) (ECS, 2D rendering, UI)
- **Animation:** `bevy_tweening`
- **Particles:** `bevy_hanabi`
- **Audio:** `bevy_kira_audio`
- **Lexicon:** CSW24 (Collins Scrabble Words 2024) — ~280k words, bundled offline-first
- **Custom Lexicon Support:** File loader for TWL, SOWPODS, or regional word lists
- **Word Validation:** Trie-based data structure for O(n) lookup performance
- **Persistence:** Local storage (RON/JSON) for progress tracking, settings, and custom lexicons
- **Build Targets:** Desktop (Windows/Mac/Linux), Web (WASM), Mobile (future)

---

## 🧱 Project Structure
```
project_root/
├─ src/
│  ├─ main.rs
│  ├─ states/           # menu, gameplay, gameover, stage selection
│  ├─ systems/          # input, animations, scoring, word validation
│  ├─ ui/               # overlays, buttons, panels, mascot
│  ├─ lexicon/          # word dictionary, validation engine, custom lexicon loader
│  ├─ game_stages/      # stage 1-6 gameplay logic
│  └─ ai/               # opponent AI for stages 5-6
├─ assets/
│  ├─ lexicons/         # CSW24.txt (bundled), user-provided lexicons
│  ├─ fonts/
│  ├─ sounds/
│  ├─ sprites/
│  └─ animations/
├─ docs/
│  ├─ ARCHITECTURE.md
│  ├─ GAME_DESIGN.md
│  └─ IMPLEMENTATION_ROADMAP.md
├─ Cargo.toml
└─ README.md
```

---

## 📈 Long-Term Vision
- 🏆 National youth Scrabble training platform.  
- 🧑‍🏫 Teacher dashboards to track student progress.  
- 🌐 Online multiplayer tournaments.  
- 🧠 AI analysis of missed word opportunities.  
- 🪙 Gamification system (XP, badges, streaks).

---

## 📥 Deliverables (Phase 1)
- ✅ Stage 1 playable “Falling Letter” game.  
- 🪄 Basic scoring, feedback, and UI system.  
- 🌈 Responsive animations and background.  
- 🌐 WASM build for browser play.  
- 📊 Player progress tracking (local).

---

## 🚀 Development Timeline (Initial Phase)
| Milestone                  | Target Duration | Deliverable                                |
|----------------------------|------------------|--------------------------------------------|
| Game skeleton & Bevy setup | Week 1           | Base project + rendering loop              |
| Stage 1 gameplay           | Week 2           | Falling letters, scoring, sound            |
| UI & polish                | Week 3           | Mascot, animations, particle effects       |
| Web build & test           | Week 4           | Playable in browser                        |

---

## 🧭 Strategic Advantage
- Builds Scrabble instinct early (key for competitive play).  
- Fun-first design keeps kids engaged.  
- Rust + Bevy = fast, cross-platform, minimal dependency.  
- Scalable architecture for years of growth.

---

## 🏁 Conclusion
The Scrabble Learning Game transforms what is usually **rote memorization** into a **rewarding, playful, and strategic journey** — laying the foundation for tomorrow’s Scrabble champions.
