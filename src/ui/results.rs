use bevy::prelude::*;
use crate::plugins::state::GameState;

#[derive(Component)]
pub struct ResultsScreen;

pub fn update_results(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<ResultsScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::Results {
        if query.is_empty() {
            spawn_results_ui(&mut commands);
        }

        // Keyboard shortcut: SPACE or ENTER to return to main menu
        if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
            next_state.set(GameState::MainMenu);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_results_ui(commands: &mut Commands) {
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
            BackgroundColor(Color::srgb(0.15, 0.1, 0.2)),
            ResultsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("🏆 Game Results"),
                TextFont {
                    font_size: 70.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.9, 0.4)),
            ));

            // Stats placeholder
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        padding: UiRect::all(Val::Px(40.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.15, 0.25)),
                ))
                .with_children(|stats| {
                    stats.spawn((
                        Text::new("Score: 1,234"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    stats.spawn((
                        Text::new("Words Played: 23"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                    stats.spawn((
                        Text::new("Best Word: QUIZZIFY (128 pts)"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));

                    stats.spawn((
                        Text::new("Time: 12:34"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                });

            // Instructions
            parent.spawn((
                Text::new("Press SPACE or ENTER to return to Main Menu"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.6, 0.8)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}
