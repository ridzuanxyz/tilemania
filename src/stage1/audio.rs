/// Audio system for Stage 1
///
/// This module provides audio hooks for sound effects and music.
/// Actual audio playback requires bevy_kira_audio to be properly configured.

use bevy::prelude::*;

/// Audio events that can be triggered
#[derive(Event, Debug)]
pub enum AudioEvent {
    // UI Sounds
    ButtonClick,
    ButtonHover,

    // Gameplay Sounds
    TileSelect,
    TileDeselect,
    WordValid,
    WordInvalid,
    TileSpawn,
    TileDespawn,

    // Power-ups
    PowerUpCollect,
    PowerUpActivate(PowerUpType),

    // Score & Combos
    ScorePopup(u32), // Points earned
    ComboIncrease(u32), // New combo level
    ComboBreak,

    // Game State
    GameStart,
    GamePause,
    GameResume,
    GameOver,
    TimeWarning, // When <10 seconds remain
}

#[derive(Debug, Clone, Copy)]
pub enum PowerUpType {
    SlowMotion,
    Bomb,
    Shuffle,
    ExtraTime,
}

/// Resource for managing background music
#[derive(Resource)]
pub struct BackgroundMusic {
    pub current_track: MusicTrack,
    pub volume: f32,
}

impl Default for BackgroundMusic {
    fn default() -> Self {
        Self {
            current_track: MusicTrack::MainTheme,
            volume: 0.7,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicTrack {
    MainTheme,
    IntenseMode, // When time < 30s
}

/// System to trigger audio events based on game state
pub fn trigger_audio_events(
    mut audio_events: EventWriter<AudioEvent>,
    // Add parameters for game state changes that should trigger audio
) {
    // This is a hook system - actual implementation would check
    // game state changes and trigger appropriate events

    // Example:
    // if word_validated {
    //     audio_events.send(AudioEvent::WordValid);
    // }
}

/// Placeholder for audio playback
/// In a real implementation, this would use bevy_kira_audio
pub fn play_audio_events(
    mut audio_events: EventReader<AudioEvent>,
) {
    for event in audio_events.read() {
        match event {
            AudioEvent::ButtonClick => {
                info!("🔊 [SFX] Button click");
                // audio.play("sounds/button_click.ogg");
            }
            AudioEvent::TileSelect => {
                info!("🔊 [SFX] Tile select");
                // audio.play("sounds/tile_select.ogg");
            }
            AudioEvent::WordValid => {
                info!("🔊 [SFX] Valid word! (ding)");
                // audio.play("sounds/word_valid.ogg");
            }
            AudioEvent::WordInvalid => {
                info!("🔊 [SFX] Invalid word (buzz)");
                // audio.play("sounds/word_invalid.ogg");
            }
            AudioEvent::PowerUpCollect => {
                info!("🔊 [SFX] Power-up collected!");
                // audio.play("sounds/powerup_collect.ogg");
            }
            AudioEvent::PowerUpActivate(powerup_type) => {
                info!("🔊 [SFX] Power-up activated: {:?}", powerup_type);
                // audio.play(format!("sounds/powerup_{:?}.ogg", powerup_type));
            }
            AudioEvent::ComboIncrease(level) => {
                info!("🔊 [SFX] Combo level {}!", level);
                // audio.play("sounds/combo_up.ogg");
            }
            AudioEvent::GameStart => {
                info!("🎵 [Music] Starting gameplay track");
                // audio.play_looped("music/gameplay_theme.ogg");
            }
            AudioEvent::GameOver => {
                info!("🎵 [Music] Game over");
                // audio.stop_music();
                // audio.play("sounds/game_over.ogg");
            }
            _ => {
                info!("🔊 [Audio Event] {:?}", event);
            }
        }
    }
}

/// Updates background music based on game state
pub fn update_background_music(
    time_remaining: u32,
    total_time: u32,
    mut music: ResMut<BackgroundMusic>,
) {
    // Switch to intense music when time is running low
    let time_percent = (time_remaining as f32) / (total_time as f32);

    if time_percent < 0.33 && music.current_track != MusicTrack::IntenseMode {
        info!("🎵 [Music] Switching to intense mode!");
        music.current_track = MusicTrack::IntenseMode;
        // In real implementation:
        // audio.stop_music();
        // audio.play_looped("music/intense_theme.ogg");
    }
}

// Suggested sound effects for Stage 1:
//
// UI Sounds:
// - sounds/button_click.ogg (sharp click)
// - sounds/button_hover.ogg (soft whoosh)
//
// Gameplay:
// - sounds/tile_select.ogg (soft pop)
// - sounds/word_valid.ogg (pleasant ding/chime)
// - sounds/word_invalid.ogg (gentle buzz)
// - sounds/tile_spawn.ogg (subtle whoosh)
//
// Power-ups:
// - sounds/powerup_collect.ogg (sparkle sound)
// - sounds/powerup_slowmotion.ogg (time-slow effect)
// - sounds/powerup_bomb.ogg (explosion)
// - sounds/powerup_shuffle.ogg (shuffle/mix sound)
// - sounds/powerup_extratime.ogg (clock tick)
//
// Combos:
// - sounds/combo_up.ogg (ascending pitch)
// - sounds/combo_break.ogg (descending pitch)
//
// Music:
// - music/gameplay_theme.ogg (upbeat, ~120 BPM)
// - music/intense_theme.ogg (faster, ~140 BPM)
//
// All sounds should be:
// - Short (<1 second for SFX)
// - Normalized volume
// - OGG Vorbis format (best for games)
// - 44.1kHz sample rate
