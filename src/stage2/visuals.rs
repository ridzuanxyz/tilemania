/// Visual effects for Stage 2 (Match-3 gameplay)

use bevy::prelude::*;
use super::components::*;

/// Color scheme for Stage 2 tiles
pub struct TileColors;

impl TileColors {
    pub const NORMAL: Color = Color::srgb(0.85, 0.85, 0.95);
    pub const SELECTED: Color = Color::srgb(1.0, 0.95, 0.4); // Yellow glow
    pub const MATCHED: Color = Color::srgb(0.3, 0.9, 0.3); // Green flash
    pub const HOVER: Color = Color::srgb(0.95, 0.95, 1.0);
    pub const LOCKED: Color = Color::srgb(0.6, 0.6, 0.65); // Grayed out
}

/// Component for tiles that are currently animating a match
#[derive(Component)]
pub struct MatchAnimation {
    pub timer: Timer,
    pub flash_count: u32,
    pub max_flashes: u32,
}

impl Default for MatchAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            flash_count: 0,
            max_flashes: 6, // 3 full flashes (on-off-on-off-on-off)
        }
    }
}

/// Component for score popup text
#[derive(Component)]
pub struct ScorePopup {
    pub lifetime: Timer,
    pub initial_y: f32,
    pub rise_speed: f32, // pixels per second
}

impl Default for ScorePopup {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            initial_y: 0.0,
            rise_speed: 100.0,
        }
    }
}

/// Component for particle effects
#[derive(Component)]
pub struct Particle {
    pub lifetime: Timer,
    pub velocity: Vec2,
    pub initial_scale: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            lifetime: Timer::from_seconds(0.8, TimerMode::Once),
            velocity: Vec2::ZERO,
            initial_scale: 1.0,
        }
    }
}

/// Update tile visual states based on selection/matching
pub fn update_tile_visuals(
    mut tile_query: Query<(&GridTile, &mut Sprite), Without<MatchAnimation>>,
) {
    for (tile, mut sprite) in tile_query.iter_mut() {
        if tile.is_matched {
            sprite.color = TileColors::MATCHED;
        } else if tile.is_selected {
            sprite.color = TileColors::SELECTED;
        } else {
            sprite.color = TileColors::NORMAL;
        }
    }
}

/// Update match animations (flashing effect)
pub fn update_match_animations(
    mut commands: Commands,
    time: Res<Time>,
    mut anim_query: Query<(Entity, &mut MatchAnimation, &mut Sprite)>,
) {
    for (entity, mut anim, mut sprite) in anim_query.iter_mut() {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            anim.flash_count += 1;

            // Toggle between matched and normal color
            if anim.flash_count % 2 == 0 {
                sprite.color = TileColors::MATCHED;
            } else {
                sprite.color = TileColors::NORMAL;
            }

            // Remove animation after max flashes
            if anim.flash_count >= anim.max_flashes {
                commands.entity(entity).remove::<MatchAnimation>();
                // Tile will be removed by cascade system
            }
        }
    }
}

/// Update cascade animations (falling tiles)
pub fn update_cascade_animations(
    time: Res<Time>,
    mut tile_query: Query<(&mut Transform, &mut CascadingTile, &GridTile)>,
) {
    const GRID_CELL_SIZE: f32 = 64.0;
    const GRID_OFFSET_X: f32 = -256.0; // Center 8x8 grid
    const GRID_OFFSET_Y: f32 = 256.0;

    for (mut transform, mut cascade, tile) in tile_query.iter_mut() {
        let target_x = GRID_OFFSET_X + (tile.grid_pos.1 as f32 * GRID_CELL_SIZE);
        let target_y = GRID_OFFSET_Y - (tile.grid_pos.0 as f32 * GRID_CELL_SIZE);

        // Move towards target position
        let target_pos = Vec3::new(target_x, target_y, 0.0);
        let direction = (target_pos - transform.translation).normalize_or_zero();
        let move_distance = cascade.speed * time.delta_secs();

        transform.translation += direction * move_distance;

        // Snap to target if close enough
        if transform.translation.distance(target_pos) < 2.0 {
            transform.translation = target_pos;
            // Animation complete - system will remove component
        }
    }
}

/// Spawn score popup at tile position
pub fn spawn_score_popup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    score: u32,
) {
    commands.spawn((
        Text2d::new(format!("+{}", score)),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.95, 0.3)),
        Transform::from_translation(position + Vec3::new(0.0, 30.0, 10.0)),
        ScorePopup {
            initial_y: position.y,
            ..default()
        },
    ));
}

/// Update score popups (rise and fade)
pub fn update_score_popups(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_query: Query<(Entity, &mut Transform, &mut Text, &mut ScorePopup)>,
) {
    for (entity, mut transform, mut text, mut popup) in popup_query.iter_mut() {
        popup.lifetime.tick(time.delta());

        // Rise upward
        transform.translation.y += popup.rise_speed * time.delta_secs();

        // Fade out
        let alpha = 1.0 - popup.lifetime.fraction();
        for section in &mut text.sections {
            let current_color = section.style.color;
            section.style.color = Color::srgba(
                current_color.to_srgba().red,
                current_color.to_srgba().green,
                current_color.to_srgba().blue,
                alpha,
            );
        }

        // Remove when finished
        if popup.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawn particle burst at position
pub fn spawn_particle_burst(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
) {
    const PARTICLE_COUNT: u32 = 12;
    const PARTICLE_SPEED: f32 = 150.0;
    const PARTICLE_SIZE: f32 = 8.0;

    for i in 0..PARTICLE_COUNT {
        let angle = (i as f32 / PARTICLE_COUNT as f32) * std::f32::consts::TAU;
        let velocity = Vec2::new(angle.cos(), angle.sin()) * PARTICLE_SPEED;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(PARTICLE_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(position + Vec3::new(0.0, 0.0, 5.0)),
                ..default()
            },
            Particle {
                velocity,
                initial_scale: 1.0,
                ..default()
            },
        ));
    }
}

/// Update particles (move, fade, shrink)
pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        // Move based on velocity
        let velocity_3d = Vec3::new(particle.velocity.x, particle.velocity.y, 0.0);
        transform.translation += velocity_3d * time.delta_secs();

        // Apply friction
        particle.velocity *= 0.95;

        // Fade out and shrink
        let progress = particle.lifetime.fraction();
        let alpha = 1.0 - progress;
        let scale = particle.initial_scale * (1.0 - progress);

        sprite.color = Color::srgba(
            sprite.color.to_srgba().red,
            sprite.color.to_srgba().green,
            sprite.color.to_srgba().blue,
            alpha,
        );
        transform.scale = Vec3::splat(scale);

        // Remove when finished
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Add match animation to tiles
pub fn add_match_animations(
    mut commands: Commands,
    matched_tiles: Query<Entity, (With<GridTile>, Changed<GridTile>)>,
    tile_query: Query<&GridTile>,
) {
    for entity in matched_tiles.iter() {
        if let Ok(tile) = tile_query.get(entity) {
            if tile.is_matched {
                commands.entity(entity).insert(MatchAnimation::default());
            }
        }
    }
}
