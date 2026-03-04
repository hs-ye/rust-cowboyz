//! Turn Manager - Advances all game systems synchronously during turn-based gameplay
//! Based on ADR 0002: Movement Mechanics System
//!
//! This module ensures that when time advances (through travel or wait actions),
//! all game systems (planets, markets, etc.) are updated consistently.

use crate::game_state::GameClock;
use crate::simulation::orbits::{self, Planet};
use crate::simulation::economy::MarketManager;

/// Result of advancing the game state by one or more turns
#[derive(Debug, Clone)]
pub struct TurnAdvanceResult {
    pub turns_advanced: u32,
    pub new_turn: u32,
    pub is_game_over: bool,
    pub planet_positions_updated: bool,
    pub markets_updated: bool,
}

impl TurnAdvanceResult {
    pub fn new(turns_advanced: u32, new_turn: u32, is_game_over: bool) -> Self {
        TurnAdvanceResult {
            turns_advanced,
            new_turn,
            is_game_over,
            planet_positions_updated: true,
            markets_updated: true,
        }
    }

    pub fn no_changes(turns_advanced: u32, new_turn: u32, is_game_over: bool) -> Self {
        TurnAdvanceResult {
            turns_advanced,
            new_turn,
            is_game_over,
            planet_positions_updated: false,
            markets_updated: false,
        }
    }
}

/// Manages turn-based advancement of all game systems
pub struct TurnManager;

impl TurnManager {
    /// Advance the game by a specified number of turns
    /// This updates:
    /// - Game clock
    /// - Planet orbital positions
    /// - Market prices
    /// 
    /// Returns the result of the turn advancement
    pub fn advance_turns(
        planets: &mut [Planet],
        market_manager: Option<&mut MarketManager>,
        game_clock: &mut GameClock,
        turns: u32,
    ) -> TurnAdvanceResult {
        if turns == 0 {
            return TurnAdvanceResult::no_changes(0, game_clock.current_turn, game_clock.is_game_over());
        }

        // Advance the game clock
        let actual_turns = game_clock.advance(turns);
        
        if actual_turns == 0 {
            return TurnAdvanceResult::no_changes(0, game_clock.current_turn, game_clock.is_game_over());
        }

        // Advance planet orbital positions for each turn
        for _ in 0..actual_turns {
            orbits::advance_planet_positions(planets);
        }

        // Update market prices for each turn
        if let Some(market_mgr) = market_manager {
            for _ in 0..actual_turns {
                market_mgr.update_all_markets();
            }
        }

        TurnAdvanceResult::new(
            actual_turns,
            game_clock.current_turn,
            game_clock.is_game_over(),
        )
    }

    /// Advance by a single turn (convenience method)
    pub fn advance_single_turn(
        planets: &mut [Planet],
        market_manager: Option<&mut MarketManager>,
        game_clock: &mut GameClock,
    ) -> TurnAdvanceResult {
        Self::advance_turns(planets, market_manager, game_clock, 1)
    }

    /// Calculate travel time and return the result without actually advancing
    /// This is useful for displaying travel options to the player
    pub fn preview_travel(
        departure: &Planet,
        destination: &Planet,
        ship_acceleration: u32,
    ) -> u32 {
        crate::simulation::travel::calculate_travel_turns(departure, destination, ship_acceleration)
    }

    /// Check if the game has ended
    pub fn is_game_over(game_clock: &GameClock) -> bool {
        game_clock.is_game_over()
    }

    /// Get the current turn number
    pub fn current_turn(game_clock: &GameClock) -> u32 {
        game_clock.current_turn
    }

    /// Get the total turns in the game
    pub fn total_turns(game_clock: &GameClock) -> u32 {
        game_clock.total_turns
    }

