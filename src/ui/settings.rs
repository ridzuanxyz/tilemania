use bevy::prelude::*;
use crate::plugins::state::GameState;

#[derive(Component)]
pub struct SettingsScreen;

pub fn update_settings(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<SettingsScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::Settings {
        if query.is_empty() {
            spawn_settings_ui(&mut commands);
        }

        // Keyboard shortcut: ESC to return to main menu
        if keyboard.just_pressed(KeyCode::Escape) {
            next_state.set(GameState::MainMenu);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_settings_ui(commands: &mut Commands) {
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
            BackgroundColor(Color::srgb(0.12, 0.12, 0.18)),
            SettingsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("⚙ Settings"),
                TextFont {
                    font_size: 70.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ));

            // Settings placeholder
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Start,
                        row_gap: Val::Px(15.0),
                        padding: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.18, 0.18, 0.24)),
                ))
                .with_children(|settings| {
                    settings.spawn((
                        Text::new("🔊 Sound: ON"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    settings.spawn((
                        Text::new("🎵 Music: ON"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    settings.spawn((
                        Text::new("📚 Dictionary: CSW24"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    settings.spawn((
                        Text::new("⏱ Timer: 25:00"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    settings.spawn((
                        Text::new("🎮 Difficulty: Medium"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Note
            parent.spawn((
                Text::new("Settings functionality will be implemented in Sprint 2"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Instructions
            parent.spawn((
                Text::new("Press ESC to return to Main Menu"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}
