/// Components for Stage 4 (Speed Challenge)

use bevy::prelude::*;

/// Rack tile component
#[derive(Component)]
pub struct RackTile {
    pub letter: char,
    pub index: usize,
    pub is_selected: bool,
}

/// Streak indicator component
#[derive(Component)]
pub struct StreakIndicator {
    pub current_streak: u32,
}

/// Score popup component
#[derive(Component)]
pub struct ScorePopup {
    pub score: u32,
    pub lifetime: Timer,
    pub rise_speed: f32,
}

/// Particle effect component
#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub initial_scale: f32,
}

/// Timer display component
#[derive(Component)]
pub struct TimerDisplay;

/// Panic mode indicator
#[derive(Component)]
pub struct PanicModeIndicator {
    pub pulse_timer: Timer,
}
