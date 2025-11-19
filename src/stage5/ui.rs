/// UI for Stage 5 (AI Tournaments)

use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::ui::Style;
use super::{TournamentState, CurrentMatch};
use super::components::*;

#[derive(Component)]
pub struct TournamentBracket;

#[derive(Component)]
pub struct MatchDisplay;

#[derive(Component)]
pub struct OpponentInfo;

/// Spawn tournament bracket UI
pub fn spawn_tournament_bracket(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tournament_state: Res<TournamentState>,
) {
    // Spawn 8-player bracket visualization
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(800.0),
                height: Val::Px(600.0),
                position_type: PositionType::Absolute,
                right: Val::Px(20.0),
                top: Val::Px(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::srgba(0.1, 0.1, 0.15, 0.9).into(),
            ..default()
        },
        TournamentBracket,
    ));
}

/// Update bracket display
pub fn update_tournament_bracket(
    tournament_state: Res<TournamentState>,
) {
    // Update visual representation of bracket as matches complete
}

/// Update match display
pub fn update_match_display(
    current_match: Res<CurrentMatch>,
) {
    // Show current match score (best-of-3)
}

/// Update opponent info panel
pub fn update_opponent_info(
    current_match: Res<CurrentMatch>,
) {
    // Show opponent name, difficulty, personality
}
