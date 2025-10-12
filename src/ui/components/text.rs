use bevy::prelude::*;

/// Reusable text component with consistent typography and colors
#[derive(Component, Debug, Clone)]
pub struct TextComponent {
    pub content: String,
    pub style: TextStyle,
    pub color_variant: TextColorVariant,
}

/// Typography scale for consistent text sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Some variants reserved for Sprint 2+
pub enum TextStyle {
    Title,      // 80pt - Screen titles
    Heading,    // 40pt - Section headings
    Subheading, // 30pt - Subheadings
    Body,       // 24pt - Body text, descriptions
    Caption,    // 20pt - Small text, instructions
}

impl TextStyle {
    /// Get font size in points
    pub fn font_size(&self) -> f32 {
        match self {
            TextStyle::Title => 80.0,
            TextStyle::Heading => 40.0,
            TextStyle::Subheading => 30.0,
            TextStyle::Body => 24.0,
            TextStyle::Caption => 20.0,
        }
    }
}

/// Text color variants for different UI purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Some variants reserved for Sprint 2+
pub enum TextColorVariant {
    Primary,    // White - Main text
    Secondary,  // Light gray - Secondary text
    Muted,      // Gray - Muted text
    Accent,     // Light blue - Highlighted text
    Success,    // Green - Success messages
    Warning,    // Yellow - Warning messages
    Error,      // Red - Error messages
}

impl TextColorVariant {
    /// Get the color for this variant
    pub fn color(&self) -> Color {
        match self {
            TextColorVariant::Primary => Color::srgb(1.0, 1.0, 1.0),        // #FFFFFF (White)
            TextColorVariant::Secondary => Color::srgb(0.7, 0.7, 0.8),      // #B3B3CC (Light gray)
            TextColorVariant::Muted => Color::srgb(0.5, 0.5, 0.6),          // #808099 (Gray)
            TextColorVariant::Accent => Color::srgb(0.6, 0.6, 1.0),         // #9999FF (Light blue)
            TextColorVariant::Success => Color::srgb(0.3, 0.7, 0.3),        // #4DB34D (Green)
            TextColorVariant::Warning => Color::srgb(0.9, 0.78, 0.3),       // #E6C84D (Yellow)
            TextColorVariant::Error => Color::srgb(0.83, 0.3, 0.3),         // #D34D4D (Red)
        }
    }
}

impl TextComponent {
    /// Create a new text component
    pub fn new(
        content: impl Into<String>,
        style: TextStyle,
        color_variant: TextColorVariant,
    ) -> Self {
        Self {
            content: content.into(),
            style,
            color_variant,
        }
    }

    /// Spawn a text entity with specified properties
    pub fn spawn(
        commands: &mut Commands,
        content: impl Into<String>,
        style: TextStyle,
        color_variant: TextColorVariant,
    ) -> Entity {
        let text_component = Self::new(content, style, color_variant);

        commands
            .spawn((
                Text::new(&text_component.content),
                TextFont {
                    font_size: text_component.style.font_size(),
                    ..default()
                },
                TextColor(text_component.color_variant.color()),
                text_component,
            ))
            .id()
    }

    /// Spawn text with optional custom node styling (margins, etc.)
    pub fn spawn_with_node(
        commands: &mut Commands,
        content: impl Into<String>,
        style: TextStyle,
        color_variant: TextColorVariant,
        node: Node,
    ) -> Entity {
        let text_component = Self::new(content, style, color_variant);

        commands
            .spawn((
                Text::new(&text_component.content),
                TextFont {
                    font_size: text_component.style.font_size(),
                    ..default()
                },
                TextColor(text_component.color_variant.color()),
                node,
                text_component,
            ))
            .id()
    }
}
