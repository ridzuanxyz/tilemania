/// AI opponent system for Stage 3

use bevy::prelude::*;
use super::{Stage3State, Stage3Config, Turn, Direction, MoveEvent};
use super::board::Board;
use super::difficulty::get_ai_difficulty;
use crate::lexicon::Lexicon;
use crate::scoring::ScoreCalculator;

/// AI move calculation state
#[derive(Resource, Default)]
pub struct AIState {
    pub is_thinking: bool,
    pub think_timer_ms: u32,
    pub current_best_move: Option<AIMove>,
}

/// Represents a potential AI move
#[derive(Clone)]
pub struct AIMove {
    pub word: String,
    pub position: (usize, usize),
    pub direction: Direction,
    pub score: u32,
    pub tiles_used: Vec<char>,
}

/// Calculate AI move (runs during AI's turn)
pub fn calculate_ai_move(
    mut ai_state: ResMut<AIState>,
    state: Res<Stage3State>,
    config: Res<Stage3Config>,
    board: Res<Board>,
    lexicon: Res<Lexicon>,
    scorer: Res<ScoreCalculator>,
    time: Res<Time>,
) {
    // Only calculate when it's AI's turn
    if state.current_turn != Turn::AI || !state.is_active {
        return;
    }

    if !ai_state.is_thinking {
        // Start thinking
        ai_state.is_thinking = true;
        ai_state.think_timer_ms = 0;
        ai_state.current_best_move = None;

        // Calculate best move
        let difficulty = get_ai_difficulty(config.difficulty);
        let best_move = find_best_move(
            &board,
            &state.ai_rack,
            &lexicon,
            &scorer,
            difficulty,
        );

        ai_state.current_best_move = best_move;
    } else {
        // Simulate thinking time
        ai_state.think_timer_ms += (time.delta_secs() * 1000.0) as u32;
    }
}

/// Execute AI move when thinking is complete
pub fn execute_ai_move(
    mut commands: Commands,
    mut ai_state: ResMut<AIState>,
    mut state: ResMut<Stage3State>,
    mut board: ResMut<Board>,
    config: Res<Stage3Config>,
    mut move_events: EventWriter<MoveEvent>,
) {
    if !ai_state.is_thinking || state.current_turn != Turn::AI {
        return;
    }

    // Check if AI has finished thinking
    let difficulty = get_ai_difficulty(config.difficulty);
    if ai_state.think_timer_ms < difficulty.think_time_ms {
        return;
    }

    // Execute the move
    if let Some(ai_move) = &ai_state.current_best_move {
        // Place tiles on board
        place_move_on_board(&mut board, ai_move);

        // Update AI score
        state.ai_score += ai_move.score;

        // Remove used tiles from AI rack
        for tile in &ai_move.tiles_used {
            if let Some(pos) = state.ai_rack.iter().position(|&t| t == *tile) {
                state.ai_rack.remove(pos);
            }
        }

        // Send move event
        move_events.send(MoveEvent {
            player: Turn::AI,
            word: ai_move.word.clone(),
            position: ai_move.position,
            direction: ai_move.direction,
            score: ai_move.score,
        });

        // Record move
        state.moves_history.push(super::MoveRecord {
            turn_number: state.moves_history.len() as u32 + 1,
            player: Turn::AI,
            word: ai_move.word.clone(),
            score: ai_move.score,
            position: ai_move.position,
            direction: ai_move.direction,
        });
    }

    // Reset AI state and switch turns
    ai_state.is_thinking = false;
    ai_state.think_timer_ms = 0;
    state.current_turn = Turn::Player;
}

/// Find the best move for AI
fn find_best_move(
    board: &Board,
    rack: &[char],
    lexicon: &Lexicon,
    scorer: &ScoreCalculator,
    difficulty: &super::difficulty::AIDifficulty,
) -> Option<AIMove> {
    let mut candidate_moves = Vec::new();

    // If board is empty, play first move
    if board.is_board_empty() {
        candidate_moves = find_first_move_candidates(rack, lexicon, scorer, difficulty);
    } else {
        // Find all valid moves on current board
        candidate_moves = find_all_valid_moves(board, rack, lexicon, scorer, difficulty);
    }

    if candidate_moves.is_empty() {
        return None;
    }

    // Apply AI difficulty filter
    let max_candidates = difficulty.candidate_move_count();
    candidate_moves.truncate(max_candidates);

    // Potentially make a suboptimal move based on error rate
    if difficulty.should_make_error() && candidate_moves.len() > 1 {
        // Choose a random move from top candidates instead of best
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        candidate_moves.choose(&mut rng).cloned()
    } else {
        // Choose best move
        candidate_moves.into_iter().next()
    }
}

