use bevy::prelude::*;
use crate::plugins::state::GameState;

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SettingsButton;

pub fn update_main_menu(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<MainMenuScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &PlayButton), Changed<Interaction>>,
    settings_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::MainMenu {
        if query.is_empty() {
            spawn_main_menu_ui(&mut commands);
        }

        // Handle Play button click
        for (interaction, _) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::GameBoard);
            }
        }

        // Handle Settings button click
        for (interaction, _) in settings_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::Settings);
            }
        }

        // Keyboard shortcut: SPACE to play
        if keyboard.just_pressed(KeyCode::Space) {
            next_state.set(GameState::GameBoard);
        }

        // Keyboard shortcut: S for settings
        if keyboard.just_pressed(KeyCode::KeyS) {
            next_state.set(GameState::Settings);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_main_menu_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            MainMenuScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("📚 TileMania"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Scrabble Learning Game"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Play button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.6, 0.3)),
                    PlayButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("▶ Play (SPACE)"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Settings button
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.4, 0.4, 0.5)),
                    SettingsButton,
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("⚙ Settings (S)"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new("Press SPACE to start | S for Settings | ESC to quit"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}
