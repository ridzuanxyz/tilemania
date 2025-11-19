/// Visual effects for Stage 4 (Speed Challenge)

use bevy::prelude::*;
use bevy::text::TextStyle;
use super::components::*;

pub fn update_tile_visuals(
    tile_query: Query<(&RackTile, &mut Sprite)>,
) {
    // Highlight selected tiles
}

pub fn update_streak_display(
    streak_query: Query<&StreakIndicator>,
) {
    // Visual feedback for streaks
}

pub fn update_timer_visuals(
    state: Res<super::Stage4State>,
) {
    // Pulse/flash timer when in panic mode
}

pub fn update_score_popups(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_query: Query<(Entity, &mut Transform, &mut Text, &mut ScorePopup)>,
) {
    for (entity, mut transform, mut text, mut popup) in popup_query.iter_mut() {
        popup.lifetime.tick(time.delta());

        transform.translation.y += popup.rise_speed * time.delta_secs();

        let alpha = 1.0 - popup.lifetime.fraction();
        for section in &mut text.sections {
            let color = section.style.color;
            section.style.color = Color::srgba(
                color.to_srgba().red,
                color.to_srgba().green,
                color.to_srgba().blue,
                alpha,
            );
        }

        if popup.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particle_query: Query<(Entity, &mut Transform, &mut Sprite, &mut Particle)>,
) {
    for (entity, mut transform, mut sprite, mut particle) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());

        let velocity_3d = Vec3::new(particle.velocity.x, particle.velocity.y, 0.0);
        transform.translation += velocity_3d * time.delta_secs();

        particle.velocity *= 0.95;

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

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
