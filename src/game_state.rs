//! Game state management for turn-based system
//! Based on ADR 0002: Movement Mechanics System
//! Data models based on ADR 0006: Data Models/Schema for Space-Western Trading Game

use crate::simulation::commodity::CommodityType;
use crate::simulation::economy::PlanetEconomy;
use crate::simulation::orbits::Position;
use crate::simulation::planet_types::PlanetType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Movement System Types (ADR 0002: Movement Mechanics System)
// ============================================================================

/// Represents the current travel state of a ship
/// Ships can either be idle at a planet or in transit to a destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TravelState {
    /// Ship is stationary at a planet
    Idle {
        /// The ID of the planet where the ship is currently docked
        at_planet: String,
    },
    /// Ship is currently traveling between planets
    InTransit {
        /// The ID of the destination planet
        destination: String,
        /// The turn number when the ship will arrive
        arrival_turn: u32,
        /// The turn when the journey started
        departure_turn: u32,
    },
}

#[allow(dead_code)]
impl TravelState {
    /// Creates a new idle state at the given planet
    pub fn idle(at_planet: String) -> Self {
        TravelState::Idle { at_planet }
    }

    /// Creates a new in-transit state
    pub fn in_transit(destination: String, arrival_turn: u32, departure_turn: u32) -> Self {
        TravelState::InTransit {
            destination,
            arrival_turn,
            departure_turn,
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
    pub fn current_planet(&self) -> Option<&String> {
        match self {
            TravelState::Idle { at_planet } => Some(at_planet),
            TravelState::InTransit { .. } => None,
        }
    }

    /// Gets the destination planet ID if in transit, None if idle
    pub fn destination(&self) -> Option<&String> {
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

/// Event emitted when a ship arrives at a destination
/// Used for UI notifications
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ArrivalEvent {
    pub destination_planet_id: String,
    pub arrival_turn: u32,
    pub departure_turn: u32,
    pub travel_turns: u32,
}

#[allow(dead_code)]
impl ArrivalEvent {
    /// Creates a new arrival event
    pub fn new(
        destination_planet_id: String,
        arrival_turn: u32,
        departure_turn: u32,
    ) -> Self {
        let travel_turns = arrival_turn.saturating_sub(departure_turn);
        ArrivalEvent {
            destination_planet_id,
            arrival_turn,
            departure_turn,
            travel_turns,
        }
    }
}

/// Errors that can occur during travel initiation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TravelError {
    /// Ship is already in transit
    AlreadyInTransit,
    /// Destination is the same as current location
    SameDestination,
    /// Destination planet does not exist
    InvalidDestination,
    /// Not enough fuel for the journey
    InsufficientFuel,
    /// Ship is destroyed and cannot travel
    ShipDestroyed,
    /// Game is over
    GameOver,
}

impl std::fmt::Display for TravelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TravelError::AlreadyInTransit => write!(f, "Ship is already in transit"),
            TravelError::SameDestination => {
                write!(f, "Destination is the same as current location")
            }
            TravelError::InvalidDestination => write!(f, "Destination planet does not exist"),
            TravelError::InsufficientFuel => write!(f, "Not enough fuel for the journey"),
            TravelError::ShipDestroyed => write!(f, "Ship is destroyed and cannot travel"),
            TravelError::GameOver => write!(f, "Cannot travel: game is over"),
        }
    }
}

impl std::error::Error for TravelError {}

/// Game clock that tracks turns in the game
/// The clock advances during travel and wait actions, synchronizing all game systems
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
        if new_turn > self.total_turns {
            // Cap at total_turns
            let remaining = self.total_turns.saturating_sub(self.current_turn);
            self.current_turn = self.total_turns;
            remaining
        } else {
            self.current_turn = new_turn;
            turns
        }
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

// ============================================================================
// ADR 0006: Data Models/Schema for Space-Western Trading Game
// ============================================================================

/// Represents a planet in the solar system with its orbital position and economy
/// This is the main data structure for planets used in the game state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Planet {
    pub id: String,
    pub name: String,
    pub orbit_radius: u32,  // Integer distance from star (in arbitrary units)
    pub orbit_period: u32,  // Turns to complete one orbit
    pub position: Position, // Current orbital position
    pub economy: PlanetEconomy,
    pub planet_type: PlanetType,
}

#[allow(dead_code)]
impl Planet {
    /// Create a new planet with the given parameters
    pub fn new(
        id: String,
        name: String,
        orbit_radius: u32,
        orbit_period: u32,
        planet_type: PlanetType,
    ) -> Self {
        let economy = PlanetEconomy::new(planet_type.clone());
        Planet {
            id,
            name,
            orbit_radius,
            orbit_period,
            position: Position::start(),
            economy,
            planet_type,
        }
    }

    /// Calculate the planet's position at a given turn
    pub fn calculate_position_at_turn(&self, turn: u32) -> Position {
        crate::simulation::orbits::calculate_orbit_position(self.orbit_period, turn)
    }
}

/// The solar system containing all planets
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SolarSystem {
    pub planets: Vec<Planet>,
    pub name: String,
}

#[allow(dead_code)]
impl SolarSystem {
    /// Create a new solar system with the given planets
    pub fn new(name: String, planets: Vec<Planet>) -> Self {
        SolarSystem { planets, name }
    }

    /// Find a planet by its ID
    pub fn get_planet(&self, planet_id: &str) -> Option<&Planet> {
        self.planets.iter().find(|p| p.id == planet_id)
    }

    /// Find a planet by its ID (mutable)
    pub fn get_planet_mut(&mut self, planet_id: &str) -> Option<&mut Planet> {
        self.planets.iter_mut().find(|p| p.id == planet_id)
    }

    /// Get all planet IDs in the system
    pub fn get_all_planet_ids(&self) -> Vec<&str> {
        self.planets.iter().map(|p| p.id.as_str()).collect()
    }
}

/// Player's ship with cargo capacity and travel capabilities
/// Based on ADR 0002: Movement Mechanics System
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Ship {
    pub speed: f64,          // Legacy field (kept for compatibility)
    pub acceleration: u32,   // Ship acceleration in units/turn² (default: 1)
    pub cargo_capacity: u32, // Maximum cargo capacity
    pub fuel: u32,           // Current fuel level
    pub max_fuel: u32,       // Maximum fuel capacity
    pub hull: u32,           // Current hull integrity
    pub max_hull: u32,       // Maximum hull integrity
}

