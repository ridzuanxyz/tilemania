use bevy::prelude::*;
use crate::plugins::state::GameState;
use crate::plugins::assets::{GameAssets, AssetLoadingState};

/// Marker component for splash screen UI
#[derive(Component)]
pub struct SplashScreen;

/// Text component for splash screen title
#[derive(Component)]
pub struct SplashTitle;

/// Text component for loading status
#[derive(Component)]
pub struct LoadingText;

/// Component for progress bar fill
#[derive(Component)]
pub struct ProgressBarFill;

pub fn update_splash(
    mut commands: Commands,
    state: Res<State<GameState>>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
    screen_query: Query<Entity, With<SplashScreen>>,
    mut loading_text_query: Query<&mut Text, With<LoadingText>>,
    mut progress_bar_query: Query<&mut Node, With<ProgressBarFill>>,
) {
    if *state.get() == GameState::Splash {
        // Spawn splash screen if not already present
        if screen_query.is_empty() {
            spawn_splash_ui(&mut commands);
        }

        // Update loading text
        if let Ok(mut text) = loading_text_query.get_single_mut() {
            match assets.state {
                AssetLoadingState::NotStarted => {
                    text.0 = "Initializing...".to_string();
                }
                AssetLoadingState::Loading => {
                    text.0 = format!(
                        "Loading... {}/{}  ({:.0}%)",
                        assets.loaded_assets,
                        assets.total_assets,
                        assets.progress * 100.0
                    );
                }
                AssetLoadingState::Loaded => {
                    text.0 = "Ready!".to_string();
                }
                AssetLoadingState::Failed => {
                    text.0 = "Loading failed!".to_string();
                }
            }
        }

        // Update progress bar
        if let Ok(mut node) = progress_bar_query.get_single_mut() {
            node.width = Val::Percent(assets.progress * 100.0);
        }

        // Auto-transition when assets are loaded
        if assets.is_loaded() {
            next_state.set(GameState::MainMenu);
        }
    } else {
        // Clean up splash screen when leaving state
        for entity in screen_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_splash_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            SplashScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("🎮 TileMania"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                SplashTitle,
            ));

            // Subtitle
            parent.spawn((
                Text::new("Scrabble Learning Game"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.8)),
                TextLayout::new_with_justify(JustifyText::Center),
            ));

            // Progress bar container
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(30.0),
                        border: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                    BorderColor(Color::srgb(0.4, 0.4, 0.5)),
                ))
                .with_children(|bar| {
                    // Progress bar fill
                    bar.spawn((
                        Node {
                            width: Val::Percent(0.0),  // Will be updated
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.3, 0.7, 0.4)),
                        ProgressBarFill,
                    ));
                });

            // Loading text
            parent.spawn((
                Text::new("Loading..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.9)),
                TextLayout::new_with_justify(JustifyText::Center),
                LoadingText,
            ));
        });
}
