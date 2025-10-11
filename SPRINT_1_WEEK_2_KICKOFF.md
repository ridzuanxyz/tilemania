# 🚀 Sprint 1 Week 2 Kickoff - Core Architecture

**Scrabble Learning Game (TileMania)**

---

## 📋 Week 2 Overview

**Sprint:** 1 of 13
**Week:** 2 of 2
**Dates:** 2025-10-10 to 2025-10-17
**Goal:** Core Architecture Implementation
**Status:** 🟢 **READY TO BEGIN**

---

## ✅ Week 1 Achievements

**Decision Gate Result:** 🟢 GREEN - All validation objectives exceeded!

### Completed Validations
- ✅ **Lexicon:** CSW24.kwg validated (15ms load time, 66x faster than target)
- ✅ **WASM:** Browser tests pass (14KB bundle, 1000x smaller than target)
- ✅ **Bevy:** Dependencies compile cleanly (zero errors, 848MB target directory)
- ✅ **Documentation:** 4 comprehensive guides created
- ✅ **Performance:** All targets exceeded by massive margins

### Key Metrics
| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| KWG Load Time | <1000ms | 15ms | ✅ 66x faster |
| WASM Bundle | <15MB | 14KB | ✅ 1000x smaller |
| Bevy Compile | 0 errors | 0 errors | ✅ Clean |
| Disk Space | Manageable | 3.7GB free | 🟡 Monitored |

---

## 🎯 Week 2 Objectives

### Core Architecture Goals
1. **Bevy Plugin Architecture** - Modular, maintainable plugin structure
2. **State Management** - GameState enum with smooth transitions
3. **Asset Pipeline** - Efficient loading and hot-reload support
4. **Input System** - Unified keyboard/mouse/touch abstraction

### End-of-Week Deliverable
A navigable state machine with placeholder screens demonstrating:
- State transitions (Splash → MainMenu → GameBoard → Results)
- Asset loading system
- Input handling across all screens
- Clean plugin architecture

---

## 📅 Week 2 Daily Breakdown

### 🗓️ Monday (Day 6) - 2025-10-10: Plugin Architecture

**Status:** 🚧 IN PROGRESS

**Morning: Core Plugin Structure**

**Objective:** Create modular Bevy plugin architecture

**Tasks:**
1. Create plugin module structure
2. Implement CorePlugin (base systems)
3. Implement StatePlugin (state management)
4. Implement AssetPlugin (asset loading)
5. Implement InputPlugin (input abstraction)

**Implementation:**

```bash
# Create plugin directory structure
mkdir -p src/plugins
touch src/plugins/mod.rs
touch src/plugins/core.rs
touch src/plugins/state.rs
touch src/plugins/assets.rs
touch src/plugins/input.rs
```

**File: `src/plugins/mod.rs`**
```rust
pub mod core;
pub mod state;
pub mod assets;
pub mod input;

pub use core::CorePlugin;
pub use state::StatePlugin;
pub use assets::AssetPlugin;
pub use input::InputPlugin;
```

**File: `src/plugins/core.rs`**
```rust
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_core)
            .add_systems(Update, core_systems);
    }
}

fn setup_core(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    info!("🎮 TileMania Core initialized");
}

fn core_systems() {
    // Core update systems
}
```

**File: `src/plugins/state.rs`**
```rust
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    GameBoard,
    Results,
    Settings,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(OnEnter(GameState::Splash), enter_splash)
            .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
            .add_systems(OnEnter(GameState::GameBoard), enter_game_board)
            .add_systems(OnEnter(GameState::Results), enter_results);
    }
}

fn enter_splash(mut next_state: ResMut<NextState<GameState>>) {
    info!("📺 Entering Splash screen");
    // Auto-transition to main menu after 2 seconds (will implement timer later)
    // For now, immediate transition
    next_state.set(GameState::MainMenu);
}

fn enter_main_menu() {
    info!("📋 Entering Main Menu");
}

fn enter_game_board() {
    info!("🎮 Entering Game Board");
}

fn enter_results() {
    info!("🏆 Entering Results");
}
```

