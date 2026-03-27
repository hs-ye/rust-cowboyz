# Bug Investigation: Market Panel Shows Same Prices for All Planets

## Investigation Date
2026-03-21

## Investigator
Qwen (Playtester Agent)

## Summary
All planets display identical prices in the market panel, despite ADR 0005 specifying that different planet types should have different supply/demand patterns affecting prices.

---

## Expected Behavior (Per ADR 0005)

According to `design/adr/0005-market-economy-system.md`, each planet type should have unique economic characteristics:

### Planet Type Economic Profiles

| Planet Type | Supplies (Lower Prices) | Demands (Higher Prices) |
|-------------|------------------------|-------------------------|
| **Agricultural** | Water, Foodstuffs | Medicine, Firearms, Ammunition, Electronics |
| **Mega City** | Electronics, Medicine, Narcotics | Water, Foodstuffs, Firearms, Ammunition |
| **Mining** | Metals, Antimatter, Electronics | Water, Foodstuffs, Medicine, Ammunition |
| **Pirate Station** | Narcotics, Ammunition | Foodstuffs, Firearms, Medicine |
| **Research Outpost** | Electronics, Medicine, Alien Artefacts | Water, Foodstuffs |
| **Industrial** | Electronics, Metals, Ammunition, Antimatter | Water, Foodstuffs, Medicine |
| **Frontier Colony** | Water, Foodstuffs | Medicine, Firearms, Ammunition, Electronics, Metals, Antimatter, Alien Artefacts |

### Pricing Formula (ADR 0005)
```
Current Price = Base Price × Local Multiplier × Supply Factor × Demand Factor
```

Where:
- **Local Multiplier**: 0.7 for supplied goods, 1.3 for demanded goods, 1.0 for ignored goods
- **Supply Factor**: Higher supply = lower price (inverted in formula)
- **Demand Factor**: Higher demand = higher price

### Expected Price Variations Example

For **Water** (Base Price: 10):
- **Agricultural Planet**: Should be ~5-7 (supplied, low price)
- **Mining Planet**: Should be ~13-16 (demanded, high price)
- **Pirate Station**: Should be ~10 (ignored, base price)

For **Electronics** (Base Price: 50):
- **Mega City**: Should be ~35 (supplied)
- **Agricultural Planet**: Should be ~65 (demanded)
- **Pirate Station**: Should be ~50 (ignored)

---

## Actual Behavior

All planets display **identical prices** regardless of planet type. When selecting different planets in the UI:
- Mercury (Mining) shows same prices as Earth (Agricultural)
- Jupiter (Mega City) shows same prices as Mars (Mining)
- No price variation based on planet specialization

---

## Root Cause Analysis

### Location of Bug
**File**: `src/ui/web.rs`  
**Component**: `App()` function  
**Lines**: ~100-150 (MarketPanelReactive callbacks)

### Technical Analysis

The bug is caused by **how the reactive callbacks capture and use planet data**. Let me trace through the code:

#### 1. Planet Data Structure (CORRECT)
In `src/ui/web.rs`, planets are created with unique economies:
```rust
let planets: Vec<Planet> = vec![
    Planet {
        id: "mercury".to_string(),
        planet_type: PlanetType::Mining,
        economy: PlanetEconomy::new(PlanetType::Mining), // ✓ Unique economy
    },
    Planet {
        id: "earth".to_string(),
        planet_type: PlanetType::Agricultural,
        economy: PlanetEconomy::new(PlanetType::Agricultural), // ✓ Unique economy
    },
    // ... etc
];
```

Each planet correctly has its own `PlanetEconomy` with planet-specific prices.

#### 2. Market Panel Callbacks (PROBLEMATIC)
The `MarketPanelReactive` component receives three callbacks:

```rust
get_economy={Box::new({
    let planets_clone = planets.clone();
    move || {
        let selected_id = selected_planet.get();
        let location_id = location.get();
        let planet_id = selected_id.unwrap_or(location_id);

        planets_clone.iter()
            .find(|p| p.id == planet_id)
            .map(|p| p.economy.clone())
            .unwrap_or_else(|| PlanetEconomy::new(PlanetType::Agricultural))
    }
})}
```

**The Issue**: The callbacks use `.get()` on signals (`selected_planet.get()`, `location.get()`) but these are **not reactive signals in the Leptos sense** - they're being called inside a `move ||` closure that's passed as a plain `Box<dyn Fn()>`.

#### 3. The Real Problem: Signal Reactivity

Looking at `src/ui/market_panel.rs`:

```rust
pub fn MarketPanelReactive(
    get_planet_name: Box<dyn Fn() -> String>,
    get_planet_type: Box<dyn Fn() -> PlanetType>,
    get_economy: Box<dyn Fn() -> PlanetEconomy>,
) -> impl IntoView {
    // ...
    view! {
        // ...
        <span class="panel-subtitle">{move || get_planet_name()}</span>
        // ...
        {
            commodities.into_iter().map(move |commodity| {
                let economy = get_economy();  // ← Called during rendering
                // ...
            })
        }
    }
}
```

The callbacks are called **during rendering**, but they're not wrapped in Leptos `Signal` or `ReadSignal` types. This means:

1. The callbacks **do capture** the correct planet data initially
2. However, when a planet is clicked, the `selected_planet` signal updates
3. The `MarketPanelReactive` component **does not re-render** because the callbacks themselves haven't changed - they're the same `Box<dyn Fn()>` instances
4. Even if it did re-render, the closure captures `planets_clone` by value, and each planet's economy is **cloned at creation time**

