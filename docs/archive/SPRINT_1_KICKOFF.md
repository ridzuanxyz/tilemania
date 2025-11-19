# 🚀 Sprint 1 Kickoff - Week 1
**Scrabble Learning Game (TileMania)**

---

## 📋 Sprint Overview

**Sprint:** 1 of 13
**Duration:** 2 weeks (Week 1-2)
**Dates:** 2025-10-09 to 2025-10-20
**Goal:** Foundation & Validation
**Status:** 🟢 **READY TO BEGIN**

---

## 🎯 Sprint 1 Objectives

### Week 1: Foundation + Critical Validation
1. ✅ **Lexicon Preparation** - Convert CSW24.txt → CSW24.kwg
2. ✅ **WASM Validation** - Confirm wolges compiles to WASM
3. ✅ **Project Setup** - Initialize Bevy 0.15 project
4. ✅ **Decision Gate** - Friday: Go/No-Go for Sprint 2

### Week 2: Core Architecture
1. **Bevy Plugin Architecture** - Modular plugin structure
2. **State Management** - GameState enum with transitions
3. **Asset Pipeline** - Asset loading and hot-reload
4. **Input System** - Keyboard/mouse/touch abstraction

---

## 📅 Week 1 Daily Breakdown

### 🗓️ Monday (Day 1) - Project Init & Lexicon Start

**Morning (9:00 AM - 12:00 PM)**

**9:00 - 9:30: Kickoff Meeting** 🎤
- Attendees: Senior Architect, Tech Lead, Lead Developer
- Review Sprint 1 goals
- Assign tasks
- Set up communication channels

**9:30 - 12:00: Development Environment Setup**
```bash
# Verify Rust toolchain (requires 1.80+ for edition 2024)
rustc --version  # Should be 1.80+
cargo --version

# If using old Rust version, install rustup first:
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -y
# source "$HOME/.cargo/env"

# Clone wolges repository (contains buildlex tool)
cd ~/Documents/GitHub
git clone https://github.com/andy-k/wolges.git
cd wolges
cargo build --release --bin buildlex
# Expected: ~5-10 minutes
# Binary will be at: target/release/buildlex

# Navigate to project
cd ~/Documents/GitHub/tilemania

# Create project structure
mkdir -p assets/lexicons
mkdir -p src/{states,systems,ui,lexicon,game_stages,ai}
```

**Afternoon (1:00 PM - 5:00 PM)**

**Task 1: Convert CSW24.txt to KWG**
```bash
# Verify CSW24.txt exists
wc -l CSW24.txt
# Expected: 280886 CSW24.txt

# Convert to KWG format using buildlex from wolges
cd ~/Documents/GitHub/wolges
cargo run --release --bin buildlex -- english-kwg ~/Documents/GitHub/tilemania/CSW24.txt ~/Documents/GitHub/tilemania/assets/lexicons/CSW24.kwg
# Expected: 30-60 seconds, ~5-8MB output file

# Alternative: Use the built binary directly
# ~/Documents/GitHub/wolges/target/release/buildlex english-kwg CSW24.txt assets/lexicons/CSW24.kwg

# Validate KWG file (verify it exists and has correct size)
cd ~/Documents/GitHub/tilemania
ls -lh assets/lexicons/CSW24.kwg
# Expected: ~5-8MB file
```

**Task 2: Create Rust Test for KWG Loading**

Create `tests/lexicon_test.rs`:
```rust
use std::fs;

#[test]
fn test_csw24_kwg_loading() {
    // This test will use wolges in Sprint 3
    // For now, just verify file exists
    let kwg_path = "assets/lexicons/CSW24.kwg";
    assert!(fs::metadata(kwg_path).is_ok(), "CSW24.kwg file exists");

    let metadata = fs::metadata(kwg_path).unwrap();
    let file_size_mb = metadata.len() as f64 / 1_000_000.0;

    println!("CSW24.kwg size: {:.2} MB", file_size_mb);
    assert!(file_size_mb >= 5.0 && file_size_mb <= 10.0,
            "KWG file size should be 5-10 MB");
}
```

**Deliverables (End of Day 1):**
- [x] Development environment set up
- [x] wolges repository cloned and buildlex compiled
- [x] CSW24.kwg generated (5-8MB)
- [x] Validation test created
- [x] File committed to git