#[allow(dead_code)]
impl Ship {
    /// Create a new ship with default acceleration of 1 unit/turn²
    pub fn new(speed: f64, cargo_capacity: u32) -> Self {
        Ship {
            speed,
            acceleration: 1, // Default acceleration
            cargo_capacity,
            fuel: 100,     // Default fuel
            max_fuel: 100, // Default max fuel
            hull: 100,     // Default hull
            max_hull: 100, // Default max hull
        }
    }

    /// Create a ship with custom acceleration
    pub fn with_acceleration(speed: f64, acceleration: u32, cargo_capacity: u32) -> Self {
        Ship {
            speed,
            acceleration: acceleration.max(1), // Ensure at least 1
            cargo_capacity,
            fuel: 100,
            max_fuel: 100,
            hull: 100,
            max_hull: 100,
        }
    }

    /// Create a ship with full customization
    pub fn with_full_config(
        speed: f64,
        acceleration: u32,
        cargo_capacity: u32,
        max_fuel: u32,
        max_hull: u32,
    ) -> Self {
        Ship {
            speed,
            acceleration: acceleration.max(1),
            cargo_capacity,
            fuel: max_fuel,
            max_fuel,
            hull: max_hull,
            max_hull,
        }
    }

    /// Check if the ship can travel the given distance
    pub fn can_travel(&self, distance: u32) -> bool {
        // Simple fuel calculation: 1 fuel per unit of distance
        self.fuel >= distance
    }

    /// Travel a given distance, consuming fuel
    pub fn travel(&mut self, distance: u32) -> Result<(), &'static str> {
        if !self.can_travel(distance) {
            return Err("Not enough fuel for travel");
        }
        self.fuel = self.fuel.saturating_sub(distance);
        Ok(())
    }

    /// Refuel the ship
    pub fn refuel(&mut self, amount: u32) {
        self.fuel = std::cmp::min(self.fuel + amount, self.max_fuel);
    }

    /// Repair the hull
    pub fn repair(&mut self, amount: u32) {
        self.hull = std::cmp::min(self.hull + amount, self.max_hull);
    }

    /// Take damage to the hull
    pub fn take_damage(&mut self, amount: u32) {
        self.hull = self.hull.saturating_sub(amount);
    }

    /// Check if the ship is destroyed
    pub fn is_destroyed(&self) -> bool {
        self.hull == 0
    }
}

/// Player's inventory/cargo hold
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CargoHold {
    pub capacity: u32,
    pub commodities: HashMap<CommodityType, u32>,
}

#[allow(dead_code)]
impl CargoHold {
    /// Create a new cargo hold with the given capacity
    pub fn new(capacity: u32) -> Self {
        CargoHold {
            capacity,
            commodities: HashMap::new(),
        }
    }

    /// Add a commodity to the cargo hold
    pub fn add_commodity(
        &mut self,
        commodity_type: CommodityType,
        quantity: u32,
    ) -> Result<(), &'static str> {
        let current_total = self.total_cargo_space_used();
        if current_total + quantity > self.capacity {
            return Err("Not enough cargo space");
        }
        *self.commodities.entry(commodity_type).or_insert(0) += quantity;
        Ok(())
    }

    /// Remove a commodity from the cargo hold
    pub fn remove_commodity(
        &mut self,
        commodity_type: &CommodityType,
        quantity: u32,
    ) -> Result<u32, &'static str> {
        match self.commodities.get_mut(commodity_type) {
            Some(current_qty) => {
                if *current_qty < quantity {
                    return Err("Not enough of this commodity in inventory");
                }
                if *current_qty == quantity {
                    self.commodities.remove(commodity_type);
                } else {
                    *current_qty -= quantity;
                }
                Ok(quantity)
            }
            None => Err("Commodity not found in inventory"),
        }
    }

    /// Get the quantity of a specific commodity
    pub fn get_quantity(&self, commodity_type: &CommodityType) -> u32 {
        *self.commodities.get(commodity_type).unwrap_or(&0)
    }

    /// Check if the cargo hold contains a specific commodity
    pub fn contains(&self, commodity_type: &CommodityType) -> bool {
        self.commodities.contains_key(commodity_type)
    }

    /// Calculate total cargo space currently used
    pub fn total_cargo_space_used(&self) -> u32 {
        self.commodities.values().sum()
    }

    /// Calculate remaining cargo space
    pub fn remaining_cargo_space(&self) -> u32 {
        self.capacity.saturating_sub(self.total_cargo_space_used())
    }

    /// List all commodity types currently in cargo
    pub fn list_commodity_types(&self) -> Vec<CommodityType> {
        self.commodities.keys().cloned().collect()
    }

    /// Clear all commodities from cargo hold
    pub fn clear(&mut self) {
        self.commodities.clear();
    }

    /// Check if cargo hold is empty
    pub fn is_empty(&self) -> bool {
        self.commodities.is_empty()
    }
}

/// Player entity representing the user's game state
/// Based on ADR 0006: Player Entity
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Player {
    pub money: u32,
    pub location: String, // Current planet ID
    pub ship: Ship,
    pub cargo: CargoHold,
    pub visited_planets: Vec<String>, // Track which planets have been visited
    pub total_trades: u32,            // Total number of trades made
    pub total_earnings: u32,          // Total money earned from trades
    pub travel_state: TravelState,    // Current travel state of the ship
}

#[allow(dead_code)]
impl Player {
    /// Create a new player with default values
    pub fn new() -> Self {
        Player {
            money: 1000,                   // Starting money
            location: "earth".to_string(), // Starting planet
            ship: Ship::new(10.0, 10),     // Default ship speed and cargo capacity
            cargo: CargoHold::new(10),     // Default cargo hold capacity
            visited_planets: vec!["earth".to_string()],
            total_trades: 0,
            total_earnings: 0,
            travel_state: TravelState::idle("earth".to_string()),
        }
    }

    /// Create a player with custom starting values
    pub fn with_values(money: u32, location: String, ship: Ship, cargo_capacity: u32) -> Self {
        let location_clone = location.clone();
        Player {
            money,
            location: location_clone.clone(),
            ship,
            cargo: CargoHold::new(cargo_capacity),
            visited_planets: vec![location],
            total_trades: 0,
            total_earnings: 0,
            travel_state: TravelState::idle(location_clone),
        }
    }

    /// Record a visit to a planet
    pub fn visit_planet(&mut self, planet_id: &str) {
        if !self.visited_planets.contains(&planet_id.to_string()) {
            self.visited_planets.push(planet_id.to_string());
        }
    }

    /// Record a trade
    pub fn record_trade(&mut self, earnings: u32) {
        self.total_trades += 1;
        self.total_earnings += earnings;
    }

