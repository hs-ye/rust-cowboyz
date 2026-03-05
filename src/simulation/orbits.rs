use crate::simulation::economy;
use crate::simulation::planet_types::PlanetType;
use serde::{Deserialize, Serialize};

/// Turn-based orbital position: integer from 0 to (orbital_period - 1)
/// This represents the planet's position in its orbit at a given turn
#[derive(Debug, Clone, PartialEq, Copy, Eq, Default, Serialize, Deserialize)]
pub struct Position {
    pub orbital_position: u32,  // Integer position in orbit (0 to orbital_period-1)
}

impl Position {
    /// Creates a new position at the given orbital position
    pub fn new(orbital_position: u32) -> Self {
        Position { orbital_position }
    }

    /// Creates a position at the starting point (position 0)
    pub fn start() -> Self {
        Position { orbital_position: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub id: String,
    pub orbit_radius: u32,  // Integer distance from star (in arbitrary units)
    pub orbit_period: u32,  // Turns to complete one orbit
    pub position: Position,
    pub economy: economy::PlanetEconomy,
    pub planet_type: PlanetType,
}

/// Calculates planet position at given turn using turn-based orbital mechanics
/// Position is an integer from 0 to (orbital_period - 1)
pub fn calculate_orbit_position(orbit_period: u32, current_turn: u32) -> Position {
    if orbit_period == 0 {
        return Position::start();
    }
    Position::new(current_turn % orbit_period)
}

/// Advances all planet positions by 1 turn, wrapping around at orbital period
pub fn advance_planet_positions(planets: &mut [Planet]) {
    for planet in planets.iter_mut() {
        if planet.orbit_period > 0 {
            planet.position.orbital_position = (planet.position.orbital_position + 1) % planet.orbit_period;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::economy::PlanetEconomy;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_orbit_position() {
        // Test position calculation for a planet with 10-turn orbit period
        assert_eq!(calculate_orbit_position(10, 0).orbital_position, 0);
        assert_eq!(calculate_orbit_position(10, 5).orbital_position, 5);
        assert_eq!(calculate_orbit_position(10, 10).orbital_position, 0); // Wraps around
        assert_eq!(calculate_orbit_position(10, 15).orbital_position, 5);
    }

    #[test]
    fn test_calculate_orbit_position_zero_period() {
        // Edge case: zero orbital period
        let pos = calculate_orbit_position(0, 100);
        assert_eq!(pos.orbital_position, 0);
    }

    #[test]
    fn test_advance_planet_positions() {
        let mut planets = vec![
            Planet {
                id: "earth".to_string(),
                orbit_radius: 5,
                orbit_period: 10,
                position: Position::new(5),
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
                position: Position::new(14),
                economy: PlanetEconomy { 
                    market: HashMap::new(),
                    planet_type: PlanetType::Mining,
                    active_events: Vec::new(),
                },
                planet_type: PlanetType::Mining,
            },
        ];

        // Advance positions
        advance_planet_positions(&mut planets);

        // Earth: 5 -> 6
        assert_eq!(planets[0].position.orbital_position, 6);
        // Mars: 14 -> 15 -> wraps to 0
        assert_eq!(planets[1].position.orbital_position, 0);
    }

    #[test]
    fn test_advance_planet_positions_zero_period() {
        // Edge case: planet with zero orbital period
        let mut planets = vec![
            Planet {
                id: "station".to_string(),
                orbit_radius: 3,
                orbit_period: 0,
                position: Position::new(5),
                economy: PlanetEconomy { 
                    market: HashMap::new(),
                    planet_type: PlanetType::PirateSpaceStation,
                    active_events: Vec::new(),
                },
                planet_type: PlanetType::PirateSpaceStation,
            },
        ];

        advance_planet_positions(&mut planets);
        // Should not change since orbit_period is 0
        assert_eq!(planets[0].position.orbital_position, 5);
    }

    #[test]
    fn test_position_new_and_start() {
        let pos1 = Position::new(5);
        assert_eq!(pos1.orbital_position, 5);

        let pos2 = Position::start();
        assert_eq!(pos2.orbital_position, 0);
    }
}