# Bug Investigation: Market Panel Doesn't Update When Planet is Selected

**Date:** 2026-03-21
**Investigator:** Qwen Playtester
**Status:** Root Cause Identified
**Related Bug:** BUG-INVESTIGATION-001.md (Planet Selection and Next Turn bugs)

---

## Issue Summary

**Bug Title:** Market panel doesn't update when a planet is selected on the map

**Severity:** Critical - Core gameplay feature broken

**Description:**
When a player clicks on a planet in the solar system map, the market panel on the right side of the screen should update to show that planet's market data (commodity prices, supply/demand information). Currently, the market panel displays static hardcoded data for "Earth" and never updates regardless of which planet is selected.

---

## Steps to Reproduce

1. Start the game at http://localhost:8080
2. Wait for WASM to load (approximately 5 seconds)
3. Observe the market panel on the right side - it shows "Earth" with static commodity prices
4. Click on any planet in the solar system map canvas
5. Observe the market panel

**Expected Behavior:**
- Market panel subtitle should update to show the selected planet's name
- Market prices should update to reflect the selected planet's economy
- Different planets should show different prices based on their planet type (e.g., Mining planets should have cheaper Metals)

**Actual Behavior:**
- Market panel always shows "Earth" regardless of selection
- Prices remain static (Water: $10/$8, Food: $25/$20, Ore: $50/$40, Electronics: $100/$80)
- No connection between planet selection and market display

---

## Test Evidence

### Automated Test Results

```
=== Market Panel Bug Test ===

Initial market panel state: {
  "subtitle": "Earth",
  "rowCount": 4,
  "firstRowContent": "Water$10$8"
}

Market panel after canvas click: {
  "subtitle": "Earth",
  "rowCount": 4,
  "firstRowContent": "Water$10$8"
}

Reactivity check: {
  "marketPanelExists": true,
  "playerLocationExists": true,
  "playerLocationText": "earth",
  "marketPanelSubtitle": "Earth",
  "locationsMatch": false  // ← BUG: Should match but doesn't update reactively
}

Bug confirmed: true
```

### Visual Evidence

The market panel HTML structure shows hardcoded data:

```html
<div class="panel market-panel">
    <div class="panel-header">
        <h3>"Market"</h3>
        <span class="panel-subtitle">"Earth"</span>  <!-- Static text, not reactive -->
    </div>
    <div class="panel-content">
        <div class="market-table">
            <div class="market-header">
                <span>"Item"</span>
                <span>"Buy"</span>
                <span>"Sell"</span>
            </div>
            <!-- Hardcoded rows, not generated from game state -->
            <div class="market-row">
                <span>"Water"</span>
                <span class="buy-price">"$10"</span>
                <span class="sell-price">"$8"</span>
            </div>
            ...
        </div>
    </div>
</div>
```

---

## Root Cause Analysis

### Primary Issue: Hardcoded Market Data

**Location:** `src/ui/web.rs` lines 192-224

**Code:**
```rust
// Market Panel - Lines 192-224
<div class="panel market-panel">
    <div class="panel-header">
        <h3>"Market"</h3>
        <span class="panel-subtitle">"Earth"</span>  // ❌ Hardcoded string literal
    </div>
    <div class="panel-content">
        <div class="market-table">
            <div class="market-header">
                <span>"Item"</span>
                <span>"Buy"</span>
                <span>"Sell"</span>
            </div>
            <div class="market-row">
                <span>"Water"</span>
                <span class="buy-price">"$10"</span>  // ❌ Hardcoded prices
                <span class="sell-price">"$8"</span>
            </div>
            <div class="market-row">
                <span>"Food"</span>
                <span class="buy-price">"$25"</span>  // ❌ Hardcoded prices
                <span class="sell-price">"$20"</span>
            </div>
            <div class="market-row">
                <span>"Ore"</span>
                <span class="buy-price">"$50"</span>  // ❌ Hardcoded prices
                <span class="sell-price">"$40"</span>
            </div>
            <div class="market-row">
                <span>"Electronics"</span>
                <span class="buy-price">"$100"</span>  // ❌ Hardcoded prices
                <span class="sell-price">"$80"</span>
            </div>
        </div>
    </div>
</div>
```

**Problem:**
The market panel is implemented as static HTML with hardcoded values. It:
1. Does not read from the game state's solar system data
2. Does not track the `selected_planet` signal
3. Does not access the `PlanetEconomy` system that calculates dynamic prices
4. Is completely disconnected from the reactive data flow

