/// Audio system for Stage 2 (Match-3 gameplay)

use bevy::prelude::*;

/// Audio events for Stage 2
#[derive(Event)]
pub enum AudioEvent {
    TileSelect,      // Click to select a tile
    TileSwap,        // Successful tile swap
    InvalidSwap,     // Invalid swap attempt
    WordMatch,       // Valid word found
    CascadeStart,    // Tiles begin falling
    NewTileSpawn,    // New tiles spawn
    ComboIncrease,   // Combo multiplier increased
    ComboBreak,      // Combo reset to 1
    LevelComplete,   // Target score reached
    GameOver,        // Time/moves ran out
    ButtonClick,     // UI button click
    PauseToggle,     // Pause menu opened/closed
}

/// Background music resource
#[derive(Resource, Default)]
pub struct BackgroundMusic {
    pub current_track: MusicTrack,
    pub volume: f32,
    pub is_playing: bool,
}

/// Music tracks for Stage 2
#[derive(Default, PartialEq, Clone, Copy)]
pub enum MusicTrack {
    #[default]
    MainTheme,       // Normal gameplay music
    IntenseMode,     // When time < 30 seconds
    Victory,         // Level complete
}

impl BackgroundMusic {
    pub fn new() -> Self {
        Self {
            current_track: MusicTrack::MainTheme,
            volume: 0.5,
            is_playing: true,
        }
    }

