/// Difficulty levels for Stage 1

/// Difficulty level configuration
#[derive(Debug, Clone, Copy)]
pub struct DifficultyLevel {
    pub level: u8,
    pub name: &'static str,
    pub total_time_seconds: u32,
    pub fall_speed: f32,
    pub spawn_interval: f32,
}

/// Available difficulty levels for Stage 1
pub const DIFFICULTY_LEVELS: [DifficultyLevel; 5] = [
    // D1: Beginner
    DifficultyLevel {
        level: 1,
        name: "Beginner",
        total_time_seconds: 90,
        fall_speed: 80.0,
        spawn_interval: 3.0,
    },
    // D2: Easy
    DifficultyLevel {
        level: 2,
        name: "Easy",
        total_time_seconds: 75,
        fall_speed: 100.0,
        spawn_interval: 2.5,
    },
    // D3: Medium
    DifficultyLevel {
        level: 3,
        name: "Medium",
        total_time_seconds: 60,
        fall_speed: 130.0,
        spawn_interval: 2.0,
    },
    // D4: Hard
    DifficultyLevel {
        level: 4,
        name: "Hard",
        total_time_seconds: 50,
        fall_speed: 160.0,
        spawn_interval: 1.5,
    },
    // D5: Expert
    DifficultyLevel {
        level: 5,
        name: "Expert",
        total_time_seconds: 45,
        fall_speed: 200.0,
        spawn_interval: 1.0,
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
        assert_eq!(d1.total_time_seconds, 90);

        let d5 = get_difficulty(5);
        assert_eq!(d5.level, 5);
        assert_eq!(d5.name, "Expert");
        assert_eq!(d5.total_time_seconds, 45);
    }

    #[test]
    fn test_difficulty_clamping() {
        // Test out of range values
        let d_low = get_difficulty(0);
        assert_eq!(d_low.level, 1); // Should clamp to 1

        let d_high = get_difficulty(10);
        assert_eq!(d_high.level, 5); // Should clamp to 5
    }

    #[test]
    fn test_difficulty_progression() {
        // Ensure difficulty increases
        for i in 1..5 {
            let current = get_difficulty(i);
            let next = get_difficulty(i + 1);

            assert!(next.fall_speed > current.fall_speed);
            assert!(next.spawn_interval < current.spawn_interval);
            assert!(next.total_time_seconds <= current.total_time_seconds);
        }
    }
}
