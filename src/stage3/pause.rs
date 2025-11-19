/// Pause menu for Stage 3 (Classic Board)

use bevy::prelude::*;
use crate::plugins::state::GameState;

/// Marker for pause menu
#[derive(Component)]
pub struct PauseMenu;

/// Pause menu buttons
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
            GameState::Stage3Playing => {
                next_state.set(GameState::Stage3Paused);
            }
            GameState::Stage3Paused => {
                next_state.set(GameState::Stage3Playing);
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
    let font_bold = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn((
            NodeBundle {
                node: Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
                z_index: ZIndex(100),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font: font_bold.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(50.0)),
                    ..default()
                },
            ));

            parent
                .spawn(NodeBundle {
                    node: Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons| {
                    buttons
                        .spawn((
                            ButtonBundle {
                                node: Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.3, 0.7, 0.3).into(),
                                ..default()
                            },
                            PauseButton::Resume,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Resume"),
                                TextFont {
                                    font: font_bold.clone(),
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.1, 0.1, 0.15)),
                            ));
                        });

                    buttons
                        .spawn((
                            ButtonBundle {
                                node: Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.9, 0.7, 0.3).into(),
                                ..default()
                            },
                            PauseButton::Restart,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Restart"),
                                TextFont {
                                    font: font_bold.clone(),
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.1, 0.1, 0.15)),
                            ));
                        });

                    buttons
                        .spawn((
                            ButtonBundle {
                                node: Node {
                                    width: Val::Px(300.0),
                                    height: Val::Px(70.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::srgb(0.7, 0.3, 0.3).into(),
                                ..default()
                            },
                            PauseButton::Quit,
                        ))
                        .with_children(|button| {
                            button.spawn((
                                Text::new("Quit to Menu"),
                                TextFont {
                                    font: font_bold.clone(),
                                    font_size: 32.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.1, 0.1, 0.15)),
                            ));
                        });
                });
        });
}

/// Handle pause menu button clicks
pub fn handle_pause_menu_buttons(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, &PauseButton, &mut BackgroundColor), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for (interaction, button, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                for entity in pause_menu_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                match button {
                    PauseButton::Resume => next_state.set(GameState::Stage3Playing),
                    PauseButton::Restart => next_state.set(GameState::MainMenu),
                    PauseButton::Quit => next_state.set(GameState::MainMenu),
                }
            }
            Interaction::Hovered => {
                *color = lighten_color(get_button_color(button)).into();
            }
            Interaction::None => {
                *color = get_button_color(button).into();
            }
        }
    }
}

/// Cleanup pause menu
pub fn cleanup_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn get_button_color(button: &PauseButton) -> Color {
    match button {
        PauseButton::Resume => Color::srgb(0.3, 0.7, 0.3),
        PauseButton::Restart => Color::srgb(0.9, 0.7, 0.3),
        PauseButton::Quit => Color::srgb(0.7, 0.3, 0.3),
    }
}

fn lighten_color(color: Color) -> Color {
    let rgb = color.to_srgba();
    Color::srgb(
        (rgb.red + 0.15).min(1.0),
        (rgb.green + 0.15).min(1.0),
        (rgb.blue + 0.15).min(1.0),
    )
}
