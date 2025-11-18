/// Difficulty levels for Stage 2

/// Difficulty level configuration
#[derive(Debug, Clone, Copy)]
pub struct DifficultyLevel {
    pub level: u8,
    pub name: &'static str,
    pub time_limit_seconds: u32,
    pub target_score: u32,
    pub moves_limit: u32, // 0 = unlimited
}

/// Available difficulty levels for Stage 2
pub const DIFFICULTY_LEVELS: [DifficultyLevel; 5] = [
    // D1: Beginner
    DifficultyLevel {
        level: 1,
        name: "Beginner",
        time_limit_seconds: 180, // 3 minutes
        target_score: 500,
        moves_limit: 0, // Unlimited
    },
    // D2: Easy
    DifficultyLevel {
        level: 2,
        name: "Easy",
        time_limit_seconds: 150, // 2.5 minutes
        target_score: 750,
        moves_limit: 0, // Unlimited
    },
    // D3: Medium
    DifficultyLevel {
        level: 3,
        name: "Medium",
        time_limit_seconds: 120, // 2 minutes
        target_score: 1000,
        moves_limit: 50, // Limited moves
    },
    // D4: Hard
    DifficultyLevel {
        level: 4,
        name: "Hard",
        time_limit_seconds: 90, // 1.5 minutes
        target_score: 1500,
        moves_limit: 40, // Limited moves
    },
    // D5: Expert
    DifficultyLevel {
        level: 5,
        name: "Expert",
        time_limit_seconds: 60, // 1 minute
        target_score: 2000,
        moves_limit: 30, // Very limited moves
    },
];

/// Get difficulty configuration by level (1-5)
pub fn get_difficulty(level: u8) -> DifficultyLevel {
    let index = (level.clamp(1, 5) - 1) as usize;
    DIFFICULTY_LEVELS[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_levels() {
        let d1 = get_difficulty(1);
        assert_eq!(d1.level, 1);
        assert_eq!(d1.name, "Beginner");
        assert_eq!(d1.time_limit_seconds, 180);

        let d5 = get_difficulty(5);
        assert_eq!(d5.level, 5);
        assert_eq!(d5.name, "Expert");
        assert_eq!(d5.time_limit_seconds, 60);
    }

    #[test]
    fn test_difficulty_clamping() {
        let d_low = get_difficulty(0);
        assert_eq!(d_low.level, 1);

        let d_high = get_difficulty(10);
        assert_eq!(d_high.level, 5);
    }

    #[test]
    fn test_difficulty_progression() {
        for i in 1..5 {
            let current = get_difficulty(i);
            let next = get_difficulty(i + 1);

            // Time should decrease
            assert!(next.time_limit_seconds < current.time_limit_seconds);
            // Target score should increase
            assert!(next.target_score > current.target_score);
        }
    }
}
