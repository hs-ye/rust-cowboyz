# 0005: Data Models/Schema for Space-Western Trading Game

## Status
proposed

## Date
2026-03-01

## Deciders
- User
- software-architect

## Context
We need to define comprehensive data models and schema for the space-western trading game that will support all core game entities and their relationships. The game requires persistent state management using browser storage (localStorage) to enable single-player functionality without backend requirements. The data models must support the market/economy system, orbital mechanics, player progression, and the overall trading gameplay loop.

Key requirements for the data models:
- Support browser-based state persistence using localStorage
- Enable efficient serialization/deserialization with serde
- Represent all core game entities (planets, resources, markets, ships, player state)
- Support the dynamic market economy system defined in ADR #0001
- Integrate with the orbital mechanics system defined in ADR #0002
- Facilitate the turn-based gameplay mechanics
- Allow for future extensibility while maintaining simplicity for MVP

## Decision
We will implement a comprehensive data model schema with the following core entities and relationships:

### Technical Objectives
- Define clear data structures for all game entities with appropriate relationships
- Ensure efficient serialization for browser storage using serde
- Support all gameplay mechanics including trading, movement, and progression
- Enable state restoration after page reloads while maintaining data integrity
- Provide a foundation for future feature additions while keeping the MVP focused

### Architecture Considerations

**Scalability**: The data models are designed to scale with additional planets, resources, and complexity while maintaining performance for browser-based storage.

**Security**: Since data is stored client-side, we implement validation mechanisms to prevent cheating while acknowledging inherent limitations.

**Maintainability**: Clear entity relationships and well-documented structures facilitate ongoing development and debugging.

**Performance**: Data structures are optimized for frequent read/write operations during gameplay while minimizing serialization overhead.

### Technical Specifications

#### Core Game State Structure
```rust
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub player: Player,
    pub solar_system: SolarSystem,
    pub current_turn: u32,
    pub game_date: String, // Human-readable date representation
    pub settings: GameSettings,
    pub achievements: Vec<Achievement>,
    pub tutorial_flags: HashMap<String, bool>, // Track tutorial completion
    pub version: u32, // For version migration purposes
}
```

