/// AI opponent personalities for Stage 5

/// AI playing style personality
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIPersonality {
    Aggressive,  // Prioritizes high-scoring moves, takes risks
    Defensive,   // Blocks opponent opportunities, plays safe
    Balanced,    // Mix of offensive and defensive play
}

impl AIPersonality {
    /// Get move selection bias
    pub fn get_move_bias(&self) -> MoveBias {
        match self {
            AIPersonality::Aggressive => MoveBias {
                min_score_threshold: 25,
                prefers_high_risk: true,
                blocking_priority: 0.3,
                scoring_priority: 0.9,
            },
            AIPersonality::Defensive => MoveBias {
                min_score_threshold: 10,
                prefers_high_risk: false,
                blocking_priority: 0.8,
                scoring_priority: 0.5,
            },
            AIPersonality::Balanced => MoveBias {
                min_score_threshold: 15,
                prefers_high_risk: false,
                blocking_priority: 0.6,
                scoring_priority: 0.7,
            },
        }
    }

    /// Get personality description
    pub fn description(&self) -> &'static str {
        match self {
            AIPersonality::Aggressive => "Plays aggressively for high scores",
            AIPersonality::Defensive => "Plays defensively, blocks opportunities",
            AIPersonality::Balanced => "Balanced offensive and defensive play",
        }
    }
}

/// Move selection bias for AI
pub struct MoveBias {
    pub min_score_threshold: u32,
    pub prefers_high_risk: bool,
    pub blocking_priority: f32,
    pub scoring_priority: f32,
}
