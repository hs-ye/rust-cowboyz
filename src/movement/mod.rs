//! Movement mechanics system for Rust Cowboyz
//!
//! This module implements the data structures for the movement system
//! as defined in ADR 0002: Movement Mechanics System.
//!
//! The system uses simplified turn-based orbital mechanics with discrete
//! integer calculations for planet positions and travel time calculations.

use serde::{Deserialize, Serialize};

/// Unique identifier for a planet
/// Using String for flexibility, but could be optimized to u32 indices
pub type PlanetId = String;

/// Represents a planet in the solar system with orbital mechanics
///
/// Planets orbit the central star with fixed orbital periods and radii.
/// Position is tracked as an integer from 0 to (orbital_period - 1),
/// representing the planet's current location in its orbit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Planet {
    /// Human-readable name of the planet
    pub name: String,

    /// Distance from the central star (in arbitrary distance units)
    /// Used for calculating travel distances between planets
    pub orbital_radius: u32,

    /// Number of turns required to complete one full orbit
    /// Determines how quickly the planet moves around the star
    pub orbital_period: u32,

    /// Current position in the orbit (0 to orbital_period - 1)
    /// Advances by 1 each turn, wrapping around at orbital_period
    pub position: u32,
}

impl Planet {
    /// Creates a new planet with the given parameters
    ///
    /// # Arguments
    /// * `name` - The human-readable name of the planet
    /// * `orbital_radius` - Distance from the star
    /// * `orbital_period` - Turns to complete one orbit
    /// * `position` - Starting position in orbit (0 to orbital_period - 1)
    ///
    /// # Example
    /// ```
    /// use cowboyz::movement::Planet;
    ///
    /// let earth = Planet::new(
    ///     "Earth".to_string(),
    ///     10,  // orbital radius
    ///     20,  // orbital period
    ///     0    // starting position
    /// );
    /// ```
    pub fn new(name: String, orbital_radius: u32, orbital_period: u32, position: u32) -> Self {
        Planet {
            name,
            orbital_radius,
            orbital_period,
            position: position % orbital_period.max(1),
        }
    }

    /// Creates a planet at position 0 (the starting point of its orbit)
    pub fn at_start(name: String, orbital_radius: u32, orbital_period: u32) -> Self {
        Self::new(name, orbital_radius, orbital_period, 0)
    }

    /// Advances the planet's position by one turn
    /// Wraps around to 0 after reaching orbital_period
    pub fn advance_position(&mut self) {
        if self.orbital_period > 0 {
            self.position = (self.position + 1) % self.orbital_period;
        }
    }

    /// Calculates the planet's position at a specific turn
    /// Useful for predicting future positions
    pub fn position_at_turn(&self, turn: u32) -> u32 {
        if self.orbital_period == 0 {
            return 0;
        }
        (self.position + turn) % self.orbital_period
    }

    /// Returns the distance from this planet to another planet
    /// Based on the difference in orbital radii
    pub fn distance_to(&self, other: &Planet) -> u32 {
        self.orbital_radius.abs_diff(other.orbital_radius)
    }
}

impl Default for Planet {
    fn default() -> Self {
        Planet {
            name: "Unnamed Planet".to_string(),
            orbital_radius: 10,
            orbital_period: 20,
            position: 0,
        }
    }
}

/// Represents a player's ship with movement capabilities
///
/// Ships can travel between planets using the Brachistochrone travel model
/// where they accelerate for half the journey and decelerate for the second half.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ship {
    /// The ID of the planet where the ship is currently located
    /// Empty string indicates the ship is not at any planet (should not happen in normal gameplay)
    pub current_location: PlanetId,

    /// Ship acceleration in units per turn squared (units/turn²)
    /// Default is 1, can be upgraded through ship improvements
    /// Higher acceleration means faster travel times
    pub acceleration: u32,

    /// Maximum fuel capacity of the ship
    /// Determines how far the ship can travel before needing to refuel
    pub fuel_capacity: u32,

    /// Current fuel level
    /// Decreases when traveling, can be refueled at planets
    pub current_fuel: u32,
}

