use bevy::prelude::*;
use crate::plugins::state::GameState;

/// Marker component for splash screen UI
#[derive(Component)]
pub struct SplashScreen;

/// Text component for splash screen
#[derive(Component)]
pub struct SplashText;

pub fn update_splash(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<SplashScreen>>,
) {
    if *state.get() == GameState::Splash {
        // Spawn splash screen if not already present
        if query.is_empty() {
            spawn_splash_ui(&mut commands);
        }
    } else {
        // Clean up splash screen when leaving state
        for entity in query.iter() {
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
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            SplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("🎮 TileMania\n\nLoading..."),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout::new_with_justify(JustifyText::Center),
                SplashText,
            ));
        });
}
