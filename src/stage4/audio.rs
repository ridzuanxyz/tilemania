/// Audio system for Stage 4 (Speed Challenge)

use bevy::prelude::*;

#[derive(Event)]
pub enum AudioEvent {
    TileSelect,
    WordSubmit,
    WordValid,
    WordInvalid,
    StreakIncrease,
    StreakBreak,
    HighStreak,      // Streak >= 10
    PanicMode,       // Time < 10s
    TimeWarning,     // Time < 30s
    GameOver,
    ButtonClick,
    PauseToggle,
}

#[derive(Resource, Default)]
pub struct BackgroundMusic {
    pub current_track: MusicTrack,
    pub volume: f32,
    pub is_playing: bool,
}

#[derive(Default, PartialEq, Clone, Copy)]
pub enum MusicTrack {
    #[default]
    SpeedMode,       // Fast-paced music
    PanicMode,       // Intense music when time < 10s
}

pub fn play_audio_events(
    mut events: EventReader<AudioEvent>,
) {
    for event in events.read() {
        match event {
            AudioEvent::TileSelect => {
                // Play: "assets/audio/stage4/tile_select.ogg"
            }
            AudioEvent::WordValid => {
                // Play: "assets/audio/stage4/word_valid.ogg"
            }
            AudioEvent::StreakIncrease => {
                // Play: "assets/audio/stage4/streak_up.ogg"
                // Pitch increases with streak level
            }
            AudioEvent::PanicMode => {
                // Play: "assets/audio/stage4/panic_mode.ogg"
            }
            _ => {}
        }
    }
}

pub fn update_background_music(
    state: Res<super::Stage4State>,
    mut music: ResMut<BackgroundMusic>,
) {
    if !music.is_playing {
        return;
    }

    if state.is_panic_mode {
        if music.current_track != MusicTrack::PanicMode {
            music.current_track = MusicTrack::PanicMode;
        }
    } else if music.current_track != MusicTrack::SpeedMode {
        music.current_track = MusicTrack::SpeedMode;
    }
}