    /// Check if player can afford a purchase
    pub fn can_afford(&self, amount: u32) -> bool {
        self.money >= amount
    }

    /// Add money to player's account
    pub fn add_money(&mut self, amount: u32) {
        self.money += amount;
    }

    /// Spend money (returns false if insufficient funds)
    pub fn spend_money(&mut self, amount: u32) -> bool {
        if self.money >= amount {
            self.money -= amount;
            true
        } else {
            false
        }
    }
}

/// Game settings and configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GameSettings {
    pub audio_enabled: bool,
    pub music_volume: f32,
    pub sfx_volume: f32,
    pub show_price_history: bool,
    pub show_market_events: bool,
    pub difficulty: GameDifficulty,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            audio_enabled: true,
            music_volume: 0.5,
            sfx_volume: 0.5,
            show_price_history: true,
            show_market_events: true,
            difficulty: GameDifficulty::Normal,
        }
    }
}

/// Game difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum GameDifficulty {
    Easy,
    Normal,
    Hard,
    Custom {
        price_volatility: f64,
        starting_money: u32,
        turn_limit: u32,
    },
}

#[allow(dead_code)]
impl GameDifficulty {
    /// Get the price volatility multiplier for this difficulty
    pub fn price_volatility_multiplier(&self) -> f64 {
        match self {
            GameDifficulty::Easy => 0.5,
            GameDifficulty::Normal => 1.0,
            GameDifficulty::Hard => 1.5,
            GameDifficulty::Custom {
                price_volatility, ..
            } => *price_volatility,
        }
    }

    /// Get the starting money for this difficulty
    pub fn starting_money(&self) -> u32 {
        match self {
            GameDifficulty::Easy => 2000,
            GameDifficulty::Normal => 1000,
            GameDifficulty::Hard => 500,
            GameDifficulty::Custom { starting_money, .. } => *starting_money,
        }
    }

    /// Get the turn limit for this difficulty
    pub fn turn_limit(&self) -> u32 {
        match self {
            GameDifficulty::Easy => 20,
            GameDifficulty::Normal => 10,
            GameDifficulty::Hard => 5,
            GameDifficulty::Custom { turn_limit, .. } => *turn_limit,
        }
    }
}

/// Transaction record for tracking trades
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Transaction {
    pub turn: u32,
    pub planet_id: String,
    pub commodity: CommodityType,
    pub quantity: u32,
    pub price_per_unit: u32,
    pub transaction_type: TransactionType,
}

/// Type of transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TransactionType {
    Buy,
    Sell,
}

/// The complete game state
/// This is the main structure that gets serialized to localStorage
/// Based on ADR 0006: Core Game State Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GameState {
    pub version: String, // Game state version for migration support
    pub player: Player,
    pub solar_system: SolarSystem,
    pub game_clock: GameClock,
    pub settings: GameSettings,
    pub transaction_history: Vec<Transaction>,
    pub is_game_over: bool,
    pub game_over_reason: Option<String>,
}

#[allow(dead_code)]
impl GameState {
    /// Create a new game state with default values
    pub fn new() -> Self {
        GameState {
            version: "1.0.0".to_string(),
            player: Player::new(),
            solar_system: SolarSystem::new("Sol System".to_string(), Vec::new()),
            game_clock: GameClock::default(),
            settings: GameSettings::default(),
            transaction_history: Vec::new(),
            is_game_over: false,
            game_over_reason: None,
        }
    }

    /// Create a game state with custom settings
    pub fn with_settings(
        player: Player,
        solar_system: SolarSystem,
        settings: GameSettings,
    ) -> Self {
        let total_turns = settings.difficulty.turn_limit();
        GameState {
            version: "1.0.0".to_string(),
            player,
            solar_system,
            game_clock: GameClock::new(total_turns),
            settings,
            transaction_history: Vec::new(),
            is_game_over: false,
            game_over_reason: None,
        }
    }

    /// Get the current planet the player is on
    pub fn get_current_planet(&self) -> Option<&Planet> {
        self.solar_system.get_planet(&self.player.location)
    }

    /// Get the current planet the player is on (mutable)
    pub fn get_current_planet_mut(&mut self) -> Option<&mut Planet> {
        self.solar_system.get_planet_mut(&self.player.location)
    }

    /// Record a transaction
    pub fn record_transaction(&mut self, transaction: Transaction) {
        self.transaction_history.push(transaction);
    }

    /// End the game
    pub fn end_game(&mut self, reason: String) {
        self.is_game_over = true;
        self.game_over_reason = Some(reason);
        self.game_clock.current_turn = self.game_clock.total_turns;
    }

    /// Advance the game clock by a number of turns
    pub fn advance_turns(&mut self, turns: u32) -> u32 {
        let actual_advance = self.game_clock.advance(turns);

        // Update planet positions based on new turn
        for planet in &mut self.solar_system.planets {
            planet.position = planet.calculate_position_at_turn(self.game_clock.current_turn);
        }

        // Update market prices
        for planet in &mut self.solar_system.planets {
            planet.economy.update_market();
        }

        // Check for game over
        if self.game_clock.is_game_over() {
            self.end_game("Game time has run out!".to_string());
        }

        actual_advance
    }

    // ============================================================================
    // Movement System Methods (ADR 0002: Movement Mechanics System)
    // ============================================================================

    /// Advances the game by one turn, updating planet positions and checking for arrivals
    ///
    /// This method:
    /// 1. Advances all planet positions by one turn
    /// 2. Increments the current turn counter
    /// 3. Checks if the ship has arrived at its destination
    /// 4. Updates the travel state and player location if arrived
    /// 5. Updates market prices
    /// 6. Checks for game over conditions
    ///
    /// # Returns
    /// * `Some(ArrivalEvent)` if the ship arrived at a destination this turn
    /// * `None` if no arrival occurred
    pub fn next_turn(&mut self) -> Option<ArrivalEvent> {
        // Don't advance if game is over
        if self.is_game_over {
            return None;
        }

        // Advance the game clock by 1 turn
        self.game_clock.advance(1);

        // Advance all planet positions (using the same logic as advance_turns)
        for planet in &mut self.solar_system.planets {
            if planet.orbit_period > 0 {
                planet.position.orbital_position =
                    (planet.position.orbital_position + 1) % planet.orbit_period;
            }
        }

        // Update market prices
        for planet in &mut self.solar_system.planets {
            planet.economy.update_market();
        }

        // Check for ship arrival
        let arrival_event = self.check_and_process_arrival();

        // Check for game over
        if self.game_clock.is_game_over() {
            self.end_game("Game time has run out!".to_string());
        }

        arrival_event
    }

