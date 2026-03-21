# 0011: Travel/Preview Feature for Planet-to-Planet Movement

## Status
Proposed

## Date
2026-03-21

## Deciders
- User
- game-designer

## Context
Currently the game only has a "pass turn" button that advances time but doesn't allow travel between planets. Players need a way to actually move between planets to engage with the trading gameplay loop defined in ADR #0001. The core loop of "buy low on one planet, travel to another, sell high" cannot function without a travel mechanism.

Existing foundations:
- ADR #0002 defines orbital mechanics and travel time calculations using the brachistochrone model
- ADR #0003 specifies a solar system map showing planets in orbital positions with interactive selection
- ADR #0005 defines market/economy system where trading happens at planet locations
- ADR #0006 defines data models including `player.current_planet_id`, planet positions, and orbital data

Current design gap: While ADR #0002 defines travel time calculations and ADR #0003 mentions "interactive visualization: selection of planets should change the display", the specific travel interaction flow, visual path preview, destination prediction, and confirmation mechanics are not defined.

Players need to:
1. See where they can travel from their current location
2. Understand the time cost of each travel option
3. Preview their destination before committing
4. Visualize the journey as time advances
5. Arrive at the destination ready to trade

## Decision
We will implement a travel/preview feature with the following interaction patterns and visual feedback systems:

### Game Design Objectives
- Enable intuitive planet-to-planet travel as the core movement mechanic
- Provide clear visual feedback throughout the travel decision flow
- Integrate seamlessly with orbital mechanics from ADR #0002
- Support strategic route planning with destination preview
- Maintain turn-based pacing without breaking game flow
- MVP simplicity while being intuitive and extensible

### Travel UI Interaction Flow

#### 1. Travel Mode Activation
**Trigger:** Player clicks "Travel" button (replaces or supplements "Pass Turn" button)

**State Change:**
- Enter "travel selection mode"
- Solar system map highlights all reachable planets
- Current planet marked with "You Are Here" indicator
- Travel panel opens showing available options

**Visual Feedback:**
- Current planet glows or pulses with distinct color
- All other planets show subtle highlight indicating they are selectable
- Non-reachable planets (if any restrictions exist) appear dimmed with tooltip explaining why

#### 2. Destination Selection with Visual Path Preview
**Player Action:** Click on a destination planet on the solar system map

**System Response:**
- Draw a graphic line (travel path) from current planet to selected destination
- Display travel time calculation based on ADR #0002 orbital mechanics
- Show destination preview information

**Visual Path Rendering:**
- **Line Style:** Dashed or animated line indicating "planned route"
- **Color Coding:** 
  - Green: Short travel time (1-3 turns)
  - Yellow: Medium travel time (4-7 turns)
  - Orange: Long travel time (8+ turns)
- **Animation:** Subtle pulse animation along the line to indicate direction of travel
- **Multiple Selection:** Player can click different planets to compare options; previous path fades, new path draws

**Travel Time Display:**
- Show calculated travel time using ADR #0002 formula: `travel_turns = 2 * sqrt(base_distance / acceleration)`
- Display format: "Travel Time: 5 turns"
- Include ship acceleration factor: "(at current ship speed: 1 unit/turn²)"
- Show distance reference: "Distance: 12.5 units"