#### 4. The Core Issue: Cloning Economics

Each `PlanetEconomy` is **cloned** when the callback returns it:
```rust
.map(|p| p.economy.clone())
```

This means:
- The economy data is **snapshotted** when the callback is created
- Changes to planet selection don't trigger re-evaluation with fresh data
- The market panel shows the **initial** economy (likely Earth's Agricultural economy)

### Why All Planets Show The Same Prices

The most likely scenario:

1. On initial render, `selected_planet` is `None`
2. The callback falls back to `location.get()` which returns `"earth"`
3. Earth's economy (Agricultural) is cloned and displayed
4. When clicking other planets:
   - `selected_planet` signal updates
   - But the market panel doesn't properly react to this change
   - OR the callback still returns Earth's economy due to closure capture issues

---

## Evidence from Code

### Economy System Works Correctly
From `src/simulation/economy.rs`:
```rust
fn calculate_local_multiplier(planet_type: &PlanetType, commodity: &CommodityType) -> f64 {
    let supplies = planet_type.supplies();
    let demands = planet_type.demands();

    if supplies.contains(commodity) {
        0.7  // ✓ Lower price for supplied goods
    } else if demands.contains(commodity) {
        1.3  // ✓ Higher price for demanded goods
    } else {
        1.0  // ✓ Base price for ignored goods
    }
}
```

The economy calculation logic is **correct**.

### Test Confirms Economics Should Differ
From `src/ui/market_panel.rs` tests:
```rust
#[test]
fn test_different_planet_types_have_different_prices() {
    let agricultural_economy = PlanetEconomy::new(PlanetType::Agricultural);
    let mining_economy = PlanetEconomy::new(PlanetType::Mining);
    
    let ag_water_sell = agricultural_economy.get_sell_price(&CommodityType::Water).unwrap();
    let mining_water_sell = mining_economy.get_sell_price(&CommodityType::Water).unwrap();
    
    assert!(ag_water_sell < mining_water_sell, 
        "Agricultural planets should have cheaper Water");
}
```

The tests **pass**, proving the economy system works. The bug is in the **UI integration**.

---

## Solution

### Fix Required: Proper Reactive Signals

The `MarketPanelReactive` component needs to accept **Leptos signals** instead of plain closures:

```rust
#[component]
pub fn MarketPanelReactive(
    selected_planet: ReadSignal<Option<String>>,
    player_location: ReadSignal<String>,
    planets: Vec<Planet>, // Or a signal containing planets
) -> impl IntoView {
    // Use create_memo to reactively compute the current economy
    let current_economy = create_memo(move |_| {
        let selected_id = selected_planet.get();
        let location_id = player_location.get();
        let planet_id = selected_id.unwrap_or(location_id);
        
        planets.iter()
            .find(|p| p.id == planet_id)
            .map(|p| p.economy.clone())
            .unwrap_or_else(|| PlanetEconomy::new(PlanetType::Agricultural))
    });
    
    view! {
        // ...
        {
            commodities.into_iter().map(move |commodity| {
                let economy = current_economy.get(); // ← This will now trigger re-renders
                // ...
            })
        }
    }
}
```

### Alternative Fix: Pass Planet ID Directly

Simpler approach - pass the planet ID and look up economy internally:

```rust
#[component]
pub fn MarketPanel(
    planet_id: String,
    planets: Vec<Planet>,
) -> impl IntoView {
    let economy = planets.iter()
        .find(|p| p.id == planet_id)
        .map(|p| p.economy.clone())
        .unwrap_or_else(|| PlanetEconomy::new(PlanetType::Agricultural));
    
    // ... render with this economy
}
```

Then in `App()`:
```rust
let current_planet_id = create_memo(move |_| {
    selected_planet.get().unwrap_or(location.get())
});

<MarketPanel 
    planet_id={move || current_planet_id.get()}
    planets={planets.clone()}
/>
```

---

## Impact

### Gameplay Impact
- **High**: Removes core strategic depth from the trading game
- Players cannot identify profitable trade routes
- Planet specialization (core ADR 0005 feature) is invisible
- Trading becomes random rather than strategic

### Affected Features
1. Market price display for all planets
2. Trade route planning
3. Planet type specialization visibility
4. Economic strategy gameplay

---

## Files Involved

| File | Role | Status |
|------|------|--------|
| `src/ui/web.rs` | UI component with bug | **Needs Fix** |
| `src/ui/market_panel.rs` | Market panel component | Working correctly |
| `src/simulation/economy.rs` | Economy calculations | Working correctly |
| `src/simulation/planet_types.rs` | Planet type definitions | Working correctly |
| `design/adr/0005-market-economy-system.md` | Design specification | Reference |

---

## Verification Steps

After fix:
1. Start game and observe Earth's market prices (Agricultural)
2. Click on Mercury (Mining) - should see different prices:
   - Water should be MORE expensive on Mercury
   - Metals should be CHEAPER on Mercury
3. Click on Jupiter (Mega City) - should see different prices:
   - Electronics should be CHEAPER
   - Water/Food should be MORE expensive
4. Verify price differences match ADR 0005 specifications

---

## Related Issues

This bug may be related to how other reactive components handle signal updates in the web UI. Similar patterns should be checked in:
- `SolarMap` component
- `TravelPanel` component
- Any component using `create_signal` and callbacks