    /// Checks if the ship has arrived and updates state accordingly
    ///
    /// # Returns
    /// * `Some(ArrivalEvent)` if the ship arrived at its destination
    /// * `None` if no arrival occurred
    fn check_and_process_arrival(&mut self) -> Option<ArrivalEvent> {
        let current_turn = self.game_clock.current_turn;

        // Check if ship is in transit and has arrived
        if let TravelState::InTransit {
            destination,
            arrival_turn,
            departure_turn,
        } = &self.player.travel_state
        {
            if current_turn >= *arrival_turn {
                // Ship has arrived!
                let destination_clone = destination.clone();
                let departure = *departure_turn;

                // Update player location
                self.player.location = destination_clone.clone();

                // Update travel state to idle at the destination
                self.player.travel_state = TravelState::idle(destination_clone.clone());

                // Record the visit
                self.player.visit_planet(&destination_clone);

                // Create and return the arrival event
                return Some(ArrivalEvent::new(
                    destination_clone,
                    current_turn,
                    departure,
                ));
            }
        }

        None
    }

    /// Initiates travel to a destination planet
    ///
    /// This method:
    /// 1. Validates the destination is different from current location
    /// 2. Validates the destination planet exists
    /// 3. Calculates travel time using the Brachistochrone model
    /// 4. Checks fuel availability
    /// 5. Deducts fuel cost atomically
    /// 6. Sets the ship to InTransit state
    ///
    /// # Arguments
    /// * `destination_planet_id` - The ID of the destination planet
    ///
    /// # Returns
    /// * `Ok(())` if travel was initiated successfully
    /// * `Err(TravelError)` if travel could not be initiated
    pub fn initiate_travel(&mut self, destination_planet_id: &str) -> Result<(), TravelError> {
        // Check if game is over
        if self.is_game_over {
            return Err(TravelError::GameOver);
        }

        // Check if ship is already in transit
        if self.player.travel_state.is_in_transit() {
            return Err(TravelError::AlreadyInTransit);
        }

        // Get current planet ID
        let current_planet_id = match self.player.travel_state.current_planet() {
            Some(planet_id) => planet_id.clone(),
            None => self.player.location.clone(),
        };

        // Validate destination is different from current location
        if destination_planet_id == current_planet_id {
            return Err(TravelError::SameDestination);
        }

        // Validate destination planet exists
        let destination_planet = self
            .solar_system
            .get_planet(destination_planet_id)
            .ok_or(TravelError::InvalidDestination)?;

        // Get current planet for travel calculation
        let current_planet = self
            .solar_system
            .get_planet(&current_planet_id)
            .ok_or(TravelError::InvalidDestination)?;

        // Check if ship is destroyed
        if self.player.ship.is_destroyed() {
            return Err(TravelError::ShipDestroyed);
        }

        // Calculate distance based on orbital radii
        let distance = current_planet
            .orbit_radius
            .abs_diff(destination_planet.orbit_radius);

        // Calculate travel turns using the Brachistochrone model
        // Formula: travel_turns = 2 * sqrt(distance / acceleration)
        let travel_turns = if distance == 0 {
            1 // Minimum 1 turn for same-planet "travel"
        } else {
            let accel = self.player.ship.acceleration.max(1);
            let turns = 2.0 * (distance as f64 / accel as f64).sqrt();
            std::cmp::max(turns.ceil() as u32, 1)
        };

        // Check fuel availability (1 fuel per unit of distance, minimum 1)
        let _fuel_cost = distance.max(1);
        if !self.player.ship.can_travel(distance) {
            return Err(TravelError::InsufficientFuel);
        }

        // Deduct fuel atomically
        self.player
            .ship
            .travel(distance)
            .map_err(|_| TravelError::InsufficientFuel)?;

        // Calculate arrival turn
        let current_turn = self.game_clock.current_turn;
        let arrival_turn = current_turn + travel_turns;

        // Set travel state to in-transit
        self.player.travel_state = TravelState::in_transit(
            destination_planet_id.to_string(),
            arrival_turn,
            current_turn,
        );

        Ok(())
    }

    /// Gets the current travel state of the player
    pub fn get_travel_state(&self) -> &TravelState {
        &self.player.travel_state
    }

    /// Checks if the ship is currently in transit
    pub fn is_in_transit(&self) -> bool {
        self.player.travel_state.is_in_transit()
    }

    /// Gets the number of turns remaining until arrival
    /// Returns 0 if not in transit
    pub fn turns_until_arrival(&self) -> u32 {
        self.player
            .travel_state
            .turns_remaining(self.game_clock.current_turn)
    }

    /// Gets the current planet ID if the ship is idle, None if in transit
    pub fn get_current_location(&self) -> Option<&String> {
        self.player.travel_state.current_planet()
    }

