/// Stage 3: Classic Board
///
/// Traditional word tile gameplay on a 15×15 board with AI opponent.
/// Features premium squares (DW, TW, DL, TL), 7-tile rack, and turn-based play.

use bevy::prelude::*;
use crate::plugins::state::GameState;

pub mod components;
pub mod difficulty;
pub mod systems;
pub mod ai;
pub mod board;
pub mod ui;
pub mod visuals;
pub mod pause;
pub mod audio;

use components::*;
use systems::*;
use ai::*;
use board::*;

/// Stage 3 Plugin
pub struct Stage3Plugin;

impl Plugin for Stage3Plugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<Stage3Config>()
            .init_resource::<Stage3State>()
            .init_resource::<Board>()
            .init_resource::<TileBag>()

            // Events
            .add_event::<audio::AudioEvent>()
            .add_event::<MoveEvent>()

            // Startup systems
            .add_systems(OnEnter(GameState::Stage3Playing), (
                ui::spawn_stage3_hud,
                spawn_board,
                initialize_game,
                deal_initial_tiles,
            ))

            // Core gameplay systems
            .add_systems(Update, (
                handle_player_input,
                validate_player_move,
                execute_move,
                ai::calculate_ai_move,
                ai::execute_ai_move,
                update_turn,
                check_game_over,
                update_timer,
            ).run_if(in_state(GameState::Stage3Playing)))

            // Visual systems
            .add_systems(Update, (
                visuals::update_tile_visuals,
                visuals::update_board_highlights,
                visuals::update_score_popups,
                visuals::update_move_preview,
            ).run_if(in_state(GameState::Stage3Playing)))

            // UI systems
            .add_systems(Update, (
                ui::update_stage3_hud,
                ui::update_rack_display,
                ui::update_turn_indicator,
            ).run_if(in_state(GameState::Stage3Playing)))

            // Pause systems
            .add_systems(Update, pause::handle_pause_input.run_if(in_state(GameState::Stage3Playing)))
            .add_systems(OnEnter(GameState::Stage3Paused), pause::spawn_pause_menu)
            .add_systems(Update, pause::handle_pause_menu_buttons.run_if(in_state(GameState::Stage3Paused)))
            .add_systems(OnExit(GameState::Stage3Paused), pause::cleanup_pause_menu)

            // Audio systems
            .add_systems(Update, (
                audio::play_audio_events,
                audio::update_background_music,
            ).run_if(in_state(GameState::Stage3Playing)))

            // Cleanup
            .add_systems(OnExit(GameState::Stage3Playing), cleanup_stage3);
    }
}

/// Configuration for Stage 3
#[derive(Resource)]
pub struct Stage3Config {
    pub difficulty: u8,
    pub time_limit_seconds: u32, // 0 = unlimited
    pub ai_think_time_ms: u32,   // Simulated AI delay
    pub allow_hints: bool,
}

impl Default for Stage3Config {
    fn default() -> Self {
        Self {
            difficulty: 3,
            time_limit_seconds: 1800, // 30 minutes default
            ai_think_time_ms: 2000,
            allow_hints: true,
        }
    }
}

/// Game state for Stage 3
#[derive(Resource)]
pub struct Stage3State {
    pub player_score: u32,
    pub ai_score: u32,
    pub time_remaining_ms: u32,
    pub current_turn: Turn,
    pub player_rack: Vec<char>,
    pub ai_rack: Vec<char>,
    pub moves_history: Vec<MoveRecord>,
    pub is_active: bool,
    pub game_over_reason: Option<GameOverReason>,
}

impl Default for Stage3State {
    fn default() -> Self {
        Self {
            player_score: 0,
            ai_score: 0,
            time_remaining_ms: 1800_000,
            current_turn: Turn::Player,
            player_rack: Vec::new(),
            ai_rack: Vec::new(),
            moves_history: Vec::new(),
            is_active: true,
            game_over_reason: None,
        }
    }
}

/// Whose turn it is
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Turn {
    Player,
    AI,
}

/// Move event
#[derive(Event)]
pub struct MoveEvent {
    pub player: Turn,
    pub word: String,
    pub position: (usize, usize),
    pub direction: Direction,
    pub score: u32,
}

/// Direction for word placement
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

/// Record of a move
#[derive(Clone)]
pub struct MoveRecord {
    pub turn_number: u32,
    pub player: Turn,
    pub word: String,
    pub score: u32,
    pub position: (usize, usize),
    pub direction: Direction,
}

/// Reason for game over
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameOverReason {
    TimeExpired,
    NoMovesAvailable,
    TileBagEmpty,
    PlayerQuit,
}

/// Initialize game
fn initialize_game(
    mut state: ResMut<Stage3State>,
    mut board: ResMut<Board>,
    mut tile_bag: ResMut<TileBag>,
) {
    // Reset game state
    *state = Stage3State::default();
    board.clear();
    tile_bag.reset();
}

/// Deal initial tiles to both players
fn deal_initial_tiles(
    mut state: ResMut<Stage3State>,
    mut tile_bag: ResMut<TileBag>,
) {
    // Deal 7 tiles to player
    state.player_rack = tile_bag.draw_tiles(7);

    // Deal 7 tiles to AI
    state.ai_rack = tile_bag.draw_tiles(7);
}

/// Cleanup Stage 3 entities
fn cleanup_stage3(
    mut commands: Commands,
    entities: Query<Entity, Or<(
        With<BoardTile>,
        With<RackTile>,
        With<ui::Stage3HUD>,
    )>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