impl Ship {
    /// Creates a new ship with default values
    ///
    /// # Arguments
    /// * `current_location` - The planet ID where the ship starts
    /// * `fuel_capacity` - Maximum fuel capacity
    ///
    /// # Example
    /// ```
    /// use cowboyz::movement::Ship;
    ///
    /// let ship = Ship::new("earth".to_string(), 100);
    /// assert_eq!(ship.acceleration, 1);  // Default acceleration
    /// assert_eq!(ship.current_fuel, 100); // Starts with full fuel
    /// ```
    pub fn new(current_location: PlanetId, fuel_capacity: u32) -> Self {
        Ship {
            current_location,
            acceleration: 1, // Default acceleration as per ADR 0002
            fuel_capacity,
            current_fuel: fuel_capacity, // Start with full tank
        }
    }

    /// Creates a ship with custom acceleration
    pub fn with_acceleration(
        current_location: PlanetId,
        fuel_capacity: u32,
        acceleration: u32,
    ) -> Self {
        Ship {
            current_location,
            acceleration: acceleration.max(1), // Ensure at least 1
            fuel_capacity,
            current_fuel: fuel_capacity,
        }
    }

    /// Creates a ship with full configuration
    pub fn with_full_config(
        current_location: PlanetId,
        fuel_capacity: u32,
        current_fuel: u32,
        acceleration: u32,
    ) -> Self {
        Ship {
            current_location,
            acceleration: acceleration.max(1),
            fuel_capacity,
            current_fuel: current_fuel.min(fuel_capacity),
        }
    }

    /// Checks if the ship has enough fuel for a journey
    ///
    /// # Arguments
    /// * `fuel_cost` - The amount of fuel required for the journey
    pub fn has_enough_fuel(&self, fuel_cost: u32) -> bool {
        self.current_fuel >= fuel_cost
    }

    /// Consumes fuel for travel
    ///
    /// # Arguments
    /// * `amount` - Amount of fuel to consume
    ///
    /// # Returns
    /// * `Ok(())` if successful
    /// * `Err(&str)` if not enough fuel
    pub fn consume_fuel(&mut self, amount: u32) -> Result<(), &'static str> {
        if !self.has_enough_fuel(amount) {
            return Err("Insufficient fuel for travel");
        }
        self.current_fuel -= amount;
        Ok(())
    }

    /// Refuels the ship
    ///
    /// # Arguments
    /// * `amount` - Amount of fuel to add (will be capped at fuel_capacity)
    pub fn refuel(&mut self, amount: u32) {
        self.current_fuel = (self.current_fuel + amount).min(self.fuel_capacity);
    }

    /// Refuels the ship to full capacity
    pub fn refuel_full(&mut self) {
        self.current_fuel = self.fuel_capacity;
    }

    /// Updates the ship's location
    pub fn set_location(&mut self, planet_id: PlanetId) {
        self.current_location = planet_id;
    }

    /// Calculates the fuel cost for traveling a given distance
    /// Simple model: 1 fuel per unit of distance
    pub fn calculate_fuel_cost(distance: u32) -> u32 {
        distance.max(1) // Minimum 1 fuel for any travel
    }

    /// Calculates travel time using the Brachistochrone model
    ///
    /// Formula: travel_turns = 2 * sqrt(distance / acceleration)
    ///
    /// The ship accelerates for half the journey and decelerates for the second half.
    pub fn calculate_travel_turns(&self, distance: u32) -> u32 {
        if distance == 0 {
            return 1; // Minimum 1 turn even for zero distance
        }

        // travel_time = 2 * sqrt(distance / acceleration)
        let travel_turns = 2.0 * (distance as f64 / self.acceleration as f64).sqrt();

        // Round up and ensure at least 1 turn
        travel_turns.ceil() as u32
    }
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            current_location: "earth".to_string(),
            acceleration: 1,
            fuel_capacity: 100,
            current_fuel: 100,
        }
    }
}

/// Represents the current travel state of a ship
///
/// Ships can either be idle at a planet or in transit to a destination.
/// This enum tracks which state the ship is in and the relevant details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TravelState {
    /// Ship is stationary at a planet
    Idle {
        /// The ID of the planet where the ship is currently docked
        at_planet: PlanetId,
    },

    /// Ship is currently traveling between planets
    InTransit {
        /// The ID of the destination planet
        destination: PlanetId,
        /// The turn number when the ship will arrive
        arrival_turn: u32,
    },
}

