/// Stage 1: Falling Letters - 2-letter word gameplay
///
/// Players form 2-letter words from falling tiles in a 7-column grid.
/// Features 5 difficulty levels, combo system, and power-ups.

use bevy::prelude::*;
use crate::lexicon::Lexicon;
use crate::scoring::ScoreCalculator;
use crate::plugins::state::GameState;

pub mod components;
pub mod systems;
pub mod difficulty;
pub mod visuals;
pub mod ui;
pub mod pause;
pub mod powerups;
pub mod audio;

use components::*;
use systems::*;
use visuals::*;
use ui::*;
use pause::*;
use powerups::*;
use audio::{AudioEvent, BackgroundMusic, play_audio_events};

/// Plugin for Stage 1 gameplay
pub struct Stage1Plugin;

impl Plugin for Stage1Plugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Stage1Config>()
            .init_resource::<Stage1State>()
            .init_resource::<ActivePowerUps>()
            .init_resource::<BackgroundMusic>()

            // Events
            .add_event::<AudioEvent>()

            // Systems
            .add_systems(Startup, setup_stage1)

            // Start screen (when entering from main menu)
            .add_systems(OnEnter(GameState::GameBoard), spawn_start_screen)
            .add_systems(Update, handle_difficulty_selection.run_if(in_state(GameState::GameBoard)))

            // Gameplay
            .add_systems(OnEnter(GameState::Stage1Playing), (spawn_stage1_hud, spawn_powerup_ui))
            // Core gameplay systems
            .add_systems(Update, (
                handle_pause_input,
                spawn_falling_tiles,
                update_falling_tiles,
                handle_tile_selection,
                handle_keyboard_tile_selection,
                validate_word,
                update_score_display,
                update_timer,
                check_game_over,
            ).run_if(in_state(GameState::Stage1Playing)))
            // Visual feedback systems
            .add_systems(Update, (
                update_tile_visuals,
                update_score_popups,
                update_validation_flash,
                update_combo_glow,
                update_particles,
            ).run_if(in_state(GameState::Stage1Playing)))
            // UI and power-up systems
            .add_systems(Update, (
                update_combo_display,
                update_word_display,
                spawn_powerup_pickups,
                collect_powerups,
                activate_powerups,
                update_powerup_timers,
                update_powerup_display,
                play_audio_events,
            ).run_if(in_state(GameState::Stage1Playing)))

            // Pause menu
            .add_systems(OnEnter(GameState::Stage1Paused), spawn_pause_menu)
            .add_systems(Update, (
                handle_pause_input,
                handle_pause_menu_buttons,
            ).run_if(in_state(GameState::Stage1Paused)))
            .add_systems(OnExit(GameState::Stage1Paused), despawn_pause_menu)

            // Results screen
            .add_systems(OnEnter(GameState::Results), spawn_results_screen);
    }
}

/// Configuration for Stage 1 gameplay
#[derive(Resource)]
pub struct Stage1Config {
    /// Current difficulty level (1-5)
    pub difficulty: u8,
    /// Total time allowed (milliseconds)
    pub total_time_ms: u32,
    /// Column count (always 7 for Stage 1)
    pub column_count: usize,
    /// Tile fall speed (pixels per second)
    pub fall_speed: f32,
    /// Words to learn (2-letter words from CSW24)
    pub two_letter_words: Vec<String>,
}

impl Default for Stage1Config {
    fn default() -> Self {
        Self {
            difficulty: 1,
            total_time_ms: 90_000, // 90 seconds for difficulty 1
            column_count: 7,
            fall_speed: 100.0,
            two_letter_words: Vec::new(),
        }
    }
}

/// Current game state for Stage 1
#[derive(Resource, Default)]
pub struct Stage1State {
    /// Current score
    pub score: u32,
    /// Time remaining (milliseconds)
    pub time_remaining_ms: u32,
    /// Current combo count
    pub combo_count: u32,
    /// Selected tiles
    pub selected_tiles: Vec<Entity>,
    /// Words found this session
    pub words_found: Vec<String>,
    /// Is game active
    pub is_active: bool,
}

/// Setup system for Stage 1
fn setup_stage1(
    mut commands: Commands,
    mut config: ResMut<Stage1Config>,
) {
    // Load lexicon and get 2-letter words (try multiple sources)
    match Lexicon::load_default() {
        Ok(lexicon) => {
            config.two_letter_words = lexicon.get_two_letter_words();
            info!("Loaded {} two-letter words for Stage 1 from {} lexicon",
                  config.two_letter_words.len(), lexicon.lexicon_name);

            // Store lexicon as resource for word validation
            commands.insert_resource(lexicon);
        }
        Err(e) => {
            error!("Failed to load lexicon: {}", e);
            error!("Stage 1 will not function correctly without a word list!");
        }
    }

    // Spawn game board
    commands.spawn((
        GameBoard {
            columns: config.column_count,
        },
        SpatialBundle::default(),
    ));
}