#### 3. Destination Preview (Where You'll Land)
**Critical Feature:** Since planets move during travel (per ADR #0002), show where the destination planet will be when player arrives.

**Preview Display:**
- **Ghost Planet:** Show semi-transparent outline of destination planet at its future position
- **Future Position Label:** "Planet will be here after 5 turns"
- **Orbital Position Indicator:** Show current position vs. arrival position on planet's orbital track
- **Market Preview:** Show what market prices might be at arrival (future enhancement, MVP shows current prices with disclaimer)

**Calculation Logic:**
```
// Pseudo-code for destination prediction
fn predict_destination_position(planet: &Planet, travel_turns: u32, current_turn: u32) -> PlanetPosition {
    let future_turn = current_turn + travel_turns;
    let future_position = (planet.position + travel_turns) % planet.orbital_period;
    let future_angle = (future_position as f64 / planet.orbital_period as f64) * 2.0 * PI;
    PlanetPosition {
        angle: future_angle,
        radius: planet.orbital_radius,
        x: planet.orbital_radius * cos(future_angle),
        y: planet.orbital_radius * sin(future_angle),
    }
}
```

**Visual Representation:**
- Current planet position: Solid circle with planet icon
- Future planet position: Ghosted/dashed circle with planet icon
- Orbital path: Faint line showing planet's orbital track
- Turn indicator: Small markers along orbital track showing turns 1, 2, 3... to arrival

#### 4. Travel Confirmation
**Player Action:** Click "Confirm Travel" button after selecting destination

**Confirmation Modal:**
```
┌─────────────────────────────────────────┐
│  CONFIRM TRAVEL                         │
├─────────────────────────────────────────┤
│  Destination: Mars                      │
│  Travel Time: 5 turns                   │
│  Distance: 12.5 units                   │
│                                         │
│  You will arrive on turn 15             │
│  Mars will be at orbital position 7     │
│                                         │
│  [Cancel]          [Depart]             │
└─────────────────────────────────────────┘
```

**Confirmation Requirements:**
- Must explicitly confirm before travel begins (no accidental travel)
- Show all critical information before commitment
- Cancel returns to travel selection mode
- Depart initiates travel sequence

#### 5. Time Advancement Visualization (Travel Animation)
**Player Action:** Click "Depart" on confirmation modal

**Animation Sequence:**
1. **Ship Icon Movement:** Player ship icon travels along the drawn path from current planet to destination
2. **Turn Counter Display:** Show turn advancement: "Turn 10 → 11 → 12 → 13 → 14 → 15"
3. **Planet Position Updates:** All planets advance in their orbits in real-time during animation
4. **Speed Control:** Animation plays at accelerated speed (1 turn per 0.5 seconds)
5. **Progress Indicator:** "Traveling... 3/5 turns completed"

**Animation Implementation:**
```
// Pseudo-code for travel animation
fn animate_travel(current_pos: Position, dest_pos: Position, travel_turns: u32) {
    let animation_duration = travel_turns * 500ms; // 500ms per turn
    let frames = 60; // Smooth animation
    
    for frame in 0..frames {
        let progress = frame as f64 / frames as f64;
        let current_turn = current_turn + (progress * travel_turns as f64) as u32;
        
        // Interpolate ship position
        ship.x = lerp(current_pos.x, dest_pos.x, progress);
        ship.y = lerp(current_pos.y, dest_pos.y, progress);
        
        // Update all planet positions
        advance_planet_positions(current_turn);
        
        // Render frame
        render();
    }
}
```

**Visual Feedback During Travel:**
- Ship icon moves smoothly along path
- Path line changes from dashed to solid, following ship
- Turn counter increments visibly
- Planets move along orbital tracks
- Optional: Starfield background scrolls for motion effect

**Skip Animation Option (MVP Future Enhancement):**
- "Skip to Arrival" button for players who want instant travel
- Still processes all game state updates
- Only skips visual animation

#### 6. Arrival and State Updates
**On Animation Complete:**
1. **Location Update:** `player.current_planet_id = destination_planet.id`
2. **Turn Update:** `game_state.current_turn += travel_turns`
3. **Planet Positions:** All planets advanced by `travel_turns` per ADR #0002
4. **Market Updates:** Market prices recalculated per ADR #0005 (turn-based fluctuations)
5. **UI Update:** Exit travel mode, return to normal view with market panel for new planet

**Arrival Notification:**
- Toast message: "Arrived at Mars on turn 15"
- Market panel opens automatically (if trading is intended next action)
- Ship status panel shows any changes (fuel consumption if implemented)

**State Persistence:**
- Save complete game state to localStorage per ADR #0006
- Include new player position, turn count, planet positions, market data

### Visual Path Rendering Specifications

**Technical Implementation (SVG/Canvas per ADR #0003):**
```
// SVG path rendering example
<svg class="solar-system-map">
  <!-- Orbital tracks -->
  <circle class="orbit-track" cx="400" cy="300" r="100" />
  <circle class="orbit-track" cx="400" cy="300" r="150" />
  
  <!-- Planets at current positions -->
  <circle class="planet current" cx="500" cy="300" r="10" />
  <circle class="planet destination" cx="400" cy="150" r="8" />
  
  <!-- Ghost planet at future position -->
  <circle class="planet ghost" cx="350" cy="180" r="8" stroke-dasharray="4,4" opacity="0.5" />
  
  <!-- Travel path -->
  <line class="travel-path" x1="500" y1="300" x2="400" y2="150" stroke="#4CAF50" stroke-width="2" stroke-dasharray="5,5">
    <animate attributeName="stroke-dashoffset" from="0" to="10" dur="1s" repeatCount="indefinite" />
  </line>
  
  <!-- Ship icon -->
  <g class="ship" transform="translate(450, 225)">
    <!-- Ship SVG icon -->
  </g>
</svg>
```

**CSS Styling:**
```css
.travel-path {
  stroke: var(--travel-path-color);
  stroke-width: 3;
  stroke-dasharray: 8, 4;
  animation: dash-flow 1s linear infinite;
}

.planet.ghost {
  fill: transparent;
  stroke: var(--planet-color);
  stroke-dasharray: 4, 4;
  opacity: 0.5;
}

.ship {
  transition: transform 0.5s ease-out;
}

@keyframes dash-flow {
  to { stroke-dashoffset: -12; }
}
```

### Travel Time Calculation Integration

**Direct Integration with ADR #0002:**
```rust
// Rust implementation using ADR #0002 formula
pub fn calculate_travel_time(departure: &Planet, destination: &Planet, ship_acceleration: f64) -> u32 {
    let base_distance = (destination.orbital_radius as f64 - departure.orbital_radius as f64).abs();
    let travel_turns = 2.0 * (base_distance / ship_acceleration).sqrt();
    travel_turns.ceil() as u32 // Round up to ensure at least partial turn counts
}

pub fn predict_arrival_turn(current_turn: u32, travel_turns: u32) -> u32 {
    current_turn + travel_turns
}

pub fn predict_planet_position(planet: &Planet, turns_ahead: u32) -> PlanetPosition {
    let future_position = (planet.position + turns_ahead) % planet.orbital_period;
    let angle = (future_position as f64 / planet.orbital_period as f64) * 2.0 * std::f64::consts::PI;
    PlanetPosition {
        angle,
        x: planet.orbital_radius as f64 * angle.cos(),
        y: planet.orbital_radius as f64 * angle.sin(),
    }
}
```

### Error Handling

**Error Types and Player Feedback:**

| Error | When | Player Message | UI Response |
|-------|------|----------------|-------------|
| Same Planet Selected | Player clicks current planet | "You are already at this planet" | Disable selection, show tooltip |
| Invalid Destination | Destination not reachable (future: fuel limits) | "Cannot travel to this planet: [reason]" | Dim planet, show lock icon |
| Travel Interrupted | Player tries to cancel mid-travel | "Travel in progress. Cannot cancel." | Disable cancel during animation |
| Data Corruption | Planet ID not found in game state | "Navigation error. Please reload game." | Show error modal, offer reset |
| Calculation Overflow | Travel time exceeds reasonable limits | "Destination too far. Upgrade ship." | Cap at max, show warning |

**Validation Logic:**
```rust
pub fn validate_travel(player: &Player, destination_id: PlanetId, game_state: &GameState) -> Result<(), TravelError> {
    if player.current_planet_id == destination_id {
        return Err(TravelError::SamePlanet);
    }
    
    let destination = game_state.planets.get(&destination_id)
        .ok_or(TravelError::InvalidDestination)?;
    
    // Future: Check fuel, check ship capability, etc.
    
    Ok(())
}
```

### State Updates During Travel Flow

**Complete State Transition:**
```
1. PRE-TRAVEL STATE:
   - player.current_planet_id = planet_a
   - game_state.current_turn = 10
   - planets[].position = current positions
   - markets[].prices = current prices

2. TRAVEL SELECTION STATE:
   - travel_mode = true
   - selected_destination = planet_b
   - travel_path = drawn on map
   - travel_time = calculated

3. TRAVEL ANIMATION STATE:
   - ship.position = interpolating
   - animation_playing = true
   - turn_counter = animating 10 → 15

4. POST-TRAVEL STATE:
   - player.current_planet_id = planet_b
   - game_state.current_turn = 15
   - planets[].position = advanced by 5 turns
   - markets[].prices = recalculated
   - travel_mode = false
   - saved to localStorage
```

### MVP Simplicity Decisions

**What's Included (MVP):**
- Single planet-to-planet travel
- Visual path preview with travel time
- Destination ghost preview
- Turn-based animation
- Basic confirmation modal
- State updates per ADR #0006

**What's Deferred (Future Enhancement):**
- Multi-stop route planning
- Fuel consumption mechanics
- Travel interruptions/random encounters
- "Skip animation" option
- Market price prediction at arrival
- Travel history log
- Quick-travel favorites
- Auto-pilot to best trade destination

### Accessibility Considerations

**Visual Accessibility:**
- High contrast path lines (color + pattern)
- Color-blind friendly path indicators (patterns + colors)
- Clear text labels for travel time and destination
- Minimum planet size for click targets (44x44px)

**Interaction Accessibility:**
- Keyboard navigation between planets
- Enter key confirms selection
- Escape cancels travel mode
- Screen reader announcements for travel status
- No time pressure on travel decisions

**Motion Sensitivity:**
- Option to disable travel animation (future enhancement)
- Instant travel mode for players who prefer no animation
- Reduced motion still shows state updates

## Consequences

### Positive
- Enables core trading gameplay loop (buy low, travel, sell high)
- Visual path preview helps players understand travel commitments
- Destination ghost preview teaches orbital mechanics intuitively
- Turn-based animation reinforces time advancement concept
- Clear confirmation prevents accidental travel
- Integration with ADR #0002 maintains mechanical consistency
- State updates work seamlessly with existing data models

### Negative
- Animation may feel slow for experienced players (mitigated by future skip option)
- Ghost planet visualization adds visual complexity to map
- Travel confirmation adds extra clicks to common action
- Orbital prediction calculation adds computational overhead (minimal for MVP scale)
- May require tutorial or onboarding to explain ghost planet concept

## References
- [General Gameplay Scenario ADR #0001](./0001-general-gameplay-scenario.md)
- [Movement Mechanics System ADR #0002](./0002-movement-mechanics-system.md)
- [Web UI View ADR #0003](./0003-web-ui-view.md)
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Market Trading UI Interaction ADR #0009](./0009-market-trading-ui-interaction.md)
