/// Components for Stage 5 (AI Tournaments)

use bevy::prelude::*;

/// Tournament player component
#[derive(Component, Clone)]
pub struct TournamentPlayer {
    pub name: String,
    pub is_human: bool,
    pub difficulty: u8,
    pub personality: Option<super::ai_personality::AIPersonality>,
    pub wins: u32,
    pub losses: u32,
}

/// Match scoreboard component
#[derive(Component)]
pub struct MatchScoreboard {
    pub player_score: u32,
    pub opponent_score: u32,
    pub games_won_player: u32,
    pub games_won_opponent: u32,
}

/// Bracket node component
#[derive(Component)]
pub struct BracketNode {
    pub round: TournamentRound,
    pub match_index: usize,
    pub player1: Option<TournamentPlayer>,
    pub player2: Option<TournamentPlayer>,
    pub winner: Option<TournamentPlayer>,
}

/// Tournament round
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TournamentRound {
    Quarterfinals,  // 8 → 4
    Semifinals,     // 4 → 2
    Finals,         // 2 → 1
}

/// Victory celebration component
#[derive(Component)]
pub struct VictoryCelebration {
    pub lifetime: Timer,
}
