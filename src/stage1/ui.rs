/// UI elements for Stage 1 (HUD, start screen, results)

use bevy::prelude::*;
use super::components::*;
use super::{Stage1Config, Stage1State};
use super::difficulty::{get_difficulty, DIFFICULTY_LEVELS};
use crate::plugins::state::GameState;

/// Spawns the in-game HUD (score, timer, combo)
pub fn spawn_stage1_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Root HUD container
    commands
        .spawn(NodeBundle {
            style: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top bar (score, timer, combo)
            parent
                .spawn(NodeBundle {
                    style: Node {
                        width: Val::Percent(100.0),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|top_bar| {
                    // Score display (top-left)
                    top_bar.spawn((
                        Text::new("Score: 0"),
                        TextFont {
                            font: font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ScoreDisplay,
                    ));

                    // Timer display (top-center)
                    top_bar.spawn((
                        Text::new("Time: 90s"),
                        TextFont {
                            font: font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TimerDisplay,
                    ));

                    // Combo display (top-right)
                    top_bar.spawn((
                        Text::new("Combo: 0x"),
                        TextFont {
                            font: font.clone(),
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.9, 0.4)),
                        ComboDisplay,
                    ));
                });

            // Current word display (bottom-center)
            parent
                .spawn(NodeBundle {
                    style: Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(100.0),
                        left: Val::Percent(50.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|bottom| {
                    bottom.spawn((
                        Text::new(""),
                        TextFont {
                            font: font.clone(),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 0.5)),
                        Node {
                            position_type: PositionType::Relative,
                            left: Val::Px(-100.0), // Center adjustment
                            ..default()
                        },
                        WordDisplay,
                    ));
                });
        });
}

/// Updates combo display with current multiplier
pub fn update_combo_display(
    mut query: Query<&mut Text, With<ComboDisplay>>,
    state: Res<Stage1State>,
) {
    if !state.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        let multiplier = if state.combo_count == 0 {
            1.0
        } else {
            (1.0 + (state.combo_count as f32 * 0.5)).min(3.0)
        };

        text.sections[0].value = if state.combo_count > 0 {
            format!("Combo: {}x ({:.1}x)", state.combo_count, multiplier)
        } else {
            "Combo: 0x".to_string()
        };

        // Color based on combo level
        text.sections[0].style.color = match state.combo_count {
            0 => Color::srgb(0.7, 0.7, 0.7),
            1 => Color::srgb(1.0, 1.0, 1.0),
            2 => Color::srgb(0.5, 0.9, 1.0),
            3 => Color::srgb(0.8, 0.5, 1.0),
            4 => Color::srgb(1.0, 0.7, 0.9),
            _ => Color::srgb(1.0, 0.9, 0.3),
        };
    }
}

/// Updates current word display based on selected tiles
pub fn update_word_display(
    mut word_query: Query<&mut Text, With<WordDisplay>>,
    state: Res<Stage1State>,
    tile_query: Query<&FallingTile>,
) {
    if !state.is_changed() {
        return;
    }

    for mut text in word_query.iter_mut() {
        let mut word = String::new();
        for entity in &state.selected_tiles {
            if let Ok(tile) = tile_query.get(*entity) {
                word.push(tile.letter);
            }
        }

        text.sections[0].value = if word.is_empty() {
            "Select 2 tiles...".to_string()
        } else {
            word
        };
    }
}

/// Marker component for start screen
#[derive(Component)]
pub struct StartScreen;

/// Marker component for difficulty button
#[derive(Component)]
pub struct DifficultyButton(pub u8);

/// Spawns Stage 1 start screen with difficulty selection
pub fn spawn_start_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.1, 0.15).into(),
                ..default()
            },
            StartScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Stage 1: Falling Letters"),
                TextFont {
                    font: font.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Form 2-letter words!"),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Difficulty buttons
            parent.spawn((
                Text::new("Select Difficulty:"),
                TextFont {
                    font: font.clone(),
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            for diff in &DIFFICULTY_LEVELS {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Node {
                                width: Val::Px(300.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.3, 0.4).into(),
                            ..default()
                        },
                        DifficultyButton(diff.level),
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new(format!(
                                "D{}: {} ({}s)",
                                diff.level, diff.name, diff.total_time_seconds
                            )),
                            TextFont {
                                font: font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }

            // Instructions
            parent.spawn((
                Text::new("Click tiles to select • Press SPACE to submit"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
            ));
        });
}

/// Handles difficulty button clicks
pub fn handle_difficulty_selection(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &DifficultyButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut config: ResMut<Stage1Config>,
    mut state: ResMut<Stage1State>,
    mut next_state: ResMut<NextState<GameState>>,
    start_screen_query: Query<Entity, With<StartScreen>>,
) {
    for (interaction, button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                info!("Starting Stage 1 at difficulty {}", button.0);

                // Set difficulty
                let difficulty = get_difficulty(button.0);
                config.difficulty = difficulty.level;
                config.total_time_ms = difficulty.total_time_seconds * 1000;
                config.fall_speed = difficulty.fall_speed;

                // Initialize game state
                state.score = 0;
                state.time_remaining_ms = config.total_time_ms;
                state.combo_count = 0;
                state.selected_tiles.clear();
                state.words_found.clear();
                state.is_active = true;

                // Despawn start screen
                for entity in start_screen_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Transition to gameplay
                next_state.set(GameState::Stage1Playing);
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.3, 0.4, 0.5).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.2, 0.3, 0.4).into();
            }
        }
    }
}

/// Marker component for results screen
#[derive(Component)]
pub struct ResultsScreen;

/// Spawns results screen showing final score and words found
pub fn spawn_results_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<Stage1State>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.1, 0.15).into(),
                ..default()
            },
            ResultsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Time's Up!"),
                TextFont {
                    font: font.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Final score
            parent.spawn((
                Text::new(format!("Final Score: {}", state.score)),
                TextFont {
                    font: font.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.3)),
            ));

            // Words found
            parent.spawn((
                Text::new(format!("Words Found: {}", state.words_found.len())),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Word list (first 10)
            let word_list = state
                .words_found
                .iter()
                .take(10)
                .cloned()
                .collect::<Vec<_>>()
                .join(", ");

            if !word_list.is_empty() {
                parent.spawn((
                    Text::new(word_list),
                    TextFont {
                        font: font.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.8, 0.8, 0.8)),
                ));
            }

            // Play again button
            parent
                .spawn(ButtonBundle {
                    style: Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(40.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn((
                        Text::new("Play Again"),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Back to menu button
            parent
                .spawn(ButtonBundle {
                    style: Node {
                        width: Val::Px(200.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.4, 0.4, 0.5).into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn((
                        Text::new("Main Menu"),
                        TextFont {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}