    /// Gets the destination planet ID if in transit, None if idle
    pub fn get_destination(&self) -> Option<&String> {
        self.player.travel_state.destination()
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Data Validation Functions (ADR 0006: Data Persistence Implementation)
// ============================================================================

/// Validation result with optional error message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[allow(dead_code)]
impl ValidationResult {
    /// Create a valid result
    pub fn valid() -> Self {
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create an invalid result with errors
    pub fn invalid(errors: Vec<String>) -> Self {
        ValidationResult {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add an error to the result
    pub fn with_error(mut self, error: String) -> Self {
        self.is_valid = false;
        self.errors.push(error);
        self
    }
}

/// Validate the game state for data integrity
/// Based on ADR 0006: Implement validation functions to verify data integrity on load
#[allow(dead_code)]
pub fn validate_game_state(state: &GameState) -> ValidationResult {
    let mut result = ValidationResult::valid();

    // Validate version
    if state.version.is_empty() {
        result = result.with_error("Game state version is missing".to_string());
    }

    // Validate player
    if state.player.money > 1_000_000 {
        result = result.with_warning("Player has unusually high money".to_string());
    }

    // Validate player location
    if state
        .solar_system
        .get_planet(&state.player.location)
        .is_none()
    {
        result = result.with_error(format!(
            "Player location '{}' is not a valid planet",
            state.player.location
        ));
    }

    // Validate ship
    if state.player.ship.cargo_capacity == 0 {
        result = result.with_error("Ship cargo capacity cannot be zero".to_string());
    }

    if state.player.ship.hull > state.player.ship.max_hull {
        result = result.with_error("Ship hull integrity exceeds maximum".to_string());
    }

    if state.player.ship.fuel > state.player.ship.max_fuel {
        result = result.with_error("Ship fuel exceeds maximum".to_string());
    }

    // Validate cargo
    let cargo_used = state.player.cargo.total_cargo_space_used();
    if cargo_used > state.player.cargo.capacity {
        result = result.with_error("Cargo hold exceeds capacity".to_string());
    }

    // Validate travel state
    match &state.player.travel_state {
        TravelState::Idle { at_planet } => {
            // Validate that the idle planet exists
            if state.solar_system.get_planet(at_planet).is_none() {
                result = result.with_error(format!(
                    "Travel state idle planet '{}' is not a valid planet",
                    at_planet
                ));
            }
            // Validate that idle planet matches player location
            if at_planet != &state.player.location {
                result = result.with_warning(
                    "Travel state idle planet does not match player location".to_string(),
                );
            }
        }
        TravelState::InTransit {
            destination,
            arrival_turn,
            departure_turn,
        } => {
            // Validate that the destination planet exists
            if state.solar_system.get_planet(destination).is_none() {
                result = result.with_error(format!(
                    "Travel state destination '{}' is not a valid planet",
                    destination
                ));
            }
            // Validate that arrival turn is after departure turn
            if arrival_turn <= departure_turn {
                result = result.with_error(
                    "Travel state arrival turn must be after departure turn".to_string(),
                );
            }
            // Validate that arrival turn is not in the past
            if *arrival_turn < state.game_clock.current_turn {
                result = result.with_warning(
                    "Travel state arrival turn is in the past".to_string(),
                );
            }
        }
    }

    // Validate game clock
    if state.game_clock.total_turns == 0 {
        result = result.with_error("Game turn limit cannot be zero".to_string());
    }

    if state.game_clock.current_turn > state.game_clock.total_turns {
        result = result.with_error("Current turn exceeds total turns".to_string());
    }

    // Validate solar system
    if state.solar_system.planets.is_empty() {
        result = result.with_warning("Solar system has no planets".to_string());
    }

    // Check for duplicate planet IDs
    let planet_ids: Vec<&str> = state
        .solar_system
        .planets
        .iter()
        .map(|p| p.id.as_str())
        .collect();
    let mut seen = std::collections::HashSet::new();
    for id in &planet_ids {
        if !seen.insert(id) {
            result = result.with_error(format!("Duplicate planet ID: {}", id));
        }
    }

    // Validate planets
    for planet in &state.solar_system.planets {
        if planet.id.is_empty() {
            result = result.with_error("Planet ID cannot be empty".to_string());
        }
        if planet.orbit_radius == 0 {
            result = result.with_warning(format!("Planet '{}' has zero orbit radius", planet.id));
        }
    }

    result
}

/// Validate a player for data integrity
#[allow(dead_code)]
pub fn validate_player(player: &Player) -> ValidationResult {
    let mut result = ValidationResult::valid();

    if player.money > 1_000_000 {
        result = result.with_warning("Player has unusually high money".to_string());
    }

    if player.ship.cargo_capacity == 0 {
        result = result.with_error("Ship cargo capacity cannot be zero".to_string());
    }

    let cargo_used = player.cargo.total_cargo_space_used();
    if cargo_used > player.cargo.capacity {
        result = result.with_error("Cargo hold exceeds capacity".to_string());
    }

    result
}

/// Validate a planet for data integrity
#[allow(dead_code)]
pub fn validate_planet(planet: &Planet) -> ValidationResult {
    let mut result = ValidationResult::valid();

    if planet.id.is_empty() {
        result = result.with_error("Planet ID cannot be empty".to_string());
    }

    if planet.name.is_empty() {
        result = result.with_warning("Planet name is empty".to_string());
    }

    if planet.orbit_radius == 0 {
        result = result.with_warning("Orbit radius is zero".to_string());
    }

    result
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

    // =========================================================================
    // Tests for ADR 0006 Data Models
    // =========================================================================

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new();

        assert_eq!(state.version, "1.0.0");
        assert_eq!(state.player.money, 1000);
        assert_eq!(state.player.location, "earth");
        assert_eq!(state.game_clock.current_turn, 1);
        assert_eq!(state.game_clock.total_turns, 10);
        assert!(!state.is_game_over);
        assert!(state.transaction_history.is_empty());
    }

    #[test]
    fn test_game_state_with_settings() {
        let player = Player::new();
        let solar_system = SolarSystem::new("Test System".to_string(), Vec::new());
        let settings = GameSettings {
            difficulty: GameDifficulty::Hard,
            ..Default::default()
        };

        let state = GameState::with_settings(player, solar_system, settings);

        assert_eq!(state.game_clock.total_turns, 5); // Hard difficulty has 5 turns
    }

    #[test]
    fn test_player_creation() {
        let player = Player::new();

        assert_eq!(player.money, 1000);
        assert_eq!(player.location, "earth");
        assert_eq!(player.ship.cargo_capacity, 10);
        assert_eq!(player.cargo.capacity, 10);
        assert_eq!(player.visited_planets.len(), 1);
        assert!(player.visited_planets.contains(&"earth".to_string()));
        assert_eq!(player.total_trades, 0);
        assert_eq!(player.total_earnings, 0);
    }

    #[test]
    fn test_player_visit_planet() {
        let mut player = Player::new();

        player.visit_planet("mars");
        assert!(player.visited_planets.contains(&"mars".to_string()));
        assert_eq!(player.visited_planets.len(), 2);

        // Visiting same planet again should not add duplicate
        player.visit_planet("mars");
        assert_eq!(player.visited_planets.len(), 2);
    }

    #[test]
    fn test_player_money_operations() {
        let mut player = Player::new();

        // Test add_money
        player.add_money(500);
        assert_eq!(player.money, 1500);

        // Test spend_money (successful)
        assert!(player.spend_money(300));
        assert_eq!(player.money, 1200);

        // Test spend_money (insufficient funds)
        assert!(!player.spend_money(2000));
        assert_eq!(player.money, 1200); // Should not change
    }

    #[test]
    fn test_player_record_trade() {
        let mut player = Player::new();

        player.record_trade(100);
        assert_eq!(player.total_trades, 1);
        assert_eq!(player.total_earnings, 100);

        player.record_trade(250);
        assert_eq!(player.total_trades, 2);
        assert_eq!(player.total_earnings, 350);
    }

    #[test]
    fn test_ship_creation() {
        let ship = Ship::new(10.0, 50);

        assert_eq!(ship.speed, 10.0);
        assert_eq!(ship.acceleration, 1);
        assert_eq!(ship.cargo_capacity, 50);
        assert_eq!(ship.fuel, 100);
        assert_eq!(ship.max_fuel, 100);
        assert_eq!(ship.hull, 100);
        assert_eq!(ship.max_hull, 100);
    }

    #[test]
    fn test_ship_fuel_operations() {
        let mut ship = Ship::new(10.0, 50);

        // Test travel (successful)
        assert!(ship.can_travel(50));
        ship.travel(50).unwrap();
        assert_eq!(ship.fuel, 50);

        // Test travel (insufficient fuel)
        assert!(!ship.can_travel(60));
        assert!(ship.travel(60).is_err());

        // Test refuel
        ship.refuel(30);
        assert_eq!(ship.fuel, 80); // 50 + 30 = 80, capped at max_fuel (100)
    }

    #[test]
    fn test_ship_hull_operations() {
        let mut ship = Ship::new(10.0, 50);

        // Test take damage
        ship.take_damage(30);
        assert_eq!(ship.hull, 70);

        // Test repair
        ship.repair(20);
        assert_eq!(ship.hull, 90);

        // Test is_destroyed
        ship.take_damage(100);
        assert!(ship.is_destroyed());
    }

    #[test]
    fn test_cargo_hold_operations() {
        let mut cargo = CargoHold::new(20);

        // Test add commodity
        cargo.add_commodity(CommodityType::Water, 5).unwrap();
        assert_eq!(cargo.get_quantity(&CommodityType::Water), 5);
        assert_eq!(cargo.total_cargo_space_used(), 5);

        // Test add more of same commodity
        cargo.add_commodity(CommodityType::Water, 3).unwrap();
        assert_eq!(cargo.get_quantity(&CommodityType::Water), 8);

        // Test add different commodity
        cargo.add_commodity(CommodityType::Foodstuffs, 4).unwrap();
        assert_eq!(cargo.total_cargo_space_used(), 12);

        // Test capacity exceeded
        let mut small_cargo = CargoHold::new(5);
        assert!(small_cargo.add_commodity(CommodityType::Water, 10).is_err());

        // Test remove commodity
        cargo.remove_commodity(&CommodityType::Water, 3).unwrap();
        assert_eq!(cargo.get_quantity(&CommodityType::Water), 5);

        // Test remove all of commodity
        cargo.remove_commodity(&CommodityType::Water, 5).unwrap();
        assert!(!cargo.contains(&CommodityType::Water));
    }

    #[test]
    fn test_game_difficulty() {
        // Test Easy
        assert_eq!(GameDifficulty::Easy.price_volatility_multiplier(), 0.5);
        assert_eq!(GameDifficulty::Easy.starting_money(), 2000);
        assert_eq!(GameDifficulty::Easy.turn_limit(), 20);

        // Test Normal
        assert_eq!(GameDifficulty::Normal.price_volatility_multiplier(), 1.0);
        assert_eq!(GameDifficulty::Normal.starting_money(), 1000);
        assert_eq!(GameDifficulty::Normal.turn_limit(), 10);

        // Test Hard
        assert_eq!(GameDifficulty::Hard.price_volatility_multiplier(), 1.5);
        assert_eq!(GameDifficulty::Hard.starting_money(), 500);
        assert_eq!(GameDifficulty::Hard.turn_limit(), 5);

        // Test Custom
        let custom = GameDifficulty::Custom {
            price_volatility: 2.0,
            starting_money: 3000,
            turn_limit: 30,
        };
        assert_eq!(custom.price_volatility_multiplier(), 2.0);
        assert_eq!(custom.starting_money(), 3000);
        assert_eq!(custom.turn_limit(), 30);
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::valid();
        assert!(valid.is_valid);
        assert!(valid.errors.is_empty());

        let invalid = ValidationResult::invalid(vec!["Error 1".to_string()]);
        assert!(!invalid.is_valid);
        assert_eq!(invalid.errors.len(), 1);

        let with_warning = valid.clone().with_warning("Warning message".to_string());
        assert!(with_warning.is_valid);
        assert_eq!(with_warning.warnings.len(), 1);

        let with_error = valid.clone().with_error("Error message".to_string());
        assert!(!with_error.is_valid);
        assert_eq!(with_error.errors.len(), 1);
    }

    #[test]
    fn test_validate_planet() {
        // Valid planet
        let planet = Planet::new(
            "earth".to_string(),
            "Earth".to_string(),
            5,
            10,
            PlanetType::Agricultural,
        );
        let result = validate_planet(&planet);
        assert!(result.is_valid);

        // Invalid planet (empty ID)
        let invalid_planet = Planet {
            id: "".to_string(),
            name: "Test".to_string(),
            orbit_radius: 5,
            orbit_period: 10,
            position: Position::start(),
            economy: PlanetEconomy::new(PlanetType::Agricultural),
            planet_type: PlanetType::Agricultural,
        };
        let result = validate_planet(&invalid_planet);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_serialization_roundtrip() {
        // Test GameState serialization
        let state = GameState::new();
        let json = serde_json::to_string(&state).unwrap();
        let loaded: GameState = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.version, state.version);
        assert_eq!(loaded.player.money, state.player.money);
        assert_eq!(loaded.game_clock.total_turns, state.game_clock.total_turns);
    }

    #[test]
    fn test_game_state_advance_turns() {
        let mut state = GameState::new();

        // Add a planet to the solar system
        let planet = Planet::new(
            "earth".to_string(),
            "Earth".to_string(),
            5,
            10,
            PlanetType::Agricultural,
        );
        state.solar_system.planets.push(planet);

        // Advance turns
        let advanced = state.advance_turns(3);
        assert_eq!(advanced, 3);
        assert_eq!(state.game_clock.current_turn, 4);

        // Game should not be over yet
        assert!(!state.is_game_over);
    }

    #[test]
    fn test_game_state_end_game() {
        let mut state = GameState::new();

        state.end_game("Test game over".to_string());

        assert!(state.is_game_over);
        assert_eq!(state.game_over_reason, Some("Test game over".to_string()));
        assert_eq!(state.game_clock.current_turn, state.game_clock.total_turns);
    }

    #[test]
    fn test_transaction_record() {
        let mut state = GameState::new();

        let transaction = Transaction {
            turn: 1,
            planet_id: "earth".to_string(),
            commodity: CommodityType::Water,
            quantity: 10,
            price_per_unit: 10,
            transaction_type: TransactionType::Buy,
        };

        state.record_transaction(transaction);

        assert_eq!(state.transaction_history.len(), 1);
        assert_eq!(state.transaction_history[0].turn, 1);
        assert_eq!(state.transaction_history[0].commodity, CommodityType::Water);
    }

    // =========================================================================
    // Tests for Movement System (ADR 0002)
    // =========================================================================

    #[test]
    fn test_travel_state_idle() {
        let state = TravelState::idle("earth".to_string());
        assert!(state.is_idle());
        assert!(!state.is_in_transit());
        assert_eq!(state.current_planet(), Some(&"earth".to_string()));
        assert_eq!(state.destination(), None);
        assert!(!state.has_arrived(10));
        assert_eq!(state.turns_remaining(10), 0);
    }

    #[test]
    fn test_travel_state_in_transit() {
        let state = TravelState::in_transit("mars".to_string(), 10, 5);
        assert!(!state.is_idle());
        assert!(state.is_in_transit());
        assert_eq!(state.current_planet(), None);
        assert_eq!(state.destination(), Some(&"mars".to_string()));
        assert!(!state.has_arrived(9));
        assert!(state.has_arrived(10));
        assert!(state.has_arrived(15));
        assert_eq!(state.turns_remaining(7), 3);
        assert_eq!(state.turns_remaining(10), 0);
    }

    #[test]
    fn test_arrival_event_creation() {
        let event = ArrivalEvent::new("mars".to_string(), 10, 5);
        assert_eq!(event.destination_planet_id, "mars");
        assert_eq!(event.arrival_turn, 10);
        assert_eq!(event.departure_turn, 5);
        assert_eq!(event.travel_turns, 5);
    }

    #[test]
    fn test_travel_error_display() {
        assert_eq!(
            TravelError::AlreadyInTransit.to_string(),
            "Ship is already in transit"
        );
        assert_eq!(
            TravelError::SameDestination.to_string(),
            "Destination is the same as current location"
        );
        assert_eq!(
            TravelError::InvalidDestination.to_string(),
            "Destination planet does not exist"
        );
        assert_eq!(
            TravelError::InsufficientFuel.to_string(),
            "Not enough fuel for the journey"
        );
        assert_eq!(
            TravelError::ShipDestroyed.to_string(),
            "Ship is destroyed and cannot travel"
        );
        assert_eq!(
            TravelError::GameOver.to_string(),
            "Cannot travel: game is over"
        );
    }

    #[test]
    fn test_player_has_travel_state() {
        let player = Player::new();
        assert!(player.travel_state.is_idle());
        assert_eq!(player.travel_state.current_planet(), Some(&"earth".to_string()));
    }

    fn create_test_game_state_with_planets() -> GameState {
        let mut state = GameState::new();

        // Add planets to the solar system
        let earth = Planet::new(
            "earth".to_string(),
            "Earth".to_string(),
            5,  // orbit_radius
            10, // orbit_period
            PlanetType::Agricultural,
        );
        let mars = Planet::new(
            "mars".to_string(),
            "Mars".to_string(),
            12, // orbit_radius
            15, // orbit_period
            PlanetType::Mining,
        );

        state.solar_system.planets.push(earth);
        state.solar_system.planets.push(mars);

        // Ensure player starts at earth with idle state
        state.player.location = "earth".to_string();
        state.player.travel_state = TravelState::idle("earth".to_string());

        state
    }

    #[test]
    fn test_initiate_travel_success() {
        let mut state = create_test_game_state_with_planets();

        // Ensure ship has enough fuel
        state.player.ship.fuel = 100;
        state.player.ship.max_fuel = 100;

        // Initiate travel to mars
        let result = state.initiate_travel("mars");

        assert!(result.is_ok());
        assert!(state.is_in_transit());
        assert_eq!(state.get_destination(), Some(&"mars".to_string()));
        assert_eq!(state.get_current_location(), None);

        // Fuel should be consumed (distance = 12 - 5 = 7)
        assert_eq!(state.player.ship.fuel, 93); // 100 - 7 = 93
    }

    #[test]
    fn test_initiate_travel_same_destination() {
        let mut state = create_test_game_state_with_planets();

        let result = state.initiate_travel("earth");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TravelError::SameDestination);
        assert!(!state.is_in_transit());
    }

    #[test]
    fn test_initiate_travel_invalid_destination() {
        let mut state = create_test_game_state_with_planets();

        let result = state.initiate_travel("jupiter");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TravelError::InvalidDestination);
        assert!(!state.is_in_transit());
    }

    #[test]
    fn test_initiate_travel_already_in_transit() {
        let mut state = create_test_game_state_with_planets();

        // First, initiate travel
        state.player.ship.fuel = 100;
        state.initiate_travel("mars").unwrap();

        // Try to initiate travel again while in transit
        let result = state.initiate_travel("earth");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TravelError::AlreadyInTransit);
    }

    #[test]
    fn test_initiate_travel_insufficient_fuel() {
        let mut state = create_test_game_state_with_planets();

        // Set fuel to 0
        state.player.ship.fuel = 0;
        state.player.ship.max_fuel = 100;

        let result = state.initiate_travel("mars");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TravelError::InsufficientFuel);
        assert!(!state.is_in_transit());
    }

