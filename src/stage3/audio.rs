/// Audio system for Stage 3 (Classic Board)

use bevy::prelude::*;

/// Audio events for Stage 3
#[derive(Event)]
pub enum AudioEvent {
    TilePlace,       // Tile placed on board
    TilePickup,      // Tile selected from rack
    WordSubmit,      // Player submits move
    WordValid,       // Valid word played
    WordInvalid,     // Invalid word attempted
    TurnChange,      // Turn switches
    AIThinking,      // AI is calculating move
    AIPlays,         // AI plays a word
    HighScore,       // Word scores 50+ points
    GameOver,        // Game ends
    ButtonClick,     // UI button click
    PauseToggle,     // Pause menu
}

/// Background music resource
#[derive(Resource, Default)]
pub struct BackgroundMusic {
    pub current_track: MusicTrack,
    pub volume: f32,
    pub is_playing: bool,
}

/// Music tracks for Stage 3
#[derive(Default, PartialEq, Clone, Copy)]
pub enum MusicTrack {
    #[default]
    ClassicMode,     // Strategic, contemplative music
    Endgame,         // When tile bag < 20 tiles
}

impl BackgroundMusic {
    pub fn new() -> Self {
        Self {
            current_track: MusicTrack::ClassicMode,
            volume: 0.4,
            is_playing: true,
        }
    }
}

/// Play audio events
pub fn play_audio_events(
    mut events: EventReader<AudioEvent>,
) {
    for event in events.read() {
        match event {
            AudioEvent::TilePlace => {
                // Play: "assets/audio/stage3/tile_place.ogg"
                // Volume: 0.5
            }
            AudioEvent::TilePickup => {
                // Play: "assets/audio/stage3/tile_pickup.ogg"
                // Volume: 0.3
            }
            AudioEvent::WordSubmit => {
                // Play: "assets/audio/stage3/word_submit.ogg"
                // Volume: 0.6
            }
            AudioEvent::WordValid => {
                // Play: "assets/audio/stage3/word_valid.ogg"
                // Volume: 0.7
            }
            AudioEvent::WordInvalid => {
                // Play: "assets/audio/stage3/word_invalid.ogg"
                // Volume: 0.5
            }
            AudioEvent::TurnChange => {
                // Play: "assets/audio/stage3/turn_change.ogg"
                // Volume: 0.4
            }
            AudioEvent::AIThinking => {
                // Play: "assets/audio/stage3/ai_thinking.ogg" (loop)
                // Volume: 0.3
            }
            AudioEvent::AIPlays => {
                // Play: "assets/audio/stage3/ai_plays.ogg"
                // Volume: 0.6
            }
            AudioEvent::HighScore => {
                // Play: "assets/audio/stage3/high_score.ogg"
                // Volume: 0.8
            }
            AudioEvent::GameOver => {
                // Play: "assets/audio/stage3/game_over.ogg"
                // Volume: 0.7
            }
            AudioEvent::ButtonClick => {
                // Play: "assets/audio/sfx/button_click.ogg"
                // Volume: 0.4
            }
            AudioEvent::PauseToggle => {
                // Play: "assets/audio/sfx/pause.ogg"
                // Volume: 0.5
            }
        }
    }
}

/// Update background music
pub fn update_background_music(
    tile_bag: Res<super::board::TileBag>,
    mut music: ResMut<BackgroundMusic>,
) {
    if !music.is_playing {
        return;
    }

    // Switch to endgame music when tiles are running low
    if tile_bag.count() < 20 {
        if music.current_track != MusicTrack::Endgame {
            music.current_track = MusicTrack::Endgame;
        }
    }
}
