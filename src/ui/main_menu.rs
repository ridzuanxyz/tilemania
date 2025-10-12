use bevy::prelude::*;
use crate::plugins::state::GameState;
use super::components::{ButtonComponent, ButtonSize, ButtonVariant, TextComponent, TextStyle, TextColorVariant};

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
) {
    if *state.get() == GameState::MainMenu {
        if query.is_empty() {
            spawn_main_menu_ui(&mut commands);
        }

        // Handle Play button click
        for (interaction, _) in interaction_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::GameBoard);
            }
        }

        // Handle Settings button click
        for (interaction, _) in settings_query.iter() {
            if *interaction == Interaction::Pressed {
                next_state.set(GameState::Settings);
            }
        }

        // Keyboard shortcut: SPACE to play
        if keyboard.just_pressed(KeyCode::Space) {
            next_state.set(GameState::GameBoard);
        }

        // Keyboard shortcut: S for settings
        if keyboard.just_pressed(KeyCode::KeyS) {
            next_state.set(GameState::Settings);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_main_menu_ui(commands: &mut Commands) {
    let screen_id = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
        MainMenuScreen,
    )).id();

    // Title (using TextComponent)
    let title = TextComponent::spawn(
        commands,
        "📚 TileMania",
        TextStyle::Title,
        TextColorVariant::Accent,
    );
    commands.entity(screen_id).add_child(title);

    // Subtitle (using TextComponent with custom node)
    let subtitle = TextComponent::spawn_with_node(
        commands,
        "Scrabble Learning Game",
        TextStyle::Subheading,
        TextColorVariant::Secondary,
        Node {
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        },
    );
    commands.entity(screen_id).add_child(subtitle);

    // Play button (using new ButtonComponent)
    let play_button = ButtonComponent::spawn(
        commands,
        "▶ Play (SPACE)",
        ButtonSize::Large,
        ButtonVariant::Primary,
        PlayButton,
    );
    commands.entity(screen_id).add_child(play_button);

    // Settings button (using new ButtonComponent)
    let settings_button = ButtonComponent::spawn(
        commands,
        "⚙ Settings (S)",
        ButtonSize::Large,
        ButtonVariant::Secondary,
        SettingsButton,
    );
    commands.entity(screen_id).add_child(settings_button);

    // Instructions (using TextComponent)
    let instructions = TextComponent::spawn_with_node(
        commands,
        "Press SPACE to start | S for Settings | ESC to quit",
        TextStyle::Caption,
        TextColorVariant::Muted,
        Node {
            margin: UiRect::top(Val::Px(40.0)),
            ..default()
        },
    );
    commands.entity(screen_id).add_child(instructions);
}