    #[test]
    fn test_initiate_travel_ship_destroyed() {
        let mut state = create_test_game_state_with_planets();

        // Destroy the ship
        state.player.ship.hull = 0;

        let result = state.initiate_travel("mars");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TravelError::ShipDestroyed);
        assert!(!state.is_in_transit());
    }

    #[test]
    fn test_initiate_travel_game_over() {
        let mut state = create_test_game_state_with_planets();

        // End the game
        state.end_game("Test".to_string());

        let result = state.initiate_travel("mars");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TravelError::GameOver);
    }

    #[test]
    fn test_next_turn_advances_planets() {
        let mut state = create_test_game_state_with_planets();

        // Get initial positions
        let earth_initial_pos = state.solar_system.planets[0].position.orbital_position;

        // Advance one turn
        let arrival = state.next_turn();

        // No arrival should occur (not in transit)
        assert!(arrival.is_none());

        // Turn should be advanced
        assert_eq!(state.game_clock.current_turn, 2);

        // Planet positions should be advanced
        let earth_new_pos = state.solar_system.planets[0].position.orbital_position;
        assert_eq!(earth_new_pos, (earth_initial_pos + 1) % 10);
    }

    #[test]
    fn test_next_turn_arrival_detection() {
        let mut state = create_test_game_state_with_planets();

        // Set up ship to be in transit with arrival at turn 3
        state.player.travel_state = TravelState::in_transit(
            "mars".to_string(),
            3, // arrival_turn
            1, // departure_turn
        );
        state.game_clock.current_turn = 2;

        // Advance to turn 3
        let arrival = state.next_turn();

        // Should detect arrival
        assert!(arrival.is_some());
        let event = arrival.unwrap();
        assert_eq!(event.destination_planet_id, "mars");
        assert_eq!(event.arrival_turn, 3);

        // Player should now be at mars
        assert_eq!(state.player.location, "mars");

        // Travel state should be idle at mars
        assert!(state.player.travel_state.is_idle());
        assert_eq!(state.player.travel_state.current_planet(), Some(&"mars".to_string()));

        // Mars should be in visited planets
        assert!(state.player.visited_planets.contains(&"mars".to_string()));
    }

    #[test]
    fn test_next_turn_no_arrival_when_not_in_transit() {
        let mut state = create_test_game_state_with_planets();

        // Not in transit
        assert!(state.player.travel_state.is_idle());

        let arrival = state.next_turn();

        assert!(arrival.is_none());
        assert!(state.player.travel_state.is_idle());
    }

    #[test]
    fn test_next_turn_no_advance_when_game_over() {
        let mut state = create_test_game_state_with_planets();

        // End the game
        state.end_game("Test".to_string());
        let current_turn = state.game_clock.current_turn;

        let arrival = state.next_turn();

        assert!(arrival.is_none());
        assert_eq!(state.game_clock.current_turn, current_turn); // Should not advance
    }

    #[test]
    fn test_turns_until_arrival() {
        let mut state = create_test_game_state_with_planets();

        // Not in transit
        assert_eq!(state.turns_until_arrival(), 0);

        // Set up in transit
        state.player.ship.fuel = 100;
        state.initiate_travel("mars").unwrap();

        // Should have some turns remaining
        assert!(state.turns_until_arrival() > 0);
    }

    #[test]
    fn test_travel_state_serialization() {
        let idle = TravelState::idle("earth".to_string());
        let in_transit = TravelState::in_transit("mars".to_string(), 10, 5);

        // Test serialization roundtrip
        let idle_json = serde_json::to_string(&idle).unwrap();
        let in_transit_json = serde_json::to_string(&in_transit).unwrap();

        let idle_loaded: TravelState = serde_json::from_str(&idle_json).unwrap();
        let in_transit_loaded: TravelState = serde_json::from_str(&in_transit_json).unwrap();

        assert_eq!(idle, idle_loaded);
        assert_eq!(in_transit, in_transit_loaded);
    }

    #[test]
    fn test_arrival_event_serialization() {
        let event = ArrivalEvent::new("mars".to_string(), 10, 5);

        let json = serde_json::to_string(&event).unwrap();
        let loaded: ArrivalEvent = serde_json::from_str(&json).unwrap();

        assert_eq!(event, loaded);
    }

    #[test]
    fn test_travel_error_serialization() {
        let errors = vec![
            TravelError::AlreadyInTransit,
            TravelError::SameDestination,
            TravelError::InvalidDestination,
            TravelError::InsufficientFuel,
            TravelError::ShipDestroyed,
            TravelError::GameOver,
        ];

        for error in errors {
            let json = serde_json::to_string(&error).unwrap();
            let loaded: TravelError = serde_json::from_str(&json).unwrap();
            assert_eq!(error, loaded);
        }
    }

    #[test]
    fn test_game_state_serialization_with_travel_state() {
        let mut state = create_test_game_state_with_planets();
        state.player.ship.fuel = 100;

        // Initiate travel
        state.initiate_travel("mars").unwrap();

        // Serialize and deserialize
        let json = serde_json::to_string(&state).unwrap();
        let loaded: GameState = serde_json::from_str(&json).unwrap();

        assert_eq!(state.player.travel_state, loaded.player.travel_state);
        assert_eq!(state.is_in_transit(), loaded.is_in_transit());
    }

    #[test]
    fn test_validate_travel_state_idle_valid() {
        let mut state = create_test_game_state_with_planets();
        state.player.travel_state = TravelState::idle("earth".to_string());
        state.player.location = "earth".to_string();

        let result = validate_game_state(&state);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_travel_state_idle_invalid_planet() {
        let mut state = create_test_game_state_with_planets();
        state.player.travel_state = TravelState::idle("jupiter".to_string());

        let result = validate_game_state(&state);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("jupiter")));
    }

    #[test]
    fn test_validate_travel_state_in_transit_valid() {
        let mut state = create_test_game_state_with_planets();
        state.player.travel_state = TravelState::in_transit("mars".to_string(), 10, 5);

        let result = validate_game_state(&state);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_travel_state_in_transit_invalid_arrival() {
        let mut state = create_test_game_state_with_planets();
        // Arrival turn before departure turn
        state.player.travel_state = TravelState::in_transit("mars".to_string(), 5, 10);

        let result = validate_game_state(&state);
        assert!(!result.is_valid);
        assert!(result
            .errors
            .iter()
            .any(|e| e.contains("arrival turn must be after departure")));
    }

    #[test]
    fn test_validate_travel_state_in_transit_past_arrival() {
        let mut state = create_test_game_state_with_planets();
        // Set current turn to be after arrival, but within total turns
        state.game_clock.current_turn = 20;
        state.game_clock.total_turns = 30;
        // Arrival turn in the past
        state.player.travel_state = TravelState::in_transit("mars".to_string(), 10, 5);

        let result = validate_game_state(&state);
        
        // Should be a warning, not an error
        assert!(result.is_valid, "Expected valid result but got errors: {:?}", result.errors);
        assert!(result
            .warnings
            .iter()
            .any(|w| w.contains("arrival turn is in the past")));
    }

    #[test]
    fn test_full_travel_journey() {
        let mut state = create_test_game_state_with_planets();
        state.player.ship.fuel = 100;

        // Start at earth
        assert_eq!(state.player.location, "earth");
        assert!(state.player.travel_state.is_idle());

        // Initiate travel to mars
        let result = state.initiate_travel("mars");
        assert!(result.is_ok());
        assert!(state.is_in_transit());

        // Get the arrival turn
        let arrival_turn = match &state.player.travel_state {
            TravelState::InTransit { arrival_turn, .. } => *arrival_turn,
            _ => panic!("Should be in transit"),
        };

        // Advance turns until arrival
        let mut arrival_event = None;
        while state.game_clock.current_turn < arrival_turn {
            arrival_event = state.next_turn();
            if arrival_event.is_some() {
                break;
            }
        }

        // Should have arrived
        assert!(arrival_event.is_some());
        assert_eq!(arrival_event.unwrap().destination_planet_id, "mars");
        assert_eq!(state.player.location, "mars");
        assert!(state.player.travel_state.is_idle());
        assert!(state.player.visited_planets.contains(&"mars".to_string()));
    }
}
