# 0001: General Gameplay Scenario

## Status
Accepted

## Date
2026-02-28

## Deciders
- User
- software-architect
- game-designer

## Context
We need to define the general gameplay scenario for the Rust Cowboyz - The game is a single-player space-western themed commodity trading game with simplified orbital mechanics. This decision will guide all subsequent design and implementation choices, ensuring consistency across all game systems. The core gameplay should focus on resource trading with strategic orbital mechanics planning.

## Decision
The general gameplay scenario for Rust Cowboyz will be a single-player space-western commodity trading game with the following core elements:

### Key Objective
Create an engaging space-western trading experience focused on "buy low, sell high" mechanics with strategic orbital planning challenges in a single solar system.

### Game Design Considerations
- **Simple Gameplay Loop**: Trading is the core gameplay - buy low on one planet, travel to another, sell high
- **Self-Explanatory UI**: Simple, single-page web interface that is intuitive without tutorials
- **Strategic Challenge**: Orbital mechanics as the key strategic element - players must plan routes between orbiting planets
- **Scope Limitation**: Single solar system with multiple planets orbiting a star, no complex features beyond trading and movement
- **Randomisation element**: Background Events may happen that presents opportunity or risks (e.g. War, Market Crash, new innovation etc.) that player needs to change their strategy to adapt to.

### Technical Specifications
- **Main Inputs**: Player trading decisions, route planning between planets, ship upgrade purchases
- **Player Interaction Patterns**: Trading commodities, planning travel routes, upgrading ship capabilities (speed, cargo capacity)
- **Main 'Happy Path' Use Case**: Player starts on a planet, checks market prices, buys low-value goods, travels to another planet, sells high-value goods, earns profit, upgrades ship, repeats
- **Edge Cases**: Market fluctuations, insufficient cargo space, travel time optimization, ship upgrade decisions
- **High-Level Technical Decisions**: Single-page web application using Leptos framework, state saved in browser storage, Rust-based game logic compiled to WASM

### Integration Points
- Connect with movement mechanics system for planet-to-planet travel
- Interface with market/economy system for commodity pricing and trading
- Integrate with ship upgrade system for customization
- Link to orbital mechanics system for travel timing and route planning
- Coordinate with solar system map for visual representation

## Consequences

### Positive
- Provides clear direction for all development teams based on actual project principles
- Ensures consistency with the single-player, trading-focused vision
- Enables focused development on the core gameplay loop
- Establishes foundation for the strategic orbital mechanics challenge

### Negative
- Limits scope to trading mechanics, excluding combat or complex RPG elements
- Single player locks technical architecture - later pivot to multi-player might require extensive re-write.
- Strategic planning may be too complex for casual players
- Single solar system constraint limits exploration possibilities

## References
- [Project Principles ADR #0000](./0000-project-principles.md)
- [Movement Mechanics ADR #0002] (to be updated)