### Secondary Issue: No Market Panel Component

**Location:** `src/ui/mod.rs`

The UI module structure shows:
```rust
pub mod cli;
pub mod game_config_modal;
pub mod solar_map;
pub mod travel_panel;

#[cfg(feature = "web")]
pub mod web;
```

There is **no `market_panel` module**. The market display should be extracted into its own component (similar to `travel_panel.rs`) that:
1. Accepts a planet ID or planet data as a prop
2. Reads the planet's economy from the game state
3. Dynamically renders commodity prices using the `PlanetEconomy` system

### Missing Connection: Selected Planet → Market Data

**Signal Flow (Current - Broken):**
```
User clicks planet on canvas
  → on_planet_select callback fires
  → set_selected_planet.set(Some(id)) updates signal
  → ❌ Market panel doesn't listen to selected_planet signal
  → ❌ Market panel shows static data
```

**Signal Flow (Expected - Fixed):**
```
User clicks planet on canvas
  → on_planet_select callback fires
  → set_selected_planet.set(Some(id)) updates signal
  → Market panel receives selected_planet signal
  → Market panel queries PlanetEconomy for selected planet
  → Market panel re-renders with updated prices
```

### Related Issues from BUG-INVESTIGATION-001.md

The planet selection bug identified in BUG-INVESTIGATION-001.md compounds this issue:
- Planet selection itself is broken (clicking doesn't select planets)
- Even if the market panel were reactive, it wouldn't receive selection events

Both bugs must be fixed together for the feature to work.

---

## Code Analysis

### Available Economy System (Working)

The economy system in `src/simulation/economy.rs` is fully implemented and working:

```rust
/// Represents the economy of a single planet/station
pub struct PlanetEconomy {
    pub planet_type: PlanetType,
    pub market: HashMap<CommodityType, MarketGood>,
    pub active_events: Vec<MarketEvent>,
}

impl PlanetEconomy {
    /// Get the buy price for a specific commodity
    pub fn get_buy_price(&self, commodity_type: &CommodityType) -> Option<u32>
    
    /// Get the sell price for a specific commodity
    pub fn get_sell_price(&self, commodity_type: &CommodityType) -> Option<u32>
    
    /// Get market summary for UI display
    pub fn get_market_summary(&self) -> MarketSummary
}
```

The `Planet` struct in `src/game_state.rs` includes an economy field:

```rust
pub struct Planet {
    pub id: String,
    pub name: String,
    pub orbit_radius: u32,
    pub orbit_period: u32,
    pub position: Position,
    pub economy: PlanetEconomy,  // ← Economy data is available!
    pub planet_type: PlanetType,
}
```

### Game State Access (Available)

The `GameState` struct provides methods to access planet economies:

```rust
impl GameState {
    pub fn get_current_planet(&self) -> Option<&Planet>
    pub fn get_current_planet_mut(&mut self) -> Option<&mut Planet>
}
```

### Missing: Reactive Connection in Web UI

The `App` component in `src/ui/web.rs` has:
- `selected_planet` signal (line 18)
- `location` signal (line 15)
- Access to planet data via the `planets` vector (lines 24-91)

But the market panel does NOT use any of these signals. It's completely static.

---

## Recommended Fix

### Fix 1: Create Market Panel Component

Create `src/ui/market_panel.rs` similar to `travel_panel.rs`:

```rust
//! Market Panel Component
//! Displays commodity prices for a selected planet

#[cfg(feature = "web")]
use leptos::*;
#[cfg(feature = "web")]
use crate::game_state::Planet;
#[cfg(feature = "web")]
use crate::simulation::commodity::CommodityType;

#[component]
pub fn MarketPanel(
    planet: Option<Planet>,  // Selected planet
) -> impl IntoView {
    view! {
        <div class="panel market-panel">
            <div class="panel-header">
                <h3>"Market"</h3>
                <span class="panel-subtitle">
                    {move || planet.as_ref().map(|p| p.name.clone()).unwrap_or_else(|| "Select a planet".to_string())}
                </span>
            </div>
            <div class="panel-content">
                {move || match &planet {
                    Some(planet) => {
                        // Get economy data and render market table
                        let commodities = CommodityType::all();
                        view! {
                            <div class="market-table">
                                <div class="market-header">
                                    <span>"Item"</span>
                                    <span>"Buy"</span>
                                    <span>"Sell"</span>
                                </div>
                                {commodities.into_iter().map(|commodity| {
                                    let sell_price = planet.economy.get_sell_price(&commodity).unwrap_or(0);
                                    let buy_price = planet.economy.get_buy_price(&commodity).unwrap_or(0);
                                    view! {
                                        <div class="market-row">
                                            <span>{commodity.display_name()}</span>
                                            <span class="buy-price">{format!("${}", buy_price)}</span>
                                            <span class="sell-price">{format!("${}", sell_price)}</span>
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        }
                    }
                    None => view! {
                        <div class="market-no-selection">
                            <p>"Select a planet to view its market"</p>
                        </div>
                    }
                }}
            </div>
        </div>
    }
}
```

### Fix 2: Update App Component to Use Market Panel

In `src/ui/web.rs`, replace the hardcoded market panel (lines 192-224) with the new component:

```rust
// First, import the component
use crate::ui::market_panel::MarketPanel;

// Then replace the hardcoded HTML with:
<MarketPanel 
    planet={move || {
        let selected_id = selected_planet.get();
        planets.iter()
            .find(|p| Some(p.id.clone()) == selected_id)
            .cloned()
    }}
/>
```

### Fix 3: Ensure Planet Selection Works

The planet selection bug from BUG-INVESTIGATION-001.md must also be fixed:
- Update `SolarMap` component to properly track `selected_planet` signal
- Ensure click handlers on the canvas trigger selection
- Verify visual feedback appears when planets are selected

### Fix 4: Use Player Location as Default

When no planet is explicitly selected, show the market for the player's current location:

```rust
<MarketPanel 
    planet={move || {
        let selected_id = selected_planet.get();
        let location = location.get();
        
        // Use selected planet, or fall back to player location
        let planet_id = selected_id.unwrap_or(location);
        
        planets.iter()
            .find(|p| p.id == planet_id)
            .cloned()
    }}
/>
```

---

## Files to Modify

| File | Change Type | Description |
|------|-------------|-------------|
| `src/ui/market_panel.rs` | **Create** | New market panel component |
| `src/ui/mod.rs` | Modify | Add `pub mod market_panel;` |
| `src/ui/web.rs` | Modify | Replace hardcoded market panel with component |
| `src/ui/solar_map.rs` | Modify | Fix planet selection (see BUG-INVESTIGATION-001.md) |

---

## Verification Steps

After applying fixes:

1. **Build:** `cargo build --features web`
2. **Serve:** `trunk serve --port 8080`
3. **Test:**
   - Open http://localhost:8080
   - Market panel should show Earth's market by default (player's starting location)
   - Click on Mars → Market panel should update to show Mars's prices
   - Click on Jupiter → Market panel should update to show Jupiter's prices
   - Prices should differ based on planet type (Mining vs Agricultural vs MegaCity)

