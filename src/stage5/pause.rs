/// Pause menu for Stage 5 (AI Tournaments)

use bevy::prelude::*;
use crate::plugins::state::GameState;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButton {
    Resume,
    Forfeit,
    Quit,
}

pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Stage5Playing => next_state.set(GameState::Stage5Paused),
            GameState::Stage5Paused => next_state.set(GameState::Stage5Playing),
            _ => {}
        }
    }
}

pub fn spawn_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        NodeBundle {
            style: Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.8).into(),
            z_index: ZIndex::Global(100),
            ..default()
        },
        PauseMenu,
    ));
}

pub fn handle_pause_menu_buttons(
    mut commands: Commands,
    interaction_query: Query<(&Interaction, &PauseButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            for entity in pause_menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }

            match button {
                PauseButton::Resume => next_state.set(GameState::Stage5Playing),
                PauseButton::Forfeit => next_state.set(GameState::MainMenu),
                PauseButton::Quit => next_state.set(GameState::MainMenu),
            }
        }
    }
}

pub fn cleanup_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    for entity in pause_menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