    pub fn set_track(&mut self, track: MusicTrack) {
        if self.current_track != track {
            self.current_track = track;
            // In real implementation, would trigger music change
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn toggle_pause(&mut self) {
        self.is_playing = !self.is_playing;
    }
}

/// Sound effects resource
#[derive(Resource, Default)]
pub struct SoundEffects {
    pub volume: f32,
    pub enabled: bool,
}

impl SoundEffects {
    pub fn new() -> Self {
        Self {
            volume: 0.7,
            enabled: true,
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

/// Play audio events (placeholder - hooks ready for actual audio files)
pub fn play_audio_events(
    mut events: EventReader<AudioEvent>,
    sfx: Res<SoundEffects>,
) {
    if !sfx.enabled {
        return;
    }

    for event in events.read() {
        // Placeholder: In real implementation, would load and play audio files
        match event {
            AudioEvent::TileSelect => {
                // Play: "assets/audio/sfx/tile_select.ogg"
                // Volume: 0.4 * sfx.volume
            }
            AudioEvent::TileSwap => {
                // Play: "assets/audio/sfx/tile_swap.ogg"
                // Volume: 0.5 * sfx.volume
            }
            AudioEvent::InvalidSwap => {
                // Play: "assets/audio/sfx/invalid_swap.ogg"
                // Volume: 0.3 * sfx.volume
            }
            AudioEvent::WordMatch => {
                // Play: "assets/audio/sfx/word_match.ogg"
                // Volume: 0.6 * sfx.volume
                // Pitch: Varies based on word length (3-letter: 1.0, 4-letter: 1.2)
            }
            AudioEvent::CascadeStart => {
                // Play: "assets/audio/sfx/cascade.ogg"
                // Volume: 0.4 * sfx.volume
            }
            AudioEvent::NewTileSpawn => {
                // Play: "assets/audio/sfx/tile_spawn.ogg"
                // Volume: 0.3 * sfx.volume
            }
            AudioEvent::ComboIncrease => {
                // Play: "assets/audio/sfx/combo_up.ogg"
                // Volume: 0.5 * sfx.volume
                // Pitch: 1.0 + (combo * 0.05) - higher combos = higher pitch
            }
            AudioEvent::ComboBreak => {
                // Play: "assets/audio/sfx/combo_break.ogg"
                // Volume: 0.4 * sfx.volume
            }
            AudioEvent::LevelComplete => {
                // Play: "assets/audio/sfx/level_complete.ogg"
                // Volume: 0.7 * sfx.volume
            }
            AudioEvent::GameOver => {
                // Play: "assets/audio/sfx/game_over.ogg"
                // Volume: 0.6 * sfx.volume
            }
            AudioEvent::ButtonClick => {
                // Play: "assets/audio/sfx/button_click.ogg"
                // Volume: 0.4 * sfx.volume
            }
            AudioEvent::PauseToggle => {
                // Play: "assets/audio/sfx/pause.ogg"
                // Volume: 0.5 * sfx.volume
            }
        }
    }
}

/// Update background music based on game state
pub fn update_background_music(
    state: Res<super::Stage2State>,
    mut music: ResMut<BackgroundMusic>,
) {
    if !music.is_playing {
        return;
    }

    // Switch to intense music when time is low
    let time_remaining_seconds = state.time_remaining_ms / 1000;

    if time_remaining_seconds > 0 && time_remaining_seconds < 30 {
        music.set_track(MusicTrack::IntenseMode);
    } else if state.is_active {
        music.set_track(MusicTrack::MainTheme);
    }

    // Play victory music when target reached
    if state.score >= 1000 { // Will use config.target_score in real system
        music.set_track(MusicTrack::Victory);
    }
}

/// Pause/resume music when game is paused
pub fn toggle_music_on_pause(
    mut music: ResMut<BackgroundMusic>,
) {
    music.toggle_pause();
}

/// Helper function to send audio events from other systems
pub fn send_audio_event(
    event_writer: &mut EventWriter<AudioEvent>,
    event: AudioEvent,
) {
    event_writer.send(event);
}

/// Audio event triggers for specific game events
pub mod triggers {
    use super::*;

    /// Trigger tile selection sound
    pub fn on_tile_selected(
        mut events: EventWriter<AudioEvent>,
    ) {
        events.send(AudioEvent::TileSelect);
    }

    /// Trigger tile swap sound
    pub fn on_tile_swapped(
        mut events: EventWriter<AudioEvent>,
    ) {
        events.send(AudioEvent::TileSwap);
    }

    /// Trigger word match sound
    pub fn on_word_matched(
        mut events: EventWriter<AudioEvent>,
        word_length: usize,
    ) {
        events.send(AudioEvent::WordMatch);
        // Real implementation would vary pitch based on word_length
    }

    /// Trigger combo increase sound
    pub fn on_combo_increased(
        mut events: EventWriter<AudioEvent>,
        combo_level: u32,
    ) {
        events.send(AudioEvent::ComboIncrease);
        // Real implementation would vary pitch based on combo_level
    }

    /// Trigger combo break sound
    pub fn on_combo_broken(
        mut events: EventWriter<AudioEvent>,
    ) {
        events.send(AudioEvent::ComboBreak);
    }

    /// Trigger level complete sound
    pub fn on_level_complete(
        mut events: EventWriter<AudioEvent>,
    ) {
        events.send(AudioEvent::LevelComplete);
    }

    /// Trigger game over sound
    pub fn on_game_over(
        mut events: EventWriter<AudioEvent>,
    ) {
        events.send(AudioEvent::GameOver);
    }
}

/// Audio configuration presets
pub mod presets {
    use super::*;

    /// Full volume preset
    pub fn full_volume() -> (BackgroundMusic, SoundEffects) {
        (
            BackgroundMusic {
                current_track: MusicTrack::MainTheme,
                volume: 1.0,
                is_playing: true,
            },
            SoundEffects {
                volume: 1.0,
                enabled: true,
            },
        )
    }

    /// Quiet preset
    pub fn quiet() -> (BackgroundMusic, SoundEffects) {
        (
            BackgroundMusic {
                current_track: MusicTrack::MainTheme,
                volume: 0.3,
                is_playing: true,
            },
            SoundEffects {
                volume: 0.3,
                enabled: true,
            },
        )
    }

    /// Music only preset
    pub fn music_only() -> (BackgroundMusic, SoundEffects) {
        (
            BackgroundMusic {
                current_track: MusicTrack::MainTheme,
                volume: 0.5,
                is_playing: true,
            },
            SoundEffects {
                volume: 0.0,
                enabled: false,
            },
        )
    }

    /// SFX only preset
    pub fn sfx_only() -> (BackgroundMusic, SoundEffects) {
        (
            BackgroundMusic {
                current_track: MusicTrack::MainTheme,
                volume: 0.0,
                is_playing: false,
            },
            SoundEffects {
                volume: 0.7,
                enabled: true,
            },
        )
    }

    /// Muted preset
    pub fn muted() -> (BackgroundMusic, SoundEffects) {
        (
            BackgroundMusic {
                current_track: MusicTrack::MainTheme,
                volume: 0.0,
                is_playing: false,
            },
            SoundEffects {
                volume: 0.0,
                enabled: false,
            },
        )
    }
}
