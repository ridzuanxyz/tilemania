/// Systems for Stage 1 gameplay logic

use bevy::prelude::*;
use rand::Rng;
use super::components::*;
use super::{Stage1Config, Stage1State};
use super::visuals::{spawn_score_popup, spawn_particle_burst, TileColors, ValidationFlash};
use super::powerups::{ActivePowerUps, get_fall_speed_multiplier};
use crate::plugins::state::GameState;
use crate::lexicon::Lexicon;
use crate::scoring::ScoreCalculator;

const TILE_SIZE: f32 = 64.0;
const SPAWN_INTERVAL: f32 = 2.0; // Spawn new tile every 2 seconds

/// Timer for spawning new tiles
#[derive(Resource)]
pub struct TileSpawnTimer {
    pub timer: Timer,
}

impl Default for TileSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating),
        }
    }
}

/// Spawns falling tiles at intervals
pub fn spawn_falling_tiles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: Local<Option<Timer>>,
    config: Res<Stage1Config>,
    asset_server: Res<AssetServer>,
) {
    // Initialize timer on first run
    if spawn_timer.is_none() {
        *spawn_timer = Some(Timer::from_seconds(SPAWN_INTERVAL, TimerMode::Repeating));
    }

    if let Some(timer) = spawn_timer.as_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            // Randomly select a letter weighted by Scrabble distribution
            let letter = get_weighted_random_letter();

            // Randomly select a column
            let column = rand::thread_rng().gen_range(0..config.column_count);

            // Calculate spawn position
            let x_pos = -400.0 + (column as f32 * 120.0); // Spread across screen
            let y_pos = 400.0; // Top of screen

            // Spawn the tile
            commands.spawn((
                FallingTile {
                    letter,
                    column,
                    speed: config.fall_speed,
                    is_selected: false,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.8, 0.8, 0.9),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                    ..default()
                },
                Text2dBundle {
                    text: Text::from_section(
                        letter.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::srgb(0.1, 0.1, 0.1),
                        },
                    ),
                    transform: Transform::from_xyz(x_pos, y_pos, 1.0),
                    ..default()
                },
            ));
        }
    }
}

/// Updates falling tile positions
pub fn update_falling_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &FallingTile)>,
    time: Res<Time>,
    active_powerups: Res<ActivePowerUps>,
) {
    let speed_multiplier = get_fall_speed_multiplier(&active_powerups);

    for (entity, mut transform, tile) in query.iter_mut() {
        // Move tile downward (with power-up speed modifier)
        transform.translation.y -= tile.speed * speed_multiplier * time.delta_seconds();

        // Despawn if off-screen
        if transform.translation.y < -400.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Handles tile selection via mouse/touch
pub fn handle_tile_selection(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut tile_query: Query<(Entity, &mut FallingTile, &Transform)>,
    mut state: ResMut<Stage1State>,
) {
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // Get mouse position in world coordinates
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // Check if click hit a tile
            for (entity, mut tile, transform) in tile_query.iter_mut() {
                let tile_pos = transform.translation.truncate();
                let distance = tile_pos.distance(world_pos);

                if distance < TILE_SIZE / 2.0 && !tile.is_selected {
                    // Tile selected!
                    tile.is_selected = true;
                    state.selected_tiles.push(entity);

                    // Add visual feedback
                    commands.entity(entity).insert(SelectedTile);

                    info!("Selected tile: {}", tile.letter);
                    break;
                }
            }
        }
    }
}

