/// Audio system for Stage 2 (placeholder - reuse Stage 1 pattern)

use bevy::prelude::*;

/// Audio events for Stage 2
#[derive(Event)]
pub enum AudioEvent {
    TileSelect,
    TileSwap,
    WordMatch,
    CascadeStart,
    NewTileSpawn,
    LevelComplete,
    GameOver,
}

/// Background music resource
#[derive(Resource, Default)]
pub struct BackgroundMusic {
    pub volume: f32,
}

/// Placeholder: Play audio events
pub fn play_audio_events() {
    // TODO: Implement audio event playback (reuse Stage 1 pattern)
}