**File: `src/plugins/assets.rs`**
```rust
use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    // Will be populated in future sprints
    pub loaded: bool,
}

impl Default for GameAssets {
    fn default() -> Self {
        Self { loaded: false }
    }
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut assets: ResMut<GameAssets>) {
    info!("📦 Loading game assets");
    // Asset loading will be implemented in Sprint 2-4
    assets.loaded = true;
}
```

**File: `src/plugins/input.rs`**
```rust
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputState {
    pub mouse_position: Vec2,
    pub left_click: bool,
    pub right_click: bool,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .add_systems(Update, update_input);
    }
}

fn update_input(
    mut input_state: ResMut<InputState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
) {
    // Update mouse position
    if let Ok(window) = windows.get_single() {
        if let Some(position) = window.cursor_position() {
            input_state.mouse_position = position;
        }
    }

    // Update button states
    input_state.left_click = mouse_button.just_pressed(MouseButton::Left);
    input_state.right_click = mouse_button.just_pressed(MouseButton::Right);
}
```

**Update `src/main.rs`:**
```rust
use bevy::prelude::*;

mod plugins;
use plugins::{CorePlugin, StatePlugin, AssetPlugin, InputPlugin};

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
        .add_plugins((
            CorePlugin,
            StatePlugin,
            AssetPlugin,
            InputPlugin,
        ))
        .run();
}
```

**Afternoon: Test and Validate**

```bash
# Test compilation
cargo check

# Run the application
cargo run

# Expected output:
# 🎮 TileMania Core initialized
# 📦 Loading game assets
# 📺 Entering Splash screen
# 📋 Entering Main Menu
```

**Deliverables (End of Day 6):**
- [ ] Plugin module structure created
- [ ] CorePlugin implemented
- [ ] StatePlugin with GameState enum
- [ ] AssetPlugin skeleton
- [ ] InputPlugin with basic mouse tracking
- [ ] Application runs with clean state transitions
- [ ] All plugins integrate cleanly

---

### 🗓️ Tuesday (Day 7) - 2025-10-11: State Transitions

**Objective:** Implement smooth state transitions with UI indicators

**Morning: Add UI for Each State**

Create `src/ui/mod.rs`:
```rust
pub mod splash;
pub mod main_menu;
pub mod game_board;
pub mod results;
```

**Task List:**
1. Create UI module structure
2. Add text labels for each state
3. Implement state transition buttons
4. Add keyboard shortcuts for transitions

**Afternoon: Polish Transitions**

1. Add fade-in/fade-out effects (using bevy_tweening)
2. Add transition timers
3. Test all state flows
4. Document state machine diagram

**Deliverables (End of Day 7):**
- [ ] UI placeholders for all states
- [ ] Clickable transitions between states
- [ ] Keyboard shortcuts (ESC, SPACE, etc.)
- [ ] State machine documented

---

### 🗓️ Wednesday (Day 8) - 2025-10-12: Asset Pipeline

**Objective:** Implement robust asset loading system

**Morning: Asset Loading Framework**

1. Define AssetCollection resource
2. Create asset manifest (RON format)
3. Implement async loading
4. Add loading progress indicator

**Afternoon: Hot-Reload Support**

1. Configure Bevy asset hot-reload
2. Test asset updates at runtime
3. Document asset pipeline

**Deliverables (End of Day 8):**
- [ ] Asset loading system functional
- [ ] Asset manifest (assets.ron)
- [ ] Loading screen with progress bar
- [ ] Hot-reload working in dev mode

---

### 🗓️ Thursday (Day 9) - 2025-10-13: Input System

**Objective:** Unified input abstraction layer

**Morning: Input Actions**

1. Define InputAction enum (Move, Select, Cancel, etc.)
2. Map keyboard to actions
3. Map mouse to actions
4. Map touch to actions (WASM prep)

**Afternoon: Input Testing**

1. Test keyboard navigation
2. Test mouse interaction
3. Create input test screen
4. Document input mappings

**Deliverables (End of Day 9):**
- [ ] InputAction enum complete
- [ ] All input devices mapped
- [ ] Touch support ready (for Sprint 7)
- [ ] Input test screen

---

### 🗓️ Friday (Day 10) - 2025-10-14: Integration & Testing

**Objective:** Integrate all Week 2 components and validate

