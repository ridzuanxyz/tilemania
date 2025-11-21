/// Visual effects and feedback systems for Stage 1

use bevy::prelude::*;
use super::components::*;

/// Colors for tile states
pub struct TileColors;

impl TileColors {
    /// Default tile color (unselected)
    pub const NORMAL: Color = Color::srgb(0.85, 0.85, 0.95);

    /// Highlighted tile color (keyboard focus - bright cyan border)
    pub const HIGHLIGHTED: Color = Color::srgb(0.3, 0.9, 1.0);

    /// Selected tile color (yellow glow)
    pub const SELECTED: Color = Color::srgb(1.0, 0.95, 0.4);

    /// Valid word flash (green)
    pub const VALID: Color = Color::srgb(0.3, 0.9, 0.3);

    /// Invalid word flash (red)
    pub const INVALID: Color = Color::srgb(0.95, 0.3, 0.3);

    /// Hover color (slightly brighter with warm tone)
    pub const HOVER: Color = Color::srgb(0.95, 0.95, 1.0);

    /// Combo glow colors (by combo level)
    pub fn combo_color(combo: u32) -> Color {
        match combo {
            0 => Self::NORMAL,
            1 => Color::srgb(0.9, 0.95, 1.0),  // Light blue
            2 => Color::srgb(0.8, 0.9, 1.0),   // Blue
            3 => Color::srgb(0.9, 0.8, 1.0),   // Purple
            4 => Color::srgb(1.0, 0.8, 0.9),   // Pink
            _ => Color::srgb(1.0, 0.9, 0.4),   // Gold (5+)
        }
    }
}

/// System to update tile visual states based on selection, highlight, and hover
pub fn update_tile_visuals(
    mut tile_query: Query<(
        &FallingTile,
        &mut Sprite,
        &mut Transform,
        Option<&SelectedTile>,
        Option<&HighlightedTile>,
        Option<&HoveredTile>,
    )>,
) {
    for (_tile, mut sprite, mut transform, selected, highlighted, hovered) in tile_query.iter_mut() {
        // Determine color based on priority: selected > highlighted > hovered > normal
        if selected.is_some() {
            // Selected state (yellow)
            sprite.color = TileColors::SELECTED;
            transform.scale = Vec3::splat(1.0); // Reset scale when selected
        } else if highlighted.is_some() {
            // Highlighted state for keyboard focus (bright cyan)
            sprite.color = TileColors::HIGHLIGHTED;
            transform.scale = Vec3::splat(1.0); // Reset scale when highlighted
        } else if hovered.is_some() {
            // Hovered state (brighter with scale-up)
            sprite.color = TileColors::HOVER;
            transform.scale = Vec3::splat(1.1); // Scale up 10% on hover
        } else {
            // Normal state
            sprite.color = TileColors::NORMAL;
            transform.scale = Vec3::splat(1.0); // Normal scale
        }
    }
}

/// Component for score popup animation
#[derive(Component)]
pub struct ScorePopup {
    pub points: u32,
    pub lifetime: f32,
    pub elapsed: f32,
}

/// Spawns a floating score popup when word is validated
pub fn spawn_score_popup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    points: u32,
    is_valid: bool,
) {
    let color = if is_valid {
        TileColors::VALID
    } else {
        TileColors::INVALID
    };

    commands.spawn((
        ScorePopup {
            points,
            lifetime: 1.5,
            elapsed: 0.0,
        },
        Text2d::new(format!("+{}", points)),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 36.0,
            ..default()
        },
        TextColor(color),
        Transform::from_translation(position),
    ));
}

/// Updates and removes score popups
pub fn update_score_popups(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ScorePopup, &mut Transform, &mut TextColor)>,
    time: Res<Time>,
) {
    for (entity, mut popup, mut transform, mut text_color) in query.iter_mut() {
        popup.elapsed += time.delta_secs();

        // Float upward
        transform.translation.y += 100.0 * time.delta_secs();

        // Fade out
        let alpha = 1.0 - (popup.elapsed / popup.lifetime);
        text_color.0 = text_color.0.with_alpha(alpha);

        // Despawn when done
        if popup.elapsed >= popup.lifetime {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Component for word validation flash effect
#[derive(Component)]
pub struct ValidationFlash {
    pub is_valid: bool,
    pub duration: f32,
    pub elapsed: f32,
}

/// System to animate validation flash on tiles
pub fn update_validation_flash(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ValidationFlash, &mut Sprite)>,
    time: Res<Time>,
) {
    for (entity, mut flash, mut sprite) in query.iter_mut() {
        flash.elapsed += time.delta_secs();

        // Flash effect (fade from colored to normal)
        let progress = flash.elapsed / flash.duration;
        let base_color = if flash.is_valid {
            TileColors::VALID
        } else {
            TileColors::INVALID
        };

        // Interpolate from flash color to normal
        sprite.color = Color::srgb(
            base_color.to_srgba().red * (1.0 - progress) + TileColors::NORMAL.to_srgba().red * progress,
            base_color.to_srgba().green * (1.0 - progress) + TileColors::NORMAL.to_srgba().green * progress,
            base_color.to_srgba().blue * (1.0 - progress) + TileColors::NORMAL.to_srgba().blue * progress,
        );

        // Remove component when done
        if flash.elapsed >= flash.duration {
            sprite.color = TileColors::NORMAL;
            commands.entity(entity).remove::<ValidationFlash>();
        }
    }
}

/// Component for combo multiplier display glow
#[derive(Component)]
pub struct ComboGlow {
    pub pulse_speed: f32,
    pub time: f32,
}

/// Animates combo multiplier glow effect
pub fn update_combo_glow(
    mut query: Query<(&ComboGlow, &mut TextColor)>,
    time: Res<Time>,
) {
    for (glow, mut text_color) in query.iter_mut() {
        // Pulsing glow effect
        let pulse = (glow.time * glow.pulse_speed).sin() * 0.5 + 0.5;
        let alpha = 0.7 + pulse * 0.3;

        text_color.0 = text_color.0.with_alpha(alpha);
    }
}

/// Spawns particle effect at position
pub fn spawn_particle_burst(
    commands: &mut Commands,
    position: Vec3,
    color: Color,
    count: u32,
) {
    for i in 0..count {
        let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
        let velocity = Vec3::new(
            angle.cos() * 200.0,
            angle.sin() * 200.0,
            0.0,
        );

        commands.spawn((
            ParticleEffect {
                lifetime: 0.8,
                elapsed: 0.0,
            },
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(8.0, 8.0)),
                    ..default()
                },
                transform: Transform::from_translation(position),
                ..default()
            },
            Velocity(velocity),
        ));
    }
}

/// Component for particle velocity
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Updates particle positions and despawns expired ones
pub fn update_particles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ParticleEffect, &mut Transform, &mut Sprite, &Velocity)>,
    time: Res<Time>,
) {
    for (entity, mut particle, mut transform, mut sprite, velocity) in query.iter_mut() {
        particle.elapsed += time.delta_secs();

        // Move particle
        transform.translation += velocity.0 * time.delta_secs();

        // Fade out
        let alpha = 1.0 - (particle.elapsed / particle.lifetime);
        sprite.color = sprite.color.with_alpha(alpha);

        // Despawn when expired
        if particle.elapsed >= particle.lifetime {
            commands.entity(entity).despawn_recursive();
        }
    }
}
