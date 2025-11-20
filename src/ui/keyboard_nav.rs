/// Keyboard navigation system for menus
/// Provides arrow key navigation, Enter/Space to select, and visual focus feedback

use bevy::prelude::*;

/// Marker component for items that can be navigated with keyboard
#[derive(Component)]
pub struct KeyboardNavigable {
    /// Index in the navigation order (0 = first, 1 = second, etc.)
    pub index: usize,
}

/// Resource to track which item is currently focused in each screen
#[derive(Resource, Default)]
pub struct KeyboardFocus {
    /// Currently focused index (None = no focus yet)
    pub focused_index: Option<usize>,
    /// Total number of navigable items
    pub total_items: usize,
}

impl KeyboardFocus {
    pub fn new(total_items: usize) -> Self {
        Self {
            focused_index: if total_items > 0 { Some(0) } else { None },
            total_items,
        }
    }

    pub fn move_up(&mut self) {
        if let Some(current) = self.focused_index {
            if current > 0 {
                self.focused_index = Some(current - 1);
            } else {
                // Wrap to bottom
                self.focused_index = Some(self.total_items - 1);
            }
        }
    }

    pub fn move_down(&mut self) {
        if let Some(current) = self.focused_index {
            if current < self.total_items - 1 {
                self.focused_index = Some(current + 1);
            } else {
                // Wrap to top
                self.focused_index = Some(0);
            }
        }
    }

    pub fn is_focused(&self, index: usize) -> bool {
        self.focused_index == Some(index)
    }
}

/// Update visual feedback for focused items
pub fn apply_focus_visual(
    focus: &KeyboardFocus,
    mut query: Query<(&KeyboardNavigable, &mut BorderColor), With<Button>>,
) {
    for (nav, mut border) in query.iter_mut() {
        if focus.is_focused(nav.index) {
            // Focused state - bright border
            *border = BorderColor(Color::srgb(0.9, 0.9, 1.0));
        } else {
            // Unfocused state - no border
            *border = BorderColor(Color::NONE);
        }
    }
}

/// System to handle keyboard navigation input
pub fn handle_keyboard_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut focus: ResMut<KeyboardFocus>,
    query: Query<(&KeyboardNavigable, &Interaction), With<Button>>,
) {
    // Move focus up
    if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
        focus.move_up();
    }

    // Move focus down
    if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        focus.move_down();
    }

    // Simulate button press on focused item with Enter or Space
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if let Some(focused_idx) = focus.focused_index {
            for (nav, _interaction) in query.iter() {
                if nav.index == focused_idx {
                    // The button interaction system will handle the actual button press
                    // We just need to trigger it by simulating a click
                    // This is handled by setting Interaction to Pressed in another system
                }
            }
        }
    }
}

/// Activate focused button with Enter/Space
pub fn apply_focused_activation(
    keyboard: &ButtonInput<KeyCode>,
    focus: &KeyboardFocus,
    mut query: Query<(&KeyboardNavigable, &mut Interaction), With<Button>>,
) {
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if let Some(focused_idx) = focus.focused_index {
            for (nav, mut interaction) in query.iter_mut() {
                if nav.index == focused_idx {
                    *interaction = Interaction::Pressed;
                }
            }
        }
    }
}

/// Helper to add keyboard navigability to a button entity
pub fn make_navigable(commands: &mut Commands, entity: Entity, index: usize) {
    commands.entity(entity).insert((
        KeyboardNavigable { index },
        BorderColor(Color::NONE),
    ));
}

/// System to handle left/right navigation for settings
pub fn handle_horizontal_navigation(
    keyboard: Res<ButtonInput<KeyCode>>,
    focus: Res<KeyboardFocus>,
    mut query: Query<(&KeyboardNavigable, &mut Interaction), With<Button>>,
) {
    if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        if let Some(focused_idx) = focus.focused_index {
            for (nav, mut interaction) in query.iter_mut() {
                if nav.index == focused_idx {
                    *interaction = Interaction::Pressed;
                }
            }
        }
    }

    if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
        if let Some(focused_idx) = focus.focused_index {
            for (nav, mut interaction) in query.iter_mut() {
                if nav.index == focused_idx {
                    *interaction = Interaction::Pressed;
                }
            }
        }
    }
}
