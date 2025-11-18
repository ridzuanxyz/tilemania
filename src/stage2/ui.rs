/// UI for Stage 2 (Match-3 gameplay)

use bevy::prelude::*;
use super::{Stage2Config, Stage2State};
use super::difficulty::*;
use crate::plugins::state::GameState;

/// Marker for Stage 2 start screen
#[derive(Component)]
pub struct Stage2StartScreen;

/// Marker for difficulty selection buttons
#[derive(Component)]
pub struct DifficultyButton {
    pub level: u8,
}

/// Marker for Stage 2 HUD
#[derive(Component)]
pub struct Stage2HUD;

/// Marker for HUD text elements
#[derive(Component)]
pub enum HUDElement {
    Score,
    Timer,
    Moves,
    Target,
    ComboCounter,
}

/// Marker for results screen
#[derive(Component)]
pub struct Stage2ResultsScreen;

/// Marker for results screen buttons
#[derive(Component)]
pub enum ResultsButton {
    PlayAgain,
    MainMenu,
}

/// Spawn start screen with difficulty selection
pub fn spawn_start_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let title_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::srgb(0.9, 0.9, 1.0),
    };

    let subtitle_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 24.0,
        color: Color::srgb(0.7, 0.7, 0.8),
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 28.0,
        color: Color::srgb(0.1, 0.1, 0.15),
    };

    let description_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 18.0,
        color: Color::srgb(0.5, 0.5, 0.6),
    };

    // Root container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgb(0.1, 0.1, 0.15).into(),
                ..default()
            },
            Stage2StartScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section("STAGE 2: TILE MATCHING", title_style),
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });

            // Subtitle
            parent.spawn(TextBundle {
                text: Text::from_section("Match 3-4 letter words in the grid!", subtitle_style),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            });

            // Difficulty selection
            parent.spawn(TextBundle {
                text: Text::from_section("Select Difficulty:", subtitle_style.clone()),
                style: Style {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
                ..default()
            });

            // Difficulty buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons| {
                    for difficulty in &DIFFICULTY_LEVELS {
                        buttons
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Px(500.0),
                                        height: Val::Px(80.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        ..default()
                                    },
                                    background_color: get_difficulty_color(difficulty.level).into(),
                                    ..default()
                                },
                                DifficultyButton {
                                    level: difficulty.level,
                                },
                            ))
                            .with_children(|button| {
                                button.spawn(
                                    TextBundle::from_section(
                                        format!("D{}: {}", difficulty.level, difficulty.name),
                                        button_text_style.clone(),
                                    )
                                    .with_style(Style {
                                        flex_grow: 1.0,
                                        ..default()
                                    }),
                                );

                                button.spawn(
                                    TextBundle::from_section(
                                        format!(
                                            "{}s | Target: {} | Moves: {}",
                                            difficulty.time_limit_seconds,
                                            difficulty.target_score,
                                            if difficulty.moves_limit == 0 { "∞".to_string() } else { difficulty.moves_limit.to_string() }
                                        ),
                                        description_style.clone(),
                                    ),
                                );
                            });
                    }
                });
        });
}

/// Handle difficulty selection button clicks
pub fn handle_difficulty_selection(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &DifficultyButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    start_screen_query: Query<Entity, With<Stage2StartScreen>>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Initialize game with selected difficulty
                let difficulty = get_difficulty_config(button.level);

                commands.insert_resource(Stage2Config {
                    difficulty: button.level,
                    grid_size: 8,
                    time_limit_seconds: difficulty.time_limit_seconds,
                    moves_limit: difficulty.moves_limit,
                    target_score: difficulty.target_score,
                    three_letter_words: vec![], // Will be loaded from lexicon
                    four_letter_words: vec![],
                });

                commands.insert_resource(Stage2State {
                    score: 0,
                    time_remaining_ms: difficulty.time_limit_seconds * 1000,
                    moves_made: 0,
                    words_found: vec![],
                    selected_tile: None,
                    combo_count: 0,
                    is_active: true,
                });

                // Cleanup start screen
                for entity in start_screen_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Transition to gameplay
                next_state.set(GameState::Stage2Playing);
            }
            Interaction::Hovered => {
                *color = lighten_color(get_difficulty_color(button.level)).into();
            }
            Interaction::None => {
                *color = get_difficulty_color(button.level).into();
            }
        }
    }
}

