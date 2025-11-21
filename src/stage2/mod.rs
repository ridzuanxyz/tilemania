/// Stage 2: Tile Matching - 3-4 letter word matching game
///
/// Match-3 style gameplay where players swap tiles to form 3-4 letter words.
/// Words are cleared and new tiles cascade down from the top.

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
pub mod audio;

use components::*;
use systems::*;
use visuals::*;
use ui::*;
use pause::*;
use audio::*;

/// Plugin for Stage 2 gameplay
pub struct Stage2Plugin;

impl Plugin for Stage2Plugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Stage2Config>()
            .init_resource::<Stage2State>()
            .init_resource::<BackgroundMusic>()
            .init_resource::<SoundEffects>()

            // Events
            .add_event::<AudioEvent>()

            // Systems
            .add_systems(Startup, setup_stage2)

            // Start screen
            .add_systems(OnEnter(GameState::Stage2Start), spawn_start_screen)
            .add_systems(Update, handle_difficulty_selection.run_if(in_state(GameState::Stage2Start)))

            // Gameplay (Stage2Playing state to be added)
            .add_systems(OnEnter(GameState::Stage2Playing), spawn_stage2_hud)
            .add_systems(Update, (
                // Pause handling
                handle_pause_input,
                // Core gameplay
                detect_tile_hover,
                handle_tile_selection,
                handle_tile_swap,
                find_word_matches,
                clear_matched_words,
                cascade_tiles,
                spawn_new_tiles,
                check_game_over,
                // Visual feedback
                update_tile_visuals,
                update_match_animations,
                update_cascade_animations,
                update_score_popups,
                update_particles,
                // UI
                update_score_display,
                update_timer,
                update_moves_display,
                // Audio
                play_audio_events,
            ).run_if(in_state(GameState::Stage2Playing)))

            // Pause menu
            .add_systems(OnEnter(GameState::Stage2Paused), spawn_pause_menu)
            .add_systems(Update, (
                handle_pause_input,
                handle_pause_menu_buttons,
            ).run_if(in_state(GameState::Stage2Paused)))
            .add_systems(OnExit(GameState::Stage2Paused), despawn_pause_menu)

            // Results screen
            .add_systems(OnEnter(GameState::Results), spawn_results_screen);
    }
}

/// Configuration for Stage 2 gameplay
#[derive(Resource)]
pub struct Stage2Config {
    /// Current difficulty level (1-5)
    pub difficulty: u8,
    /// Grid size (8x8)
    pub grid_size: usize,
    /// Time limit (seconds)
    pub time_limit_seconds: u32,
    /// Moves available (0 = unlimited)
    pub moves_limit: u32,
    /// Target score to win
    pub target_score: u32,
    /// 3-4 letter words from CSW24
    pub three_letter_words: Vec<String>,
    pub four_letter_words: Vec<String>,
}

impl Default for Stage2Config {
    fn default() -> Self {
        Self {
            difficulty: 1,
            grid_size: 8,
            time_limit_seconds: 120, // 2 minutes for D1
            moves_limit: 0, // Unlimited moves
            target_score: 500, // D1 target
            three_letter_words: Vec::new(),
            four_letter_words: Vec::new(),
        }
    }
}

/// Current game state for Stage 2
#[derive(Resource, Default)]
pub struct Stage2State {
    /// Current score
    pub score: u32,
    /// Time remaining (milliseconds)
    pub time_remaining_ms: u32,
    /// Moves made
    pub moves_made: u32,
    /// Words found this session
    pub words_found: Vec<String>,
    /// Currently selected tile
    pub selected_tile: Option<Entity>,
    /// Is game active
    pub is_active: bool,
    /// Combo counter
    pub combo_count: u32,
}

/// Setup system for Stage 2
fn setup_stage2(
    mut commands: Commands,
    mut config: ResMut<Stage2Config>,
) {
    // Load lexicon and get 3-4 letter words
    match Lexicon::load_from_file("CSW24.txt") {
        Ok(lexicon) => {
            config.three_letter_words = lexicon.get_words_by_length(3);
            config.four_letter_words = lexicon.get_words_by_length(4);
            info!(
                "Loaded {} 3-letter words and {} 4-letter words for Stage 2",
                config.three_letter_words.len(),
                config.four_letter_words.len()
            );
        }
        Err(e) => {
            error!("Failed to load lexicon: {}", e);
        }
    }

    // Grid will be spawned when gameplay starts
}