impl TravelState {
    /// Creates a new idle state at the given planet
    pub fn idle(at_planet: PlanetId) -> Self {
        TravelState::Idle { at_planet }
    }

    /// Creates a new in-transit state
    pub fn in_transit(destination: PlanetId, arrival_turn: u32) -> Self {
        TravelState::InTransit {
            destination,
            arrival_turn,
        }
    }

    /// Checks if the ship is currently idle
    pub fn is_idle(&self) -> bool {
        matches!(self, TravelState::Idle { .. })
    }

    /// Checks if the ship is currently in transit
    pub fn is_in_transit(&self) -> bool {
        matches!(self, TravelState::InTransit { .. })
    }

    /// Gets the current planet ID if idle, None if in transit
    pub fn current_planet(&self) -> Option<&PlanetId> {
        match self {
            TravelState::Idle { at_planet } => Some(at_planet),
            TravelState::InTransit { .. } => None,
        }
    }

    /// Gets the destination planet ID if in transit, None if idle
    pub fn destination(&self) -> Option<&PlanetId> {
        match self {
            TravelState::Idle { .. } => None,
            TravelState::InTransit { destination, .. } => Some(destination),
        }
    }

    /// Checks if the ship has arrived at its destination
    /// Returns true if in transit and current_turn >= arrival_turn
    pub fn has_arrived(&self, current_turn: u32) -> bool {
        match self {
            TravelState::Idle { .. } => false,
            TravelState::InTransit { arrival_turn, .. } => current_turn >= *arrival_turn,
        }
    }

    /// Returns the number of turns remaining until arrival
    /// Returns 0 if idle or if already arrived
    pub fn turns_remaining(&self, current_turn: u32) -> u32 {
        match self {
            TravelState::Idle { .. } => 0,
            TravelState::InTransit { arrival_turn, .. } => {
                arrival_turn.saturating_sub(current_turn)
            }
        }
    }
}

impl Default for TravelState {
    fn default() -> Self {
        TravelState::Idle {
            at_planet: "earth".to_string(),
        }
    }
}

/// Calculates travel time between two planets using the Brachistochrone model
///
/// Formula: travel_turns = 2 * sqrt(distance / acceleration)
///
/// Where:
/// - distance = |destination.orbital_radius - departure.orbital_radius|
/// - acceleration = ship acceleration (default 1 unit/turn²)
///
/// The ship accelerates for half the journey and decelerates for the second half.
///
/// # Returns
/// The number of turns required for travel, minimum of 1
pub fn calculate_travel_turns(departure: &Planet, destination: &Planet, acceleration: u32) -> u32 {
    let distance = departure.distance_to(destination);

    if distance == 0 {
        return 1; // Minimum 1 turn for same-planet "travel"
    }

    let accel = acceleration.max(1);
    let travel_turns = 2.0 * (distance as f64 / accel as f64).sqrt();

    // Use ceiling and ensure at least 1 turn
    std::cmp::max(travel_turns.ceil() as u32, 1)
}

/// Calculates the fuel cost for traveling between two planets
/// Simple model: 1 fuel per unit of distance
pub fn calculate_fuel_cost(departure: &Planet, destination: &Planet) -> u32 {
    let distance = departure.distance_to(destination);
    Ship::calculate_fuel_cost(distance)
}

/// Advances all planet positions by one turn
///
/// Each planet's position is incremented by 1 and wraps around at its orbital period.
/// This is an O(n) operation where n is the number of planets.
///
/// # Arguments
/// * `planets` - Mutable slice of planets to advance
///
/// # Example
/// ```
/// use cowboyz::movement::{Planet, advance_planet_positions};
///
/// let mut planets = vec![
///     Planet::new("Earth".to_string(), 10, 20, 5),
///     Planet::new("Mars".to_string(), 15, 30, 29),
/// ];
///
/// advance_planet_positions(&mut planets);
///
/// assert_eq!(planets[0].position, 6);  // 5 + 1
/// assert_eq!(planets[1].position, 0);  // 29 + 1 wraps to 0
/// ```
pub fn advance_planet_positions(planets: &mut [Planet]) {
    for planet in planets.iter_mut() {
        planet.advance_position();
    }
}

