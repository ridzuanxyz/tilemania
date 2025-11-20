use bevy::prelude::*;
use crate::plugins::state::GameState;
use super::components::{
    ButtonComponent, ButtonSize, ButtonVariant,
    TextComponent, TextStyle, TextColorVariant,
    Stack, StackDirection, Spacing, Alignment, Spacer,
};
use super::keyboard_nav::{KeyboardFocus, KeyboardNavigable, apply_focused_activation, apply_focus_visual};

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SettingsButton;

pub fn update_main_menu(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<MainMenuScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &PlayButton), Changed<Interaction>>,
    settings_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    focus: Option<ResMut<KeyboardFocus>>,
    nav_query: Query<(&KeyboardNavigable, &mut Interaction), With<Button>>,
) {
    if *state.get() == GameState::MainMenu {
        if query.is_empty() {
            spawn_main_menu_ui(&mut commands);
            // Initialize keyboard focus with 2 items (Play, Settings)
            commands.insert_resource(KeyboardFocus::new(2));
        }

        // Handle keyboard navigation
        if let Some(mut focus) = focus {
            // Arrow key navigation
            if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) {
                focus.move_up();
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
                focus.move_down();
            }

            // Activate focused button with Enter
            apply_focused_activation(keyboard.as_ref(), focus.as_ref(), nav_query);
        }

        // Handle Play button click
        for (interaction, _) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::StageSelect);
            }
        }

        // Handle Settings button click
        for (interaction, _) in settings_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::Settings);
            }
        }
    } else {
        // Clean up when leaving
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands.remove_resource::<KeyboardFocus>();
    }
}

fn spawn_main_menu_ui(commands: &mut Commands) {
    // Create main screen container with centered vertical stack
    let screen_id = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
        MainMenuScreen,
    )).id();

    // Create centered vertical stack for content
    let stack_id = Stack::spawn_centered(
        commands,
        StackDirection::Vertical,
        Spacing::MD,
    );
    commands.entity(screen_id).add_child(stack_id);

    // Title (using TextComponent)
    let title = TextComponent::spawn(
        commands,
        "📚 TileMania",
        TextStyle::Title,
        TextColorVariant::Accent,
    );
    commands.entity(stack_id).add_child(title);

    // Subtitle (using TextComponent)
    let subtitle = TextComponent::spawn(
        commands,
        "Word Tile Strategy Game",
        TextStyle::Subheading,
        TextColorVariant::Secondary,
    );
    commands.entity(stack_id).add_child(subtitle);

    // Spacer between subtitle and buttons
    let spacer1 = Spacer::spawn_vertical(commands, Spacing::LG);
    commands.entity(stack_id).add_child(spacer1);

    // Play button (using ButtonComponent)
    let play_button = ButtonComponent::spawn(
        commands,
        "▶ Play",
        ButtonSize::Large,
        ButtonVariant::Primary,
        PlayButton,
    );
    // Make it keyboard navigable (index 0)
    commands.entity(play_button).insert((
        KeyboardNavigable { index: 0 },
        BorderColor(Color::NONE),
    ));
    commands.entity(stack_id).add_child(play_button);

    // Settings button (using ButtonComponent)
    let settings_button = ButtonComponent::spawn(
        commands,
        "⚙ Settings",
        ButtonSize::Large,
        ButtonVariant::Secondary,
        SettingsButton,
    );
    // Make it keyboard navigable (index 1)
    commands.entity(settings_button).insert((
        KeyboardNavigable { index: 1 },
        BorderColor(Color::NONE),
    ));
    commands.entity(stack_id).add_child(settings_button);

    // Spacer between buttons and instructions
    let spacer2 = Spacer::spawn_vertical(commands, Spacing::LG);
    commands.entity(stack_id).add_child(spacer2);

    // Instructions (using TextComponent)
    let instructions = TextComponent::spawn(
        commands,
        "Arrow Keys: Navigate | Enter: Select | ESC: Quit",
        TextStyle::Caption,
        TextColorVariant::Muted,
    );
    commands.entity(stack_id).add_child(instructions);
}
