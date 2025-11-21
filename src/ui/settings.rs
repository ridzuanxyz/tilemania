use bevy::prelude::*;
use crate::plugins::state::GameState;
use crate::plugins::settings::GameSettings;
use super::keyboard_nav::{KeyboardFocus, KeyboardNavigable};

#[derive(Component)]
pub struct SettingsScreen;

#[derive(Component)]
pub enum SettingButton {
    MusicToggle,
    SfxToggle,
    MusicVolumeUp,
    MusicVolumeDown,
    SfxVolumeUp,
    SfxVolumeDown,
    DictionaryCycle,
    TimerCycle,
    DifficultyCycle,
    SaveSettings,
    BackToMenu,
}

#[derive(Component)]
pub struct SettingLabel {
    pub setting_type: SettingType,
}

#[derive(Clone)]
pub enum SettingType {
    MusicEnabled,
    SfxEnabled,
    MusicVolume,
    SfxVolume,
    Dictionary,
    Timer,
    Difficulty,
}

pub fn update_settings(
    mut commands: Commands,
    state: Res<State<GameState>>,
    query: Query<Entity, With<SettingsScreen>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<GameSettings>,
    asset_server: Res<AssetServer>,
    focus: Option<ResMut<KeyboardFocus>>,
    mut label_query: Query<(&SettingLabel, &mut Text)>,
) {
    if *state.get() == GameState::Settings {
        let is_first_frame = query.is_empty();

        if is_first_frame {
            spawn_settings_ui(&mut commands, &settings, &asset_server);
            // Initialize keyboard focus with 9 items (7 settings + 2 buttons)
            commands.insert_resource(KeyboardFocus::new(9));
            // Skip keyboard navigation this frame - resource won't be available until next frame
            return;
        }

        // Debug: Log ALL keyboard inputs to diagnose arrow key issue
        for key in keyboard.get_just_pressed() {
            info!("🔍 SETTINGS KEY PRESSED: {:?}", key);
        }

        // Handle keyboard navigation
        if let Some(mut focus) = focus {
            // Arrow key navigation
            // WSL2/X11 bug workaround: Arrow DOWN → NumpadEnter, Arrow UP → Lang3
            if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW) ||
               keyboard.just_pressed(KeyCode::Lang3) {
                focus.move_up();
            }
            if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) ||
               keyboard.just_pressed(KeyCode::NumpadEnter) {
                focus.move_down();
            }

            // Left/Right arrows for direct adjustment
            // WSL2/X11 bug: Arrow LEFT → Convert, Arrow RIGHT → NonConvert
            if let Some(focused_idx) = focus.focused_index {
                let mut changed = false;

                if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) ||
                   keyboard.just_pressed(KeyCode::Convert) {
                    changed = handle_left_arrow(focused_idx, &mut settings);
                }

                if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) ||
                   keyboard.just_pressed(KeyCode::NonConvert) {
                    changed = handle_right_arrow(focused_idx, &mut settings);
                }

                if changed {
                    update_labels(&settings, &mut label_query);
                }

                // Handle Enter/Space for activation (settings adjustments or navigation)
                if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
                    let changed = handle_enter_activation(focused_idx, &mut settings, &mut next_state);
                    if changed {
                        update_labels(&settings, &mut label_query);
                    }
                }
            }
        }

        // Keyboard shortcuts: ESC or Backspace to return to main menu
        if keyboard.just_pressed(KeyCode::Escape) || keyboard.just_pressed(KeyCode::Backspace) {
            next_state.set(GameState::MainMenu);
        }
    } else {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands.remove_resource::<KeyboardFocus>();
    }
}

