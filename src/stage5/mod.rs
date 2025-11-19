/// Stage 5: AI Tournaments
///
/// 8-player elimination tournament bracket. Player faces AI opponents with unique personalities.
/// Win best-of-3 matches to progress through quarterfinals, semifinals, and finals.

use bevy::prelude::*;
use crate::plugins::state::GameState;

pub mod components;
pub mod tournament;
pub mod ai_personality;
pub mod systems;
pub mod ui;
pub mod visuals;
pub mod pause;
pub mod audio;

use components::*;
use tournament::*;
use systems::*;

/// Stage 5 Plugin
pub struct Stage5Plugin;

impl Plugin for Stage5Plugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .init_resource::<TournamentState>()
            .init_resource::<CurrentMatch>()

            // Events
            .add_event::<audio::AudioEvent>()
            .add_event::<MatchResult>()

            // Startup
            .add_systems(OnEnter(GameState::Stage5Playing), (
                initialize_tournament,
                ui::spawn_tournament_bracket,
                start_next_match,
            ))

            // Core gameplay
            .add_systems(Update, (
                handle_match_gameplay,
                process_match_result,
                advance_tournament,
                check_tournament_complete,
            ).run_if(in_state(GameState::Stage5Playing)))

            // UI systems
            .add_systems(Update, (
                ui::update_tournament_bracket,
                ui::update_match_display,
                ui::update_opponent_info,
            ).run_if(in_state(GameState::Stage5Playing)))

            // Visual systems
            .add_systems(Update, (
                visuals::update_bracket_animations,
                visuals::update_victory_effects,
            ).run_if(in_state(GameState::Stage5Playing)))

            // Pause
            .add_systems(Update, pause::handle_pause_input.run_if(in_state(GameState::Stage5Playing)))
            .add_systems(OnEnter(GameState::Stage5Paused), pause::spawn_pause_menu)
            .add_systems(Update, pause::handle_pause_menu_buttons.run_if(in_state(GameState::Stage5Paused)))
            .add_systems(OnExit(GameState::Stage5Paused), pause::cleanup_pause_menu)

            // Audio
            .add_systems(Update, (
                audio::play_audio_events,
                audio::update_background_music,
            ).run_if(in_state(GameState::Stage5Playing)))

            // Cleanup
            .add_systems(OnExit(GameState::Stage5Playing), cleanup_stage5);
    }
}

/// Match result event
#[derive(Event)]
pub struct MatchResult {
    pub player_won: bool,
    pub player_score: u32,
    pub opponent_score: u32,
}

/// Initialize tournament
fn initialize_tournament(
    mut tournament_state: ResMut<TournamentState>,
) {
    *tournament_state = TournamentState::new();
}

/// Cleanup Stage 5
fn cleanup_stage5(
    mut commands: Commands,
    entities: Query<Entity, Or<(
        With<ui::TournamentBracket>,
        With<ui::MatchDisplay>,
    )>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