/// Find candidate moves for first turn (center square)
fn find_first_move_candidates(
    rack: &[char],
    lexicon: &Lexicon,
    scorer: &ScoreCalculator,
    difficulty: &super::difficulty::AIDifficulty,
) -> Vec<AIMove> {
    let mut moves = Vec::new();
    let max_length = difficulty.max_word_length().min(rack.len());

    // Generate words from rack tiles
    for length in 2..=max_length {
        let words = generate_words_from_tiles(rack, length, lexicon);

        for word in words {
            // First move must go through center (7, 7)
            // Try horizontal placement
            let score = scorer.calculate_word_score(&word, &[]);
            moves.push(AIMove {
                word: word.clone(),
                position: (7, 7 - (word.len() / 2)),
                direction: Direction::Horizontal,
                score,
                tiles_used: word.chars().collect(),
            });

            // Try vertical placement
            moves.push(AIMove {
                word: word.clone(),
                position: (7 - (word.len() / 2), 7),
                direction: Direction::Vertical,
                score,
                tiles_used: word.chars().collect(),
            });
        }
    }

    // Sort by score descending
    moves.sort_by(|a, b| b.score.cmp(&a.score));

    // Filter by minimum score
    let min_score = difficulty.min_move_score();
    moves.retain(|m| m.score >= min_score);

    moves
}

/// Find all valid moves on current board
fn find_all_valid_moves(
    board: &Board,
    rack: &[char],
    lexicon: &Lexicon,
    scorer: &ScoreCalculator,
    difficulty: &super::difficulty::AIDifficulty,
) -> Vec<AIMove> {
    let mut moves = Vec::new();

    // For each position on the board
    for row in 0..15 {
        for col in 0..15 {
            // Skip occupied squares
            if board.get(row, col).is_some() {
                continue;
            }

            // Try placing each letter from rack
            for &letter in rack {
                // Try horizontal word formation
                if let Some(ai_move) = try_form_word_horizontal(
                    board, rack, (row, col), letter, lexicon, scorer,
                ) {
                    if ai_move.score >= difficulty.min_move_score() {
                        moves.push(ai_move);
                    }
                }

                // Try vertical word formation
                if let Some(ai_move) = try_form_word_vertical(
                    board, rack, (row, col), letter, lexicon, scorer,
                ) {
                    if ai_move.score >= difficulty.min_move_score() {
                        moves.push(ai_move);
                    }
                }
            }
        }
    }

    // Sort by score descending
    moves.sort_by(|a, b| b.score.cmp(&a.score));

    moves
}

/// Generate valid words from tiles (placeholder - simplified)
fn generate_words_from_tiles(
    tiles: &[char],
    length: usize,
    lexicon: &Lexicon,
) -> Vec<String> {
    // Placeholder: In real implementation, would use GADDAG/wolges
    // For now, generate simple permutations and check lexicon

    let mut words = Vec::new();

    // Simple brute force for demo (real implementation uses GADDAG)
    if length > tiles.len() {
        return words;
    }

    // Try all permutations of length N
    generate_permutations(tiles, length, &mut |perm| {
        let word: String = perm.iter().collect();
        if lexicon.is_valid(&word.to_uppercase()) {
            words.push(word.to_uppercase());
        }
    });

    words
}

/// Generate permutations (helper)
fn generate_permutations<F>(tiles: &[char], length: usize, callback: &mut F)
where
    F: FnMut(&[char]),
{
    fn permute<F>(
        tiles: &[char],
        length: usize,
        current: &mut Vec<char>,
        used: &mut Vec<bool>,
        callback: &mut F,
    )
    where
        F: FnMut(&[char]),
    {
        if current.len() == length {
            callback(current);
            return;
        }

        for i in 0..tiles.len() {
            if used[i] {
                continue;
            }

            used[i] = true;
            current.push(tiles[i]);
            permute(tiles, length, current, used, callback);
            current.pop();
            used[i] = false;
        }
    }

    let mut current = Vec::new();
    let mut used = vec![false; tiles.len()];
    permute(tiles, length, &mut current, &mut used, callback);
}

/// Try to form a horizontal word at position
fn try_form_word_horizontal(
    board: &Board,
    rack: &[char],
    position: (usize, usize),
    first_letter: char,
    lexicon: &Lexicon,
    scorer: &ScoreCalculator,
) -> Option<AIMove> {
    // Placeholder: Simplified word formation
    // Real implementation would use anchor-based GADDAG search
    None
}

/// Try to form a vertical word at position
fn try_form_word_vertical(
    board: &Board,
    rack: &[char],
    position: (usize, usize),
    first_letter: char,
    lexicon: &Lexicon,
    scorer: &ScoreCalculator,
) -> Option<AIMove> {
    // Placeholder: Simplified word formation
    // Real implementation would use anchor-based GADDAG search
    None
}

/// Place AI move on board
fn place_move_on_board(board: &mut Board, ai_move: &AIMove) {
    let (start_row, start_col) = ai_move.position;

    for (i, letter) in ai_move.word.chars().enumerate() {
        let (row, col) = match ai_move.direction {
            Direction::Horizontal => (start_row, start_col + i),
            Direction::Vertical => (start_row + i, start_col),
        };

        let _ = board.place(row, col, letter);
    }
}