pub fn handle_setting_buttons(
    mut interaction_query: Query<
        (&Interaction, &SettingButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<GameState>>,
    mut label_query: Query<(&SettingLabel, &mut Text)>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                SettingButton::MusicToggle => {
                    settings.audio.music_enabled = !settings.audio.music_enabled;
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::SfxToggle => {
                    settings.audio.sfx_enabled = !settings.audio.sfx_enabled;
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::MusicVolumeUp => {
                    settings.audio.music_volume = (settings.audio.music_volume + 0.1).min(1.0);
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::MusicVolumeDown => {
                    settings.audio.music_volume = (settings.audio.music_volume - 0.1).max(0.0);
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::SfxVolumeUp => {
                    settings.audio.sfx_volume = (settings.audio.sfx_volume + 0.1).min(1.0);
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::SfxVolumeDown => {
                    settings.audio.sfx_volume = (settings.audio.sfx_volume - 0.1).max(0.0);
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::DictionaryCycle => {
                    settings.gameplay.dictionary = match settings.gameplay.dictionary.as_str() {
                        "TML" => "RE-ENABLE".to_string(),
                        "RE-ENABLE" => "ENABLE".to_string(),
                        "ENABLE" => "CSW24".to_string(),
                        _ => "TML".to_string(),
                    };
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::TimerCycle => {
                    settings.gameplay.default_time_limit = match settings.gameplay.default_time_limit {
                        600 => 900,      // 10:00 -> 15:00
                        900 => 1500,     // 15:00 -> 25:00
                        1500 => 1800,    // 25:00 -> 30:00
                        1800 => 0,       // 30:00 -> Unlimited
                        _ => 600,        // Unlimited -> 10:00
                    };
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::DifficultyCycle => {
                    settings.gameplay.default_difficulty = match settings.gameplay.default_difficulty {
                        1 => 2,
                        2 => 3,
                        3 => 4,
                        4 => 5,
                        _ => 1,
                    };
                    update_labels(&settings, &mut label_query);
                }
                SettingButton::SaveSettings => {
                    if let Err(e) = settings.save() {
                        error!("Failed to save settings: {}", e);
                    } else {
                        info!("✅ Settings saved successfully");
                    }
                }
                SettingButton::BackToMenu => {
                    next_state.set(GameState::MainMenu);
                }
            }
        }
    }
}

/// Handle left arrow key for direct setting adjustment
fn handle_left_arrow(focused_idx: usize, settings: &mut GameSettings) -> bool {
    match focused_idx {
        0 => {
            // Music Toggle
            settings.audio.music_enabled = !settings.audio.music_enabled;
            true
        }
        1 => {
            // Music Volume - Decrease
            settings.audio.music_volume = (settings.audio.music_volume - 0.1).max(0.0);
            true
        }
        2 => {
            // SFX Toggle
            settings.audio.sfx_enabled = !settings.audio.sfx_enabled;
            true
        }
        3 => {
            // SFX Volume - Decrease
            settings.audio.sfx_volume = (settings.audio.sfx_volume - 0.1).max(0.0);
            true
        }
        4 => {
            // Dictionary - Cycle Backward
            settings.gameplay.dictionary = match settings.gameplay.dictionary.as_str() {
                "TML" => "CSW24".to_string(),
                "CSW24" => "ENABLE".to_string(),
                "ENABLE" => "RE-ENABLE".to_string(),
                _ => "TML".to_string(),
            };
            true
        }
        5 => {
            // Timer - Cycle Backward
            settings.gameplay.default_time_limit = match settings.gameplay.default_time_limit {
                600 => 0,        // 10:00 -> Unlimited
                900 => 600,      // 15:00 -> 10:00
                1500 => 900,     // 25:00 -> 15:00
                1800 => 1500,    // 30:00 -> 25:00
                _ => 1800,       // Unlimited -> 30:00
            };
            true
        }
        6 => {
            // Difficulty - Cycle Backward
            settings.gameplay.default_difficulty = match settings.gameplay.default_difficulty {
                1 => 5,
                2 => 1,
                3 => 2,
                4 => 3,
                5 => 4,
                _ => 3,
            };
            true
        }
        _ => false, // Indices 7-8 (buttons) don't respond to left/right
    }
}

/// Handle right arrow key for direct setting adjustment
fn handle_right_arrow(focused_idx: usize, settings: &mut GameSettings) -> bool {
    match focused_idx {
        0 => {
            // Music Toggle
            settings.audio.music_enabled = !settings.audio.music_enabled;
            true
        }
        1 => {
            // Music Volume - Increase
            settings.audio.music_volume = (settings.audio.music_volume + 0.1).min(1.0);
            true
        }
        2 => {
            // SFX Toggle
            settings.audio.sfx_enabled = !settings.audio.sfx_enabled;
            true
        }
        3 => {
            // SFX Volume - Increase
            settings.audio.sfx_volume = (settings.audio.sfx_volume + 0.1).min(1.0);
            true
        }
        4 => {
            // Dictionary - Cycle Forward
            settings.gameplay.dictionary = match settings.gameplay.dictionary.as_str() {
                "TML" => "RE-ENABLE".to_string(),
                "RE-ENABLE" => "ENABLE".to_string(),
                "ENABLE" => "CSW24".to_string(),
                _ => "TML".to_string(),
            };
            true
        }
        5 => {
            // Timer - Cycle Forward
            settings.gameplay.default_time_limit = match settings.gameplay.default_time_limit {
                600 => 900,      // 10:00 -> 15:00
                900 => 1500,     // 15:00 -> 25:00
                1500 => 1800,    // 25:00 -> 30:00
                1800 => 0,       // 30:00 -> Unlimited
                _ => 600,        // Unlimited -> 10:00
            };
            true
        }
        6 => {
            // Difficulty - Cycle Forward
            settings.gameplay.default_difficulty = match settings.gameplay.default_difficulty {
                1 => 2,
                2 => 3,
                3 => 4,
                4 => 5,
                _ => 1,
            };
            true
        }
        _ => false, // Indices 7-8 (buttons) don't respond to left/right
    }
}

fn handle_enter_activation(focused_idx: usize, settings: &mut GameSettings, next_state: &mut ResMut<NextState<GameState>>) -> bool {
    match focused_idx {
        0 => {
            // Music Toggle - same as left/right
            settings.audio.music_enabled = !settings.audio.music_enabled;
            true
        }
        1 => {
            // Music Volume - Increase (same as right arrow)
            settings.audio.music_volume = (settings.audio.music_volume + 0.1).min(1.0);
            true
        }
        2 => {
            // SFX Toggle - same as left/right
            settings.audio.sfx_enabled = !settings.audio.sfx_enabled;
            true
        }
        3 => {
            // SFX Volume - Increase (same as right arrow)
            settings.audio.sfx_volume = (settings.audio.sfx_volume + 0.1).min(1.0);
            true
        }
        4 => {
            // Dictionary - Cycle Forward
            settings.gameplay.dictionary = match settings.gameplay.dictionary.as_str() {
                "TML" => "RE-ENABLE".to_string(),
                "RE-ENABLE" => "ENABLE".to_string(),
                "ENABLE" => "CSW24".to_string(),
                _ => "TML".to_string(),
            };
            true
        }
        5 => {
            // Timer - Cycle Forward
            settings.gameplay.default_time_limit = match settings.gameplay.default_time_limit {
                600 => 900,      // 10:00 -> 15:00
                900 => 1500,     // 15:00 -> 25:00
                1500 => 1800,    // 25:00 -> 30:00
                1800 => 0,       // 30:00 -> Unlimited
                _ => 600,        // Unlimited -> 10:00
            };
            true
        }
        6 => {
            // Difficulty - Cycle Forward
            settings.gameplay.default_difficulty = match settings.gameplay.default_difficulty {
                1 => 2,
                2 => 3,
                3 => 4,
                4 => 5,
                _ => 1,
            };
            true
        }
        7 => {
            // Save Settings button (auto-saved, so just acknowledge)
            false
        }
        8 => {
            // Back to Menu button
            next_state.set(GameState::MainMenu);
            false
        }
        _ => false,
    }
}

fn update_labels(settings: &GameSettings, label_query: &mut Query<(&SettingLabel, &mut Text)>) {
    for (label, mut text) in label_query.iter_mut() {
        **text = match &label.setting_type {
            SettingType::MusicEnabled => {
                format!("🎵 Music: {}", if settings.audio.music_enabled { "ON" } else { "OFF" })
            }
            SettingType::SfxEnabled => {
                format!("🔊 Sound Effects: {}", if settings.audio.sfx_enabled { "ON" } else { "OFF" })
            }
            SettingType::MusicVolume => {
                format!("Music Volume: {}%", (settings.audio.music_volume * 100.0) as u8)
            }
            SettingType::SfxVolume => {
                format!("SFX Volume: {}%", (settings.audio.sfx_volume * 100.0) as u8)
            }
            SettingType::Dictionary => {
                format!("📚 Dictionary: {}", settings.gameplay.dictionary)
            }
            SettingType::Timer => {
                if settings.gameplay.default_time_limit == 0 {
                    "⏱ Timer: Unlimited".to_string()
                } else {
                    let minutes = settings.gameplay.default_time_limit / 60;
                    let seconds = settings.gameplay.default_time_limit % 60;
                    format!("⏱ Timer: {:02}:{:02}", minutes, seconds)
                }
            }
            SettingType::Difficulty => {
                let diff_name = match settings.gameplay.default_difficulty {
                    1 => "Very Easy",
                    2 => "Easy",
                    3 => "Medium",
                    4 => "Hard",
                    5 => "Very Hard",
                    _ => "Medium",
                };
                format!("🎮 Difficulty: {}", diff_name)
            }
        };
    }
}

fn spawn_settings_ui(commands: &mut Commands, settings: &GameSettings, asset_server: &AssetServer) {
    let font_bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let font_medium: Handle<Font> = asset_server.load("fonts/FiraSans-Medium.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(20.0),
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.18)),
            SettingsScreen,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("⚙ Settings"),
                TextFont {
                    font: font_bold.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 1.0)),
            ));

            // Settings container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(30.0)),
                    ..default()
                })
                .with_children(|container| {
                    // Music Toggle (index 0)
                    spawn_toggle_row(
                        container,
                        &font_medium,
                        SettingType::MusicEnabled,
                        SettingButton::MusicToggle,
                        settings,
                        0,
                    );

                    // Music Volume (index 1)
                    spawn_volume_row(
                        container,
                        &font_medium,
                        SettingType::MusicVolume,
                        SettingButton::MusicVolumeDown,
                        SettingButton::MusicVolumeUp,
                        settings,
                        1,
                    );

                    // SFX Toggle (index 2)
                    spawn_toggle_row(
                        container,
                        &font_medium,
                        SettingType::SfxEnabled,
                        SettingButton::SfxToggle,
                        settings,
                        2,
                    );

                    // SFX Volume (index 3)
                    spawn_volume_row(
                        container,
                        &font_medium,
                        SettingType::SfxVolume,
                        SettingButton::SfxVolumeDown,
                        SettingButton::SfxVolumeUp,
                        settings,
                        3,
                    );

                    // Dictionary Cycle (index 4)
                    spawn_cycle_row(
                        container,
                        &font_medium,
                        SettingType::Dictionary,
                        SettingButton::DictionaryCycle,
                        settings,
                        4,
                    );

                    // Timer Cycle (index 5)
                    spawn_cycle_row(
                        container,
                        &font_medium,
                        SettingType::Timer,
                        SettingButton::TimerCycle,
                        settings,
                        5,
                    );

                    // Difficulty Cycle (index 6)
                    spawn_cycle_row(
                        container,
                        &font_medium,
                        SettingType::Difficulty,
                        SettingButton::DifficultyCycle,
                        settings,
                        6,
                    );
                });

            // Action buttons
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|buttons| {
                    // Save button (index 7)
                    spawn_action_button(
                        buttons,
                        &font_bold,
                        "💾 Save",
                        SettingButton::SaveSettings,
                        Color::srgb(0.2, 0.6, 0.3),
                        7,
                    );

                    // Back button (index 8)
                    spawn_action_button(
                        buttons,
                        &font_bold,
                        "← Back",
                        SettingButton::BackToMenu,
                        Color::srgb(0.3, 0.3, 0.4),
                        8,
                    );
                });

            // Instructions
            parent.spawn((
                Text::new("↑↓: Navigate | ←→: Adjust | Enter: Select | Backspace: Back"),
                TextFont {
                    font: font_medium.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(10.0)),
                    ..default()
                },
            ));
        });
}

