/// Components for Stage 2 gameplay entities

use bevy::prelude::*;

/// Marker component for the game grid
#[derive(Component)]
pub struct GameGrid {
    pub size: usize, // 8x8
}

/// Component for grid-based tiles
#[derive(Component)]
pub struct GridTile {
    /// The letter on this tile
    pub letter: char,
    /// Grid position (row, col)
    pub grid_pos: (usize, usize),
    /// Is this tile selected?
    pub is_selected: bool,
    /// Is this tile part of a match?
    pub is_matched: bool,
}

/// Component marking a tile as selected
#[derive(Component)]
pub struct SelectedTile;

/// Component for tiles that are matched and being cleared
#[derive(Component)]
pub struct MatchedTile {
    /// Time elapsed in match animation
    pub elapsed: f32,
    /// Duration of match animation
    pub duration: f32,
}

/// Component for tiles cascading down
#[derive(Component)]
pub struct CascadingTile {
    /// Target grid position
    pub target_pos: (usize, usize),
    /// Animation speed
    pub speed: f32,
}

/// Component for displaying the current score
#[derive(Component)]
pub struct ScoreDisplay;

/// Component for timer display
#[derive(Component)]
pub struct TimerDisplay;

/// Component for moves counter display
#[derive(Component)]
pub struct MovesDisplay;

/// Component for combo counter display
#[derive(Component)]
pub struct ComboDisplay;

/// Component for target score display
#[derive(Component)]
pub struct TargetDisplay;

/// Particle effect marker
#[derive(Component)]
pub struct ParticleEffect {
    pub lifetime: f32,
    pub elapsed: f32,
}

/// Component for particle velocity
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Animation component for word match feedback
#[derive(Component)]
pub struct MatchAnimation {
    pub word: String,
    pub progress: f32,
    pub duration: f32,
}

/// Component for score popup
#[derive(Component)]
pub struct ScorePopup {
    pub points: u32,
    pub lifetime: f32,
    pub elapsed: f32,
}

/// Swap animation component
#[derive(Component)]
pub struct SwapAnimation {
    pub start_pos: Vec3,
    pub end_pos: Vec3,
    pub progress: f32,
    pub duration: f32,
}
