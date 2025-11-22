use bevy::prelude::*;

/// Reusable button component with built-in styling and state management
#[derive(Component, Debug, Clone)]
pub struct ButtonComponent {
    pub label: String,
    #[allow(dead_code)] // Used in spawn() method
    pub size: ButtonSize,
    pub variant: ButtonVariant,
}

/// Button size variants with predefined dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Medium and Small reserved for Sprint 2+
pub enum ButtonSize {
    Large,   // 300x80px - Main menu buttons
    Medium,  // 200x60px - Settings options
    Small,   // 150x40px - Inline actions
}

impl ButtonSize {
    /// Get width in pixels
    pub fn width(&self) -> f32 {
        match self {
            ButtonSize::Large => 300.0,
            ButtonSize::Medium => 200.0,
            ButtonSize::Small => 150.0,
        }
    }

    /// Get height in pixels
    pub fn height(&self) -> f32 {
        match self {
            ButtonSize::Large => 80.0,
            ButtonSize::Medium => 60.0,
            ButtonSize::Small => 40.0,
        }
    }

    /// Get font size in points
    pub fn font_size(&self) -> f32 {
        match self {
            ButtonSize::Large => 40.0,
            ButtonSize::Medium => 32.0,
            ButtonSize::Small => 24.0,
        }
    }
}

/// Button visual variants with distinct color schemes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Danger reserved for Sprint 2+
pub enum ButtonVariant {
    Primary,   // Green - Play, Confirm, Start
    Secondary, // Gray - Settings, Back, Cancel
    Danger,    // Red - Delete, Reset, Quit
}

impl ButtonVariant {
    /// Get the color scheme for this variant
    pub fn colors(&self) -> ButtonColors {
        match self {
            ButtonVariant::Primary => ButtonColors {
                normal: Color::srgb(0.3, 0.7, 0.3),    // #4DB34D
                hover: Color::srgb(0.36, 0.76, 0.36),  // #5DC35D
                pressed: Color::srgb(0.24, 0.64, 0.24), // #3DA33D
                disabled: Color::srgb(0.53, 0.53, 0.53), // #888888
            },
            ButtonVariant::Secondary => ButtonColors {
                normal: Color::srgb(0.29, 0.29, 0.35),  // #4A4A5A
                hover: Color::srgb(0.35, 0.35, 0.42),   // #5A5A6A
                pressed: Color::srgb(0.23, 0.23, 0.29), // #3A3A4A
                disabled: Color::srgb(0.53, 0.53, 0.53), // #888888
            },
            ButtonVariant::Danger => ButtonColors {
                normal: Color::srgb(0.83, 0.3, 0.3),    // #D34D4D
                hover: Color::srgb(0.89, 0.36, 0.36),   // #E35D5D
                pressed: Color::srgb(0.76, 0.24, 0.24), // #C33D3D
                disabled: Color::srgb(0.53, 0.53, 0.53), // #888888
            },
        }
    }
}

/// Button interaction state
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(dead_code)] // Disabled reserved for Sprint 2+
pub enum ButtonState {
    #[default]
    Normal,
    Hover,
    Pressed,
    Disabled,
}

/// Color palette for button states
#[derive(Debug, Clone, Copy)]
pub struct ButtonColors {
    pub normal: Color,
    pub hover: Color,
    pub pressed: Color,
    pub disabled: Color,
}

impl ButtonColors {
    /// Get color for a specific state
    pub fn for_state(&self, state: ButtonState) -> Color {
        match state {
            ButtonState::Normal => self.normal,
            ButtonState::Hover => self.hover,
            ButtonState::Pressed => self.pressed,
            ButtonState::Disabled => self.disabled,
        }
    }
}

/// Marker component for button text child entity
#[derive(Component)]
pub struct ButtonText;

impl ButtonComponent {
    /// Create a new button with specified properties
    pub fn new(label: impl Into<String>, size: ButtonSize, variant: ButtonVariant) -> Self {
        Self {
            label: label.into(),
            size,
            variant,
        }
    }

    /// Spawn a button entity with all required components
    pub fn spawn<T: Component>(
        commands: &mut Commands,
        label: impl Into<String>,
        size: ButtonSize,
        variant: ButtonVariant,
        marker: T,
    ) -> Entity {
        let button = Self::new(label, size, variant);
        let colors = variant.colors();

        // Create button container
        let button_entity = commands
            .spawn((
                Button,
                Node {
                    width: Val::Px(size.width()),
                    height: Val::Px(size.height()),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(colors.normal),
                BorderColor(Color::srgb(0.0, 0.0, 0.0)),
                BorderRadius::all(Val::Px(8.0)),
                Interaction::default(),
                button.clone(),
                ButtonState::Normal,
                marker,
            ))
            .id();

        // Create button text as child
        let text_entity = commands
            .spawn((
                Text::new(&button.label),
                TextFont {
                    font_size: size.font_size(),
                    ..default()
                },
                TextColor(Color::WHITE),
                ButtonText,
            ))
            .id();

        // Parent text to button
        commands.entity(button_entity).add_child(text_entity);

        button_entity
    }
}

/// System to handle button interaction (hover, press)
pub fn update_button_interaction(
    mut query: Query<
        (&Interaction, &ButtonComponent, &mut ButtonState, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, button, mut state, mut bg_color) in query.iter_mut() {
        let colors = button.variant.colors();

        let new_state = match *interaction {
            Interaction::Pressed => ButtonState::Pressed,
            Interaction::Hovered => ButtonState::Hover,
            Interaction::None => ButtonState::Normal,
        };

        if *state != new_state {
            *state = new_state;
            *bg_color = BackgroundColor(colors.for_state(new_state));
        }
    }
}
