/// Scoring module for word tile game mechanics
///
/// Implements standard tile point values and scoring rules

use bevy::prelude::*;
use std::collections::HashMap;

/// Tile point values (standard letter distribution)
pub struct TileValues {
    values: HashMap<char, u32>,
}

impl TileValues {
    /// Creates a new TileValues with standard tile point values
    pub fn new() -> Self {
        let mut values = HashMap::new();

        // 1 point: A, E, I, O, U, L, N, S, T, R
        for c in ['A', 'E', 'I', 'O', 'U', 'L', 'N', 'S', 'T', 'R'] {
            values.insert(c, 1);
        }

        // 2 points: D, G
        for c in ['D', 'G'] {
            values.insert(c, 2);
        }

        // 3 points: B, C, M, P
        for c in ['B', 'C', 'M', 'P'] {
            values.insert(c, 3);
        }

        // 4 points: F, H, V, W, Y
        for c in ['F', 'H', 'V', 'W', 'Y'] {
            values.insert(c, 4);
        }

        // 5 points: K
        values.insert('K', 5);

        // 8 points: J, X
        for c in ['J', 'X'] {
            values.insert(c, 8);
        }

        // 10 points: Q, Z
        for c in ['Q', 'Z'] {
            values.insert(c, 10);
        }

        Self { values }
    }

    /// Gets the point value for a single tile
    ///
    /// # Arguments
    /// * `tile` - The letter (case-insensitive)
    ///
    /// # Returns
    /// * Point value (0 if not found)
    pub fn get_value(&self, tile: char) -> u32 {
        let uppercase = tile.to_ascii_uppercase();
        *self.values.get(&uppercase).unwrap_or(&0)
    }

    /// Calculates the base score for a word (sum of tile values)
    ///
    /// # Arguments
    /// * `word` - The word to score
    ///
    /// # Returns
    /// * Base score (sum of all tile values)
    pub fn score_word(&self, word: &str) -> u32 {
        word.chars()
            .map(|c| self.get_value(c))
            .sum()
    }
}

impl Default for TileValues {
    fn default() -> Self {
        Self::new()
    }
}

/// Scoring calculator for gameplay
#[derive(Resource)]
pub struct ScoreCalculator {
    tile_values: TileValues,
}

impl ScoreCalculator {
    pub fn new() -> Self {
        Self {
            tile_values: TileValues::new(),
        }
    }

    /// Calculates score for a word with time bonus and multiplier
    ///
    /// # Arguments
    /// * `word` - The word formed
    /// * `time_remaining_percent` - Percentage of time remaining (0.0 - 1.0)
    /// * `combo_multiplier` - Combo multiplier (1.0 = no combo, 2.0 = 2x, etc.)
    ///
    /// # Returns
    /// * Total score including all bonuses
    pub fn calculate_score(
        &self,
        word: &str,
        time_remaining_percent: f32,
        combo_multiplier: f32,
    ) -> u32 {
        let base_score = self.tile_values.score_word(word);

        // Time bonus: up to 50% bonus for quick plays
        let time_bonus = (base_score as f32 * time_remaining_percent * 0.5) as u32;

        // Apply combo multiplier
        let total = ((base_score + time_bonus) as f32 * combo_multiplier) as u32;

        total
    }

    /// Calculates score for Stage 1 (Falling Letters)
    ///
    /// # Arguments
    /// * `word` - The word formed
    /// * `time_remaining_ms` - Milliseconds remaining
    /// * `total_time_ms` - Total time allowed
    /// * `combo_count` - Current combo count (0 = no combo)
    ///
    /// # Returns
    /// * Total score
    pub fn calculate_stage1_score(
        &self,
        word: &str,
        time_remaining_ms: u32,
        total_time_ms: u32,
        combo_count: u32,
    ) -> u32 {
        let time_percent = (time_remaining_ms as f32) / (total_time_ms as f32);

        // Combo multiplier: 1x, 1.5x, 2x, 2.5x, 3x, capped at 3x
        let combo_multiplier = if combo_count == 0 {
            1.0
        } else {
            (1.0 + (combo_count as f32 * 0.5)).min(3.0)
        };

        self.calculate_score(word, time_percent, combo_multiplier)
    }

    /// Gets the base tile value for a character
    pub fn get_tile_value(&self, tile: char) -> u32 {
        self.tile_values.get_value(tile)
    }
}

impl Default for ScoreCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_values() {
        let values = TileValues::new();

        // 1-point tiles
        assert_eq!(values.get_value('A'), 1);
        assert_eq!(values.get_value('E'), 1);
        assert_eq!(values.get_value('a'), 1); // case-insensitive

        // High-value tiles
        assert_eq!(values.get_value('Q'), 10);
        assert_eq!(values.get_value('Z'), 10);
        assert_eq!(values.get_value('X'), 8);
        assert_eq!(values.get_value('J'), 8);
    }

    #[test]
    fn test_score_word() {
        let values = TileValues::new();

        // AA = 1 + 1 = 2
        assert_eq!(values.score_word("AA"), 2);

        // QI = 10 + 1 = 11
        assert_eq!(values.score_word("QI"), 11);

        // ZA = 10 + 1 = 11
        assert_eq!(values.score_word("ZA"), 11);
    }

    #[test]
    fn test_score_calculator() {
        let calc = ScoreCalculator::new();

        // Base score with no bonuses
        let score = calc.calculate_score("AA", 0.0, 1.0);
        assert_eq!(score, 2);

        // With time bonus (50% time remaining = +25% score)
        let score = calc.calculate_score("AA", 0.5, 1.0);
        assert_eq!(score, 2); // 2 base + 0.5 bonus = 2 (rounded)

        // With combo multiplier
        let score = calc.calculate_score("AA", 0.0, 2.0);
        assert_eq!(score, 4); // 2 base * 2x = 4
    }

    #[test]
    fn test_stage1_scoring() {
        let calc = ScoreCalculator::new();

        // No combo, full time
        let score = calc.calculate_stage1_score("AA", 45000, 45000, 0);
        assert_eq!(score, 3); // 2 base + 50% time bonus

        // 2x combo
        let score = calc.calculate_stage1_score("AA", 45000, 45000, 2);
        assert_eq!(score, 6); // (2 base + 1 time bonus) * 2x

        // No time remaining
        let score = calc.calculate_stage1_score("AA", 0, 45000, 0);
        assert_eq!(score, 2); // Just base score
    }
}
