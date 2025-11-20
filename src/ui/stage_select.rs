use bevy::prelude::*;
use crate::plugins::state::GameState;

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
) {
    if *state.get() == GameState::StageSelect {
        if query.is_empty() {
            spawn_stage_select_ui(&mut commands, &asset_server);
        }

        // Keyboard shortcut: ESC to return to main menu
        if keyboard.just_pressed(KeyCode::Escape) {
            next_state.set(GameState::MainMenu);
        }

        // Keyboard shortcuts for stages (1-5)
        if keyboard.just_pressed(KeyCode::Digit1) {
            next_state.set(GameState::Stage1Playing);
        }
        if keyboard.just_pressed(KeyCode::Digit2) {
            next_state.set(GameState::Stage2Playing);
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
                StageButton::Stage1 => next_state.set(GameState::Stage1Playing),
                StageButton::Stage2 => next_state.set(GameState::Stage2Playing),
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
                    spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 1: FALLING LETTERS",
                        "Master all 127 two-letter words",
                        "🔤",
                        StageButton::Stage1,
                        Color::srgb(0.3, 0.5, 0.7),
                    );

                    // Stage 2
                    spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 2: TILE MATCHING",
                        "Match 3-4 letter words in the grid",
                        "🎮",
                        StageButton::Stage2,
                        Color::srgb(0.4, 0.6, 0.3),
                    );

                    // Stage 3
                    spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 3: CLASSIC BOARD",
                        "Full 15×15 board strategy vs AI",
                        "📋",
                        StageButton::Stage3,
                        Color::srgb(0.6, 0.4, 0.5),
                    );

                    // Stage 4
                    spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 4: SPEED CHALLENGE",
                        "Rapid word formation under pressure",
                        "⚡",
                        StageButton::Stage4,
                        Color::srgb(0.7, 0.5, 0.2),
                    );

                    // Stage 5
                    spawn_stage_card(
                        stages,
                        &font_bold,
                        &font_medium,
                        "STAGE 5: AI TOURNAMENT",
                        "8-player bracket elimination",
                        "🏆",
                        StageButton::Stage5,
                        Color::srgb(0.5, 0.3, 0.6),
                    );
                });

            // Instructions
            parent.spawn((
                Text::new("Click a stage or press 1-5 • ESC: Back to Menu"),
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
) {
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
                ..default()
            },
            BackgroundColor(color),
            button_type,
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
        });
}
