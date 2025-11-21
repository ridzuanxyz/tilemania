use bevy::prelude::*;
use crate::plugins::state::GameState;
use super::keyboard_nav::{KeyboardFocus, KeyboardNavigable};

#[derive(Component)]
pub struct StageSelectScreen;

#[derive(Component)]
pub enum StageButton {
    Stage1,
    Stage2,
    Stage3,
    Stage4,
    Stage5,
}

pub fn update_stage_select(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<StageSelectScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    focus: Option<ResMut<KeyboardFocus>>,
) {
    if *state.get() == GameState::StageSelect {
        if query.is_empty() {
            spawn_stage_select_ui(&mut commands, &asset_server);
            // Initialize keyboard focus with 5 stage cards
            commands.insert_resource(KeyboardFocus::new(5));
        }

        // Handle keyboard navigation
        if let Some(mut focus) = focus {
            // Arrow key navigation
            // WSL2/X11 bug workaround: Arrow Up → NumpadEnter, Arrow Down → Lang3
            if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) ||
               keyboard.just_pressed(KeyCode::NumpadEnter) {
                focus.move_up();
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) ||
               keyboard.just_pressed(KeyCode::Lang3) {
                focus.move_down();
            }

            // Handle Enter key activation (navigate to stage start screen with difficulty selection)
            if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
                if let Some(focused_idx) = focus.focused_index {
                    match focused_idx {
                        0 => next_state.set(GameState::GameBoard),  // Stage 1 start screen
                        1 => next_state.set(GameState::Stage2Start),  // Stage 2 start screen
                        2 => next_state.set(GameState::Stage3Playing),
                        3 => next_state.set(GameState::Stage4Playing),
                        4 => next_state.set(GameState::Stage5Playing),
                        _ => {}
                    }
                }
            }
        }

        // Keyboard shortcuts: ESC or Backspace to return to main menu
        if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Backspace) {
            next_state.set(GameState::MainMenu);
        }

        // Keyboard shortcuts for stages (1-5) - go to start screen first
        if keyboard.just_pressed(KeyCode::Digit1) {
            next_state.set(GameState::GameBoard);  // Stage 1 with difficulty selection
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            next_state.set(GameState::Stage2Start);  // Stage 2 with difficulty selection
        }
        if keyboard.just_pressed(KeyCode::Digit3) {
            next_state.set(GameState::Stage3Playing);
        }
        if keyboard.just_pressed(KeyCode::Digit4) {
            next_state.set(GameState::Stage4Playing);
        }
        if keyboard.just_pressed(KeyCode::Digit5) {
            next_state.set(GameState::Stage5Playing);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands.remove_resource::<KeyboardFocus>();
    }
}

pub fn handle_stage_buttons(
    mut interaction_query: Query<
        (&Interaction, &StageButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                StageButton::Stage1 => next_state.set(GameState::GameBoard),  // Go to difficulty screen first
                StageButton::Stage2 => next_state.set(GameState::Stage2Start),  // Go to difficulty screen first
                StageButton::Stage3 => next_state.set(GameState::Stage3Playing),
                StageButton::Stage4 => next_state.set(GameState::Stage4Playing),
                StageButton::Stage5 => next_state.set(GameState::Stage5Playing),
            }
        }
    }
}

fn spawn_stage_select_ui(commands: &mut Commands, asset_server: &AssetServer) {
    let font_bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/FiraSans-Medium.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(30.0),
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.10, 0.12, 0.18)),
            StageSelectScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("🎯 SELECT STAGE"),
                TextFont {
                    font: font_bold.clone(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ));

            // Subtitle
            parent.spawn((
                Text::new("Choose your word-building challenge"),
                TextFont {
                    font: font_medium.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            // Stage buttons grid
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|stages| {
                    // Stage 1
                    let stage1 = spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 1: FALLING LETTERS",
                        "Master all 127 two-letter words",
                        "🔤",
                        StageButton::Stage1,
                        Color::srgb(0.3, 0.5, 0.7),
                        0,
                    );

                    // Stage 2
                    let stage2 = spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 2: TILE MATCHING",
                        "Match 3-4 letter words in the grid",
                        "🎮",
                        StageButton::Stage2,
                        Color::srgb(0.4, 0.6, 0.3),
                        1,
                    );

                    // Stage 3
                    let stage3 = spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 3: CLASSIC BOARD",
                        "Full 15×15 board strategy vs AI",
                        "📋",
                        StageButton::Stage3,
                        Color::srgb(0.6, 0.4, 0.5),
                        2,
                    );

                    // Stage 4
                    let stage4 = spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 4: SPEED CHALLENGE",
                        "Rapid word formation under pressure",
                        "⚡",
                        StageButton::Stage4,
                        Color::srgb(0.7, 0.5, 0.2),
                        3,
                    );

                    // Stage 5
                    let stage5 = spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 5: AI TOURNAMENT",
                        "8-player bracket elimination",
                        "🏆",
                        StageButton::Stage5,
                        Color::srgb(0.5, 0.3, 0.6),
                        4,
                    );
                });

            // Instructions
            parent.spawn((
                Text::new("↑↓: Navigate | Enter: Select | 1-5: Quick Jump | Backspace: Back"),
                TextFont {
                    font: font_medium.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_stage_card(
    parent: &mut ChildBuilder,
    font_bold: &Handle<Font>,
    font_medium: &Handle<Font>,
    title: &str,
    description: &str,
    icon: &str,
    button_type: StageButton,
    color: Color,
    index: usize,
) -> Entity {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(700.0),
                height: Val::Px(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(20.0)),
                column_gap: Val::Px(20.0),
                border: UiRect::all(Val::Px(4.0)), // Thick border for keyboard focus
                ..default()
            },
            BackgroundColor(color),
            button_type,
            KeyboardNavigable { index },
            BorderColor(Color::NONE),
        ))
        .with_children(|card| {
            // Icon
            card.spawn((
                Text::new(icon),
                TextFont {
                    font: font_bold.clone(),
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Text container
            card.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            })
            .with_children(|text_container| {
                // Title
                text_container.spawn((
                    Text::new(title),
                    TextFont {
                        font: font_bold.clone(),
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Description
                text_container.spawn((
                    Text::new(description),
                    TextFont {
                        font: font_medium.clone(),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
                ));
            });
        })
        .id()
}