/// Calculates fuel consumption for a journey considering ship efficiency
///
/// # Arguments
/// * `travel_turns` - Number of turns the journey takes
/// * `base_fuel_cost` - Base fuel cost for the distance
/// * `fuel_efficiency` - Ship's fuel efficiency multiplier (higher = more efficient, less fuel used).
///   Default is 1.0, values > 1.0 reduce fuel cost, values < 1.0 increase it
///
/// # Returns
/// The calculated fuel cost, minimum of 1
///
/// # Example
/// ```
/// use cowboyz::movement::calculate_fuel_consumption;
///
/// // 5 turns, base cost 10, default efficiency
/// let fuel = calculate_fuel_consumption(5, 10, 1.0);
/// assert_eq!(fuel, 10);
///
/// // With 2.0 efficiency (double efficient), fuel cost is halved
/// let fuel_efficient = calculate_fuel_consumption(5, 10, 2.0);
/// assert_eq!(fuel_efficient, 5);
/// ```
pub fn calculate_fuel_consumption(travel_turns: u32, base_fuel_cost: u32, fuel_efficiency: f64) -> u32 {
    if travel_turns == 0 {
        return 1; // Minimum 1 fuel for any travel attempt
    }

    let efficiency = fuel_efficiency.max(0.1); // Prevent division by zero or negative efficiency
    let adjusted_cost = (base_fuel_cost as f64 * travel_turns as f64) / efficiency;

    // Ensure at least 1 fuel is consumed
    adjusted_cost.max(1.0) as u32
}

