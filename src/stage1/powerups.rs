/// Power-ups system for Stage 1

use bevy::prelude::*;
use bevy::text::TextStyle;
use bevy::ui::Style;
use rand::Rng;
use super::components::*;
use super::{Stage1Config, Stage1State};

/// Chance of power-up spawning (5% per tile)
const POWERUP_SPAWN_CHANCE: f32 = 0.05;

/// Power-up effects duration (milliseconds)
const SLOW_MOTION_DURATION: u32 = 10_000; // 10 seconds
const BOMB_RADIUS: f32 = 150.0; // Pixels

/// Resource tracking active power-ups
#[derive(Resource, Default)]
pub struct ActivePowerUps {
    pub slow_motion_remaining_ms: u32,
    pub available_powerups: Vec<PowerUp>,
}

/// Marker component for power-up pickup
#[derive(Component)]
pub struct PowerUpPickup {
    pub powerup_type: PowerUp,
}

/// Spawns power-ups randomly on the field
pub fn spawn_powerup_pickups(
    mut commands: Commands,
    mut last_spawn_time: Local<f32>,
    time: Res<Time>,
    config: Res<Stage1Config>,
    asset_server: Res<AssetServer>,
) {
    *last_spawn_time += time.delta_secs();

    // Try to spawn a power-up every 15 seconds
    if *last_spawn_time >= 15.0 {
        *last_spawn_time = 0.0;

        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.7 {
            // 70% chance to spawn
            let powerup_type = match rng.gen_range(0..4) {
                0 => PowerUp::SlowMotion,
                1 => PowerUp::Bomb,
                2 => PowerUp::Shuffle,
                _ => PowerUp::ExtraTime,
            };

            // Random position
            let column = rng.gen_range(0..config.column_count);
            let x_pos = -400.0 + (column as f32 * 120.0);
            let y_pos = rng.gen_range(-200.0..200.0);

            let color = powerup_color(&powerup_type);

            commands.spawn((
                PowerUpPickup { powerup_type },
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(48.0, 48.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 2.0),
                    ..default()
                },
                Text2dBundle {
                    text: Text::from_section(
                        powerup_icon(&powerup_type),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform::from_xyz(x_pos, y_pos, 3.0),
                    ..default()
                },
            ));

            info!("💎 Spawned power-up: {:?}", powerup_type);
        }
    }
}

/// Handles player collecting power-ups
pub fn collect_powerups(
    mut commands: Commands,
    pickup_query: Query<(Entity, &PowerUpPickup, &Transform)>,
    tile_query: Query<&Transform, With<FallingTile>>,
    mut active: ResMut<ActivePowerUps>,
) {
    for (pickup_entity, pickup, pickup_transform) in pickup_query.iter() {
        // Check if any tile is close to the power-up
        for tile_transform in tile_query.iter() {
            let distance = pickup_transform.translation.truncate()
                .distance(tile_transform.translation.truncate());

            if distance < 64.0 {
                // Collected!
                info!("✨ Collected power-up: {:?}", pickup.powerup_type);
                active.available_powerups.push(pickup.powerup_type);
                commands.entity(pickup_entity).despawn_recursive();
                break;
            }
        }
    }
}

/// Handles power-up activation (number keys 1-4)
pub fn activate_powerups(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActivePowerUps>,
    mut state: ResMut<Stage1State>,
    mut config: ResMut<Stage1Config>,
    tile_query: Query<(Entity, &Transform), With<FallingTile>>,
) {
    // Check if player pressed 1, 2, 3, or 4
    let powerup_index = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else {
        None
    };

    if let Some(index) = powerup_index {
        if index < active.available_powerups.len() {
            let powerup = active.available_powerups.remove(index);
            apply_powerup_effect(
                &mut commands,
                powerup,
                &mut active,
                &mut state,
                &mut config,
                &tile_query,
            );
        }
    }
}

