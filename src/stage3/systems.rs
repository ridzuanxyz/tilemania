/// Core gameplay systems for Stage 3

use bevy::prelude::*;
use super::{Stage3State, Stage3Config, Turn, Direction, MoveEvent, GameOverReason};
use super::board::{Board, TileBag};
use super::components::*;
use crate::lexicon::Lexicon;
use crate::scoring::ScoreCalculator;

/// Player input state for building moves
#[derive(Resource, Default)]
pub struct PlayerMoveBuilder {
    pub selected_rack_indices: Vec<usize>,
    pub placement_positions: Vec<(usize, usize)>,
    pub placement_direction: Option<Direction>,
    pub is_building: bool,
}

/// Handle player input (tile selection, placement, submission)
pub fn handle_player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut move_builder: ResMut<PlayerMoveBuilder>,
    mut rack_query: Query<(&mut RackTile, &Transform)>,
    board_query: Query<(&BoardSquare, &Transform)>,
    state: Res<Stage3State>,
) {
    // Only accept input during player's turn
    if state.current_turn != Turn::Player || !state.is_active {
        return;
    }

    // Get cursor position in world coordinates
    let window = windows.single();
    let (camera, camera_transform) = camera.single();

    if let Some(cursor_pos) = window.cursor_position() {
        // Convert cursor to world position
        // (Simplified - real implementation needs proper screen-to-world conversion)

        // Handle tile selection from rack
        if mouse.just_pressed(MouseButton::Left) {
            for (i, (mut tile, _transform)) in rack_query.iter_mut().enumerate() {
                // Check if cursor is over this rack tile
                // If yes, toggle selection
                tile.is_selected = !tile.is_selected;

                if tile.is_selected && !move_builder.selected_rack_indices.contains(&i) {
                    move_builder.selected_rack_indices.push(i);
                } else if !tile.is_selected {
                    move_builder.selected_rack_indices.retain(|&idx| idx != i);
                }
            }
        }
    }

    // Submit move with Enter/Space
    if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space) {
        if !move_builder.selected_rack_indices.is_empty() {
            move_builder.is_building = false; // Trigger validation
        }
    }

    // Cancel move with Escape (already handled by pause system)
    // Clear selection with 'C' key
    if keyboard.just_pressed(KeyCode::KeyC) {
        move_builder.selected_rack_indices.clear();
        move_builder.placement_positions.clear();
        move_builder.is_building = false;

        // Deselect all rack tiles
        for (mut tile, _) in rack_query.iter_mut() {
            tile.is_selected = false;
        }
    }
}

/// Validate player's move
pub fn validate_player_move(
    mut move_builder: ResMut<PlayerMoveBuilder>,
    state: Res<Stage3State>,
    board: Res<Board>,
    lexicon: Res<Lexicon>,
) {
    if move_builder.is_building || move_builder.selected_rack_indices.is_empty() {
        return;
    }

    // Build word from selected tiles and positions
    let word = build_word_from_move(&move_builder, &state);

    if word.is_empty() {
        return;
    }

    // Validate word exists in lexicon
    if !lexicon.is_valid(&word.to_uppercase()) {
        // Invalid word - show error
        move_builder.selected_rack_indices.clear();
        move_builder.placement_positions.clear();
        return;
    }

    // Validate move connects to existing tiles
    if !board.is_board_empty() {
        let connects = check_move_connects(&move_builder, &board);
        if !connects {
            // Move doesn't connect - show error
            move_builder.selected_rack_indices.clear();
            move_builder.placement_positions.clear();
            return;
        }
    }

    // Move is valid - ready to execute
}

