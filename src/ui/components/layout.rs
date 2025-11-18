use bevy::prelude::*;

/// Spacing scale for consistent layout spacing
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Spacing {
    None,        // 0px - no spacing
    XS,          // 8px - tight spacing
    SM,          // 16px - compact spacing
    MD,          // 24px - standard spacing
    LG,          // 32px - relaxed spacing
    XL,          // 48px - loose spacing
    Custom(f32), // Custom pixel value
}

impl Spacing {
    /// Convert spacing to Bevy Val
    pub fn to_val(&self) -> Val {
        match self {
            Spacing::None => Val::Px(0.0),
            Spacing::XS => Val::Px(8.0),
            Spacing::SM => Val::Px(16.0),
            Spacing::MD => Val::Px(24.0),
            Spacing::LG => Val::Px(32.0),
            Spacing::XL => Val::Px(48.0),
            Spacing::Custom(px) => Val::Px(*px),
        }
    }

    /// Convert spacing to pixel value
    pub fn to_px(&self) -> f32 {
        match self {
            Spacing::None => 0.0,
            Spacing::XS => 8.0,
            Spacing::SM => 16.0,
            Spacing::MD => 24.0,
            Spacing::LG => 32.0,
            Spacing::XL => 48.0,
            Spacing::Custom(px) => *px,
        }
    }

    /// Create UiRect with all sides using this spacing
    pub fn to_ui_rect(&self) -> UiRect {
        UiRect::all(self.to_val())
    }

    /// Create UiRect with horizontal and vertical spacing
    pub fn to_ui_rect_axes(&self, horizontal: Spacing) -> UiRect {
        UiRect::axes(horizontal.to_val(), self.to_val())
    }
}

/// Stack direction for layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum StackDirection {
    Vertical,   // Column layout (top to bottom)
    Horizontal, // Row layout (left to right)
}

impl StackDirection {
    /// Convert to Bevy FlexDirection
    pub fn to_flex_direction(&self) -> FlexDirection {
        match self {
            StackDirection::Vertical => FlexDirection::Column,
            StackDirection::Horizontal => FlexDirection::Row,
        }
    }
}

/// Alignment options for layout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Alignment {
    Start,   // Align to start (top/left)
    Center,  // Align to center
    End,     // Align to end (bottom/right)
    Stretch, // Stretch to fill
}

impl Alignment {
    /// Convert to Bevy AlignItems
    pub fn to_align_items(&self) -> AlignItems {
        match self {
            Alignment::Start => AlignItems::Start,
            Alignment::Center => AlignItems::Center,
            Alignment::End => AlignItems::End,
            Alignment::Stretch => AlignItems::Stretch,
        }
    }

    /// Convert to Bevy JustifyContent
    pub fn to_justify_content(&self) -> JustifyContent {
        match self {
            Alignment::Start => JustifyContent::Start,
            Alignment::Center => JustifyContent::Center,
            Alignment::End => JustifyContent::End,
            Alignment::Stretch => JustifyContent::SpaceBetween,
        }
    }
}

/// Container component with padding
#[derive(Component, Debug, Clone)]
pub struct Container {
    pub padding: Spacing,
}

impl Container {
    /// Create a new container with padding
    pub fn new(padding: Spacing) -> Self {
        Self { padding }
    }

    /// Spawn a container entity
    pub fn spawn(
        commands: &mut Commands,
        padding: Spacing,
    ) -> Entity {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: padding.to_ui_rect(),
                    ..default()
                },
                Container::new(padding),
            ))
            .id()
    }

    /// Spawn a container with custom width/height
    pub fn spawn_custom(
        commands: &mut Commands,
        padding: Spacing,
        width: Val,
        height: Val,
    ) -> Entity {
        commands
            .spawn((
                Node {
                    width,
                    height,
                    padding: padding.to_ui_rect(),
                    ..default()
                },
                Container::new(padding),
            ))
            .id()
    }
}

/// Stack component for vertical/horizontal layouts with gap
#[derive(Component, Debug, Clone)]
pub struct Stack {
    pub direction: StackDirection,
    pub gap: Spacing,
    pub align: Alignment,
}

impl Stack {
    /// Create a new stack
    pub fn new(direction: StackDirection, gap: Spacing, align: Alignment) -> Self {
        Self {
            direction,
            gap,
            align,
        }
    }

    /// Create a vertical stack
    pub fn vertical(gap: Spacing, align: Alignment) -> Self {
        Self::new(StackDirection::Vertical, gap, align)
    }

    /// Create a horizontal stack
    pub fn horizontal(gap: Spacing, align: Alignment) -> Self {
        Self::new(StackDirection::Horizontal, gap, align)
    }

    /// Spawn a stack entity
    pub fn spawn(
        commands: &mut Commands,
        direction: StackDirection,
        gap: Spacing,
        align: Alignment,
    ) -> Entity {
        let stack = Stack::new(direction, gap, align);

        let (row_gap, column_gap) = match direction {
            StackDirection::Vertical => (gap.to_val(), Val::Px(0.0)),
            StackDirection::Horizontal => (Val::Px(0.0), gap.to_val()),
        };

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: stack.direction.to_flex_direction(),
                    align_items: stack.align.to_align_items(),
                    justify_content: stack.align.to_justify_content(),
                    row_gap,
                    column_gap,
                    ..default()
                },
                stack,
            ))
            .id()
    }

    /// Spawn a centered stack (both axes)
    pub fn spawn_centered(
        commands: &mut Commands,
        direction: StackDirection,
        gap: Spacing,
    ) -> Entity {
        let stack = Stack::new(direction, gap, Alignment::Center);

        let (row_gap, column_gap) = match direction {
            StackDirection::Vertical => (gap.to_val(), Val::Px(0.0)),
            StackDirection::Horizontal => (Val::Px(0.0), gap.to_val()),
        };

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: stack.direction.to_flex_direction(),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap,
                    column_gap,
                    ..default()
                },
                stack,
            ))
            .id()
    }
}

/// Center component for centered content
#[derive(Component, Debug, Clone)]
pub struct Center;

impl Center {
    /// Spawn a centered container
    pub fn spawn(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Center,
            ))
            .id()
    }
}

/// Spacer component for flexible spacing
#[derive(Component, Debug, Clone)]
pub struct Spacer {
    pub size: Spacing,
}

impl Spacer {
    /// Create a new spacer
    pub fn new(size: Spacing) -> Self {
        Self { size }
    }

    /// Spawn a vertical spacer
    pub fn spawn_vertical(commands: &mut Commands, size: Spacing) -> Entity {
        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: size.to_val(),
                    ..default()
                },
                Spacer::new(size),
            ))
            .id()
    }

    /// Spawn a horizontal spacer
    pub fn spawn_horizontal(commands: &mut Commands, size: Spacing) -> Entity {
        commands
            .spawn((
                Node {
                    width: size.to_val(),
                    height: Val::Percent(100.0),
                    ..default()
                },
                Spacer::new(size),
            ))
            .id()
    }

    /// Spawn a flexible spacer (grows to fill available space)
    pub fn spawn_flexible(commands: &mut Commands) -> Entity {
        commands
            .spawn((
                Node {
                    flex_grow: 1.0,
                    ..default()
                },
                Spacer::new(Spacing::None),
            ))
            .id()
    }
}
