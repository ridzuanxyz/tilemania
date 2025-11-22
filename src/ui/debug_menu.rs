use bevy::prelude::*;
use crate::plugins::state::GameState;
use super::components::{
    ButtonComponent, ButtonSize, ButtonVariant,
    TextComponent, TextStyle, TextColorVariant,
};

#[derive(Component)]
pub struct DebugMenuScreen;

#[derive(Component)]
pub struct BackButton;

/// List of emojis to test
const TEST_EMOJIS: &[(&str, &str)] = &[
    ("🎯", "Target"),
    ("⌨️", "Keyboard"),
    ("✅", "Check"),
    ("⏱️", "Timer"),
    ("🎉", "Party"),
    ("⭐", "Star"),
    ("🌟", "Sparkles"),
    ("📚", "Books"),
    ("▶️", "Play"),
    ("⚙️", "Settings"),
    ("🐛", "Bug"),
    ("💡", "Bulb"),
    ("🔥", "Fire"),
    ("⚡", "Lightning"),
    ("💪", "Strong"),
    ("👍", "Thumbs up"),
    ("❤️", "Heart"),
    ("✨", "Sparkle"),
    ("🎮", "Game"),
    ("🏆", "Trophy"),
    ("🎪", "Tent"),
    ("🎨", "Palette"),
    ("🎭", "Masks"),
    ("🎯", "Dart"),
    ("🎲", "Dice"),
    ("🃏", "Joker"),
    ("♠️", "Spade"),
    ("♥️", "Heart"),
    ("♦️", "Diamond"),
    ("♣️", "Club"),
    ("✓", "Checkmark (simple)"),
    ("●", "Circle (filled)"),
    ("○", "Circle (empty)"),
    ("■", "Square (filled)"),
    ("□", "Square (empty)"),
    ("▲", "Triangle (up)"),
    ("▼", "Triangle (down)"),
    ("◆", "Diamond (filled)"),
    ("◇", "Diamond (empty)"),
    ("⌨", "Keyboard (simple)"),
    ("⏱", "Timer (simple)"),
];

pub fn update_debug_menu(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<DebugMenuScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    back_query: Query<(&Interaction, &BackButton), Changed<Interaction>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::DebugMenu {
        // Spawn UI if it doesn't exist
        if query.is_empty() {
            spawn_debug_menu_ui(&mut commands);
        }

        // Handle ESC key to go back
        if keyboard.just_pressed(KeyCode::Escape) {
            next_state.set(GameState::MainMenu);
        }

        // Handle Back button mouse click
        for (interaction, _) in back_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::MainMenu);
            }
        }
    } else {
        // Clean up when leaving
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_debug_menu_ui(commands: &mut Commands) {
    let font = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(40.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
            DebugMenuScreen,
        ))
        .id();

    // Title
    let title = TextComponent::spawn(
        commands,
        "🐛 Debug: Emoji Test",
        TextStyle::Title,
        TextColorVariant::Accent,
    );
    commands.entity(font).add_child(title);

    // Subtitle
    let subtitle = TextComponent::spawn(
        commands,
        "Testing which emojis render correctly",
        TextStyle::Body,
        TextColorVariant::Secondary,
    );
    commands.entity(font).add_child(subtitle);

    // Scrollable container for emoji grid
    let scroll_container = commands
        .spawn(NodeBundle {
            node: Node {
                width: Val::Percent(90.0),
                height: Val::Percent(70.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::scroll_y(),
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(12.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.3).into(),
            border_radius: BorderRadius::all(Val::Px(10.0)),
            ..default()
        })
        .id();
    commands.entity(font).add_child(scroll_container);

    // Add emoji test rows
    for (emoji, label) in TEST_EMOJIS {
        spawn_emoji_test_row(commands, scroll_container, emoji, label);
    }

    // Back button
    let back_button = ButtonComponent::spawn(
        commands,
        "← Back to Main Menu",
        ButtonSize::Large,
        ButtonVariant::Secondary,
        BackButton,
    );
    commands.entity(font).add_child(back_button);

    // Instructions
    let instructions = TextComponent::spawn(
        commands,
        "ESC: Back | Scroll to see all emojis",
        TextStyle::Caption,
        TextColorVariant::Muted,
    );
    commands.entity(font).add_child(instructions);
}

fn spawn_emoji_test_row(
    commands: &mut Commands,
    parent: Entity,
    emoji: &str,
    label: &str,
) {
    let row = commands
        .spawn(NodeBundle {
            node: Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            background_color: Color::srgba(0.2, 0.2, 0.25, 0.5).into(),
            border_radius: BorderRadius::all(Val::Px(5.0)),
            ..default()
        })
        .id();
    commands.entity(parent).add_child(row);

    // Emoji display (large size)
    let emoji_text = TextComponent::spawn(
        commands,
        emoji,
        TextStyle::Heading,
        TextColorVariant::Primary,
    );
    commands.entity(row).add_child(emoji_text);

    // Label
    let label_text = TextComponent::spawn(
        commands,
        label,
        TextStyle::Body,
        TextColorVariant::Secondary,
    );
    commands.entity(row).add_child(label_text);
}
