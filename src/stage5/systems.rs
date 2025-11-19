/// Core gameplay systems for Stage 5 (AI Tournaments)

use bevy::prelude::*;
use bevy::text::TextStyle;
use super::{TournamentState, CurrentMatch, MatchResult};
use super::components::*;

/// Handle match gameplay (reuses Stage 3 board logic)
pub fn handle_match_gameplay(
    current_match: Res<CurrentMatch>,
) {
    if !current_match.is_active {
        return;
    }

    // Match gameplay handled by Stage 3 systems
    // This system coordinates with board play
}

/// Process match result when game ends
pub fn process_match_result(
    mut current_match: ResMut<CurrentMatch>,
    mut match_results: EventWriter<MatchResult>,
) {
    if !current_match.is_active {
        return;
    }

    // Check if current game is over
    // (Simplified - would check board state in real implementation)

    // Determine game winner and update match score
    // Send MatchResult event when best-of-3 is complete
}

/// Advance tournament bracket
pub fn advance_tournament(
    mut tournament_state: ResMut<TournamentState>,
    mut match_results: EventReader<MatchResult>,
) {
    for result in match_results.read() {
        if result.player_won {
            // Player advances
            tournament_state.advance_match();
        } else {
            // Player eliminated
            tournament_state.is_complete = true;
        }

        // Check if round is complete
        if tournament_state.is_round_complete() {
            advance_to_next_round(&mut tournament_state);
        }
    }
}

/// Check if tournament is complete
pub fn check_tournament_complete(
    tournament_state: Res<TournamentState>,
) {
    if tournament_state.is_complete {
        // Show victory/defeat screen
    }
}

/// Start next match
pub fn start_next_match(
    mut commands: Commands,
    tournament_state: Res<TournamentState>,
    mut current_match: ResMut<CurrentMatch>,
) {
    // Get next opponent from bracket
    let opponent_index = tournament_state.current_match_index + 1;

    if let Some(opponent) = tournament_state.players.get(opponent_index) {
        *current_match = CurrentMatch::new(
            tournament_state.players[0].clone(), // Player is always index 0
            opponent.clone(),
        );
    }
}

fn advance_to_next_round(tournament_state: &mut TournamentState) {
    use TournamentRound::*;

    match tournament_state.current_round {
        Quarterfinals => tournament_state.start_round(Semifinals),
        Semifinals => tournament_state.start_round(Finals),
        Finals => {
            tournament_state.is_complete = true;
            // Set champion
        }
    }
}
