/// Difficulty levels for Stage 4 (Speed Challenge)

/// Speed challenge difficulty configuration
#[derive(Debug, Clone)]
pub struct SpeedDifficulty {
    pub level: u8,
    pub name: &'static str,
    pub time_limit_seconds: u32,
    pub target_score: u32,
    pub streak_bonus_multiplier: f32,
    pub panic_threshold_seconds: u32,
}

/// 5 difficulty levels for Speed Challenge
pub const SPEED_DIFFICULTIES: [SpeedDifficulty; 5] = [
    // Level 1: Relaxed
    SpeedDifficulty {
        level: 1,
        name: "Relaxed",
        time_limit_seconds: 120,   // 2 minutes
        target_score: 300,
        streak_bonus_multiplier: 1.05,
        panic_threshold_seconds: 15,
    },
    // Level 2: Moderate
    SpeedDifficulty {
        level: 2,
        name: "Moderate",
        time_limit_seconds: 90,
        target_score: 500,
        streak_bonus_multiplier: 1.1,
        panic_threshold_seconds: 12,
    },
    // Level 3: Challenging
    SpeedDifficulty {
        level: 3,
        name: "Challenging",
        time_limit_seconds: 75,
        target_score: 750,
        streak_bonus_multiplier: 1.15,
        panic_threshold_seconds: 10,
    },
    // Level 4: Intense
    SpeedDifficulty {
        level: 4,
        name: "Intense",
        time_limit_seconds: 60,
        target_score: 1000,
        streak_bonus_multiplier: 1.2,
        panic_threshold_seconds: 8,
    },
    // Level 5: Extreme
    SpeedDifficulty {
        level: 5,
        name: "Extreme",
        time_limit_seconds: 45,
        target_score: 1500,
        streak_bonus_multiplier: 1.25,
        panic_threshold_seconds: 5,
    },
];

/// Get difficulty config by level
pub fn get_speed_difficulty(level: u8) -> &'static SpeedDifficulty {
    SPEED_DIFFICULTIES
        .iter()
        .find(|d| d.level == level)
        .unwrap_or(&SPEED_DIFFICULTIES[2]) // Default to level 3
}
