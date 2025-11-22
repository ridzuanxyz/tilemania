use bevy::prelude::*;
use crate::plugins::state::GameState;
use super::components::{
    ButtonComponent, ButtonSize, ButtonVariant,
    TextComponent, TextStyle, TextColorVariant,
    Stack, StackDirection, Spacing, Alignment, Spacer,
};
use super::keyboard_nav::{KeyboardFocus, KeyboardNavigable};

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct SettingsButton;

#[derive(Component)]
pub struct DebugButton;

pub fn update_main_menu(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<MainMenuScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &PlayButton), Changed<Interaction>>,
    settings_query: Query<(&Interaction, &SettingsButton), Changed<Interaction>>,
    debug_query: Query<(&Interaction, &DebugButton), Changed<Interaction>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    focus: Option<ResMut<KeyboardFocus>>,
) {
    if *state.get() == GameState::MainMenu {
        // Spawn UI if it doesn't exist
        if query.is_empty() {
            spawn_main_menu_ui(&mut commands);
        }

        // Always ensure KeyboardFocus resource exists (it gets removed when leaving this state)
        if focus.is_none() {
            commands.insert_resource(KeyboardFocus::new(3));  // Now have 3 buttons
            return; // Skip navigation this frame - resource won't be available until next frame
        }

        // Debug: Log ALL keyboard inputs to diagnose arrow key issue
        for key in keyboard.get_just_pressed() {
            info!("🔍 KEY PRESSED: {:?}", key);
        }

        // Handle keyboard navigation and activation
        if let Some(mut focus) = focus {
            // Arrow key navigation
            // WSL2/X11 bug workaround: Arrow keys map to wrong keycodes
            // Arrow DOWN → NumpadEnter, Arrow UP → Lang3
            if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) ||
               keyboard.just_pressed(KeyCode::Lang3) {  // WSL2 bug: Arrow UP maps here
                info!("⬆️  Arrow Up pressed - moving focus up");
                focus.move_up();
                info!("   Current focus: {:?}", focus.focused_index);
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) ||
               keyboard.just_pressed(KeyCode::NumpadEnter) {  // WSL2 bug: Arrow DOWN maps here
                info!("⬇️  Arrow Down pressed - moving focus down");
                focus.move_down();
                info!("   Current focus: {:?}", focus.focused_index);
            }

            // Handle Enter key activation (direct state change, no Interaction mutation)
            if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
                info!("⏎ Enter/Space pressed with focus: {:?}", focus.focused_index);
                if let Some(focused_idx) = focus.focused_index {
                    match focused_idx {
                        0 => {
                            info!("   Navigating to Stage Select (Play button)");
                            next_state.set(GameState::StageSelect);
                        }
                        1 => {
                            info!("   Navigating to Settings");
                            next_state.set(GameState::Settings);
                        }
                        2 => {
                            info!("   Navigating to Debug Menu");
                            next_state.set(GameState::DebugMenu);
                        }
                        _ => {}
                    }
                }
            }
        } else {
            // KeyboardFocus resource doesn't exist - this shouldn't happen after first frame
            if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::ArrowDown) ||
               keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::KeyS) ||
               keyboard.just_pressed(KeyCode::NumpadEnter) || keyboard.just_pressed(KeyCode::Lang3) {
                error!("❌ CRITICAL: KeyboardFocus resource missing after UI spawn! This is a bug.");
            }
        }

        // Handle Play button mouse click
        for (interaction, _) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::StageSelect);
            }
        }

        // Handle Settings button mouse click
        for (interaction, _) in settings_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::Settings);
            }
        }

        // Handle Debug button mouse click
        for (interaction, _) in debug_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::DebugMenu);
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
    commands.entity(play_button).insert(KeyboardNavigable { index: 0 });
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
    commands.entity(settings_button).insert(KeyboardNavigable { index: 1 });
    commands.entity(stack_id).add_child(settings_button);

    // Debug button (using ButtonComponent)
    let debug_button = ButtonComponent::spawn(
        commands,
        "🐛 Debug",
        ButtonSize::Large,
        ButtonVariant::Secondary,
        DebugButton,
    );
    // Make it keyboard navigable (index 2)
    commands.entity(debug_button).insert(KeyboardNavigable { index: 2 });
    commands.entity(stack_id).add_child(debug_button);

    // Spacer between buttons and instructions
    let spacer2 = Spacer::spawn_vertical(commands, Spacing::LG);
    commands.entity(stack_id).add_child(spacer2);

    // Instructions (using TextComponent)
    let instructions = TextComponent::spawn(
        commands,
        "↑↓: Navigate | Enter: Select | ESC: Quit",
        TextStyle::Caption,
        TextColorVariant::Muted,
    );
    commands.entity(stack_id).add_child(instructions);
}
