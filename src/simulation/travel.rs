use crate::simulation::orbits::Planet;

/// Calculates travel time between two planets using the Brachistochrone model
/// Formula: travel_turns = 2 * sqrt(base_distance / acceleration)
/// 
/// Where:
/// - base_distance = |destination.orbital_radius - departure.orbital_radius|
/// - acceleration = ship acceleration (default 1 unit/turn²)
/// 
/// The ship accelerates for half the journey and decelerates for the second half.
pub fn calculate_travel_turns(departure: &Planet, destination: &Planet, ship_acceleration: u32) -> u32 {
    // Calculate base distance based on orbital radii
    let base_distance = departure.orbit_radius.abs_diff(destination.orbit_radius);
    
    // Ensure at least 1 turn for any non-zero distance
    if base_distance == 0 {
        return 1;
    }
    
    // Calculate travel time using Brachistochrone model
    // travel_time = 2 * sqrt(distance / acceleration)
    let travel_turns = 2.0 * (base_distance as f64 / ship_acceleration as f64).sqrt();
    
    // Ensure at least 1 turn and return as u32
    std::cmp::max(travel_turns.ceil() as u32, 1)
}

/// Legacy function for backward compatibility - redirects to new calculation
#[deprecated(since = "0.1.0", note = "Use calculate_travel_turns instead")]
pub fn calculate_travel_time(origin: &Planet, target: &Planet, _ship_speed: f64) -> u32 {
    calculate_travel_turns(origin, target, 1) // Default acceleration of 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::economy::PlanetEconomy;
    use crate::simulation::orbits::Position;
    use crate::simulation::planet_types::PlanetType;
    use std::collections::HashMap;

    #[test]
    fn test_calculate_travel_turns() {
        let planet1 = Planet {
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
        };
        let planet2 = Planet {
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
        };

        // Base distance = |12 - 5| = 7
        // Travel time = 2 * sqrt(7/1) = 2 * 2.645... = 5.29... → 6 turns
        let travel_time = calculate_travel_turns(&planet1, &planet2, 1);
        assert_eq!(travel_time, 6);
    }

    #[test]
    fn test_calculate_travel_turns_same_radius() {
        let planet1 = Planet {
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
        };
        let planet2 = Planet {
            id: "mars".to_string(),
            orbit_radius: 5,
            orbit_period: 15,
            position: Position::new(7),
            economy: PlanetEconomy { 
                market: HashMap::new(),
                planet_type: PlanetType::Mining,
                active_events: Vec::new(),
            },
            planet_type: PlanetType::Mining,
        };

        // Base distance = |5 - 5| = 0, should return minimum 1 turn
        let travel_time = calculate_travel_turns(&planet1, &planet2, 1);
        assert_eq!(travel_time, 1);
    }

    #[test]
    fn test_calculate_travel_turns_with_acceleration() {
        let planet1 = Planet {
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
        };
        let planet2 = Planet {
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
        };

        // Base distance = |12 - 5| = 7
        // With acceleration = 4: travel_time = 2 * sqrt(7/4) = 2 * 1.322... = 2.64... → 3 turns
        let travel_time = calculate_travel_turns(&planet1, &planet2, 4);
        assert_eq!(travel_time, 3);
    }

    #[test]
    fn test_calculate_travel_time_legacy() {
        let planet1 = Planet {
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
        };
        let planet2 = Planet {
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
        };

        // Legacy function should still work (uses default acceleration of 1)
        let travel_time = calculate_travel_time(&planet1, &planet2, 0.5);
        assert_eq!(travel_time, 6);
    }
}