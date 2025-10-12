use bevy::prelude::*;

/// Game actions that can be triggered by various inputs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]  // Some variants reserved for Sprint 2+ gameplay
pub enum InputAction {
    // Navigation
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,

    // Selection
    Select,      // Confirm/Accept (Space, Enter, Left Click, Gamepad A)
    Cancel,      // Back/Cancel (Escape, Gamepad B)

    // UI Navigation
    NextTab,     // Tab
    PrevTab,     // Shift+Tab

    // Game Actions (for future use)
    PlaceTile,   // Left Click
    RotateTile,  // R key
    SwapTiles,   // S key (or could be Settings in menu)
    Hint,        // H key
    Undo,        // Ctrl+Z or U

    // System
    Pause,       // Escape or P
    Settings,    // S key (context-dependent)
    ToggleDebug, // F3
}

/// InputState resource tracks current input state
/// Provides unified abstraction for keyboard, mouse, and touch input
#[derive(Resource, Default)]
pub struct InputState {
    pub mouse_position: Vec2,
    pub left_click: bool,
    pub right_click: bool,
    pub key_pressed: Option<KeyCode>,

    // Action tracking (new)
    actions_this_frame: Vec<InputAction>,
}

impl InputState {
    /// Check if a specific action was triggered this frame
    #[allow(dead_code)]  // Will be used in Sprint 2+ for gameplay systems
    pub fn action_just_pressed(&self, action: InputAction) -> bool {
        self.actions_this_frame.contains(&action)
    }

    /// Get all actions triggered this frame
    #[allow(dead_code)]  // Will be used in Sprint 2+ for gameplay systems
    pub fn get_actions(&self) -> &[InputAction] {
        &self.actions_this_frame
    }
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
    // Clear previous frame's actions
    input_state.actions_this_frame.clear();

    // Update mouse position
    if let Ok(window) = windows.get_single() {
        if let Some(position) = window.cursor_position() {
            input_state.mouse_position = position;
        }
    }

    // Update mouse button states
    input_state.left_click = mouse_button.just_pressed(MouseButton::Left);
    input_state.right_click = mouse_button.just_pressed(MouseButton::Right);

    // Map mouse to actions
    if input_state.left_click {
        input_state.actions_this_frame.push(InputAction::Select);
        input_state.actions_this_frame.push(InputAction::PlaceTile);
    }
    if input_state.right_click {
        input_state.actions_this_frame.push(InputAction::Cancel);
    }

    // Update keyboard state (track most recent key press)
    input_state.key_pressed = None;
    for key in keyboard.get_just_pressed() {
        input_state.key_pressed = Some(*key);

        // Map keyboard keys to actions
        map_key_to_actions(*key, &mut input_state.actions_this_frame);
    }
}

/// Maps a keyboard key to zero or more input actions
fn map_key_to_actions(key: KeyCode, actions: &mut Vec<InputAction>) {
    match key {
        // Navigation keys (WASD alternative for arrow keys, but S conflicts with Settings)
        KeyCode::ArrowUp | KeyCode::KeyW => actions.push(InputAction::MoveUp),
        KeyCode::ArrowDown => actions.push(InputAction::MoveDown),
        KeyCode::ArrowLeft | KeyCode::KeyA => actions.push(InputAction::MoveLeft),
        KeyCode::ArrowRight | KeyCode::KeyD => actions.push(InputAction::MoveRight),

        // Selection
        KeyCode::Space | KeyCode::Enter => actions.push(InputAction::Select),
        KeyCode::Escape => {
            actions.push(InputAction::Cancel);
            actions.push(InputAction::Pause);
        }

        // UI Navigation
        KeyCode::Tab => actions.push(InputAction::NextTab),
        // Note: Shift+Tab would need modifier key tracking (future enhancement)

        // Game Actions
        KeyCode::KeyR => actions.push(InputAction::RotateTile),
        KeyCode::KeyH => actions.push(InputAction::Hint),
        KeyCode::KeyU => actions.push(InputAction::Undo),
        KeyCode::KeyP => actions.push(InputAction::Pause),

        // Context-dependent: S can be Settings in menu, SwapTiles in game
        KeyCode::KeyS => {
            actions.push(InputAction::Settings);
            actions.push(InputAction::SwapTiles);
            actions.push(InputAction::MoveDown);  // Also allow S for down movement
        }

        // Debug
        KeyCode::F3 => actions.push(InputAction::ToggleDebug),

        _ => {} // Unmapped keys
    }
}
