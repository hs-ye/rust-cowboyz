//! Game state management for turn-based system
//! Based on ADR 0002: Movement Mechanics System

/// Game clock that tracks turns in the game
/// The clock advances during travel and wait actions, synchronizing all game systems
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameClock {
    pub current_turn: u32,
    pub total_turns: u32,
}

impl GameClock {
    /// Create a new game clock with the given total turns
    pub fn new(total_turns: u32) -> Self {
        GameClock {
            current_turn: 1, // Game starts at turn 1
            total_turns,
        }
    }

    /// Create a game clock with custom starting turn
    pub fn with_start_turn(current_turn: u32, total_turns: u32) -> Self {
        GameClock {
            current_turn,
            total_turns,
        }
    }

    /// Advance the clock by a specified number of turns
    /// Returns the number of turns actually advanced (capped at total_turns)
    pub fn advance(&mut self, turns: u32) -> u32 {
        let new_turn = self.current_turn + turns;
        let actual_advance = if new_turn > self.total_turns {
            // Cap at total_turns
            let remaining = self.total_turns.saturating_sub(self.current_turn);
            self.current_turn = self.total_turns;
            remaining
        } else {
            self.current_turn = new_turn;
            turns
        };
        actual_advance
    }

    /// Get the number of turns remaining in the game
    pub fn turns_remaining(&self) -> u32 {
        self.total_turns.saturating_sub(self.current_turn)
    }

    /// Check if the game has ended (current_turn >= total_turns)
    pub fn is_game_over(&self) -> bool {
        self.current_turn >= self.total_turns
    }

    /// Check if the game is near the end (within threshold of total_turns)
    pub fn is_near_end(&self, threshold: u32) -> bool {
        self.turns_remaining() <= threshold
    }

    /// Reset the clock to start a new game
    pub fn reset(&mut self) {
        self.current_turn = 1;
    }

    /// Get a progress percentage (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.total_turns == 0 {
            return 1.0;
        }
        (self.current_turn as f64 / self.total_turns as f64).min(1.0)
    }
}

impl Default for GameClock {
    fn default() -> Self {
        Self::new(10) // Default 10 turns for MVP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_clock_new() {
        let clock = GameClock::new(20);
        assert_eq!(clock.current_turn, 1);
        assert_eq!(clock.total_turns, 20);
    }

    #[test]
    fn test_game_clock_with_start_turn() {
        let clock = GameClock::with_start_turn(5, 20);
        assert_eq!(clock.current_turn, 5);
        assert_eq!(clock.total_turns, 20);
    }

    #[test]
    fn test_advance() {
        let mut clock = GameClock::new(20);
        let advanced = clock.advance(5);
        assert_eq!(advanced, 5);
        assert_eq!(clock.current_turn, 6);
    }

    #[test]
    fn test_advance_caps_at_total() {
        let mut clock = GameClock::with_start_turn(18, 20);
        let advanced = clock.advance(5);
        assert_eq!(advanced, 2); // Only 2 turns remaining
        assert_eq!(clock.current_turn, 20);
    }

    #[test]
    fn test_turns_remaining() {
        let mut clock = GameClock::new(20);
        assert_eq!(clock.turns_remaining(), 19); // 20 - 1 = 19
        
        clock.advance(10);
        assert_eq!(clock.turns_remaining(), 9); // 20 - 11 = 9
    }

    #[test]
    fn test_is_game_over() {
        let mut clock = GameClock::new(10);
        assert!(!clock.is_game_over());
        
        clock.advance(10); // Advance to turn 11 (current_turn = 11)
        assert!(clock.is_game_over());
    }

    #[test]
    fn test_is_near_end() {
        let mut clock = GameClock::new(20);
        assert!(!clock.is_near_end(5));
        
        clock.advance(16); // Advance to turn 17
        assert!(clock.is_near_end(5)); // 20 - 17 = 3 <= 5
    }

    #[test]
    fn test_reset() {
        let mut clock = GameClock::with_start_turn(15, 20);
        clock.reset();
        assert_eq!(clock.current_turn, 1);
    }

    #[test]
    fn test_progress() {
        let mut clock = GameClock::new(10);
        assert_eq!(clock.progress(), 0.1); // 1/10
        
        clock.advance(4); // Advance to turn 5
        assert_eq!(clock.progress(), 0.5); // 5/10
        
        clock.advance(10); // Should cap at 10
        assert_eq!(clock.progress(), 1.0); // 10/10
    }

    #[test]
    fn test_progress_zero_total() {
        let clock = GameClock::new(0);
        assert_eq!(clock.progress(), 1.0);
    }
}