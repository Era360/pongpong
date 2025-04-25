use std::fmt;

/// Different gameplay variants
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMode {
    /// Standard Pong gameplay
    Classic,

    /// Ball gradually increases in speed over time
    Accelerating,

    /// Ball speeds up during long rallies
    RallyFever,
}

impl GameMode {
    /// Returns a description of the game mode
    pub fn description(&self) -> &'static str {
        match self {
            GameMode::Classic => "Classic Mode",
            GameMode::Accelerating => "Accelerating Ball",
            GameMode::RallyFever => "Rally Fever",
        }
    }

    /// Cycles to the next game mode
    pub fn next(&self) -> Self {
        match self {
            GameMode::Classic => GameMode::Accelerating,
            GameMode::Accelerating => GameMode::RallyFever,
            GameMode::RallyFever => GameMode::Classic,
        }
    }
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
