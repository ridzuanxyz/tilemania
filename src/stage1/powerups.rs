/// Power-ups system for Stage 1

use bevy::prelude::*;
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
    pub assigned_number: u8,
}

/// Spawns power-ups randomly on the field
pub fn spawn_powerup_pickups(
    mut commands: Commands,
    mut last_spawn_time: Local<f32>,
    time: Res<Time>,
    config: Res<Stage1Config>,
    asset_server: Res<AssetServer>,
    pickup_query: Query<&PowerUpPickup>,
) {
    *last_spawn_time += time.delta_secs();

    // Try to spawn a power-up every 15 seconds
    if *last_spawn_time >= 15.0 {
        *last_spawn_time = 0.0;

        // Check if we already have 4 bonuses on screen
        if pickup_query.iter().count() >= 4 {
            return;
        }

        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.7 {
            // 70% chance to spawn
            let powerup_type = match rng.gen_range(0..4) {
                0 => PowerUp::SlowMotion,
                1 => PowerUp::Bomb,
                2 => PowerUp::Shuffle,
                _ => PowerUp::ExtraTime,
            };

            // Horizontal sliding: spawn from left edge at random height
            let x_pos = -450.0;
            let y_pos = rng.gen_range(-250.0..250.0);

            // Calculate sequential number (1-4)
            let assigned_number = calculate_next_number(&pickup_query);

            let color = powerup_color(&powerup_type);

            // Spawn bonus tile with text as children
            commands.spawn((
                PowerUpPickup {
                    powerup_type,
                    assigned_number,
                },
                super::components::Velocity {
                    x: 150.0,  // Horizontal speed (pixels per second)
                    y: 0.0,
                },
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(64.0, 64.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, y_pos, 2.0),
                    ..default()
                },
            ))
            .with_children(|parent| {
                // Power-up icon
                parent.spawn((
                    Text2d::new(powerup_icon(&powerup_type)),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 10.0, 1.0),  // Relative to parent
                ));

                // Number display
                parent.spawn((
                    Text2d::new(format!("{}", assigned_number)),
                    TextFont {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 48.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, -15.0, 1.0),  // Relative to parent, below icon
                ));
            });

            info!("💎 Spawned power-up #{}: {:?}", assigned_number, powerup_type);
        }
    }
}

/// Calculate the next sequential number for a new powerup
fn calculate_next_number(pickup_query: &Query<&PowerUpPickup>) -> u8 {
    let mut used_numbers = pickup_query
        .iter()
        .map(|p| p.assigned_number)
        .collect::<Vec<_>>();

    used_numbers.sort();

    // Find first available number (1-4)
    for n in 1..=4 {
        if !used_numbers.contains(&n) {
            return n;
        }
    }

    // Fallback (shouldn't happen if we limit to 4)
    1
}

/// Moves power-ups horizontally across the screen
pub fn move_powerup_pickups(
    mut query: Query<(&mut Transform, &super::components::Velocity), With<PowerUpPickup>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_secs();
    }
}

/// Despawns power-ups that have moved off-screen
pub fn despawn_offscreen_powerups(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &PowerUpPickup)>,
) {
    for (entity, transform, pickup) in query.iter() {
        // Despawn if beyond right edge
        if transform.translation.x > 450.0 {
            commands.entity(entity).despawn_recursive();
            info!("💨 Power-up #{} went off-screen", pickup.assigned_number);
        }
    }
}

/// Handles keyboard-based power-up collection (1-4 keys and spacebar)
pub fn collect_powerups(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    pickup_query: Query<(Entity, &PowerUpPickup)>,
    mut active: ResMut<ActivePowerUps>,
    mut state: ResMut<Stage1State>,
    mut config: ResMut<Stage1Config>,
    tile_query: Query<(Entity, &Transform), With<FallingTile>>,
) {
    // Check which key was pressed
    let target_number = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(2)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(3)
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        Some(4)
    } else if keyboard.just_pressed(KeyCode::Space) {
        // Spacebar collects lowest numbered bonus
        let mut numbers: Vec<u8> = pickup_query.iter().map(|(_, p)| p.assigned_number).collect();
        numbers.sort();
        numbers.first().copied()
    } else {
        None
    };

    if let Some(number) = target_number {
        // Find the powerup with matching number
        for (entity, pickup) in pickup_query.iter() {
            if pickup.assigned_number == number {
                info!("✨ Collected power-up #{}: {:?}", number, pickup.powerup_type);

                // Apply effect immediately
                apply_powerup_effect(
                    &mut commands,
                    pickup.powerup_type,
                    &mut active,
                    &mut state,
                    &mut config,
                    &tile_query,
                );

                // Despawn the collected powerup
                commands.entity(entity).despawn_recursive();
                break;
            }
        }
    }
}

