/// Stage 4: Speed Challenge
///
/// Fast-paced word formation with time pressure and streak multipliers.
/// 7-tile rack refreshes after each word. Score as many points as possible before time runs out.

use bevy::prelude::*;
use crate::plugins::state::GameState;

pub mod components;
pub mod difficulty;
pub mod systems;
pub mod ui;
pub mod visuals;
pub mod pause;
pub mod audio;

use components::*;
use systems::*;

/// Stage 4 Plugin
pub struct Stage4Plugin;

impl Plugin for Stage4Plugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Stage4Config>()
            .init_resource::<Stage4State>()
            .init_resource::<TilePool>()

            // Events
            .add_event::<audio::AudioEvent>()
            .add_event::<WordEvent>()

            // Startup
            .add_systems(OnEnter(GameState::Stage4Playing), (
                ui::spawn_stage4_hud,
                initialize_game,
                deal_initial_rack,
            ))

            // Core gameplay
            .add_systems(Update, (
                handle_tile_selection,
                handle_word_submission,
                validate_word,
                score_word,
                update_streak,
                refresh_rack,
                update_timer,
                check_game_over,
                update_panic_mode,
            ).run_if(in_state(GameState::Stage4Playing)))

            // Visual systems
            .add_systems(Update, (
                visuals::update_tile_visuals,
                visuals::update_streak_display,
                visuals::update_timer_visuals,
                visuals::update_score_popups,
                visuals::update_particles,
            ).run_if(in_state(GameState::Stage4Playing)))

            // UI systems
            .add_systems(Update, (
                ui::update_stage4_hud,
                ui::update_rack_display,
            ).run_if(in_state(GameState::Stage4Playing)))

            // Pause
            .add_systems(Update, pause::handle_pause_input.run_if(in_state(GameState::Stage4Playing)))
            .add_systems(OnEnter(GameState::Stage4Paused), pause::spawn_pause_menu)
            .add_systems(Update, pause::handle_pause_menu_buttons.run_if(in_state(GameState::Stage4Paused)))
            .add_systems(OnExit(GameState::Stage4Paused), pause::cleanup_pause_menu)

            // Audio
            .add_systems(Update, (
                audio::play_audio_events,
                audio::update_background_music,
            ).run_if(in_state(GameState::Stage4Playing)))

            // Cleanup
            .add_systems(OnExit(GameState::Stage4Playing), cleanup_stage4);
    }
}

/// Configuration for Stage 4
#[derive(Resource)]
pub struct Stage4Config {
    pub difficulty: u8,
    pub time_limit_seconds: u32,
    pub target_score: u32,
    pub streak_bonus_multiplier: f32,
}

impl Default for Stage4Config {
    fn default() -> Self {
        Self {
            difficulty: 3,
            time_limit_seconds: 90,
            target_score: 500,
            streak_bonus_multiplier: 1.1,
        }
    }
}

/// Game state for Stage 4
#[derive(Resource)]
pub struct Stage4State {
    pub score: u32,
    pub time_remaining_ms: u32,
    pub words_formed: u32,
    pub current_streak: u32,
    pub best_streak: u32,
    pub rack: Vec<char>,
    pub selected_indices: Vec<usize>,
    pub is_active: bool,
    pub is_panic_mode: bool, // Activated when time < 10s
    pub words_history: Vec<String>,
}

impl Default for Stage4State {
    fn default() -> Self {
        Self {
            score: 0,
            time_remaining_ms: 90_000,
            words_formed: 0,
            current_streak: 0,
            best_streak: 0,
            rack: Vec::new(),
            selected_indices: Vec::new(),
            is_active: true,
            is_panic_mode: false,
            words_history: Vec::new(),
        }
    }
}

/// Tile pool for drawing random tiles
#[derive(Resource, Default)]
pub struct TilePool {
    pub tiles: Vec<char>,
}

impl TilePool {
    /// Create weighted random tile pool
    pub fn new() -> Self {
        Self {
            tiles: Self::create_distribution(),
        }
    }

    fn create_distribution() -> Vec<char> {
        // Weighted distribution favoring common letters
        let distribution = [
            ('A', 9), ('B', 2), ('C', 2), ('D', 4), ('E', 12),
            ('F', 2), ('G', 3), ('H', 2), ('I', 9), ('J', 1),
            ('K', 1), ('L', 4), ('M', 2), ('N', 6), ('O', 8),
            ('P', 2), ('Q', 1), ('R', 6), ('S', 4), ('T', 6),
            ('U', 4), ('V', 2), ('W', 2), ('X', 1), ('Y', 2),
            ('Z', 1),
        ];

        let mut tiles = Vec::new();
        for (letter, count) in &distribution {
            for _ in 0..*count {
                tiles.push(*letter);
            }
        }

        tiles
    }

    /// Draw N random tiles
    pub fn draw(&self, count: usize) -> Vec<char> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();

        self.tiles
            .choose_multiple(&mut rng, count)
            .copied()
            .collect()
    }
}

/// Word formation event
#[derive(Event)]
pub struct WordEvent {
    pub word: String,
    pub score: u32,
    pub streak_multiplier: f32,
}

/// Initialize game
fn initialize_game(
    mut state: ResMut<Stage4State>,
    mut tile_pool: ResMut<TilePool>,
) {
    *state = Stage4State::default();
    *tile_pool = TilePool::new();
}

/// Deal initial rack
fn deal_initial_rack(
    mut state: ResMut<Stage4State>,
    tile_pool: Res<TilePool>,
) {
    state.rack = tile_pool.draw(7);
}

/// Cleanup Stage 4
fn cleanup_stage4(
    mut commands: Commands,
    entities: Query<Entity, Or<(
        With<RackTile>,
        With<ui::Stage4HUD>,
    )>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
