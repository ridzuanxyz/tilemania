/// Components for Stage 1 gameplay entities

use bevy::prelude::*;

/// Marker component for the game board
#[derive(Component)]
pub struct GameBoard {
    pub columns: usize,
}

/// Component for falling letter tiles
#[derive(Component)]
pub struct FallingTile {
    /// The letter on this tile
    pub letter: char,
    /// Column index (0-6 for 7 columns)
    pub column: usize,
    /// Fall speed (pixels per second)
    pub speed: f32,
    /// Is this tile selected by the player?
    pub is_selected: bool,
}

/// Component marking a tile as selected
#[derive(Component)]
pub struct SelectedTile;

/// Component for displaying the current word being formed
#[derive(Component)]
pub struct WordDisplay;

/// Component for score display
#[derive(Component)]
pub struct ScoreDisplay;

/// Component for timer display
#[derive(Component)]
pub struct TimerDisplay;

/// Component for combo counter display
#[derive(Component)]
pub struct ComboDisplay;

/// Power-up types
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerUp {
    /// Slows down tile fall speed by 50%
    SlowMotion,
    /// Clears all tiles in a column
    Bomb,
    /// Shuffles all tiles on screen
    Shuffle,
    /// Adds 10 seconds to timer
    ExtraTime,
}

/// Component for power-up entities
#[derive(Component)]
pub struct PowerUpEntity {
    pub power_up_type: PowerUp,
    pub duration_ms: u32,
    pub is_active: bool,
}

/// Particle effect marker
#[derive(Component)]
pub struct ParticleEffect {
    pub lifetime: f32,
    pub elapsed: f32,
}

/// Animation component for word validation feedback
#[derive(Component)]
pub struct ValidationAnimation {
    pub is_valid: bool,
    pub progress: f32,
}