/// Calculates the total fuel cost for a journey between planets considering ship efficiency
///
/// This is a convenience function that combines fuel cost calculation with efficiency
///
/// # Arguments
/// * `departure` - The departure planet
/// * `destination` - The destination planet
/// * `ship` - The ship making the journey (uses ship's acceleration for travel time calc)
/// * `fuel_efficiency` - Ship's fuel efficiency multiplier
///
/// # Returns
/// The total fuel cost for the journey
pub fn calculate_journey_fuel_cost(
    departure: &Planet,
    destination: &Planet,
    ship: &Ship,
    fuel_efficiency: f64,
) -> u32 {
    let travel_turns = calculate_travel_turns(departure, destination, ship.acceleration);
    let base_fuel_cost = calculate_fuel_cost(departure, destination);

    calculate_fuel_consumption(travel_turns, base_fuel_cost, fuel_efficiency)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Planet Tests
    // =========================================================================

    #[test]
    fn test_planet_new() {
        let planet = Planet::new("Mars".to_string(), 15, 30, 5);
        assert_eq!(planet.name, "Mars");
        assert_eq!(planet.orbital_radius, 15);
        assert_eq!(planet.orbital_period, 30);
        assert_eq!(planet.position, 5);
    }

    #[test]
    fn test_planet_at_start() {
        let planet = Planet::at_start("Earth".to_string(), 10, 20);
        assert_eq!(planet.position, 0);
    }

    #[test]
    fn test_planet_position_wraps() {
        let planet = Planet::new("Test".to_string(), 10, 10, 15);
        assert_eq!(planet.position, 5); // 15 % 10 = 5
    }

    #[test]
    fn test_planet_advance_position() {
        let mut planet = Planet::new("Test".to_string(), 10, 10, 5);
        planet.advance_position();
        assert_eq!(planet.position, 6);
    }

    #[test]
    fn test_planet_advance_position_wraps() {
        let mut planet = Planet::new("Test".to_string(), 10, 10, 9);
        planet.advance_position();
        assert_eq!(planet.position, 0);
    }

    #[test]
    fn test_planet_position_at_turn() {
        let planet = Planet::new("Test".to_string(), 10, 10, 5);
        assert_eq!(planet.position_at_turn(0), 5);
        assert_eq!(planet.position_at_turn(3), 8);
        assert_eq!(planet.position_at_turn(5), 0); // wraps
        assert_eq!(planet.position_at_turn(15), 0); // wraps twice
    }

    #[test]
    fn test_planet_distance_to() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 15, 20, 0);
        assert_eq!(planet1.distance_to(&planet2), 10);
        assert_eq!(planet2.distance_to(&planet1), 10);
    }

    #[test]
    fn test_planet_default() {
        let planet = Planet::default();
        assert_eq!(planet.name, "Unnamed Planet");
        assert_eq!(planet.orbital_radius, 10);
        assert_eq!(planet.orbital_period, 20);
        assert_eq!(planet.position, 0);
    }

    // =========================================================================
    // Ship Tests
    // =========================================================================

    #[test]
    fn test_ship_new() {
        let ship = Ship::new("earth".to_string(), 100);
        assert_eq!(ship.current_location, "earth");
        assert_eq!(ship.acceleration, 1);
        assert_eq!(ship.fuel_capacity, 100);
        assert_eq!(ship.current_fuel, 100);
    }

    #[test]
    fn test_ship_with_acceleration() {
        let ship = Ship::with_acceleration("mars".to_string(), 150, 4);
        assert_eq!(ship.current_location, "mars");
        assert_eq!(ship.acceleration, 4);
        assert_eq!(ship.fuel_capacity, 150);
        assert_eq!(ship.current_fuel, 150);
    }

    #[test]
    fn test_ship_acceleration_minimum() {
        let ship = Ship::with_acceleration("earth".to_string(), 100, 0);
        assert_eq!(ship.acceleration, 1); // Should be at least 1
    }

    #[test]
    fn test_ship_with_full_config() {
        let ship = Ship::with_full_config("jupiter".to_string(), 200, 150, 2);
        assert_eq!(ship.current_location, "jupiter");
        assert_eq!(ship.fuel_capacity, 200);
        assert_eq!(ship.current_fuel, 150);
        assert_eq!(ship.acceleration, 2);
    }

    #[test]
    fn test_ship_fuel_capped() {
        let ship = Ship::with_full_config("earth".to_string(), 100, 150, 1);
        assert_eq!(ship.current_fuel, 100); // Capped at capacity
    }

    #[test]
    fn test_ship_has_enough_fuel() {
        let ship = Ship::with_full_config("earth".to_string(), 100, 50, 1);
        assert!(ship.has_enough_fuel(50));
        assert!(ship.has_enough_fuel(30));
        assert!(!ship.has_enough_fuel(51));
        assert!(!ship.has_enough_fuel(100));
    }

    #[test]
    fn test_ship_consume_fuel_success() {
        let mut ship = Ship::with_full_config("earth".to_string(), 100, 100, 1);
        assert!(ship.consume_fuel(30).is_ok());
        assert_eq!(ship.current_fuel, 70);
    }

    #[test]
    fn test_ship_consume_fuel_failure() {
        let mut ship = Ship::with_full_config("earth".to_string(), 100, 20, 1);
        let result = ship.consume_fuel(30);
        assert!(result.is_err());
        assert_eq!(ship.current_fuel, 20); // Unchanged
    }

    #[test]
    fn test_ship_refuel() {
        let mut ship = Ship::with_full_config("earth".to_string(), 100, 50, 1);
        ship.refuel(30);
        assert_eq!(ship.current_fuel, 80);
    }

    #[test]
    fn test_ship_refuel_capped() {
        let mut ship = Ship::with_full_config("earth".to_string(), 100, 80, 1);
        ship.refuel(50);
        assert_eq!(ship.current_fuel, 100); // Capped at capacity
    }

    #[test]
    fn test_ship_refuel_full() {
        let mut ship = Ship::with_full_config("earth".to_string(), 100, 30, 1);
        ship.refuel_full();
        assert_eq!(ship.current_fuel, 100);
    }

    #[test]
    fn test_ship_set_location() {
        let mut ship = Ship::new("earth".to_string(), 100);
        ship.set_location("mars".to_string());
        assert_eq!(ship.current_location, "mars");
    }

    #[test]
    fn test_ship_calculate_fuel_cost() {
        assert_eq!(Ship::calculate_fuel_cost(0), 1); // Minimum 1
        assert_eq!(Ship::calculate_fuel_cost(5), 5);
        assert_eq!(Ship::calculate_fuel_cost(100), 100);
    }

    #[test]
    fn test_ship_calculate_travel_turns() {
        let ship = Ship::new("earth".to_string(), 100);
        // distance = 7, acceleration = 1
        // travel_turns = 2 * sqrt(7/1) = 2 * 2.645... = 5.29... → 6
        assert_eq!(ship.calculate_travel_turns(7), 6);
    }

    #[test]
    fn test_ship_calculate_travel_turns_zero_distance() {
        let ship = Ship::new("earth".to_string(), 100);
        assert_eq!(ship.calculate_travel_turns(0), 1); // Minimum 1 turn
    }

    #[test]
    fn test_ship_calculate_travel_turns_with_acceleration() {
        let ship = Ship::with_acceleration("earth".to_string(), 100, 4);
        // distance = 7, acceleration = 4
        // travel_turns = 2 * sqrt(7/4) = 2 * 1.322... = 2.64... → 3
        assert_eq!(ship.calculate_travel_turns(7), 3);
    }

    #[test]
    fn test_ship_default() {
        let ship = Ship::default();
        assert_eq!(ship.current_location, "earth");
        assert_eq!(ship.acceleration, 1);
        assert_eq!(ship.fuel_capacity, 100);
        assert_eq!(ship.current_fuel, 100);
    }

    // =========================================================================
    // TravelState Tests
    // =========================================================================

    #[test]
    fn test_travel_state_idle() {
        let state = TravelState::idle("earth".to_string());
        assert!(state.is_idle());
        assert!(!state.is_in_transit());
        assert_eq!(state.current_planet(), Some(&"earth".to_string()));
        assert_eq!(state.destination(), None);
    }

    #[test]
    fn test_travel_state_in_transit() {
        let state = TravelState::in_transit("mars".to_string(), 10);
        assert!(!state.is_idle());
        assert!(state.is_in_transit());
        assert_eq!(state.current_planet(), None);
        assert_eq!(state.destination(), Some(&"mars".to_string()));
    }

    #[test]
    fn test_travel_state_has_arrived() {
        let state = TravelState::in_transit("mars".to_string(), 10);
        assert!(!state.has_arrived(5));
        assert!(state.has_arrived(10));
        assert!(state.has_arrived(15));
    }

    #[test]
    fn test_travel_state_idle_never_arrives() {
        let state = TravelState::idle("earth".to_string());
        assert!(!state.has_arrived(0));
        assert!(!state.has_arrived(100));
    }

    #[test]
    fn test_travel_state_turns_remaining() {
        let state = TravelState::in_transit("mars".to_string(), 10);
        assert_eq!(state.turns_remaining(5), 5);
        assert_eq!(state.turns_remaining(10), 0);
        assert_eq!(state.turns_remaining(15), 0);
    }

    #[test]
    fn test_travel_state_turns_remaining_idle() {
        let state = TravelState::idle("earth".to_string());
        assert_eq!(state.turns_remaining(5), 0);
    }

    #[test]
    fn test_travel_state_default() {
        let state = TravelState::default();
        assert!(state.is_idle());
        assert_eq!(state.current_planet(), Some(&"earth".to_string()));
    }

    // =========================================================================
    // Utility Function Tests
    // =========================================================================

    #[test]
    fn test_calculate_travel_turns() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 12, 15, 0);
        // distance = 7, acceleration = 1
        // travel_turns = 2 * sqrt(7/1) = 2 * 2.645... = 5.29... → 6
        assert_eq!(calculate_travel_turns(&planet1, &planet2, 1), 6);
    }

    #[test]
    fn test_calculate_travel_turns_same_planet() {
        let planet = Planet::new("Earth".to_string(), 10, 20, 0);
        assert_eq!(calculate_travel_turns(&planet, &planet, 1), 1); // Minimum 1 turn
    }

    #[test]
    fn test_calculate_travel_turns_with_acceleration() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 12, 15, 0);
        // distance = 7, acceleration = 4
        // travel_turns = 2 * sqrt(7/4) = 2 * 1.322... = 2.64... → 3
        assert_eq!(calculate_travel_turns(&planet1, &planet2, 4), 3);
    }

    #[test]
    fn test_calculate_fuel_cost() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 15, 20, 0);
        assert_eq!(calculate_fuel_cost(&planet1, &planet2), 10);
    }

    #[test]
    fn test_calculate_fuel_cost_same_planet() {
        let planet = Planet::new("Earth".to_string(), 10, 20, 0);
        assert_eq!(calculate_fuel_cost(&planet, &planet), 1); // Minimum 1
    }

    // =========================================================================
    // Orbital Mechanics Algorithm Tests
    // =========================================================================

    #[test]
    fn test_advance_planet_positions_basic() {
        let mut planets = vec![
            Planet::new("Earth".to_string(), 10, 20, 5),
            Planet::new("Mars".to_string(), 15, 30, 10),
        ];

        advance_planet_positions(&mut planets);

        assert_eq!(planets[0].position, 6); // 5 + 1
        assert_eq!(planets[1].position, 11); // 10 + 1
    }

    #[test]
    fn test_advance_planet_positions_wraps() {
        let mut planets = vec![
            Planet::new("Earth".to_string(), 10, 20, 19), // At end of period
            Planet::new("Mars".to_string(), 15, 30, 29),  // At end of period
        ];

        advance_planet_positions(&mut planets);

        assert_eq!(planets[0].position, 0); // 19 + 1 wraps to 0
        assert_eq!(planets[1].position, 0); // 29 + 1 wraps to 0
    }

    #[test]
    fn test_advance_planet_positions_empty() {
        let mut planets: Vec<Planet> = vec![];
        advance_planet_positions(&mut planets); // Should not panic
        assert!(planets.is_empty());
    }

    #[test]
    fn test_advance_planet_positions_single() {
        let mut planets = vec![Planet::new("Earth".to_string(), 10, 20, 5)];

        advance_planet_positions(&mut planets);

        assert_eq!(planets[0].position, 6);
    }

    #[test]
    fn test_advance_planet_positions_multiple_advances() {
        let mut planets = vec![
            Planet::new("Earth".to_string(), 10, 10, 0), // 10-turn period
        ];

        // Advance 10 times to complete one full orbit
        for i in 0..10 {
            assert_eq!(planets[0].position, i);
            advance_planet_positions(&mut planets);
        }

        // Should be back at 0
        assert_eq!(planets[0].position, 0);
    }

    #[test]
    fn test_advance_planet_positions_zero_period() {
        // Edge case: planet with zero orbital period should not change
        let mut planets = vec![Planet {
            name: "Station".to_string(),
            orbital_radius: 5,
            orbital_period: 0,
            position: 5,
        }];

        advance_planet_positions(&mut planets);

        // Position should remain unchanged due to zero period
        assert_eq!(planets[0].position, 5);
    }

    #[test]
    fn test_calculate_travel_turns_zero_acceleration() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 12, 15, 0);

        // Zero acceleration should be treated as 1 (minimum)
        let turns = calculate_travel_turns(&planet1, &planet2, 0);
        let expected = calculate_travel_turns(&planet1, &planet2, 1);
        assert_eq!(turns, expected);
    }

    #[test]
    fn test_calculate_travel_turns_ensures_minimum_one() {
        // Very small distance with high acceleration could result in < 1 turn
        let planet1 = Planet::new("A".to_string(), 10, 20, 0);
        let planet2 = Planet::new("B".to_string(), 11, 20, 0); // Distance = 1

        // With very high acceleration, travel time would be tiny
        let turns = calculate_travel_turns(&planet1, &planet2, 100);
        assert!(turns >= 1); // Should always be at least 1
    }

    // =========================================================================
    // Fuel Consumption Algorithm Tests
    // =========================================================================

    #[test]
    fn test_calculate_fuel_consumption_basic() {
        // 5 turns, base cost 10, default efficiency (1.0)
        // Result: 10 * 5 / 1.0 = 50
        let fuel = calculate_fuel_consumption(5, 10, 1.0);
        assert_eq!(fuel, 50);
    }

    #[test]
    fn test_calculate_fuel_consumption_with_efficiency() {
        // With 2.0 efficiency (double efficient), fuel cost is halved
        let fuel = calculate_fuel_consumption(5, 10, 2.0);
        assert_eq!(fuel, 25); // (10 * 5) / 2.0 = 25
    }

    #[test]
    fn test_calculate_fuel_consumption_low_efficiency() {
        // With 0.5 efficiency (half efficient), fuel cost is doubled
        let fuel = calculate_fuel_consumption(5, 10, 0.5);
        assert_eq!(fuel, 100); // (10 * 5) / 0.5 = 100
    }

    #[test]
    fn test_calculate_fuel_consumption_minimum_one() {
        // Even with zero travel turns, should return minimum 1
        let fuel = calculate_fuel_consumption(0, 10, 1.0);
        assert_eq!(fuel, 1);
    }

    #[test]
    fn test_calculate_fuel_consumption_very_efficient() {
        // Very high efficiency should still return at least 1
        let fuel = calculate_fuel_consumption(1, 1, 100.0);
        assert_eq!(fuel, 1); // (1 * 1) / 100 = 0.01 → rounded to 1
    }

    #[test]
    fn test_calculate_fuel_consumption_zero_efficiency_protection() {
        // Zero efficiency should be clamped to 0.1 to prevent division issues
        let fuel = calculate_fuel_consumption(5, 10, 0.0);
        // Treated as efficiency 0.1: (10 * 5) / 0.1 = 500
        assert_eq!(fuel, 500);
    }

    #[test]
    fn test_calculate_fuel_consumption_negative_efficiency_protection() {
        // Negative efficiency should be clamped to 0.1
        let fuel = calculate_fuel_consumption(5, 10, -1.0);
        // Treated as efficiency 0.1: (10 * 5) / 0.1 = 500
        assert_eq!(fuel, 500);
    }

    #[test]
    fn test_calculate_journey_fuel_cost() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 12, 15, 0);
        let ship = Ship::new("earth".to_string(), 100);

        // distance = 7, acceleration = 1
        // travel_turns = 2 * sqrt(7/1) = 2 * 2.645... = 5.29... → 6
        // base_fuel_cost = 7
        // fuel_consumption = 7 * 6 / 1.0 = 42
        let fuel = calculate_journey_fuel_cost(&planet1, &planet2, &ship, 1.0);
        assert_eq!(fuel, 42);
    }

    #[test]
    fn test_calculate_journey_fuel_cost_with_efficiency() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 12, 15, 0);
        let ship = Ship::new("earth".to_string(), 100);

        // With 2.0 efficiency, fuel cost should be halved
        let fuel = calculate_journey_fuel_cost(&planet1, &planet2, &ship, 2.0);
        assert_eq!(fuel, 21); // 42 / 2 = 21
    }

    #[test]
    fn test_calculate_journey_fuel_cost_same_planet() {
        let planet = Planet::new("Earth".to_string(), 10, 20, 0);
        let ship = Ship::new("earth".to_string(), 100);

        // Same planet: travel_turns = 1, base_fuel_cost = 1
        // fuel_consumption = 1 * 1 / 1.0 = 1
        let fuel = calculate_journey_fuel_cost(&planet, &planet, &ship, 1.0);
        assert_eq!(fuel, 1);
    }

    #[test]
    fn test_calculate_journey_fuel_cost_with_ship_acceleration() {
        let planet1 = Planet::new("Inner".to_string(), 5, 10, 0);
        let planet2 = Planet::new("Outer".to_string(), 12, 15, 0);
        let ship = Ship::with_acceleration("earth".to_string(), 100, 4);

        // distance = 7, acceleration = 4
        // travel_turns = 2 * sqrt(7/4) = 2 * 1.322... = 2.64... → 3
        // base_fuel_cost = 7
        // fuel_consumption = 7 * 3 / 1.0 = 21
        let fuel = calculate_journey_fuel_cost(&planet1, &planet2, &ship, 1.0);
        assert_eq!(fuel, 21);
    }
}
