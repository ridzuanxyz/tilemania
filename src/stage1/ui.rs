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
            node: Node {
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
                    node: Node {
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
                    node: Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(60.0),
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|bottom| {
                    // Current word display
                    bottom.spawn((
                        Text::new(""),
                        TextFont {
                            font: font.clone(),
                            font_size: 52.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 0.5)),
                        WordDisplay,
                    ));

                    // Help hint (press F1)
                    bottom.spawn((
                        Text::new("Press F1 for Help"),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.5, 0.6)),
                    ));
                });
        });
}

/// Updates combo display with current multiplier
pub fn update_combo_display(
    mut query: Query<(&mut Text, &mut TextColor), With<ComboDisplay>>,
    state: Res<Stage1State>,
) {
    if !state.is_changed() {
        return;
    }

    for (mut text, mut text_color) in query.iter_mut() {
        let multiplier = if state.combo_count == 0 {
            1.0
        } else {
            (1.0 + (state.combo_count as f32 * 0.5)).min(3.0)
        };

        **text = if state.combo_count > 0 {
            format!("Combo: {}x ({:.1}x)", state.combo_count, multiplier)
        } else {
            "Combo: 0x".to_string()
        };

        // Color based on combo level
        text_color.0 = match state.combo_count {
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
    mut word_query: Query<(&mut Text, &mut TextColor), With<WordDisplay>>,
    state: Res<Stage1State>,
    tile_query: Query<&FallingTile>,
    lexicon: Option<Res<crate::lexicon::Lexicon>>,
) {
    if !state.is_changed() {
        return;
    }

    for (mut text, mut text_color) in word_query.iter_mut() {
        let mut word = String::new();
        for entity in &state.selected_tiles {
            if let Ok(tile) = tile_query.get(*entity) {
                word.push(tile.letter);
            }
        }

        if word.is_empty() {
            **text = "".to_string(); // Empty when no tiles selected
            text_color.0 = Color::srgb(0.7, 0.7, 0.7);
        } else {
            **text = word.clone();

            // Show real-time validation feedback
            if word.len() == 2 {
                let is_valid = lexicon
                    .as_ref()
                    .map(|lex| lex.is_valid(&word))
                    .unwrap_or(false);

                if is_valid {
                    // Valid word - Green with glow
                    text_color.0 = Color::srgb(0.3, 1.0, 0.3);
                    **text = format!("{} ✓", word);
                } else {
                    // Invalid word - Orange/Yellow (not red, to be encouraging)
                    text_color.0 = Color::srgb(1.0, 0.7, 0.3);
                    **text = format!("{} ?", word);
                }
            } else if word.len() < 2 {
                // Need more letters - Yellow
                text_color.0 = Color::srgb(1.0, 1.0, 0.5);
            } else {
                // Too many letters - Orange
                text_color.0 = Color::srgb(1.0, 0.6, 0.3);
                **text = format!("{} (need 2 tiles!)", word);
            }
        }
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
                node: Node {
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
                            node: Node {
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
                Text::new("Click a difficulty to begin"),
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
                state.max_combo = 0;
                state.selected_tiles.clear();
                state.words_found.clear();
                state.is_active = false; // Will be activated after help dismissal

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

/// Marker component for results screen buttons
#[derive(Component)]
pub enum ResultsButton {
    PlayAgain,
    MainMenu,
}

/// Marker component for help overlay
#[derive(Component)]
pub struct HelpOverlay;

/// Resource to track help overlay visibility
#[derive(Resource, Default)]
pub struct HelpState {
    pub is_visible: bool,
    pub is_pregame: bool, // True if this is the pre-game help, false if toggled with F1
}

/// Spawns the help overlay (pre-game or F1 toggle)
pub fn spawn_help_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    help_state: &HelpState,
) {
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium = asset_server.load("fonts/FiraSans-Medium.ttf");

    commands
        .spawn((
            NodeBundle {
                node: Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.85).into(),
                z_index: ZIndex(1000),
                ..default()
            },
            HelpOverlay,
        ))
        .with_children(|parent| {
            // Help card/panel
            parent
                .spawn(NodeBundle {
                    node: Node {
                        width: Val::Px(700.0),
                        padding: UiRect::all(Val::Px(40.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(25.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.15, 0.15, 0.2).into(),
                    border_radius: BorderRadius::all(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|card| {
                    // Title
                    card.spawn((
                        Text::new("How to Play"),
                        TextFont {
                            font: font_bold.clone(),
                            font_size: 48.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 0.95, 0.5)),
                    ));

                    // Instructions container
                    card.spawn(NodeBundle {
                        node: Node {
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Px(18.0),
                            align_items: AlignItems::Start,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|instructions| {
                        // Goal
                        instructions.spawn((
                            Text::new("🎯  Goal: Form valid 2-letter words"),
                            TextFont {
                                font: font_medium.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));

                        // Type letters
                        instructions.spawn((
                            Text::new("⌨️  Type letters (A-Z) to select tiles"),
                            TextFont {
                                font: font_medium.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));

                        // Submit word
                        instructions.spawn((
                            Text::new("✅  Press ENTER to submit your word"),
                            TextFont {
                                font: font_medium.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));

                        // Time pressure
                        instructions.spawn((
                            Text::new("⏱️  Make as many words as you can before time runs out!"),
                            TextFont {
                                font: font_medium.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                    // Bottom instruction
                    if help_state.is_pregame {
                        card.spawn((
                            Text::new("Press SPACE to Start"),
                            TextFont {
                                font: font_bold.clone(),
                                font_size: 32.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.3, 0.9, 0.3)),
                            Node {
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ));
                    } else {
                        card.spawn((
                            Text::new("Press F1 to close and resume"),
                            TextFont {
                                font: font_bold.clone(),
                                font_size: 28.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.8)),
                            Node {
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

/// Despawns help overlay
pub fn despawn_help_overlay(
    mut commands: Commands,
    help_query: Query<Entity, With<HelpOverlay>>,
) {
    for entity in help_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Shows help overlay on entering Stage1Playing
pub fn show_pregame_help(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut help_state: ResMut<HelpState>,
) {
    help_state.is_visible = true;
    help_state.is_pregame = true;
    spawn_help_overlay(commands, asset_server, &help_state);
}

/// Handles pre-game help dismissal (SPACE key)
pub fn handle_pregame_help_input(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut help_state: ResMut<HelpState>,
    mut state: ResMut<Stage1State>,
    help_query: Query<Entity, With<HelpOverlay>>,
) {
    if !help_state.is_pregame || !help_state.is_visible {
        return;
    }

    if keyboard.just_pressed(KeyCode::Space) {
        // Dismiss help
        help_state.is_visible = false;
        help_state.is_pregame = false;

        // Despawn overlay
        despawn_help_overlay(commands, help_query);

        // Activate game (start timer)
        state.is_active = true;
    }
}

/// Handles F1 help toggle during gameplay
pub fn handle_f1_help_toggle(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut help_state: ResMut<HelpState>,
    mut state: ResMut<Stage1State>,
    help_query: Query<Entity, With<HelpOverlay>>,
) {
    // Don't handle F1 during pre-game help
    if help_state.is_pregame {
        return;
    }

    if keyboard.just_pressed(KeyCode::F1) {
        if help_state.is_visible {
            // Hide help and resume game
            help_state.is_visible = false;
            state.is_active = true; // Resume timer
            despawn_help_overlay(commands, help_query);
        } else {
            // Show help and pause game
            help_state.is_visible = true;
            state.is_active = false; // Pause timer
            spawn_help_overlay(commands, asset_server, &help_state);
        }
    }
}

/// Cleanup help state when exiting Stage1Playing
pub fn cleanup_help_state(
    mut commands: Commands,
    mut help_state: ResMut<HelpState>,
    help_query: Query<Entity, With<HelpOverlay>>,
) {
    // Reset help state
    help_state.is_visible = false;
    help_state.is_pregame = false;

    // Despawn any lingering help overlays
    despawn_help_overlay(commands, help_query);
}

/// Spawns results screen showing final score and words found
pub fn spawn_results_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    state: Res<Stage1State>,
    last_stage: Res<crate::plugins::state::LastStageCompleted>,
) {
    // Only spawn if this stage just completed
    if *last_stage != crate::plugins::state::LastStageCompleted::Stage1 {
        return;
    }

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                node: Node {
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

            // Max combo
            parent.spawn((
                Text::new(format!("Max Combo: {}x", state.max_combo)),
                TextFont {
                    font: font.clone(),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.4)),
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
                .spawn((
                    ButtonBundle {
                        node: Node {
                            width: Val::Px(200.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(40.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                        ..default()
                    },
                    ResultsButton::PlayAgain,
                ))
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
                .spawn((
                    ButtonBundle {
                        node: Node {
                            width: Val::Px(200.0),
                            height: Val::Px(60.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.4, 0.4, 0.5).into(),
                        ..default()
                    },
                    ResultsButton::MainMenu,
                ))
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

/// Handles results screen button clicks
pub fn handle_results_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &ResultsButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut last_stage: ResMut<crate::plugins::state::LastStageCompleted>,
    results_screen_query: Query<Entity, With<ResultsScreen>>,
) {
    // Only handle buttons if Stage 1 was the last completed stage
    if *last_stage != crate::plugins::state::LastStageCompleted::Stage1 {
        return;
    }

    for (interaction, button, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Cleanup results screen
                for entity in results_screen_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                // Reset last stage tracker
                *last_stage = crate::plugins::state::LastStageCompleted::None;

                match button {
                    ResultsButton::PlayAgain => {
                        // Return to Stage 1 start screen (difficulty selection)
                        next_state.set(GameState::GameBoard);
                    }
                    ResultsButton::MainMenu => {
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                let base_color = match button {
                    ResultsButton::PlayAgain => Color::srgb(0.3, 0.6, 0.3),
                    ResultsButton::MainMenu => Color::srgb(0.4, 0.4, 0.5),
                };
                *bg_color = lighten_color(base_color).into();
            }
            Interaction::None => {
                *bg_color = match button {
                    ResultsButton::PlayAgain => Color::srgb(0.3, 0.6, 0.3),
                    ResultsButton::MainMenu => Color::srgb(0.4, 0.4, 0.5),
                }
                .into();
            }
        }
    }
}

/// Helper function to lighten a color for hover effect
fn lighten_color(color: Color) -> Color {
    let rgb = color.to_srgba();
    Color::srgb(
        (rgb.red + 0.1).min(1.0),
        (rgb.green + 0.1).min(1.0),
        (rgb.blue + 0.1).min(1.0),
    )
}