**Morning: Integration Testing**

1. Test full state flow (Splash → MainMenu → GameBoard → Results → MainMenu)
2. Test asset loading across states
3. Test input in all states
4. Performance check (60fps maintained)

**Afternoon: Documentation & Week Review**

1. Update SPRINT_1_TRACKER.md
2. Create WEEK_2_COMPLETION_REPORT.md
3. Document any issues or learnings
4. Plan Sprint 2 Week 1

**Deliverables (End of Week 2):**
- [ ] All plugins integrated and tested
- [ ] State machine fully functional
- [ ] Asset pipeline working
- [ ] Input system complete
- [ ] 60fps maintained
- [ ] Week 2 documentation complete
- [ ] Sprint 1 COMPLETE ✅

---

## 📊 Success Metrics

### Week 2 Targets

**Architecture:**
- [x] Modular plugin design (4+ plugins)
- [ ] Clean separation of concerns
- [ ] Easy to extend and maintain

**State Management:**
- [ ] 5 states implemented (Splash, MainMenu, GameBoard, Results, Settings)
- [ ] Smooth transitions (<100ms)
- [ ] No state bugs or crashes

**Asset Pipeline:**
- [ ] Asset loading works
- [ ] Hot-reload functional in dev
- [ ] Loading progress displayed

**Input System:**
- [ ] Keyboard fully mapped
- [ ] Mouse interactions work
- [ ] Touch-ready (for future WASM)

**Performance:**
- [ ] 60fps maintained across all states
- [ ] No memory leaks
- [ ] Fast startup time (<3s)

---

## 🚦 Risk Assessment

### Current Risks

**🟢 LOW RISK:**
- Plugin architecture (well-documented Bevy pattern)
- State management (Bevy native feature)
- Input handling (straightforward implementation)

**🟡 MEDIUM RISK:**
- Asset hot-reload (configuration dependent)
- Touch input testing (no mobile device available - will defer to Sprint 7)

**🔴 HIGH RISK:**
- None identified

### Mitigation Strategies

**Asset hot-reload:**
- Follow Bevy 0.15 documentation closely
- Test with simple assets first
- Document configuration for team

**Touch input:**
- Design API now, test in Sprint 7 (WASM deployment)
- Ensure mouse events translate to touch events

---

## 🛠️ Development Guidelines

### Code Style
- Use Rust idioms (no unnecessary clones)
- Document all public APIs
- Write tests for core logic
- Keep functions small (<50 lines)

### Git Workflow
- Commit after each major feature
- Descriptive commit messages
- Push at end of each day

### Testing Strategy
- Manual testing during development
- Automated tests in Sprint 2+
- Performance profiling on Friday

---

## 📞 Communication

### Daily Check-ins (Self-Review)

**End of day:**
1. What did I complete today?
2. What's planned for tomorrow?
3. Any blockers or concerns?
4. Update SPRINT_1_TRACKER.md

### Documentation Updates

**After each major milestone:**
- Update architecture docs
- Create usage examples
- Note any design decisions

---

## 🎯 Sprint 2 Preview

**Focus:** UI Framework & Main Menu

**Key Tasks:**
- UI framework (bevy_ui components)
- Main menu (buttons, navigation)
- Settings screen (placeholder)
- Audio system integration

**Goal:** Fully functional main menu with navigation

---

## ✅ Pre-Week 2 Checklist

**Before starting Day 6:**
- [x] Week 1 complete and validated ✅
- [x] Decision Gate: GREEN ✅
- [x] Bevy dependencies compiled ✅
- [x] Disk space: 3.7GB free 🟡
- [x] Ready to code! 🚀

---

## 🚀 LET'S BUILD THE ARCHITECTURE!

**Week 2 Goal:** Solid foundation for rapid feature development

**Confidence:** 99% in success

**Focus:** Clean, modular, maintainable code

**Timeline:** 5 days to complete architecture

**Vision:** Build the engine that powers Scrabble mastery! 🏆

---

**Ready to start?** Let's begin with Day 6: Plugin Architecture! 💪

---

**Document Status:** ✅ Week 2 Ready
**Last Updated:** 2025-10-10
**Next Update:** Daily progress in SPRINT_1_TRACKER.md
