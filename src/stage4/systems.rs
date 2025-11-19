/// Core gameplay systems for Stage 4 (Speed Challenge)

use bevy::prelude::*;
use super::{Stage4State, Stage4Config, TilePool, WordEvent};
use crate::lexicon::Lexicon;
use crate::scoring::ScoreCalculator;

/// Handle tile selection from rack
pub fn handle_tile_selection(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<Stage4State>,
) {
    if !state.is_active {
        return;
    }

    // Select tiles with number keys 1-7
    for (key, index) in [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
        (KeyCode::Digit4, 3),
        (KeyCode::Digit5, 4),
        (KeyCode::Digit6, 5),
        (KeyCode::Digit7, 6),
    ] {
        if keyboard.just_pressed(key) {
            if state.selected_indices.contains(&index) {
                state.selected_indices.retain(|&i| i != index);
            } else {
                state.selected_indices.push(index);
            }
        }
    }

    // Clear selection with 'C'
    if keyboard.just_pressed(KeyCode::KeyC) {
        state.selected_indices.clear();
    }
}

/// Handle word submission
pub fn handle_word_submission(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<Stage4State>,
) {
    if !state.is_active {
        return;
    }

    // Submit with Enter/Space
    if (keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space))
        && !state.selected_indices.is_empty()
    {
        // Validation happens in validate_word system
    }
}

/// Validate submitted word
pub fn validate_word(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<Stage4State>,
    lexicon: Res<Lexicon>,
    mut word_events: EventWriter<WordEvent>,
    scorer: Res<ScoreCalculator>,
    config: Res<Stage4Config>,
) {
    if !state.is_active {
        return;
    }

    // Check if word was just submitted
    if !(keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::Space)) {
        return;
    }

    if state.selected_indices.is_empty() {
        return;
    }

    // Build word from selected tiles
    let mut word = String::new();
    for &idx in &state.selected_indices {
        if let Some(&letter) = state.rack.get(idx) {
            word.push(letter);
        }
    }

    // Validate word
    if !lexicon.is_valid(&word.to_uppercase()) {
        // Invalid word - break streak
        state.current_streak = 0;
        state.selected_indices.clear();
        return;
    }

    // Calculate score with streak multiplier
    let base_score = scorer.calculate_word_score(&word, &[]);
    let streak_multiplier = config.streak_bonus_multiplier.powi(state.current_streak as i32);
    let final_score = (base_score as f32 * streak_multiplier) as u32;

    // Send word event
    word_events.send(WordEvent {
        word: word.clone(),
        score: final_score,
        streak_multiplier,
    });
}

/// Score valid word
pub fn score_word(
    mut state: ResMut<Stage4State>,
    mut word_events: EventReader<WordEvent>,
) {
    for event in word_events.read() {
        state.score += event.score;
        state.words_formed += 1;
        state.words_history.push(event.word.clone());
    }
}

/// Update streak
pub fn update_streak(
    mut state: ResMut<Stage4State>,
    mut word_events: EventReader<WordEvent>,
) {
    for _event in word_events.read() {
        state.current_streak += 1;
        if state.current_streak > state.best_streak {
            state.best_streak = state.current_streak;
        }
    }
}

/// Refresh rack after word
pub fn refresh_rack(
    mut state: ResMut<Stage4State>,
    tile_pool: Res<TilePool>,
    mut word_events: EventReader<WordEvent>,
) {
    for _event in word_events.read() {
        // Remove used tiles (in reverse to preserve indices)
        for &idx in state.selected_indices.iter().rev() {
            if idx < state.rack.len() {
                state.rack.remove(idx);
            }
        }

        // Refill rack to 7 tiles
        let needed = 7 - state.rack.len();
        state.rack.extend(tile_pool.draw(needed));

        // Clear selection
        state.selected_indices.clear();
    }
}

/// Update game timer
pub fn update_timer(
    mut state: ResMut<Stage4State>,
    time: Res<Time>,
) {
    if !state.is_active {
        return;
    }

    let delta_ms = (time.delta_seconds() * 1000.0) as u32;

    if state.time_remaining_ms > delta_ms {
        state.time_remaining_ms -= delta_ms;
    } else {
        state.time_remaining_ms = 0;
    }
}

/// Check for game over
pub fn check_game_over(
    mut state: ResMut<Stage4State>,
) {
    if state.time_remaining_ms == 0 {
        state.is_active = false;
    }
}

/// Update panic mode
pub fn update_panic_mode(
    mut state: ResMut<Stage4State>,
    config: Res<Stage4Config>,
) {
    let difficulty = super::difficulty::get_speed_difficulty(config.difficulty);
    let time_seconds = state.time_remaining_ms / 1000;

    state.is_panic_mode = time_seconds < difficulty.panic_threshold_seconds;
}
