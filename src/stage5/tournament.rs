/// Tournament bracket management

use bevy::prelude::*;
use super::components::*;
use super::ai_personality::*;

/// Tournament state resource
#[derive(Resource)]
pub struct TournamentState {
    pub current_round: TournamentRound,
    pub current_match_index: usize,
    pub players: Vec<TournamentPlayer>,
    pub bracket: Vec<BracketMatch>,
    pub is_complete: bool,
    pub champion: Option<TournamentPlayer>,
}

impl Default for TournamentState {
    fn default() -> Self {
        Self::new()
    }
}

impl TournamentState {
    pub fn new() -> Self {
        let mut players = vec![
            // Player (human)
            TournamentPlayer {
                name: "You".to_string(),
                is_human: true,
                difficulty: 0,
                personality: None,
                wins: 0,
                losses: 0,
            },
        ];

        // Add 7 AI opponents with varying difficulties and personalities
        let ai_opponents = [
            ("Rookie Rita", 2, AIPersonality::Defensive),
            ("Balanced Bob", 3, AIPersonality::Balanced),
            ("Aggro Alex", 3, AIPersonality::Aggressive),
            ("Strategic Sam", 4, AIPersonality::Balanced),
            ("Vocab Victor", 4, AIPersonality::Aggressive),
            ("Master Maya", 5, AIPersonality::Balanced),
            ("Champion Chen", 5, AIPersonality::Aggressive),
        ];

        for (name, difficulty, personality) in &ai_opponents {
            players.push(TournamentPlayer {
                name: name.to_string(),
                is_human: false,
                difficulty: *difficulty,
                personality: Some(*personality),
                wins: 0,
                losses: 0,
            });
        }

        Self {
            current_round: TournamentRound::Quarterfinals,
            current_match_index: 0,
            players,
            bracket: Vec::new(),
            is_complete: false,
            champion: None,
        }
    }

    pub fn start_round(&mut self, round: TournamentRound) {
        self.current_round = round;
        self.current_match_index = 0;
    }

    pub fn advance_match(&mut self) {
        self.current_match_index += 1;
    }

    pub fn is_round_complete(&self) -> bool {
        match self.current_round {
            TournamentRound::Quarterfinals => self.current_match_index >= 4,
            TournamentRound::Semifinals => self.current_match_index >= 2,
            TournamentRound::Finals => self.current_match_index >= 1,
        }
    }
}

/// Individual bracket match
#[derive(Clone)]
pub struct BracketMatch {
    pub round: TournamentRound,
    pub player1: TournamentPlayer,
    pub player2: TournamentPlayer,
    pub winner: Option<TournamentPlayer>,
    pub best_of: u32, // Best-of-3
}

/// Current match state
#[derive(Resource, Default)]
pub struct CurrentMatch {
    pub player: Option<TournamentPlayer>,
    pub opponent: Option<TournamentPlayer>,
    pub player_games_won: u32,
    pub opponent_games_won: u32,
    pub current_game_score_player: u32,
    pub current_game_score_opponent: u32,
    pub is_active: bool,
}

impl CurrentMatch {
    pub fn new(player: TournamentPlayer, opponent: TournamentPlayer) -> Self {
        Self {
            player: Some(player),
            opponent: Some(opponent),
            player_games_won: 0,
            opponent_games_won: 0,
            current_game_score_player: 0,
            current_game_score_opponent: 0,
            is_active: true,
        }
    }

    pub fn is_match_over(&self) -> bool {
        self.player_games_won >= 2 || self.opponent_games_won >= 2
    }

    pub fn player_won_match(&self) -> bool {
        self.player_games_won >= 2
    }
}
