## Implement SolarMap Integration into web.rs

Part of Epic #93

### Overview
Integrate the existing SolarMap component into the main App component in `src/ui/web.rs`. The SolarMap component is fully implemented but not currently being used - the UI shows a placeholder instead.

### Background
- **SolarMap component**: `src/ui/solar_map.rs` - fully functional with canvas rendering
- **Current UI**: `src/ui/web.rs` - has placeholder HTML instead of SolarMap
- **Related Bug**: #117 - Planets not visible on solar system map

### Implementation Tasks

#### 1. Import SolarMap Component
Add imports to `src/ui/web.rs`:
```rust
use crate::ui::solar_map::{SolarMap, MapPlanet};
use crate::simulation::planet_types::PlanetType;
use crate::simulation::orbits::Position;
```

#### 2. Create Planet Data
Create `Vec<MapPlanet>` from game state:
- Define hardcoded planet data matching the solar system configuration
- Include: id, name, orbit_radius, orbit_period, position, planet_type
- Planets: Earth, Mars, Jupiter, Mining Station, Research Outpost, etc.

#### 3. Add Reactive State for Selection
Add signals to track:
- `selected_planet: Option<String>` - currently selected planet ID
- Pass these to SolarMap component

#### 4. Replace Placeholder with SolarMap
Replace the placeholder div in `web.rs`:
```rust
// Current (placeholder):
<div class="map-placeholder">
    <div class="sun"></div>
    <p>"Solar system map will be displayed here"</p>
</div>

// Replace with:
<SolarMap
    planets=planets
    current_turn=turn.get()
    player_location=location.get()
    selected_planet=selected_planet.get()
    on_planet_select=Some(Box::new(move |id| set_selected_planet.set(Some(id))))
/>
```

#### 5. Add CSS Styles
Ensure `index.html` has styles for:
- `.solar-map-container` - main container
- `.solar-map-canvas` - the canvas element
- `.map-legend` - planet type legend

These may already exist from PR #113 CSS additions.

#### 6. Wire Up Planet Selection
- When a planet is clicked, update `selected_planet` signal
- Use selection to show planet info in side panel (future enhancement)
- Connect to travel panel for destination selection

### Files to Modify
1. `src/ui/web.rs` - Main integration
2. `index.html` - Add CSS if needed (check if styles from PR #113 exist)

### Technical Notes

**SolarMap Props**:
```rust
pub fn SolarMap(
    planets: Vec<MapPlanet>,
    current_turn: u32,
    player_location: String,
    selected_planet: Option<String>,
    on_planet_select: Option<Box<dyn Fn(String)>>,
) -> impl IntoView
```

**MapPlanet Structure**:
```rust
pub struct MapPlanet {
    pub id: String,
    pub name: String,
    pub orbit_radius: u32,
    pub orbit_period: u32,
    pub position: Position,
    pub planet_type: PlanetType,
}
```

### Acceptance Criteria
- [ ] SolarMap component renders in the map panel (60% left side)
- [ ] Planets displayed at correct orbital positions for current turn
- [ ] Sun rendered at center with glow effect
- [ ] Planets are clickable and trigger selection callback
- [ ] Selected planet shows white selection ring
- [ ] Player location shows green triangle indicator
- [ ] Map legend visible showing planet types
- [ ] Map re-renders when turn changes
- [ ] Star field background visible
- [ ] No placeholder text remains visible

### Testing Steps
1. Build with `trunk build`
2. Open game in browser
3. Verify planets are visible on the map
4. Click a planet - should trigger selection
5. Verify player location indicator is visible
6. Click "Next Turn" - planets should update positions

### Dependencies
- None - SolarMap component is already implemented

### Related
- Bug: #117 
- Epic: #93
- ADR: 0003-web-ui-view.md
- Component: `src/ui/solar_map.rs`