/// Spawn Stage 2 HUD (in-game UI)
pub fn spawn_stage2_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<Stage2Config>,
) {
    let label_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 20.0,
        color: Color::srgb(0.7, 0.7, 0.8),
    };

    let value_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };

    // Root HUD container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.15, 0.9).into(),
                ..default()
            },
            Stage2HUD,
        ))
        .with_children(|parent| {
            // Left section: Score and Target
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|section| {
                    section.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Score: ", label_style.clone()),
                            TextSection::new("0", value_style.clone()),
                            TextSection::new(&format!(" / {}", config.target_score), label_style.clone()),
                        ]),
                        HUDElement::Score,
                    ));
                });

            // Center section: Timer
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|section| {
                    section.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Time: ", label_style.clone()),
                            TextSection::new(
                                &format!("{}s", config.time_limit_seconds),
                                value_style.clone(),
                            ),
                        ]),
                        HUDElement::Timer,
                    ));
                });

            // Right section: Moves and Combo
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd,
                        row_gap: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|section| {
                    section.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Moves: ", label_style.clone()),
                            TextSection::new("0", value_style.clone()),
                            TextSection::new(
                                if config.moves_limit == 0 {
                                    "".to_string()
                                } else {
                                    format!(" / {}", config.moves_limit)
                                },
                                label_style.clone(),
                            ),
                        ]),
                        HUDElement::Moves,
                    ));

                    section.spawn((
                        TextBundle::from_sections([
                            TextSection::new("Combo: x", label_style.clone()),
                            TextSection::new("1", value_style.clone()),
                        ]),
                        HUDElement::ComboCounter,
                    ));
                });
        });
}

/// Update HUD elements
pub fn update_stage2_hud(
    state: Res<Stage2State>,
    config: Res<Stage2Config>,
    mut hud_query: Query<(&HUDElement, &mut Text)>,
) {
    for (element, mut text) in hud_query.iter_mut() {
        match element {
            HUDElement::Score => {
                text.sections[1].value = state.score.to_string();
            }
            HUDElement::Timer => {
                let seconds = state.time_remaining_ms / 1000;
                text.sections[1].value = format!("{}s", seconds);

                // Change color when time is low
                if seconds < 10 {
                    text.sections[1].style.color = Color::srgb(1.0, 0.3, 0.3);
                } else if seconds < 30 {
                    text.sections[1].style.color = Color::srgb(1.0, 0.8, 0.3);
                } else {
                    text.sections[1].style.color = Color::srgb(1.0, 1.0, 1.0);
                }
            }
            HUDElement::Moves => {
                text.sections[1].value = state.moves_made.to_string();
            }
            HUDElement::ComboCounter => {
                text.sections[1].value = state.combo_count.to_string();

                // Change color based on combo
                if state.combo_count >= 5 {
                    text.sections[1].style.color = Color::srgb(1.0, 0.3, 1.0); // Purple
                } else if state.combo_count >= 3 {
                    text.sections[1].style.color = Color::srgb(1.0, 0.8, 0.3); // Gold
                } else {
                    text.sections[1].style.color = Color::srgb(1.0, 1.0, 1.0); // White
                }
            }
            HUDElement::Target => {}
        }
    }
}