4. **Automated Test:** Run `node test-market-panel-bug.js`
   - Expected: `locationsMatch: true` when planet is selected
   - Expected: Market panel subtitle changes to match selected planet

---

## Additional Findings

### Economy System is Fully Functional

The economy system in `src/simulation/economy.rs` is complete with:
- Dynamic pricing based on supply/demand
- Planet-type-specific multipliers
- Market events and fluctuations
- Price history tracking
- Trade volume tracking

This system is **not being used** by the web UI's market panel.

### CLI Has Working Market Display

The CLI UI in `src/ui/cli.rs` has a working `display_market_status()` function that correctly reads from `PlanetEconomy`. This can serve as a reference implementation.

### Commodity Data Available

The game has 10 commodity types defined in `src/simulation/commodity.rs`:
- Water, Foodstuffs, Medicine, Firearms, Ammunition
- Metals, Antimatter, Electronics, Narcotics, Alien Artefacts

The hardcoded market panel only shows 4 items with wrong names (e.g., "Ore" instead of "Metals").

---

## Impact

- **Severity:** Critical
- **User Impact:** Players cannot see market prices for different planets, making trading impossible
- **Scope:** Affects all trading mechanics, which are core to the gameplay loop
- **Related Bugs:** Compounded by planet selection bug (BUG-INVESTIGATION-001.md)

---

## Conclusion

The market panel bug is caused by **hardcoded static HTML** that is completely disconnected from the game's reactive state and economy system. The fix requires:

1. Creating a new `MarketPanel` component that reads from `PlanetEconomy`
2. Connecting the component to the `selected_planet` signal
3. Fixing the underlying planet selection mechanism

The economy infrastructure is already built and working - it just needs to be connected to the UI.