/// Applies the effect of a power-up
fn apply_powerup_effect(
    commands: &mut Commands,
    powerup: PowerUp,
    active: &mut ActivePowerUps,
    state: &mut Stage1State,
    config: &mut Stage1Config,
    tile_query: &Query<(Entity, &Transform), With<FallingTile>>,
) {
    match powerup {
        PowerUp::SlowMotion => {
            info!("🐌 Slow Motion activated!");
            active.slow_motion_remaining_ms = SLOW_MOTION_DURATION;
            // Fall speed reduced by 50% (applied in update_falling_tiles)
        }

        PowerUp::Bomb => {
            info!("💣 Bomb activated! Clearing column");
            // Find the column with the most tiles
            let mut column_counts = vec![0; config.column_count];

            for (_, transform) in tile_query.iter() {
                let x = transform.translation.x;
                let column = ((x + 400.0) / 120.0).round() as usize;
                if column < config.column_count {
                    column_counts[column] += 1;
                }
            }

            // Find column with most tiles
            if let Some((max_column, _)) = column_counts.iter()
                .enumerate()
                .max_by_key(|(_, &count)| count)
            {
                // Despawn all tiles in that column
                for (entity, transform) in tile_query.iter() {
                    let x = transform.translation.x;
                    let column = ((x + 400.0) / 120.0).round() as usize;
                    if column == max_column {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                info!("💥 Cleared column {}", max_column);
            }
        }

        PowerUp::Shuffle => {
            info!("🔀 Shuffle activated! Randomizing positions");
            // Collect all tile positions
            let positions: Vec<Vec3> = tile_query.iter()
                .map(|(_, transform)| transform.translation)
                .collect();

            if !positions.is_empty() {
                let mut rng = rand::thread_rng();
                let mut shuffled_positions = positions.clone();

                // Fisher-Yates shuffle
                for i in (1..shuffled_positions.len()).rev() {
                    let j = rng.gen_range(0..=i);
                    shuffled_positions.swap(i, j);
                }

                // Apply shuffled positions to tiles
                for ((entity, _), new_pos) in tile_query.iter().zip(shuffled_positions.iter()) {
                    if let Some(mut entity_commands) = commands.get_entity(entity) {
                        entity_commands.insert(Transform::from_translation(*new_pos));
                    }
                }
            }
        }

        PowerUp::ExtraTime => {
            info!("⏰ Extra Time activated! +10 seconds");
            state.time_remaining_ms += 10_000; // Add 10 seconds
            // Cap at original time limit
            if state.time_remaining_ms > config.total_time_ms {
                state.time_remaining_ms = config.total_time_ms;
            }
        }
    }
}

/// Updates active power-up timers
pub fn update_powerup_timers(
    mut active: ResMut<ActivePowerUps>,
    time: Res<Time>,
) {
    if active.slow_motion_remaining_ms > 0 {
        let delta_ms = (time.delta_secs() * 1000.0) as u32;
        active.slow_motion_remaining_ms = active.slow_motion_remaining_ms.saturating_sub(delta_ms);

        if active.slow_motion_remaining_ms == 0 {
            info!("🐌 Slow Motion ended");
        }
    }
}

/// Returns the current fall speed multiplier (affected by slow motion)
pub fn get_fall_speed_multiplier(active: &ActivePowerUps) -> f32 {
    if active.slow_motion_remaining_ms > 0 {
        0.5 // 50% speed
    } else {
        1.0 // Normal speed
    }
}

/// Returns color for power-up type
fn powerup_color(powerup: &PowerUp) -> Color {
    match powerup {
        PowerUp::SlowMotion => Color::srgb(0.5, 0.7, 1.0),   // Light blue
        PowerUp::Bomb => Color::srgb(1.0, 0.5, 0.3),         // Orange
        PowerUp::Shuffle => Color::srgb(0.8, 0.5, 1.0),      // Purple
        PowerUp::ExtraTime => Color::srgb(0.5, 1.0, 0.5),    // Green
    }
}

/// Returns icon text for power-up type
fn powerup_icon(powerup: &PowerUp) -> &'static str {
    match powerup {
        PowerUp::SlowMotion => "🐌",
        PowerUp::Bomb => "💣",
        PowerUp::Shuffle => "🔀",
        PowerUp::ExtraTime => "⏰",
    }
}

/// Spawns UI display for available power-ups
pub fn spawn_powerup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(20.0),
                left: Val::Px(20.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Power-ups (1-4):",
                    TextStyle {
                        font: font.clone(),
                        font_size: 20.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                    },
                ),
                PowerUpDisplayMarker,
            ));
        });
}

/// Marker for power-up display UI
#[derive(Component)]
pub struct PowerUpDisplayMarker;

/// Updates the power-up UI display
pub fn update_powerup_display(
    mut commands: Commands,
    active: Res<ActivePowerUps>,
    display_query: Query<Entity, With<PowerUpDisplayMarker>>,
    asset_server: Res<AssetServer>,
) {
    if !active.is_changed() {
        return;
    }

    // Rebuild power-up display
    for entity in display_query.iter() {
        commands.entity(entity).despawn_descendants();

        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.with_children(|parent| {
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");

                for (i, powerup) in active.available_powerups.iter().enumerate() {
                    let icon = powerup_icon(powerup);
                    let color = powerup_color(powerup);

                    parent.spawn(TextBundle::from_section(
                        format!("{}. {} {:?}", i + 1, icon, powerup),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color,
                        },
                    ));
                }

                // Show slow motion timer if active
                if active.slow_motion_remaining_ms > 0 {
                    let seconds = active.slow_motion_remaining_ms / 1000;
                    parent.spawn(TextBundle::from_section(
                        format!("🐌 Slow Motion: {}s", seconds),
                        TextStyle {
                            font: font.clone(),
                            font_size: 18.0,
                            color: Color::srgb(0.5, 0.7, 1.0),
                        },
                    ));
                }
            });
        }
    }
}