    /// Get turns remaining
    pub fn turns_remaining(game_clock: &GameClock) -> u32 {
        game_clock.turns_remaining()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::economy::PlanetEconomy;
    use crate::simulation::orbits::Position;
    use crate::simulation::planet_types::PlanetType;
    use std::collections::HashMap;

    fn create_test_planets() -> Vec<Planet> {
        vec![
            Planet {
                id: "earth".to_string(),
                orbit_radius: 5,
                orbit_period: 10,
                position: Position::new(0),
                economy: PlanetEconomy {
                    market: HashMap::new(),
                    planet_type: PlanetType::Agricultural,
                    active_events: Vec::new(),
                },
                planet_type: PlanetType::Agricultural,
            },
            Planet {
                id: "mars".to_string(),
                orbit_radius: 12,
                orbit_period: 15,
                position: Position::new(7),
                economy: PlanetEconomy {
                    market: HashMap::new(),
                    planet_type: PlanetType::Mining,
                    active_events: Vec::new(),
                },
                planet_type: PlanetType::Mining,
            },
        ]
    }

    #[test]
    fn test_advance_turns() {
        let mut planets = create_test_planets();
        let mut game_clock = GameClock::new(20);
        
        let initial_turn = game_clock.current_turn;
        
        let result = TurnManager::advance_turns(&mut planets, None, &mut game_clock, 5);
        
        assert_eq!(result.turns_advanced, 5);
        assert_eq!(game_clock.current_turn, initial_turn + 5);
        assert!(result.planet_positions_updated);
    }

    #[test]
    fn test_advance_turns_planet_positions() {
        let mut planets = create_test_planets();
        let mut game_clock = GameClock::new(20);
        
        // Earth starts at position 0
        assert_eq!(planets[0].position.orbital_position, 0);
        // Mars starts at position 7
        assert_eq!(planets[1].position.orbital_position, 7);
        
        // Advance 3 turns
        TurnManager::advance_turns(&mut planets, None, &mut game_clock, 3);
        
        // Earth: 0 -> 3
        assert_eq!(planets[0].position.orbital_position, 3);
        // Mars: 7 -> 10, wraps around (period 15)
        assert_eq!(planets[1].position.orbital_position, 10);
    }

    #[test]
    fn test_advance_single_turn() {
        let mut planets = create_test_planets();
        let mut game_clock = GameClock::new(20);
        
        let result = TurnManager::advance_single_turn(&mut planets, None, &mut game_clock);
        
        assert_eq!(result.turns_advanced, 1);
        assert_eq!(game_clock.current_turn, 2);
    }

    #[test]
    fn test_advance_zero_turns() {
        let mut planets = create_test_planets();
        let mut game_clock = GameClock::new(20);
        
        let result = TurnManager::advance_turns(&mut planets, None, &mut game_clock, 0);
        
        assert_eq!(result.turns_advanced, 0);
        assert!(!result.planet_positions_updated);
        assert!(!result.markets_updated);
    }

    #[test]
    fn test_advance_caps_at_total_turns() {
        let mut planets = create_test_planets();
        let mut game_clock = GameClock::with_start_turn(18, 20);
        
        let result = TurnManager::advance_turns(&mut planets, None, &mut game_clock, 10);
        
        assert_eq!(result.turns_advanced, 2); // Only 2 turns remaining
        assert_eq!(game_clock.current_turn, 20);
        assert!(result.is_game_over);
    }

    #[test]
    fn test_preview_travel() {
        let planets = create_test_planets();
        let travel_time = TurnManager::preview_travel(&planets[0], &planets[1], 1);
        
        // Base distance = |12 - 5| = 7
        // Travel time = 2 * sqrt(7/1) = 5.29... → ceil = 6
        assert_eq!(travel_time, 6);
    }

    #[test]
    fn test_is_game_over() {
        let mut game_clock = GameClock::new(10);
        assert!(!TurnManager::is_game_over(&game_clock));
        
        game_clock.advance(10);
        assert!(TurnManager::is_game_over(&game_clock));
    }

    #[test]
    fn test_turns_remaining() {
        let mut game_clock = GameClock::new(20);
        assert_eq!(TurnManager::turns_remaining(&game_clock), 19);
        
        game_clock.advance(5);
        assert_eq!(TurnManager::turns_remaining(&game_clock), 14);
    }
}