/// Validates the word when player submits (spacebar or Enter)
pub fn validate_word(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<Stage1State>,
    config: Res<Stage1Config>,
    tile_query: Query<(&FallingTile, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    if !keyboard.just_pressed(KeyCode::Space) && !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }

    if state.selected_tiles.len() != 2 {
        warn!("Need exactly 2 tiles to form a word");
        clear_selection(&mut commands, &mut state);
        return;
    }

    // Build word from selected tiles and get average position for effects
    let mut word = String::new();
    let mut avg_position = Vec3::ZERO;
    let mut tile_count = 0;

    for entity in &state.selected_tiles {
        if let Ok((tile, transform)) = tile_query.get(*entity) {
            word.push(tile.letter);
            avg_position += transform.translation;
            tile_count += 1;
        }
    }

    if tile_count > 0 {
        avg_position /= tile_count as f32;
    }

    // Validate word
    let is_valid = config.two_letter_words.contains(&word.to_uppercase());

    if is_valid {
        info!("✓ Valid word: {}", word);

        // Calculate score
        let calculator = ScoreCalculator::new();
        let points = calculator.calculate_stage1_score(
            &word,
            state.time_remaining_ms,
            config.total_time_ms,
            state.combo_count,
        );

        state.score += points;
        state.combo_count += 1;
        state.words_found.push(word.to_uppercase());

        // Visual feedback for valid word
        spawn_score_popup(&mut commands, &asset_server, avg_position, points, true);
        spawn_particle_burst(&mut commands, avg_position, TileColors::VALID, 12);

        // Add validation flash to tiles before despawning
        for entity in &state.selected_tiles {
            commands.entity(*entity).insert(ValidationFlash {
                is_valid: true,
                duration: 0.3,
                elapsed: 0.0,
            });
        }

        // Despawn selected tiles (after brief delay for flash)
        for entity in &state.selected_tiles {
            commands.entity(*entity).despawn_recursive();
        }

        info!("Score: {} (+{}), Combo: {}", state.score, points, state.combo_count);
    } else {
        warn!("✗ Invalid word: {}", word);
        state.combo_count = 0; // Break combo on invalid word

        // Visual feedback for invalid word
        spawn_score_popup(&mut commands, &asset_server, avg_position, 0, false);

        // Add validation flash to tiles (they stay on screen)
        for entity in &state.selected_tiles {
            commands.entity(*entity).insert(ValidationFlash {
                is_valid: false,
                duration: 0.5,
                elapsed: 0.0,
            });
        }
    }

    clear_selection(&mut commands, &mut state);
}

/// Clears current tile selection
fn clear_selection(commands: &mut Commands, state: &mut ResMut<Stage1State>) {
    for entity in &state.selected_tiles {
        commands.entity(*entity).remove::<SelectedTile>();
    }
    state.selected_tiles.clear();
}

/// Updates score display
pub fn update_score_display(
    mut query: Query<&mut Text, With<ScoreDisplay>>,
    state: Res<Stage1State>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Score: {}", state.score);
    }
}

/// Updates timer display and counts down
pub fn update_timer(
    mut query: Query<&mut Text, With<TimerDisplay>>,
    mut state: ResMut<Stage1State>,
    time: Res<Time>,
    config: Res<Stage1Config>,
) {
    if !state.is_active {
        return;
    }

    // Countdown timer
    let delta_ms = (time.delta_seconds() * 1000.0) as u32;
    state.time_remaining_ms = state.time_remaining_ms.saturating_sub(delta_ms);

    // Update display
    for mut text in query.iter_mut() {
        let seconds = state.time_remaining_ms / 1000;
        text.sections[0].value = format!("Time: {}s", seconds);
    }
}

/// Checks if game is over (time ran out)
pub fn check_game_over(
    state: Res<Stage1State>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if state.time_remaining_ms == 0 && state.is_active {
        info!("Game Over! Final Score: {}", state.score);
        next_state.set(GameState::Results);
    }
}

/// Returns a random letter weighted by Scrabble tile distribution
fn get_weighted_random_letter() -> char {
    let mut rng = rand::thread_rng();
    let roll: u32 = rng.gen_range(0..100);

    // Approximate Scrabble distribution (simplified)
    match roll {
        0..=11 => 'E',   // 12%
        12..=20 => 'A',  // 9%
        21..=28 => 'I',  // 9%
        29..=36 => 'O',  // 8%
        37..=43 => 'N',  // 6%
        44..=49 => 'R',  // 6%
        50..=55 => 'T',  // 6%
        56..=59 => 'L',  // 4%
        60..=63 => 'S',  // 4%
        64..=67 => 'U',  // 4%
        68..=70 => 'D',  // 4%
        71..=73 => 'G',  // 3%
        74..=76 => 'B',  // 2%
        77..=79 => 'C',  // 2%
        80..=82 => 'M',  // 2%
        83..=85 => 'P',  // 2%
        86..=88 => 'F',  // 2%
        89..=90 => 'H',  // 2%
        91..=92 => 'V',  // 2%
        93..=94 => 'W',  // 2%
        95..=96 => 'Y',  // 2%
        97 => 'K',       // 1%
        98 => 'J',       // 1%
        99 => 'X',       // 1%
        _ => 'Q',        // 1%
    }
}
