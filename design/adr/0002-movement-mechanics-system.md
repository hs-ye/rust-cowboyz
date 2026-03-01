# 0002: Movement Mechanics System

## Status
Accepted

## Date
2026-02-28

## Deciders
- User
- software-architect
- game-designer

## Summary
This ADR implements simplified turn-based orbital mechanics that aim to minimize computational overhead for this simple single-player game. The approach uses discrete integer calculations for planet positions and travel time calculations, ensuring the orbital mechanics provide strategic depth without requiring intensive computation.

## Context
The movement mechanics system is a fundamental component of the Rust Cowboyz gameplay experience. In the single-player space-western trading game, players need to travel between planets in a solar system. This system must support planet-to-planet travel with simplified orbital mechanics that create the core strategic challenge of the game. 

The movement system should integrate seamlessly with the trading gameplay scenario defined in ADR #0001. Without a well-defined movement system, players cannot execute the core trading loop of buying on one planet and selling on another.

## Decision
The movement mechanics system for Rust Cowboyz will implement simplified orbital mechanics for spaceship travel between planets in a single solar system:

### Key Objective
Provide strategic planet-to-planet travel mechanics that make orbital planning the core strategic challenge of the game, while maintaining simplicity and accessibility.

### Game Design Considerations
- **Strategic Gameplay**: Orbital mechanics as the primary strategic element - players must plan routes considering planet positions and travel times
- **Player Understanding**: Keep travel mechanics intuitive - use 'turns' to represent time, show the 'time' cost of travelling to all possible destinations to the user
- **Implementation Complexity**: Simplified orbital mechanics that feel realistic but remain accessible. Should be as low-friction as possible to aid gameplay loop
- **Consistency**: Maintain uniform travel experience that complements the trading gameplay loop

### Technical Specifications
- **Main Inputs**: Planet selection for destination, travel confirmation, ship configuration settings
- **Player Interaction Patterns**: Select destination planet, view travel time and cost, confirm journey, advance time during travel
- **Main 'Happy Path' Use Case**: Player selects destination planet, sees estimated travel time based on orbital positions, confirms journey, arrives at destination with time advanced
- **Edge Cases**: Insufficient fuel for journey, cargo capacity limits, optimal route calculation, travel interruption
- **High-Level Technical Decisions**: Turn-based time advancement during travel, orbital position calculations using simplified Keplerian mechanics, fuel consumption based on distance/time, browser-based state persistence

### Simplified Turn-Based Orbital Mechanics

The game will implement a turn-based orbital system using discrete integer values that maintains the strategic essence while simplifying calculations:

**Turn-Based Orbital Position:**
- Each planet has a fixed orbital period measured in turns (e.g., Planet A orbits every 10 turns, Planet B every 15 turns)
- Each planet has a fixed orbital radius (distance from the star)
- Planet positions are represented as integers from 0 to (orbital_period - 1)
- At each turn, all planet positions advance by 1, wrapping around after reaching their orbital period

**Travel Time Calculation:**
- Travel follows an Interplanetary Brachistochrone model with constant acceleration/deceleration
- Ship accelerates at 1 unit/turn² until midpoint, then decelerates at 1 unit/turn² until destination
- Total travel time is calculated using the kinematic equation: `travel_turns = 2 * sqrt(base_distance / acceleration)`
- Where acceleration is a constant (1 unit/turn², upgradable via ship improvements)
- If the ship overshoots the destination, it automatically adjusts to arrive at the target
- This approach avoids the problem of ships being unable to reach fast-moving distant planets

**Turn-Based Algorithm for Travel Between Two Planets:**
```
// Turn-based pseudo-code
struct Planet {
    name: String,
    orbital_radius: u32,  // Integer distance from star
    orbital_period: u32,  // Turns to complete one orbit
    position: u32,        // Current position in orbit (0 to orbital_period-1)
}

fn calculate_travel_turns(departure: &Planet, destination: &Planet, ship_acceleration: u32) -> u32 {
    // Calculate base distance based on orbital radii
    let base_distance = (destination.orbital_radius as i32 - departure.orbital_radius as i32).abs() as u32;

    // Calculate travel time using Brachistochrone model (accelerate halfway, decelerate halfway)
    // travel_time = 2 * sqrt(distance / acceleration)
    let travel_turns = 2 * ((base_distance as f64) / (ship_acceleration as f64)).sqrt() as u32;

    // Ensure at least 1 turn for any non-zero distance
    std::cmp::max(travel_turns, 1)
}

fn advance_planet_positions(planets: &mut Vec<Planet>) {
    for planet in planets.iter_mut() {
        // Advance position by 1, wrap around at orbital period
        planet.position = (planet.position + 1) % planet.orbital_period;
    }
}

// Example: Planet A at radius 5, orbits every 10 turns
//          Planet B at radius 12, orbits every 15 turns
//          Ship acceleration 1 unit/turn²
//          Current positions: A at 0° (pos=0), B at 180° (pos=7)
//
// Base distance = |12 - 5| = 7 units
// Travel time = 2 * sqrt(7/1) = 2 * sqrt(7) ≈ 2 * 2.6 = 5.2 → 5 turns (rounded)
// During those 5 turns, Planet B will move 5 positions in its 15-position orbit
```

The formula travel_turns = 2 * sqrt(base_distance / acceleration) represents:
1. Acceleration phase: sqrt(base_distance / acceleration) - time to accelerate to the midpoint
2. Deceleration phase: sqrt(base_distance / acceleration) - time to decelerate from midpoint to destination

The "2 *" accounts for both phases, assuming that the spacecraft accelerates at a constant rate for half the distance and then decelerates at the same rate for the second half. This is the classic brachistochrone trajectory where the spacecraft flips and fires its engines in the opposite direction halfway through the journey to decelerate.

This turn-based approach maintains the strategic element of orbital mechanics (planning journeys based on planet positions) while using only integer mathematics, making it highly efficient for a browser-based game. Players can see exactly how many turns each journey will take and can plan accordingly.

### Integration Points
- Connect with general gameplay scenario for player positioning between planets
- Interface with market system for trading before/after journeys
- Integrate with ship upgrade system for travel speed and fuel efficiency
- Link to solar system map for visual representation of travel
- Coordinate with commodity system for cargo management during travel

## Consequences

### Positive
- Enables the core strategic challenge of orbital planning
- Supports the trading gameplay loop by connecting planet markets
- Provides meaningful strategic decisions around route planning
- Enhances player agency in navigating the solar system economically

### Negative
- Complexity of simplified orbital mechanics may confuse players
- Turn-based advancement could slow gameplay pace
- Calculating optimal routes might become mathematical rather than intuitive
- Risk of travel becoming tedious if not properly balanced

## References
- [General Gameplay Scenario ADR #0001](./0001-general-gameplay-scenario.md)
- [Project Principles ADR #0000](./0000-project-principles.md)
- [Ship Upgrade System Documentation] (to be created)