/// Execute validated move
pub fn execute_move(
    mut commands: Commands,
    mut move_builder: ResMut<PlayerMoveBuilder>,
    mut state: ResMut<Stage3State>,
    mut board: ResMut<Board>,
    mut tile_bag: ResMut<TileBag>,
    scorer: Res<ScoreCalculator>,
    mut move_events: EventWriter<MoveEvent>,
) {
    if move_builder.is_building || move_builder.selected_rack_indices.is_empty() {
        return;
    }

    // Build word
    let word = build_word_from_move(&move_builder, &state);

    if word.is_empty() {
        return;
    }

    // Calculate score
    let score = calculate_move_score(&word, &move_builder, &board, &scorer);

    // Place tiles on board
    for (i, &pos) in move_builder.placement_positions.iter().enumerate() {
        let tile_idx = move_builder.selected_rack_indices[i];
        let letter = state.player_rack[tile_idx];
        let _ = board.place(pos.0, pos.1, letter);
    }

    // Update player score
    state.player_score += score;

    // Remove used tiles from rack
    for &idx in move_builder.selected_rack_indices.iter().rev() {
        state.player_rack.remove(idx);
    }

    // Draw new tiles to refill rack to 7
    let tiles_to_draw = 7 - state.player_rack.len();
    let new_tiles = tile_bag.draw_tiles(tiles_to_draw);
    state.player_rack.extend(new_tiles);

    // Send move event
    move_events.send(MoveEvent {
        player: Turn::Player,
        word: word.clone(),
        position: move_builder.placement_positions[0],
        direction: move_builder.placement_direction.unwrap_or(Direction::Horizontal),
        score,
    });

    // Record move
    let turn_number = state.moves_history.len() as u32 + 1;
    state.moves_history.push(super::MoveRecord {
        turn_number,
        player: Turn::Player,
        word: word.clone(),
        score,
        position: move_builder.placement_positions[0],
        direction: move_builder.placement_direction.unwrap_or(Direction::Horizontal),
    });

    // Clear move builder
    move_builder.selected_rack_indices.clear();
    move_builder.placement_positions.clear();
    move_builder.placement_direction = None;

    // Switch to AI turn
    state.current_turn = Turn::AI;
}

/// Update turn logic
pub fn update_turn(
    state: Res<Stage3State>,
    tile_bag: Res<TileBag>,
) {
    // Check if game should end
    if tile_bag.is_empty() && (state.player_rack.is_empty() || state.ai_rack.is_empty()) {
        // Game ends when tile bag is empty and one player has no tiles
    }
}

/// Check for game over conditions
pub fn check_game_over(
    mut state: ResMut<Stage3State>,
    config: Res<Stage3Config>,
    tile_bag: Res<TileBag>,
) {
    if !state.is_active {
        return;
    }

    // Check time limit
    if config.time_limit_seconds > 0 && state.time_remaining_ms == 0 {
        state.is_active = false;
        state.game_over_reason = Some(GameOverReason::TimeExpired);
        return;
    }

    // Check if tile bag empty and both players have no moves
    if tile_bag.is_empty() {
        // Simplified: End game when tiles run out
        state.is_active = false;
        state.game_over_reason = Some(GameOverReason::TileBagEmpty);
    }
}

/// Update game timer
pub fn update_timer(
    mut state: ResMut<Stage3State>,
    config: Res<Stage3Config>,
    time: Res<Time>,
) {
    if !state.is_active || config.time_limit_seconds == 0 {
        return;
    }

    let delta_ms = (time.delta_secs() * 1000.0) as u32;

    if state.time_remaining_ms > delta_ms {
        state.time_remaining_ms -= delta_ms;
    } else {
        state.time_remaining_ms = 0;
    }
}

// Helper functions

fn build_word_from_move(
    move_builder: &PlayerMoveBuilder,
    state: &Stage3State,
) -> String {
    let mut word = String::new();

    for &idx in &move_builder.selected_rack_indices {
        if let Some(&letter) = state.player_rack.get(idx) {
            word.push(letter);
        }
    }

    word
}

fn check_move_connects(
    move_builder: &PlayerMoveBuilder,
    board: &Board,
) -> bool {
    // Check if at least one placed tile is adjacent to an existing tile
    for &(row, col) in &move_builder.placement_positions {
        // Check adjacent squares
        let adjacent_positions = [
            (row.wrapping_sub(1), col),
            (row + 1, col),
            (row, col.wrapping_sub(1)),
            (row, col + 1),
        ];

        for &(adj_row, adj_col) in &adjacent_positions {
            if adj_row < 15 && adj_col < 15 {
                if board.get(adj_row, adj_col).is_some() {
                    return true;
                }
            }
        }
    }

    false
}

fn calculate_move_score(
    word: &str,
    move_builder: &PlayerMoveBuilder,
    board: &Board,
    scorer: &ScoreCalculator,
) -> u32 {
    // Simplified scoring - real implementation accounts for premium squares
    let base_score = scorer.calculate_score(word, 0.0, 1.0);

    // Apply premium square bonuses
    let mut total_score = base_score;

    for &(row, col) in &move_builder.placement_positions {
        let premium = board.get_premium(row, col);
        total_score = apply_premium_bonus(total_score, premium);
    }

    total_score
}

fn apply_premium_bonus(score: u32, premium: super::components::PremiumSquare) -> u32 {
    use super::components::PremiumSquare::*;

    match premium {
        DoubleLetter => score + (score / 4),  // Rough approximation
        TripleLetter => score + (score / 2),
        DoubleWord => score * 2,
        TripleWord => score * 3,
        Center => score * 2,
        Normal => score,
    }
}
