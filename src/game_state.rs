//! Game state management for turn-based system
//! Based on ADR 0002: Movement Mechanics System
//! Data models based on ADR 0006: Data Models/Schema for Space-Western Trading Game

use crate::simulation::commodity::CommodityType;
use crate::simulation::economy::PlanetEconomy;
use crate::simulation::orbits::Position;
use crate::simulation::planet_types::PlanetType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
        }
    }

    /// Create a player with custom starting values
    pub fn with_values(money: u32, location: String, ship: Ship, cargo_capacity: u32) -> Self {
        Player {
            money,
            location: location.clone(),
            ship,
            cargo: CargoHold::new(cargo_capacity),
            visited_planets: vec![location],
            total_trades: 0,
            total_earnings: 0,
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
}