fn spawn_toggle_row(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    setting_type: SettingType,
    button_type: SettingButton,
    settings: &GameSettings,
    index: usize,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            width: Val::Px(500.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|row| {
            // Label
            let label_text = match &setting_type {
                SettingType::MusicEnabled => {
                    format!("🎵 Music: {}", if settings.audio.music_enabled { "ON" } else { "OFF" })
                }
                SettingType::SfxEnabled => {
                    format!("🔊 Sound Effects: {}", if settings.audio.sfx_enabled { "ON" } else { "OFF" })
                }
                _ => String::new(),
            };

            row.spawn((
                Text::new(label_text),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                SettingLabel {
                    setting_type: setting_type.clone(),
                },
            ));

            // Toggle button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.25, 0.25, 0.35)),
                button_type,
                KeyboardNavigable { index },
                BorderColor(Color::NONE),
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Toggle"),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn spawn_volume_row(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    setting_type: SettingType,
    button_down: SettingButton,
    button_up: SettingButton,
    settings: &GameSettings,
    index: usize,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(10.0),
            width: Val::Px(500.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|row| {
            // Label
            let label_text = match &setting_type {
                SettingType::MusicVolume => {
                    format!("Music Volume: {}%", (settings.audio.music_volume * 100.0) as u8)
                }
                SettingType::SfxVolume => {
                    format!("SFX Volume: {}%", (settings.audio.sfx_volume * 100.0) as u8)
                }
                _ => String::new(),
            };

            row.spawn((
                Text::new(label_text),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                SettingLabel {
                    setting_type: setting_type.clone(),
                },
            ));

            // Volume controls container (must be a Button for keyboard navigation)
            row.spawn((
                Button,
                Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    border: UiRect::all(Val::Px(3.0)),
                    padding: UiRect::all(Val::Px(2.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)), // Transparent background
                BorderColor(Color::NONE), // Will be updated by focus system
                KeyboardNavigable { index },
            ))
            .with_children(|controls| {
                // Decrease button
                controls
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(45.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                        BorderColor(Color::NONE),
                        button_down,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("-"),
                            TextFont {
                                font: font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });

                // Increase button
                controls
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(45.0),
                            height: Val::Px(40.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.2, 0.4, 0.2)),
                        BorderColor(Color::NONE),
                        button_up,
                    ))
                    .with_children(|button| {
                        button.spawn((
                            Text::new("+"),
                            TextFont {
                                font: font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            });
        });
}

fn spawn_cycle_row(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    setting_type: SettingType,
    button_type: SettingButton,
    settings: &GameSettings,
    index: usize,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(20.0),
            width: Val::Px(500.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .with_children(|row| {
            // Label
            let label_text = match &setting_type {
                SettingType::Dictionary => {
                    format!("📚 Dictionary: {}", settings.gameplay.dictionary)
                }
                SettingType::Timer => {
                    if settings.gameplay.default_time_limit == 0 {
                        "⏱ Timer: Unlimited".to_string()
                    } else {
                        let minutes = settings.gameplay.default_time_limit / 60;
                        let seconds = settings.gameplay.default_time_limit % 60;
                        format!("⏱ Timer: {:02}:{:02}", minutes, seconds)
                    }
                }
                SettingType::Difficulty => {
                    let diff_name = match settings.gameplay.default_difficulty {
                        1 => "Very Easy",
                        2 => "Easy",
                        3 => "Medium",
                        4 => "Hard",
                        5 => "Very Hard",
                        _ => "Medium",
                    };
                    format!("🎮 Difficulty: {}", diff_name)
                }
                _ => String::new(),
            };

            row.spawn((
                Text::new(label_text),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                SettingLabel {
                    setting_type: setting_type.clone(),
                },
            ));

            // Cycle button
            row.spawn((
                Button,
                Node {
                    width: Val::Px(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.25, 0.25, 0.35)),
                button_type,
                KeyboardNavigable { index },
                BorderColor(Color::NONE),
            ))
            .with_children(|button| {
                button.spawn((
                    Text::new("Change"),
                    TextFont {
                        font: font.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        });
}

fn spawn_action_button(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    button_type: SettingButton,
    color: Color,
    index: usize,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(color),
            button_type,
            KeyboardNavigable { index },
            BorderColor(Color::NONE),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font: font.clone(),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}
