/// UI for Stage 3 (Classic Board)

use bevy::prelude::*;
use super::{Stage3State, Stage3Config, Turn};
use crate::plugins::state::GameState;

/// Marker for Stage 3 HUD
#[derive(Component)]
pub struct Stage3HUD;

/// HUD element types
#[derive(Component)]
pub enum HUDElement {
    PlayerScore,
    AIScore,
    Timer,
    TilesRemaining,
    TurnIndicator,
}

/// Marker for rack display
#[derive(Component)]
pub struct RackDisplay;

/// Spawn Stage 3 HUD
pub fn spawn_stage3_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Stage3Config>,
) {
    let font_medium = asset_server.load("fonts/FiraSans-Medium.ttf");
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Root HUD container
    commands
        .spawn((
            NodeBundle {
                style: Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(70.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.15, 0.9).into(),
                ..default()
            },
            Stage3HUD,
        ))
        .with_children(|parent| {
            // Left: Player score
            parent.spawn((
                Text::new("Player: 0"),
                TextFont {
                    font: font_bold.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                HUDElement::PlayerScore,
            ));

            // Center: Timer and turn indicator
            parent
                .spawn(NodeBundle {
                    style: Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|center| {
                    center.spawn((
                        Text::new(format!("Time: {}:00", config.time_limit_seconds / 60)),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        HUDElement::Timer,
                    ));

                    center.spawn((
                        Text::new("Your Turn"),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.3, 0.9, 0.3)),
                        HUDElement::TurnIndicator,
                    ));
                });

            // Right: AI score and tiles remaining
            parent
                .spawn(NodeBundle {
                    style: Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|right| {
                    right.spawn((
                        Text::new("AI: 0"),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        HUDElement::AIScore,
                    ));

                    right.spawn((
                        Text::new("Tiles: 100"),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        HUDElement::TilesRemaining,
                    ));
                });
        });

    // Spawn rack display at bottom
    spawn_rack_ui(&mut commands, &asset_server);
}

/// Spawn rack UI at bottom
fn spawn_rack_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Node {
                    width: Val::Px(560.0),
                    height: Val::Px(80.0),
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    left: Val::Percent(50.0),
                    margin: UiRect::left(Val::Px(-280.0)), // Center
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.15, 0.2, 0.9).into(),
                ..default()
            },
            RackDisplay,
        ))
        .with_children(|parent| {
            // Placeholder for 7 rack tiles (will be populated dynamically)
            for _ in 0..7 {
                parent.spawn(NodeBundle {
                    style: Node {
                        width: Val::Px(70.0),
                        height: Val::Px(70.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.7, 0.7, 0.6).into(),
                    ..default()
                });
            }
        });
}

/// Update HUD elements
pub fn update_stage3_hud(
    state: Res<Stage3State>,
    tile_bag: Res<super::board::TileBag>,
    mut hud_query: Query<(&HUDElement, &mut Text, Option<&mut TextColor>)>,
) {
    for (element, mut text, text_color) in hud_query.iter_mut() {
        match element {
            HUDElement::PlayerScore => {
                **text = format!("Player: {}", state.player_score);
            }
            HUDElement::AIScore => {
                **text = format!("AI: {}", state.ai_score);
            }
            HUDElement::Timer => {
                if state.time_remaining_ms > 0 {
                    let minutes = state.time_remaining_ms / 60000;
                    let seconds = (state.time_remaining_ms % 60000) / 1000;
                    **text = format!("Time: {}:{:02}", minutes, seconds);

                    // Color warning when time is low
                    if minutes == 0 && seconds < 60 {
                        if let Some(mut color) = text_color {
                            color.0 = Color::srgb(1.0, 0.3, 0.3);
                        }
                    }
                } else {
                    **text = "Time: 0:00".to_string();
                }
            }
            HUDElement::TilesRemaining => {
                **text = format!("Tiles: {}", tile_bag.count());
            }
            HUDElement::TurnIndicator => {}
        }
    }
}

/// Update rack display
pub fn update_rack_display(
    state: Res<Stage3State>,
    // Rack tile updates would go here
) {
    // Update visual representation of player's rack tiles
    // In full implementation, would update 7 tile sprites/text
}

/// Update turn indicator
pub fn update_turn_indicator(
    state: Res<Stage3State>,
    mut hud_query: Query<(&HUDElement, &mut Text, &mut TextColor)>,
) {
    for (element, mut text, mut text_color) in hud_query.iter_mut() {
        if matches!(element, HUDElement::TurnIndicator) {
            match state.current_turn {
                Turn::Player => {
                    **text = "Your Turn".to_string();
                    text_color.0 = Color::srgb(0.3, 0.9, 0.3);
                }
                Turn::AI => {
                    **text = "AI Thinking...".to_string();
                    text_color.0 = Color::srgb(0.9, 0.7, 0.3);
                }
            }
        }
    }
}
