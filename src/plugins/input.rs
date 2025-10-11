use bevy::prelude::*;

/// InputState resource tracks current input state
/// Provides unified abstraction for keyboard, mouse, and touch input
#[derive(Resource, Default)]
pub struct InputState {
    pub mouse_position: Vec2,
    pub left_click: bool,
    pub right_click: bool,
    pub key_pressed: Option<KeyCode>,
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<InputState>()
            .add_systems(Update, update_input);
    }
}

fn update_input(
    mut input_state: ResMut<InputState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
) {
    // Update mouse position
    if let Ok(window) = windows.get_single() {
        if let Some(position) = window.cursor_position() {
            input_state.mouse_position = position;
        }
    }

    // Update mouse button states
    input_state.left_click = mouse_button.just_pressed(MouseButton::Left);
    input_state.right_click = mouse_button.just_pressed(MouseButton::Right);

    // Update keyboard state (track most recent key press)
    input_state.key_pressed = None;
    for key in keyboard.get_just_pressed() {
        input_state.key_pressed = Some(*key);
        break; // Only track one key per frame
    }
}