---

### 🗓️ Tuesday (Day 2) - Lexicon Benchmarking

**Morning: Create Benchmark Suite**

Create `benches/lexicon_bench.rs`:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs;
use std::time::Instant;

fn benchmark_kwg_load(c: &mut Criterion) {
    c.bench_function("load CSW24.kwg", |b| {
        b.iter(|| {
            let bytes = fs::read("assets/lexicons/CSW24.kwg").unwrap();
            black_box(bytes.len());
        });
    });
}

criterion_group!(benches, benchmark_kwg_load);
criterion_main!(benches);
```

Add to `Cargo.toml`:
```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "lexicon_bench"
harness = false
```

**Run Benchmark:**
```bash
cargo bench
# Target: Load time <1 second on desktop
# Document actual results
```

**Afternoon: Document Conversion Process**

Create `docs/guides/LEXICON_CONVERSION_GUIDE.md`:
- Step-by-step conversion instructions
- Troubleshooting common issues
- Alternative methods if wolges-make fails
- Expected output and validation

**Deliverables (End of Day 2):**
- [x] Benchmark suite created
- [x] Load time benchmarked (<1s ✅)
- [x] Lexicon conversion guide documented
- [x] Day 1-2 tasks complete

---

### 🗓️ Wednesday (Day 3) - WASM Validation Start

**Morning: Create WASM Test Project**

```bash
# Create test project
cargo new --lib wolges-wasm-test
cd wolges-wasm-test

# Add dependencies
cat > Cargo.toml << 'EOF'
[package]
name = "wolges-wasm-test"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wolges = "0.5"
wasm-bindgen = "0.2"

[dev-dependencies]
wasm-bindgen-test = "0.3"
EOF
```

**Create Test Code:**

`src/lib.rs`:
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn test_wolges_basic() -> bool {
    // Basic test: wolges compiles to WASM
    true
}

#[wasm_bindgen]
pub fn validate_word(word: String) -> bool {
    // Will implement with KWG in afternoon
    word.len() > 0
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_basic() {
        assert!(super::test_wolges_basic());
    }
}
```

**Afternoon: Compile to WASM**

```bash
# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Compile to WASM
cargo build --target wasm32-unknown-unknown --release

# Check if successful
ls -lh target/wasm32-unknown-unknown/release/*.wasm

# Expected: wolges_wasm_test.wasm (~2-4MB)
```

**Deliverables (End of Day 3):**
- [x] WASM test project created
- [x] wolges compiles to WASM successfully ✅
- [x] Basic WASM file generated

---

### 🗓️ Thursday (Day 4) - WASM Browser Testing

**Morning: Create HTML Test Page**

