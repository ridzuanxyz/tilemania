/// Difficulty levels for Stage 3 AI opponent

use bevy::prelude::*;

/// AI difficulty level configuration
#[derive(Debug, Clone)]
pub struct AIFifficulty {
    pub level: u8,
    pub name: &'static str,
    pub move_quality: f32,      // 0.0-1.0, higher = better moves
    pub vocabulary_size: f32,   // 0.0-1.0, percentage of lexicon used
    pub think_time_ms: u32,     // Simulated thinking time
    pub uses_strategic_play: bool, // Plans ahead, blocks player
    pub error_rate: f32,        // 0.0-1.0, chance of suboptimal move
}

/// 5 difficulty levels for AI
pub const AI_DIFFICULTIES: [AIDifficulty; 5] = [
    // Level 1: Beginner AI
    AIDifficulty {
        level: 1,
        name: "Beginner AI",
        move_quality: 0.3,      // Often plays short words
        vocabulary_size: 0.4,   // Only knows common words
        think_time_ms: 1000,
        uses_strategic_play: false,
        error_rate: 0.4,        // 40% chance of making a mistake
    },
    // Level 2: Novice AI
    AIDifficulty {
        level: 2,
        name: "Novice AI",
        move_quality: 0.5,
        vocabulary_size: 0.6,
        think_time_ms: 1500,
        uses_strategic_play: false,
        error_rate: 0.25,
    },
    // Level 3: Intermediate AI
    AIDifficulty {
        level: 3,
        name: "Intermediate AI",
        move_quality: 0.7,
        vocabulary_size: 0.8,
        think_time_ms: 2000,
        uses_strategic_play: true,  // Starts using strategy
        error_rate: 0.15,
    },
    // Level 4: Advanced AI
    AIDifficulty {
        level: 4,
        name: "Advanced AI",
        move_quality: 0.85,
        vocabulary_size: 0.95,
        think_time_ms: 2500,
        uses_strategic_play: true,
        error_rate: 0.08,
    },
    // Level 5: Expert AI
    AIDifficulty {
        level: 5,
        name: "Expert AI",
        move_quality: 0.98,     // Nearly perfect play
        vocabulary_size: 1.0,   // Knows entire lexicon
        think_time_ms: 3000,
        uses_strategic_play: true,
        error_rate: 0.02,       // Very rare mistakes
    },
];

/// Get AI difficulty config by level
pub fn get_ai_difficulty(level: u8) -> &'static AIDifficulty {
    AI_DIFFICULTIES
        .iter()
        .find(|d| d.level == level)
        .unwrap_or(&AI_DIFFICULTIES[2]) // Default to level 3
}

impl AIDifficulty {
    /// Calculate if AI should make a suboptimal move
    pub fn should_make_error(&self) -> bool {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        rng.gen::<f32>() < self.error_rate
    }

    /// Get maximum word length AI will consider
    pub fn max_word_length(&self) -> usize {
        match self.level {
            1 => 5,  // Beginner: max 5-letter words
            2 => 6,
            3 => 7,
            4 => 10,
            5 => 15, // Expert: uses all available tiles
            _ => 7,
        }
    }

    /// Get scoring threshold (min score to consider a move)
    pub fn min_move_score(&self) -> u32 {
        match self.level {
            1 => 5,   // Beginner: accepts any move >= 5 points
            2 => 8,
            3 => 12,
            4 => 15,
            5 => 20,  // Expert: prefers high-scoring moves
            _ => 10,
        }
    }

    /// Check if AI will use premium squares strategically
    pub fn uses_premium_squares(&self) -> bool {
        self.level >= 3
    }

    /// Check if AI will block player's high-scoring opportunities
    pub fn uses_blocking(&self) -> bool {
        self.level >= 4
    }

    /// Get number of candidate moves to consider
    pub fn candidate_move_count(&self) -> usize {
        match self.level {
            1 => 3,   // Consider top 3 moves
            2 => 5,
            3 => 10,
            4 => 20,
            5 => 50,  // Expert: exhaustive search
            _ => 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_levels() {
        assert_eq!(AI_DIFFICULTIES.len(), 5);
        assert_eq!(get_ai_difficulty(1).name, "Beginner AI");
        assert_eq!(get_ai_difficulty(5).name, "Expert AI");
    }

    #[test]
    fn test_difficulty_progression() {
        // Verify difficulty increases with level
        for i in 0..4 {
            let current = &AI_DIFFICULTIES[i];
            let next = &AI_DIFFICULTIES[i + 1];

            assert!(next.move_quality >= current.move_quality);
            assert!(next.vocabulary_size >= current.vocabulary_size);
            assert!(next.error_rate <= current.error_rate);
        }
    }

    #[test]
    fn test_strategic_play() {
        assert!(!AI_DIFFICULTIES[0].uses_strategic_play); // Level 1
        assert!(!AI_DIFFICULTIES[1].uses_strategic_play); // Level 2
        assert!(AI_DIFFICULTIES[2].uses_strategic_play);  // Level 3+
    }
}
