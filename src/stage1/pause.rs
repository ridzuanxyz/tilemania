/// Pause menu for Stage 1

use bevy::prelude::*;
use crate::plugins::state::GameState;

/// Marker component for pause menu
#[derive(Component)]
pub struct PauseMenu;

/// Marker component for resume button
#[derive(Component)]
pub struct ResumeButton;

/// Marker component for quit button
#[derive(Component)]
pub struct QuitButton;

/// System to handle ESC key for pausing
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Stage1Playing => {
                info!("⏸️  Pausing game");
                next_state.set(GameState::Stage1Paused);
            }
            GameState::Stage1Paused => {
                info!("▶️  Resuming game");
                next_state.set(GameState::Stage1Playing);
            }
            _ => {}
        }
    }
}

/// Spawns the pause menu overlay
pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                // Semi-transparent dark overlay
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            // "PAUSED" title
            parent.spawn(TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font: font.clone(),
                    font_size: 72.0,
                    color: Color::WHITE,
                },
            ));

            // Resume button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(70.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(40.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.6, 0.3).into(),
                        ..default()
                    },
                    ResumeButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Resume (ESC)",
                        TextStyle {
                            font: font.clone(),
                            font_size: 28.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            // Restart button
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(70.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.4, 0.4, 0.5).into(),
                    ..default()
                })
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: font.clone(),
                            font_size: 28.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            // Quit to menu button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(70.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.5, 0.3, 0.3).into(),
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section(
                        "Quit to Menu",
                        TextStyle {
                            font: font.clone(),
                            font_size: 28.0,
                            color: Color::WHITE,
                        },
                    ));
                });

            // Instructions
            parent.spawn(TextBundle::from_section(
                "Press ESC to resume",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::srgb(0.7, 0.7, 0.7),
                },
            ));
        });
}

/// Despawns the pause menu
pub fn despawn_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenu>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Handles pause menu button interactions
pub fn handle_pause_menu_buttons(
    mut interaction_query: Query<
        (&Interaction, Option<&ResumeButton>, Option<&QuitButton>, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
    mut commands: Commands,
) {
    for (interaction, resume, quit, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if resume.is_some() {
                    // Resume button
                    info!("▶️  Resuming game");
                    next_state.set(GameState::Stage1Playing);
                } else if quit.is_some() {
                    // Quit button - go back to main menu
                    info!("🚪 Quitting to menu");
                    // Despawn pause menu
                    for entity in pause_menu_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    next_state.set(GameState::MainMenu);
                }
            }
            Interaction::Hovered => {
                if resume.is_some() {
                    *bg_color = Color::srgb(0.4, 0.7, 0.4).into();
                } else if quit.is_some() {
                    *bg_color = Color::srgb(0.6, 0.4, 0.4).into();
                } else {
                    *bg_color = Color::srgb(0.5, 0.5, 0.6).into();
                }
            }
            Interaction::None => {
                if resume.is_some() {
                    *bg_color = Color::srgb(0.3, 0.6, 0.3).into();
                } else if quit.is_some() {
                    *bg_color = Color::srgb(0.5, 0.3, 0.3).into();
                } else {
                    *bg_color = Color::srgb(0.4, 0.4, 0.5).into();
                }
            }
        }
    }
}
