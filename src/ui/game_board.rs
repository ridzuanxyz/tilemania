use bevy::prelude::*;
use crate::plugins::state::GameState;

#[derive(Component)]
pub struct GameBoardScreen;

pub fn update_game_board(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<GameBoardScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::GameBoard {
        if query.is_empty() {
            spawn_game_board_ui(&mut commands);
        }

        // Keyboard shortcut: ESC to return to main menu
        if keyboard.just_pressed(KeyCode::Escape) {
            next_state.set(GameState::MainMenu);
        }

        // Keyboard shortcut: R to go to results (simulating game end)
        if keyboard.just_pressed(KeyCode::KeyR) {
            next_state.set(GameState::Results);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_game_board_ui(commands: &mut Commands) {
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
            BackgroundColor(Color::srgb(0.1, 0.2, 0.15)),
            GameBoardScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("🎮 Game Board"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 1.0, 0.9)),
            ));

            // Placeholder text
            parent.spawn((
                Text::new("Game board will be implemented in Sprint 2-4"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.8, 0.7)),
                Node {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Board placeholder
            parent.spawn((
                Node {
                    width: Val::Px(600.0),
                    height: Val::Px(600.0),
                    border: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.3, 0.25)),
                BorderColor(Color::srgb(0.4, 0.6, 0.5)),
            ));

            // Instructions
            parent.spawn((
                Text::new("ESC: Main Menu | R: Results (simulate end game)"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.7, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(30.0)),
                    ..default()
                },
            ));
        });
}