#### Player Entity
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub credits: f64,
    pub current_planet_id: String,
    pub ship: Ship,
    pub reputation: Reputation,
    pub skills: Skills,
    pub inventory: Inventory,
    pub stats: PlayerStats,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Reputation {
    pub faction_reputations: HashMap<String, f64>, // Faction name to reputation value (-100 to 100)
    pub notoriety: f64, // General reputation (-100 to 100, negative = wanted)
    pub trade_license_level: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Skills {
    pub trading: u32,
    pub navigation: u32,
    pub engineering: u32,
    pub diplomacy: u32,
    pub survival: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ship {
    pub id: String,
    pub name: String,
    pub hull_strength: f64,
    pub max_hull_strength: f64,
    pub fuel_capacity: f64,
    pub current_fuel: f64,
    pub cargo_capacity: f64,
    pub current_cargo: f64,
    pub speed_multiplier: f64,
    pub upgrades: Vec<ShipUpgrade>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ShipUpgrade {
    HullPlating { level: u32, bonus: f64 },
    Engine { level: u32, speed_bonus: f64 },
    CargoHold { level: u32, capacity_bonus: f64 },
    ShieldGenerator { level: u32, defense_bonus: f64 },
    Scanner { level: u32, range_bonus: f64 },
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    pub resources: HashMap<ResourceType, f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerStats {
    pub total_distance_traveled: f64,
    pub total_trades_made: u32,
    pub total_profits: f64,
    pub planets_visited: HashSet<String>,
    pub time_played_seconds: u64,
}
```

#### Solar System Structure
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct SolarSystem {
    pub id: String,
    pub name: String,
    pub star_class: StarClass,
    pub planets: HashMap<String, Planet>,
    pub connections: Vec<Route>, // Possible travel routes between planets
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Planet {
    pub id: String,
    pub name: String,
    pub x: f64, // Position in solar system
    pub y: f64,
    pub planet_type: PlanetType,
    pub distance_from_star: f64,
    pub orbital_period: f64, // Days to complete orbit
    pub current_angle: f64, // Current position in orbit (in radians)
    pub market: Market,
    pub population_level: PopulationLevel,
    pub government_type: GovernmentType,
    pub special_resources: Vec<ResourceType>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Market {
    pub resources: HashMap<ResourceType, MarketResource>,
    pub last_updated_turn: u32,
    pub base_price_modifiers: HashMap<ResourceType, f64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MarketResource {
    pub resource_type: ResourceType,
    pub current_price: f64,
    pub base_price: f64,
    pub supply: f64, // Normalized 0.0-2.0 where 1.0 is equilibrium
    pub demand: f64, // Normalized 0.0-2.0 where 1.0 is equilibrium
    pub local_multiplier: f64,
    pub historical_prices: Vec<f64>, // Last N price values for trends
}
```

#### Resource and Trade Entities
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Basic(BasicResource),
    Luxury(LuxuryResource),
    Technology(TechResource),
    Agricultural(AgResource),
    Special(SpecialResource),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BasicResource {
    Water,
    Minerals,
    Metals,
    Fuel,
    Textiles,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum LuxuryResource {
    Spirits,
    Tobacco,
    ExoticFoods,
    Jewelry,
    Artifacts,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TechResource {
    Electronics,
    Robotics,
    Pharmaceuticals,
    Biotech,
    Software,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AgResource {
    Grain,
    Livestock,
    Fruits,
    Vegetables,
    Spices,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpecialResource {
    #[serde(rename = "alien_artifacts")]
    AlienArtifacts,
    #[serde(rename = "rare_minerals")]
    RareMinerals,
    #[serde(rename = "exotic_materials")]
    ExoticMaterials,
    #[serde(rename = "precious_stones")]
    PreciousStones,
    #[serde(rename = "energy_crystals")]
    EnergyCrystals,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TradeTransaction {
    pub timestamp: u32, // Turn number
    pub planet_id: String,
    pub resource_type: ResourceType,
    pub quantity: f64,
    pub price_per_unit: f64,
    pub total_value: f64,
    pub transaction_type: TransactionType,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Buy,
    Sell,
}
```

#### Orbital Mechanics Data
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct Route {
    pub origin_planet_id: String,
    pub destination_planet_id: String,
    pub distance: f64, // In arbitrary units
    pub travel_time: u32, // Number of turns required
    pub fuel_cost: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OrbitalPosition {
    pub planet_id: String,
    pub angle: f64, // Radians
    pub distance_from_star: f64,
    pub orbital_period: f64, // Turns to complete orbit
}
```

#### Game Settings and Configuration
```rust
#[derive(Serialize, Deserialize, Clone)]
pub struct GameSettings {
    pub difficulty: DifficultyLevel,
    pub sound_enabled: bool,
    pub music_volume: f32,
    pub effects_volume: f32,
    pub auto_save_enabled: bool,
    pub ui_theme: UiTheme,
    pub tutorial_completed: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub unlocked_at_turn: Option<u32>,
    pub progress: Option<f64>, // For progress-based achievements
}
```

#### Enums and Supporting Types
```rust
#[derive(Serialize, Deserialize, Clone)]
pub enum PlanetType {
    Desert,
    Oceanic,
    Forest,
    Ice,
    Volcanic,
    GasGiant,
    Asteroid,
    Terraformed,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PopulationLevel {
    pub name: String,
    pub min_population: u32,
    pub max_population: Option<u32>, // None means unlimited
    pub description: String,
}

impl PopulationLevel {
    pub fn outpost() -> Self {
        Self {
            name: "Outpost".to_string(),
            min_population: 1,
            max_population: Some(50),
            description: "Small frontier settlement".to_string(),
        }
    }

    pub fn settlement() -> Self {
        Self {
            name: "Settlement".to_string(),
            min_population: 51,
            max_population: Some(500),
            description: "Growing community".to_string(),
        }
    }

    pub fn town() -> Self {
        Self {
            name: "Town".to_string(),
            min_population: 501,
            max_population: Some(2000),
            description: "Established town".to_string(),
        }
    }

    pub fn city() -> Self {
        Self {
            name: "City".to_string(),
            min_population: 2001,
            max_population: Some(10000),
            description: "Major population center".to_string(),
        }
    }

    pub fn metropolis() -> Self {
        Self {
            name: "Metropolis".to_string(),
            min_population: 10001,
            max_population: None,
            description: "Large metropolitan area".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StarClass {
    Yellow, // Like our Sun
    RedDwarf,
    BlueGiant,
    WhiteDwarf,
    Binary,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
    Realistic,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GovernmentType {
    Corporate,
    Democratic,
    Authoritarian,
    Anarchist,
    Tribal,
}
```

#### Data Persistence Implementation
The data models will be persisted using the following approach:
1. Serialize the entire GameState to JSON using serde_json
2. Store in browser localStorage with key "rust_cowboyz_game_state"
3. Implement validation functions to verify data integrity on load
4. Handle migration between different game versions when needed
5. Compress large datasets when storage limits approach browser constraints

#### Implementation Approach

**Phase 1**: Define core structs with serde derives and basic validation
- Implement GameState, Player, Ship, and Inventory structures
- Add serialization/deserialization support
- Create unit tests for serialization round-trips

**Phase 2**: Implement solar system and planetary data structures
- Define Planet, Market, and MarketResource structures
- Implement resource type enums
- Add orbital mechanics data structures

**Phase 3**: Add supporting game systems data
- Implement achievements, settings, and game statistics
- Create trade transaction logging structures
- Add difficulty and progression systems

**Phase 4**: Implement persistence layer
- Create save/load functions using localStorage
- Add data validation and error handling
- Implement version migration support

#### Data Validation and Migration Strategy

**Validation Functions**:
- Implement validate_game_state() to check for data consistency
- Verify all references between entities are valid
- Ensure numerical values are within expected bounds
- Validate that required fields are present and not corrupted

**Version Migration**:
- Include version field in GameState struct
- Implement migration functions for each version change
- Maintain backward compatibility for at least 3 previous versions
- Gracefully handle corrupted or unrecognized save data

**Storage Optimization**:
- Implement data compression for large collections
- Use delta encoding for frequently updated values
- Periodically clean up historical data that exceeds retention policies

### Integration Points
- **Market System**: Integrates with ADR #0001 for dynamic pricing
- **Orbital Mechanics**: Integrates with ADR #0002 for movement calculations
- **UI System**: Provides data structures for Leptos components (ADR #0004)
- **Trading Mechanics**: Supports core gameplay loop from project principles

### Risk Mitigation Strategies
- **Data Corruption**: Implement checksums and validation for loaded game states
- **Storage Limits**: Monitor localStorage usage and implement compression if needed
- **Version Compatibility**: Design version fields into data structures for backward compatibility
- **Performance**: Optimize frequently accessed data structures for quick lookups
- **Security**: Validate all data after deserialization to prevent exploitation

## Consequences

### Positive
- Comprehensive data model supports all planned game features
- Clear relationships between entities enable efficient querying and updates
- Serde integration provides reliable serialization for browser storage
- Extensible design allows for future feature additions
- Structured approach facilitates testing and debugging
- Well-defined data boundaries improve code maintainability

### Negative
- Complex object graph may increase serialization/deserialization time
- Large state objects could approach browser storage limits
- Version migration complexity increases with data model evolution
- Memory usage in browser may impact performance on lower-end devices
- Deep nesting of objects could complicate state updates

## References
- [Project Principles ADR #0000](./0000-project-principles.md)
- [Market/Economy System ADR #0001](./0001-market-economy-system.md)
- [Movement Mechanics System ADR #0002](./0002-movement-mechanics-system.md)
- [Tech Stack Selection ADR #0004](./0004-tech-stack-selection.md)