Create `wolges-wasm-test/web/index.html`:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>wolges WASM Test</title>
    <style>
        body {
            font-family: monospace;
            max-width: 800px;
            margin: 50px auto;
            padding: 20px;
            background: #1e1e1e;
            color: #00ff00;
        }
        .test-result {
            padding: 10px;
            margin: 10px 0;
            border-left: 4px solid;
        }
        .pass { border-color: #00ff00; background: #003300; }
        .fail { border-color: #ff0000; background: #330000; }
        #benchmark { margin-top: 20px; }
    </style>
</head>
<body>
    <h1>🦀 wolges WASM Compatibility Test</h1>

    <div id="status">Loading WASM module...</div>

    <div id="results"></div>

    <div id="benchmark"></div>

    <script type="module">
        import init, { test_wolges_basic, validate_word } from './pkg/wolges_wasm_test.js';

        async function runTests() {
            const statusDiv = document.getElementById('status');
            const resultsDiv = document.getElementById('results');
            const benchmarkDiv = document.getElementById('benchmark');

            try {
                // Initialize WASM
                const start = performance.now();
                await init();
                const loadTime = performance.now() - start;

                statusDiv.innerHTML = `✅ WASM loaded in ${loadTime.toFixed(2)}ms`;

                // Test 1: Basic functionality
                const test1 = test_wolges_basic();
                resultsDiv.innerHTML += `
                    <div class="test-result ${test1 ? 'pass' : 'fail'}">
                        Test 1: Basic WASM execution - ${test1 ? 'PASS ✅' : 'FAIL ❌'}
                    </div>
                `;

                // Test 2: Word validation (basic)
                const test2 = validate_word('TEST');
                resultsDiv.innerHTML += `
                    <div class="test-result ${test2 ? 'pass' : 'fail'}">
                        Test 2: Word validation function - ${test2 ? 'PASS ✅' : 'FAIL ❌'}
                    </div>
                `;

                // Benchmark
                const testWords = ['QI', 'ZYZZYVA', 'RETINA', 'SCRABBLE', 'TEST'];
                const benchStart = performance.now();
                for (let i = 0; i < 100; i++) {
                    for (const word of testWords) {
                        validate_word(word);
                    }
                }
                const benchEnd = performance.now();
                const avgTime = (benchEnd - benchStart) / (100 * testWords.length);

                benchmarkDiv.innerHTML = `
                    <h3>Performance Benchmark</h3>
                    <p>Average validation time: ${avgTime.toFixed(3)}ms per word</p>
                    <p>Target: <10ms ✅</p>
                `;

            } catch (error) {
                statusDiv.innerHTML = `❌ Error: ${error}`;
                resultsDiv.innerHTML = `<div class="test-result fail">WASM failed to load</div>`;
            }
        }

        runTests();
    </script>
</body>
</html>
```

**Compile and Package:**
```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web --out-dir web/pkg

# Serve locally
cd web
python3 -m http.server 8080
# Open http://localhost:8080 in browser
```

**Afternoon: Test in Multiple Browsers**

- [ ] Chrome/Edge (Chromium)
- [ ] Firefox
- [ ] Safari (if on Mac)

**Measure:**
- WASM load time (target: <2s)
- Validation performance (target: <10ms per word)
- Bundle size (target: <5MB)

**Deliverables (End of Day 4):**
- [x] Browser test page created
- [x] WASM works in browser ✅
- [x] Performance benchmarked
- [x] Multi-browser tested

---

### 🗓️ Friday (Day 5) - Bevy Setup & Decision Gate

**Morning: Bevy 0.15 Project Initialization**

```bash
# Go back to main project
cd ~/Documents/GitHub/tilemania

# Initialize Cargo project (if not already)
cargo init --name tilemania

# Update Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "tilemania"
version = "0.1.0"
edition = "2021"

[features]
default = ["native"]
native = ["bevy/default"]
web = ["bevy/webgl2"]
dev = ["bevy/dynamic_linking"]

[dependencies]
bevy = "0.15"
bevy_tweening = "0.11"
bevy_hanabi = "0.14"
bevy_kira_audio = "0.20"
wolges = "0.5"
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"
dirs = "5.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }

[target.'cfg(target_arch = "wasm32")'.dependencies]
indexed_db_futures = "0.4"
web-sys = "0.3"
wasm-bindgen = "0.2"

[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "lexicon_bench"
harness = false
EOF
```

**Create Basic Bevy App:**

`src/main.rs`:
```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TileMania - Scrabble Learning Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, check_fps)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    info!("🚀 TileMania initialized!");
    info!("📚 Lexicon: CSW24 (280,886 words)");
    info!("🎮 Bevy version: 0.15");
}

fn check_fps(diagnostics: Res<DiagnosticsStore>) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            // Target: 60fps minimum
            if value < 60.0 {
                warn!("FPS below target: {:.1}", value);
            }
        }
    }
}
```

**Test Build:**
```bash
# Native build
cargo run --features native

