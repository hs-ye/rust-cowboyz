# 0006: Data Models/Schema for Space-Western Trading Game

## Status
Accepted

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

**Simplicity**: The data models are designed to be simple and focused for a single-player game, following the project principle of "Build MVP first, get the minimum system working, don't over-engineer. Prefer simple, accessible features over complexity."

**Client-side Storage**: Since data is stored in browser localStorage with no backend requirements, we implement validation mechanisms while acknowledging inherent limitations.

**Computational Efficiency**: Following the simplified orbital mechanics from ADR #0002, the data models are designed to minimize computational overhead during gameplay, using simple integer calculations and turn-based mechanics.

**Maintainability**: Clear entity relationships facilitate development and debugging for a small indie game.

### Key Data Model Relationships

#### Core Game State Structure
The overall game state consists of the player's current status, the solar system configuration, and game progression tracking.

#### Player Entity
The player entity encompasses the core game state for the player character, including:
- Financial status (credits)
- Current location (current_planet_id)
- Ship ownership and specifications
- Inventory of resources
- Trading statistics and achievements

#### Ship Entity
The ship entity represents the player's vessel with:
- Cargo capacity for transporting resources
- Travel speed that affects journey duration
- Upgradable components that improve performance
- Current fuel and hull status

#### Solar System Structure
The solar system contains the game world with:
- Multiple planets that serve as trade destinations
- Orbital positions that change with each turn
- Distances between planets that determine travel time

#### Planet Entity
Each planet entity includes:
- Unique type that determines its supply/demand patterns for resources
- Market data with current prices for all available resources
- Position in the solar system that changes over time

#### Market System
The market system connects planets and resources with:
- Current prices for each resource type at each planet
- Dynamic pricing influenced by supply and demand
- Historical data for players to analyze trends

#### Resource and Trade Entities
The trading system includes:
- Various resource types that can be bought and sold
- Transaction records for tracking past trades
- Inventory management for purchased resources

#### Game Settings and Configuration
Configuration includes:
- Player preferences for audio and visuals
- Game difficulty settings
- Progress tracking and achievement systems

#### Data Persistence Implementation
The data models will be persisted using the following approach:
1. Serialize the entire GameState to JSON using serde_json
2. Store in browser localStorage with key "rust_cowboyz_game_state"
3. Implement validation functions to verify data integrity on load
4. Handle migration between different game versions when needed

### Integration Points
- **Market System**: Integrates with ADR #0005 for dynamic pricing
- **Orbital Mechanics**: Integrates with ADR #0002 for movement calculations
- **UI System**: Provides data structures for Leptos components (ADR #0004)
- **Trading Mechanics**: Supports core gameplay loop from project principles

## Consequences

### Positive
- Simple data model supports the core trading gameplay
- Clear relationships between entities enable efficient gameplay
- Serde integration provides reliable serialization for browser storage
- Focused design allows for rapid development and debugging
- Well-defined data boundaries improve code maintainability

### Negative
- May require future expansion as game features grow
- Browser storage limitations may constrain game complexity
- Version migration complexity increases with data model evolution

## References
- [Project Principles ADR #0000](./0000-project-principles.md)
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Movement Mechanics System ADR #0002](./0002-movement-mechanics-system.md)
- [Tech Stack Selection ADR #0004](./0004-tech-stack-selection.md)