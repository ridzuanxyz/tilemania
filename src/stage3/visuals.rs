/// Visual effects for Stage 3 (Classic Board)

use bevy::prelude::*;
use super::components::*;

/// Update tile visual states
pub fn update_tile_visuals(
    mut tile_query: Query<(&BoardTile, &mut Sprite)>,
) {
    for (tile, mut sprite) in tile_query.iter_mut() {
        if tile.is_preview {
            sprite.color = Color::srgba(1.0, 1.0, 0.7, 0.7); // Yellow preview
        } else if tile.is_locked {
            sprite.color = Color::srgb(0.95, 0.95, 0.85); // Locked tile
        } else {
            sprite.color = Color::srgb(1.0, 1.0, 0.9); // Normal tile
        }
    }
}

/// Update board highlights
pub fn update_board_highlights(
    mut square_query: Query<(&BoardSquare, &mut Sprite)>,
) {
    // Highlight valid placement squares
    for (square, mut sprite) in square_query.iter_mut() {
        if square.occupied_by.is_some() {
            sprite.color = sprite.color.with_alpha(0.5);
        }
    }
}

/// Update score popups
pub fn update_score_popups(
    mut commands: Commands,
    time: Res<Time>,
    mut popup_query: Query<(Entity, &mut Transform, &mut TextColor, &mut ScoreDisplay)>,
) {
    for (entity, mut transform, mut text_color, mut display) in popup_query.iter_mut() {
        display.lifetime.tick(time.delta());

        // Rise upward
        transform.translation.y += 50.0 * time.delta_secs();

        // Fade out
        let alpha = 1.0 - display.lifetime.fraction();
        text_color.0 = text_color.0.with_alpha(alpha);

        // Remove when finished
        if display.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Update move preview
pub fn update_move_preview(
    preview_query: Query<(&MovePreview, &mut Sprite)>,
) {
    // Visual feedback for where tiles will be placed
    for (preview, mut sprite) in preview_query.iter_mut() {
        if preview.is_valid {
            sprite.color = Color::srgba(0.3, 0.9, 0.3, 0.6); // Valid green
        } else {
            sprite.color = Color::srgba(0.9, 0.3, 0.3, 0.6); // Invalid red
        }
    }
}
