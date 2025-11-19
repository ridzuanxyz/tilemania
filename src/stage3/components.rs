/// Components for Stage 3 (Classic Board)

use bevy::prelude::*;

/// Marker for board tiles (placed on 15×15 grid)
#[derive(Component)]
pub struct BoardTile {
    pub letter: char,
    pub position: (usize, usize), // Row, col (0-14)
    pub is_locked: bool,          // true once move is confirmed
    pub is_preview: bool,         // true for move preview
}

/// Marker for rack tiles (7 tiles in player's hand)
#[derive(Component)]
pub struct RackTile {
    pub letter: char,
    pub rack_index: usize, // 0-6
    pub is_selected: bool,
    pub is_blank: bool,    // Blank tile (can be any letter)
}

/// Premium square types on the board
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PremiumSquare {
    Normal,
    DoubleLetter,  // DL
    TripleLetter,  // TL
    DoubleWord,    // DW
    TripleWord,    // TW
    Center,        // Star (DW + first move bonus)
}

/// Board square component
#[derive(Component)]
pub struct BoardSquare {
    pub position: (usize, usize),
    pub premium: PremiumSquare,
    pub occupied_by: Option<char>,
}

/// Move preview component (shows where tiles will be placed)
#[derive(Component)]
pub struct MovePreview {
    pub position: (usize, usize),
    pub letter: char,
    pub is_valid: bool,
}

/// Score display component
#[derive(Component)]
pub struct ScoreDisplay {
    pub score: u32,
    pub position: Vec3,
    pub lifetime: Timer,
}

/// Turn indicator component
#[derive(Component)]
pub struct TurnIndicator {
    pub is_player_turn: bool,
}

/// AI thinking indicator
#[derive(Component)]
pub struct AIThinking {
    pub elapsed_ms: u32,
}