/// Renumbers remaining powerups after collection/despawn
pub fn renumber_powerups(
    mut pickup_query: Query<(Entity, &mut PowerUpPickup), Changed<PowerUpPickup>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut Text2d>,
) {
    // Only run if powerups changed (collected/spawned)
    if pickup_query.is_empty() {
        return;
    }

    // Collect all powerups with their current numbers
    let mut all_pickups: Vec<(Entity, u8)> = pickup_query
        .iter()
        .map(|(entity, pickup)| (entity, pickup.assigned_number))
        .collect();

    // Sort by current assigned number
    all_pickups.sort_by_key(|(_, num)| *num);

    // Reassign sequential numbers (1, 2, 3, 4) and update display
    for (new_number, (entity, _)) in all_pickups.iter().enumerate() {
        let new_num = (new_number + 1) as u8;

        if let Ok((_, mut pickup)) = pickup_query.get_mut(*entity) {
            if pickup.assigned_number != new_num {
                pickup.assigned_number = new_num;

                // Update the number text display (second child of the powerup entity)
                if let Ok(children) = children_query.get(*entity) {
                    // Second child is the number text (first is icon)
                    if children.len() >= 2 {
                        if let Ok(mut text) = text_query.get_mut(children[1]) {
                            **text = format!("{}", new_num);
                            info!("Renumbered powerup to #{}", new_num);
                        }
                    }
                }
            }
        }
    }
}

/// Handles player collecting power-ups (OLD PROXIMITY-BASED - DISABLED)
pub fn collect_powerups_old(
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
            node: Node {
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
                Text::new("Bonuses (1-4 or SPACE):"),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                PowerUpDisplayMarker,
            ));
        });
}

/// Marker for power-up display UI
#[derive(Component)]
pub struct PowerUpDisplayMarker;

/// Updates the power-up UI display to show on-screen bonuses
pub fn update_powerup_display(
    mut commands: Commands,
    pickup_query: Query<&PowerUpPickup, Changed<PowerUpPickup>>,
    all_pickups: Query<&PowerUpPickup>,
    display_query: Query<Entity, With<PowerUpDisplayMarker>>,
    asset_server: Res<AssetServer>,
    active: Res<ActivePowerUps>,
) {
    // Only update if pickups changed (added, removed, or renumbered)
    if pickup_query.is_empty() && !pickup_query.iter().count() > 0 {
        // Check if we need to update anyway
        if all_pickups.is_empty() {
            return;
        }
    }

    // Rebuild power-up display
    for entity in display_query.iter() {
        commands.entity(entity).despawn_descendants();

        if let Some(mut entity_commands) = commands.get_entity(entity) {
            entity_commands.with_children(|parent| {
                let font = asset_server.load("fonts/FiraSans-Bold.ttf");

                // Collect and sort on-screen bonuses by number
                let mut bonuses: Vec<(u8, PowerUp)> = all_pickups
                    .iter()
                    .map(|p| (p.assigned_number, p.powerup_type))
                    .collect();
                bonuses.sort_by_key(|(num, _)| *num);

                // Display each bonus
                for (num, powerup) in bonuses.iter() {
                    let icon = powerup_icon(powerup);
                    let color = powerup_color(powerup);

                    parent.spawn((
                        Text::new(format!("{}. {} {:?}", num, icon, powerup)),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(color),
                    ));
                }

                // Show slow motion timer if active
                if active.slow_motion_remaining_ms > 0 {
                    let seconds = active.slow_motion_remaining_ms / 1000;
                    parent.spawn((
                        Text::new(format!("🐌 Slow Motion: {}s", seconds)),
                        TextFont {
                            font: font.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.5, 0.7, 1.0)),
                    ));
                }
            });
        }
    }
}
