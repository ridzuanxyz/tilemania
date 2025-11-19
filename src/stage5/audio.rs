/// Audio system for Stage 5 (AI Tournaments)

use bevy::prelude::*;

#[derive(Event)]
pub enum AudioEvent {
    MatchStart,
    GameWon,
    GameLost,
    RoundAdvance,
    TournamentVictory,
    TournamentDefeat,
    OpponentIntro,
    CrowdCheer,
    CrowdAww,
    BracketUpdate,
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
    TournamentTheme,
    IntenseMatch,     // Semifinals/Finals
    VictoryFanfare,
}

pub fn play_audio_events(
    mut events: EventReader<AudioEvent>,
) {
    for event in events.read() {
        match event {
            AudioEvent::MatchStart => {
                // Play: "assets/audio/stage5/match_start.ogg"
            }
            AudioEvent::RoundAdvance => {
                // Play: "assets/audio/stage5/round_advance.ogg"
            }
            AudioEvent::TournamentVictory => {
                // Play: "assets/audio/stage5/tournament_victory.ogg"
            }
            AudioEvent::CrowdCheer => {
                // Play: "assets/audio/stage5/crowd_cheer.ogg"
            }
            _ => {}
        }
    }
}

pub fn update_background_music(
    tournament_state: Res<super::TournamentState>,
    mut music: ResMut<BackgroundMusic>,
) {
    if !music.is_playing {
        return;
    }

    // Switch to intense music for semifinals/finals
    match tournament_state.current_round {
        super::components::TournamentRound::Finals => {
            if music.current_track != MusicTrack::IntenseMatch {
                music.current_track = MusicTrack::IntenseMatch;
            }
        }
        _ => {
            if music.current_track != MusicTrack::TournamentTheme {
                music.current_track = MusicTrack::TournamentTheme;
            }
        }
    }
}