/// Spawn results screen with final stats
pub fn spawn_results_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<Stage2State>,
    config: Res<Stage2Config>,
) {
    let title_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 56.0,
        color: if state.score >= config.target_score {
            Color::srgb(0.3, 0.9, 0.3) // Success green
        } else {
            Color::srgb(0.9, 0.9, 1.0) // Normal white
        },
    };

    let stat_label_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 22.0,
        color: Color::srgb(0.7, 0.7, 0.8),
    };

    let stat_value_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 28.0,
        color: Color::srgb(0.1, 0.1, 0.15),
    };

    // Root container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.15, 0.95).into(),
                ..default()
            },
            Stage2ResultsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle {
                text: Text::from_section(
                    if state.score >= config.target_score {
                        "LEVEL COMPLETE!"
                    } else {
                        "TIME'S UP!"
                    },
                    title_style,
                ),
                style: Style {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
                ..default()
            });

            // Stats container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(15.0),
                        margin: UiRect::bottom(Val::Px(40.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|stats| {
                    stats.spawn(TextBundle::from_sections([
                        TextSection::new("Final Score: ", stat_label_style.clone()),
                        TextSection::new(&state.score.to_string(), stat_value_style.clone()),
                    ]));

                    stats.spawn(TextBundle::from_sections([
                        TextSection::new("Words Found: ", stat_label_style.clone()),
                        TextSection::new(&state.words_found.len().to_string(), stat_value_style.clone()),
                    ]));

                    stats.spawn(TextBundle::from_sections([
                        TextSection::new("Moves Made: ", stat_label_style.clone()),
                        TextSection::new(&state.moves_made.to_string(), stat_value_style.clone()),
                    ]));

                    stats.spawn(TextBundle::from_sections([
                        TextSection::new("Max Combo: ", stat_label_style.clone()),
                        TextSection::new(&state.combo_count.to_string(), stat_value_style.clone()),
                    ]));
                });

            // Buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons| {
                    // Play Again button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.7, 0.3).into(),
                                ..default()
                            },
                            ResultsButton::PlayAgain,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Play Again",
                                button_text_style.clone(),
                            ));
                        });

                    // Main Menu button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(200.0),
                                    height: Val::Px(60.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.5, 0.5, 0.6).into(),
                                ..default()
                            },
                            ResultsButton::MainMenu,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Main Menu",
                                button_text_style.clone(),
                            ));
                        });
                });
        });
}

/// Handle results screen button clicks
pub fn handle_results_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ResultsButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    results_screen_query: Query<Entity, With<Stage2ResultsScreen>>,
    hud_query: Query<Entity, With<Stage2HUD>>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Cleanup screens
                for entity in results_screen_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                for entity in hud_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                match button {
                    ResultsButton::PlayAgain => {
                        // Return to start screen
                        next_state.set(GameState::MainMenu); // Will show Stage 2 option
                    }
                    ResultsButton::MainMenu => {
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                let base_color = match button {
                    ResultsButton::PlayAgain => Color::srgb(0.3, 0.7, 0.3),
                    ResultsButton::MainMenu => Color::srgb(0.5, 0.5, 0.6),
                };
                *color = lighten_color(base_color).into();
            }
            Interaction::None => {
                *color = match button {
                    ResultsButton::PlayAgain => Color::srgb(0.3, 0.7, 0.3),
                    ResultsButton::MainMenu => Color::srgb(0.5, 0.5, 0.6),
                }
                .into();
            }
        }
    }
}

// Helper functions

fn get_difficulty_color(level: u8) -> Color {
    match level {
        1 => Color::srgb(0.3, 0.7, 0.3),  // Green
        2 => Color::srgb(0.5, 0.7, 0.9),  // Light blue
        3 => Color::srgb(0.9, 0.7, 0.3),  // Gold
        4 => Color::srgb(0.9, 0.5, 0.3),  // Orange
        5 => Color::srgb(0.9, 0.3, 0.3),  // Red
        _ => Color::srgb(0.5, 0.5, 0.6),
    }
}

fn lighten_color(color: Color) -> Color {
    let rgb = color.to_srgba();
    Color::srgb(
        (rgb.red + 0.1).min(1.0),
        (rgb.green + 0.1).min(1.0),
        (rgb.blue + 0.1).min(1.0),
    )
}
