/// UI for Stage 4 (Speed Challenge)

use bevy::prelude::*;
use super::{Stage4State, Stage4Config};

/// Marker for Stage 4 HUD
#[derive(Component)]
pub struct Stage4HUD;

/// HUD elements
#[derive(Component)]
pub enum HUDElement {
    Score,
    Timer,
    Streak,
    WordCount,
}

/// Spawn Stage 4 HUD
pub fn spawn_stage4_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 36.0,
        color: Color::srgb(1.0, 1.0, 1.0),
    };

    commands
        .spawn((
            NodeBundle {
                style: Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.15, 0.9).into(),
                ..default()
            },
            Stage4HUD,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                text_style.clone(),
                HUDElement::Score,
            ));

            parent.spawn((
                Text::new("Time: 90s"),
                text_style.clone(),
                HUDElement::Timer,
            ));

            parent.spawn((
                Text::new("Streak: x0"),
                text_style.clone(),
                HUDElement::Streak,
            ));
        });
}

/// Update HUD
pub fn update_stage4_hud(
    state: Res<Stage4State>,
    mut hud_query: Query<(&HUDElement, &mut Text, &mut TextColor)>,
) {
    for (element, mut text, mut text_color) in hud_query.iter_mut() {
        match element {
            HUDElement::Score => {
                **text = format!("Score: {}", state.score);
            }
            HUDElement::Timer => {
                let seconds = state.time_remaining_ms / 1000;
                **text = format!("Time: {}s", seconds);

                // Color based on time remaining
                if seconds < 10 {
                    text_color.0 = Color::srgb(1.0, 0.2, 0.2);
                } else if seconds < 30 {
                    text_color.0 = Color::srgb(1.0, 0.7, 0.3);
                } else {
                    text_color.0 = Color::srgb(1.0, 1.0, 1.0);
                }
            }
            HUDElement::Streak => {
                **text = format!("Streak: x{}", state.current_streak);

                // Color based on streak
                if state.current_streak >= 10 {
                    text_color.0 = Color::srgb(1.0, 0.3, 1.0);
                } else if state.current_streak >= 5 {
                    text_color.0 = Color::srgb(1.0, 0.8, 0.3);
                } else {
                    text_color.0 = Color::srgb(1.0, 1.0, 1.0);
                }
            }
            HUDElement::WordCount => {
                **text = format!("Words: {}", state.words_formed);
            }
        }
    }
}

/// Update rack display
pub fn update_rack_display(
    state: Res<Stage4State>,
) {
    // Update 7-tile rack visual representation
}