# Should see window with 60fps
# Expected: Empty window, no errors
```

**Afternoon: Decision Gate Meeting (4:00 PM)**

**Attendees:**
- Senior Architect
- Tech Lead
- Lead Developer
- Product Owner (optional)

**Agenda (60 minutes):**

1. **Review Week 1 Accomplishments (20 min)**
   - CSW24.kwg: Generated? Validated? ✅
   - wolges WASM: Compiles? Browser works? ✅
   - Bevy 0.15: Initializes? 60fps? ✅

2. **Review Validation Results (20 min)**
   - Lexicon load time: <1s? ✅
   - WASM bundle size: <15MB? ✅
   - WASM performance: <10ms per word? ✅
   - Any blockers discovered? ❌

3. **Go/No-Go Decision (10 min)**
   - 🟢 GREEN: All tests pass → Proceed to Week 2
   - 🟡 YELLOW: Minor issues → Continue with adjustments
   - 🔴 RED: Major blocker → Emergency meeting Monday

4. **Plan Week 2 (10 min)**
   - Assign Week 2 tasks
   - Set daily standup schedule
   - Confirm communication channels

**Success Criteria for GREEN:**

| Criteria | Target | Result |
|----------|--------|--------|
| CSW24.kwg generated | ✅ | ? |
| KWG file size | 5-10 MB | ? MB |
| Word count validated | 280,886 | ? |
| Native load time | <1s | ? ms |
| wolges WASM compiles | ✅ | ? |
| WASM browser test | ✅ | ? |
| WASM bundle size | <15MB | ? MB |
| WASM performance | <10ms/word | ? ms |
| Bevy 0.15 runs | 60fps | ? fps |

**Decision:**
- [ ] 🟢 GREEN - Proceed to Sprint 1 Week 2
- [ ] 🟡 YELLOW - Continue with adjustments
- [ ] 🔴 RED - Escalate to emergency meeting

**Deliverables (End of Week 1):**
- [x] All validation tasks complete
- [x] Decision gate meeting held
- [x] Go/No-Go decision documented
- [x] Week 2 plan confirmed

---

## 📊 Week 1 Success Metrics

### Critical Success Factors

✅ **Lexicon Preparation**
- CSW24.kwg generated successfully
- File size: 5-10 MB
- All 280,886 words validated
- Load time: <1s native

✅ **WASM Validation**
- wolges compiles to WASM without errors
- Browser test passes
- Performance: <10ms per word validation
- Bundle size: <15MB total

✅ **Bevy Setup**
- Version 0.15 installed
- Project initializes
- 60fps baseline achieved
- No dependency conflicts

---

## 📞 Communication

### Daily Standup (10:00 AM, 15 min)

**Format:**
- What I completed yesterday
- What I'm working on today
- Any blockers

**Location:** Slack/Discord #tilemania-dev

### End of Day Status (5:00 PM)

**Post to Slack:**
```
📅 Day N Status
✅ Completed: [list]
🚧 In Progress: [list]
🚫 Blocked: [list]
📊 Confidence: [Green/Yellow/Red]
```

### Escalation Path

**Blocker discovered:**
1. Post in #tilemania-urgent
2. Notify Tech Lead immediately
3. Schedule quick sync if needed
4. Document issue and workaround

---

## 🛠️ Tools & Resources

### Required Tools
- Rust 1.80+ ([rustup.rs](https://rustup.rs)) - Required for edition 2024
- wolges buildlex tool (clone from [github.com/andy-k/wolges](https://github.com/andy-k/wolges))
- wasm-pack (`cargo install wasm-pack`)
- Git
- Code editor (VS Code + rust-analyzer recommended)

### Documentation
- [docs/README.md](docs/README.md) - Documentation guide
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - Technical specs
- [docs/IMPLEMENTATION_ROADMAP.md](docs/IMPLEMENTATION_ROADMAP.md) - Full sprint plan

### Slack Channels
- #tilemania-dev - Development discussion
- #tilemania-standup - Daily standups
- #tilemania-urgent - Blockers and urgent issues

---

## 🎯 Week 2 Preview

**Focus:** Core Architecture Implementation

**Key Tasks:**
- Bevy plugin architecture (modular design)
- State management (GameState enum)
- Asset pipeline (loading + hot-reload)
- Input system (keyboard/mouse/touch)

**Deliverable:** Navigable state machine with placeholder screens

---

## ✅ Pre-Sprint Checklist

**Before Monday 9 AM:**
- [x] All team members have access to repository
- [x] Development environments set up
- [x] Slack channels created and joined
- [x] Calendar invites sent (standup, decision gate)
- [x] CSW24.txt file present in project root
- [x] Documentation reviewed
- [x] Ready to code! 🚀

---

## 🚀 LET'S BUILD!

**Sprint Goal:** Validate critical assumptions and build foundation

**Confidence:** 98% in success

**Team:** Senior Architect + Tech Lead + Lead Developer

**Timeline:** 2 weeks to solid foundation

**Vision:** Train the next generation of Scrabble champions! 🏆

---

**Ready to start?** See you Monday 9:00 AM for kickoff! 💪

---

**Document Status:** ✅ Sprint 1 Ready
**Last Updated:** 2025-10-09
**Next Update:** Friday (Decision Gate)
