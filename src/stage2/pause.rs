/// Pause menu for Stage 2 (Match-3 gameplay)

use bevy::prelude::*;
use crate::plugins::state::GameState;

/// Marker for pause menu UI
#[derive(Component)]
pub struct PauseMenu;

/// Marker for pause menu buttons
#[derive(Component)]
pub enum PauseButton {
    Resume,
    Restart,
    Quit,
}

/// Handle ESC key to toggle pause
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Stage2Playing => {
                next_state.set(GameState::Stage2Paused);
            }
            GameState::Stage2Paused => {
                next_state.set(GameState::Stage2Playing);
            }
            _ => {}
        }
    }
}

/// Spawn pause menu overlay
pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let title_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 64.0,
        color: Color::srgb(0.9, 0.9, 1.0),
    };

    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 32.0,
        color: Color::srgb(0.1, 0.1, 0.15),
    };

    // Root container - semi-transparent overlay
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(), // 80% black overlay
                z_index: ZIndex::Global(100), // Ensure it's on top
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            // Pause title
            parent.spawn(TextBundle {
                text: Text::from_section("PAUSED", title_style),
                style: Style {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
                ..default()
            });

            // Buttons container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons| {
                    // Resume button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(300.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.7, 0.3).into(), // Green
                                ..default()
                            },
                            PauseButton::Resume,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Resume",
                                button_text_style.clone(),
                            ));
                        });

                    // Restart button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(300.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.9, 0.7, 0.3).into(), // Gold
                                ..default()
                            },
                            PauseButton::Restart,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Restart",
                                button_text_style.clone(),
                            ));
                        });

                    // Quit button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(300.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.7, 0.3, 0.3).into(), // Red
                                ..default()
                            },
                            PauseButton::Quit,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Quit to Menu",
                                button_text_style.clone(),
                            ));
                        });
                });

            // Hint text
            parent.spawn(
                TextBundle::from_section(
                    "Press ESC to resume",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                        font_size: 20.0,
                        color: Color::srgb(0.6, 0.6, 0.7),
                    },
                )
                .with_style(Style {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                }),
            );
        });
}

/// Handle pause menu button clicks
pub fn handle_pause_menu_buttons(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &PauseButton, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
    // Query all Stage 2 entities for cleanup on restart/quit
    stage2_entities: Query<Entity, Or<(
        With<super::components::GridTile>,
        With<super::ui::Stage2HUD>,
        With<super::ui::Stage2StartScreen>,
    )>>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Cleanup pause menu
                for entity in pause_menu_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                match button {
                    PauseButton::Resume => {
                        next_state.set(GameState::Stage2Playing);
                    }
                    PauseButton::Restart => {
                        // Cleanup all Stage 2 entities
                        for entity in stage2_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        // Return to Stage 2 start screen
                        next_state.set(GameState::MainMenu);
                    }
                    PauseButton::Quit => {
                        // Cleanup all Stage 2 entities
                        for entity in stage2_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        next_state.set(GameState::MainMenu);
                    }
                }
            }
            Interaction::Hovered => {
                let base_color = match button {
                    PauseButton::Resume => Color::srgb(0.3, 0.7, 0.3),
                    PauseButton::Restart => Color::srgb(0.9, 0.7, 0.3),
                    PauseButton::Quit => Color::srgb(0.7, 0.3, 0.3),
                };
                *color = lighten_color(base_color).into();
            }
            Interaction::None => {
                *color = match button {
                    PauseButton::Resume => Color::srgb(0.3, 0.7, 0.3),
                    PauseButton::Restart => Color::srgb(0.9, 0.7, 0.3),
                    PauseButton::Quit => Color::srgb(0.7, 0.3, 0.3),
                }
                .into();
            }
        }
    }
}

/// Cleanup pause menu when exiting pause state
pub fn cleanup_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Helper function
fn lighten_color(color: Color) -> Color {
    let rgb = color.to_srgba();
    Color::srgb(
        (rgb.red + 0.15).min(1.0),
        (rgb.green + 0.15).min(1.0),
        (rgb.blue + 0.15).min(1.0),
    )
}
