# 0008: Ship Types System - Technical Implementation

## Status
Accepted

## Date
2026-03-13

## Deciders
- User
- software-architect

## Context
Game Design ADR #0007 defines three ship archetypes (Rustbucket Sloop, Prairie Brigantine, Iron Galleon) each with distinct `acceleration`, `cargo_capacity`, and `purchase_cost`. This ADR details the technical implementation.

### Current State
- `src/player/ship.rs` defines a `Ship` struct with `speed: f64`, `acceleration: u32`, `cargo_capacity: u32`. The struct is hardcoded via `Ship::new(10.0, 10)` in `Player::new()`.
- `src/game_state.rs` has a parallel `Ship` definition that also includes `fuel` and `hull` fields (more complete).
- `src/player/mod.rs` has a `Player::new()` that hardcodes starting values.
- `data/config/` uses YAML files (`planets.yaml`, `goods.yaml`) for game data — the pattern for config-driven data is already established.
- `src/ui/game_config_modal.rs` demonstrates the existing pattern for a Leptos modal component that collects configuration before game start.
- No ship type definitions exist anywhere in the codebase.

### Key Constraints
- The `acceleration` field on `Ship` directly feeds into `calculate_travel_turns()` in `src/simulation/travel.rs`. Changing acceleration changes travel time.
- The `cargo_capacity` field on both `Ship` and `CargoHold` must remain consistent — both currently default to 10.
- The tech stack is Leptos + WASM (ADR #0004). Any new UI component must follow the existing `#[component]` + `RwSignal` pattern.
- The project uses YAML config files for game data — ship type definitions should follow this pattern.

## Decision

### 1. New YAML Config: `data/config/ships.yaml`

Add a new config file defining all available ship types. This follows the existing pattern of `planets.yaml` and `goods.yaml`.

```yaml
# data/config/ships.yaml
# Ship type definitions - Based on ADR 0007: Ship Types System
# All ships share the same commodity cargo model (1 unit of cargo = 1 commodity slot)

- id: sloop
  name: Rustbucket Sloop
  flavour: "Fast and lean. If the cargo bay is small, the margins had better be fat."
  acceleration: 3        # units/turn² - used in Brachistochrone travel formula
  cargo_capacity: 8      # commodity slots
  purchase_cost: 200     # deducted from starting gold (default 1000)

- id: brigantine
  name: Prairie Brigantine
  flavour: "The working trader's ship. Does everything well enough."
  acceleration: 1        # units/turn²
  cargo_capacity: 15     # commodity slots
  purchase_cost: 0       # free starting ship

- id: galleon
  name: Iron Galleon
  flavour: "Slow as a comet, rich as a king — if you time it right."
  acceleration: 1        # units/turn²
  cargo_capacity: 30     # commodity slots
  purchase_cost: 400     # deducted from starting gold
```

### 2. New Rust Struct: `ShipType`

Add a `ShipType` data structure, plus a loader, in the `src/player/` module. A new file `src/player/ship_types.rs` will contain:

```rust
// src/player/ship_types.rs
use serde::{Deserialize, Serialize};

/// A ship type definition loaded from ships.yaml
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShipTypeDefinition {
    pub id: String,
    pub name: String,
    pub flavour: String,
    pub acceleration: u32,
    pub cargo_capacity: u32,
    pub purchase_cost: u32,
}

/// All available ship types (loaded at startup)
pub fn default_ship_types() -> Vec<ShipTypeDefinition> {
    // In WASM context, embed the YAML at compile time
    let yaml = include_str!("../../data/config/ships.yaml");
    serde_yaml::from_str(yaml).expect("Failed to parse ships.yaml")
}
```

**Why `include_str!`?** The WASM target does not have a filesystem at runtime. The existing pattern for planets and goods loads from config files via a `setup` module at startup, but ships need to be available before game init (to show the selection screen). Embedding at compile time is the safest approach for WASM and follows the established WASM constraint.

### 3. Changes to `Ship` Struct — Add `ship_type_id`

Extend the `Ship` struct in `src/player/ship.rs` to record which ship type was selected. This supports future features (display, validation) and is a minimal, non-breaking addition:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub ship_type_id: String,    // NEW: e.g. "sloop", "brigantine", "galleon"
    pub speed: f64,              // Legacy field (kept for compatibility)
    pub acceleration: u32,
    pub cargo_capacity: u32,
}

impl Ship {
    /// Create a ship from a ShipTypeDefinition
    pub fn from_type(ship_type: &ShipTypeDefinition) -> Self {
        Ship {
            ship_type_id: ship_type.id.clone(),
            speed: 10.0,  // Legacy field, not used in travel calculations
            acceleration: ship_type.acceleration,
            cargo_capacity: ship_type.cargo_capacity,
        }
    }
}
```

The same change applies to the parallel `Ship` in `src/game_state.rs`.

### 4. Changes to `Player::new()` — Accept Ship Selection

Modify `Player::new()` to no longer hardcode ship stats. Instead, introduce `Player::with_ship_type()`:

```rust
// src/player/mod.rs
impl Player {
    /// Create a new player with a chosen ship type
    pub fn with_ship_type(ship_type: &ShipTypeDefinition, starting_money: u32) -> Self {
        let effective_money = starting_money.saturating_sub(ship_type.purchase_cost);
        Player {
            money: effective_money,
            location: "earth".to_string(),
            ship: Ship::from_type(ship_type),
            inventory: CargoHold::new(ship_type.cargo_capacity),
        }
    }

    /// Legacy: Create player with default Brigantine ship (for tests/CLI)
    pub fn new() -> Self {
        let brigantine = ShipTypeDefinition {
            id: "brigantine".to_string(),
            name: "Prairie Brigantine".to_string(),
            flavour: String::new(),
            acceleration: 1,
            cargo_capacity: 15,
            purchase_cost: 0,
        };
        Self::with_ship_type(&brigantine, 1000)
    }
}
```

**Note:** `Player::new()` is preserved (used in tests and CLI) but now defaults to the Brigantine rather than the old 10-unit ship. Existing tests that rely on `cargo_capacity == 10` must be updated to `15` or use explicit construction.

### 5. Game Initialisation Integration

The ship selection must occur **before** `Player::new()` is called, during the new game setup flow. The existing `GameConfigModal` (in `src/ui/game_config_modal.rs`) serves as the model:

- The `GameConfig` struct gains a `ship_type_id: String` field.
- The `GameConfigModal` component gains a ship selection step (see Section 6).
- When the user confirms the config, `Player::with_ship_type()` is called with the chosen `ShipTypeDefinition`.

```rust
// src/ui/game_config_modal.rs - extend GameConfig
pub struct GameConfig {
    pub difficulty: GameDifficulty,
    pub turn_limit: u32,
    pub starting_credits: u32,
    pub ship_type_id: String,    // NEW
}
```

### 6. UI: Ship Selection within `GameConfigModal`

Following the existing `GameConfigModal` pattern, add a ship selection step to the modal. The implementation uses a `RwSignal<String>` to track the selected ship type ID.

**Design:** Three clickable "ship cards" displayed in a horizontal row (same layout as the existing difficulty buttons). Each card shows:
- Ship name
- Flavour text (italic)
- Acceleration, Cargo Capacity, and effective Starting Gold (after purchase cost deduction)

```rust
// Pattern (Leptos component excerpt)
let selected_ship: RwSignal<String> = RwSignal::new("brigantine".to_string());
let ship_types = default_ship_types(); // loaded once

view! {
    <div class="form-section">
        <label class="form-label">"Choose Your Ship"</label>
        <div class="ship-options">
            {ship_types.iter().map(|ship| {
                let ship_id = ship.id.clone();
                let ship_clone = ship.clone();
                view! {
                    <button
                        class="ship-btn"
                        class:selected={move || selected_ship.get() == ship_id}
                        on:click={move |_| selected_ship.set(ship_clone.id.clone())}
                    >
                        <span class="ship-name">{ship.name.clone()}</span>
                        <span class="ship-flavour">{ship.flavour.clone()}</span>
                        <div class="ship-stats">
                            <span>"Accel: " {ship.acceleration}</span>
                            <span>"Cargo: " {ship.cargo_capacity}</span>
                            <span>"Cost: $" {ship.purchase_cost}</span>
                        </div>
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    </div>
}
```

**No separate screen needed:** Ship selection is embedded in the existing `GameConfigModal`. This minimises UI changes and follows the project principle of simplicity.

### 7. CLI Support

The CLI (`src/ui/cli.rs`) currently initialises the world via `setup::World`. For CLI usage, ship selection can be skipped (default to Brigantine) or handled via a command-line argument in a follow-up ticket. The `Player::new()` default handles backward compatibility.

### 8. Test Strategy

Tests that use `Player::new()` will see `cargo_capacity` change from 10 to 15 (Brigantine). These tests must be updated. New tests should cover:
- `ShipTypeDefinition` YAML loading
- `Ship::from_type()` creates correct fields
- `Player::with_ship_type()` correctly deducts purchase cost
- `Player::with_ship_type()` with Sloop sets acceleration to 3
- Verify `calculate_travel_turns()` gives faster result with acceleration 3 vs 1 (already implicitly tested; explicit test for Sloop would add clarity)

### 9. Data Flow Summary

```
ships.yaml
  → include_str! at compile time
  → default_ship_types() → Vec<ShipTypeDefinition>
  → ShipSelectionModal (UI) → user picks ship
  → GameConfig { ship_type_id }
  → Player::with_ship_type(&chosen_type, difficulty.starting_money())
  → Ship { ship_type_id, acceleration, cargo_capacity }
  → CargoHold::new(cargo_capacity)
  → calculate_travel_turns(origin, dest, ship.acceleration)
```

### Files to Create / Modify

| File | Change |
|------|--------|
| `data/config/ships.yaml` | Create: ship type definitions |
| `src/player/ship_types.rs` | Create: `ShipTypeDefinition` struct + loader |
| `src/player/ship.rs` | Modify: add `ship_type_id` field, add `Ship::from_type()` |
| `src/player/mod.rs` | Modify: `Player::with_ship_type()`, update `Player::new()` default |
| `src/ui/game_config_modal.rs` | Modify: add ship selection UI, extend `GameConfig` |
| `src/game_state.rs` | Modify: add `ship_type_id` to `Ship` struct there |

## Consequences

### Positive
- Config-driven ship types are easy to add/modify without code changes
- `include_str!` embedding works correctly in WASM without runtime filesystem access
- Ship selection integrates cleanly into the existing `GameConfigModal` pattern, minimising new UI surface
- `Player::new()` preserved for backward compatibility; no CLI breakage
- `acceleration` already wired into travel time formula — no changes to core game logic needed
- Serde `#[serde(default)]` can be used on `ship_type_id` to handle existing saved game states that pre-date this field

### Negative
- `Player::new()` changes default cargo capacity from 10 to 15; tests referencing `cargo_capacity == 10` need updating (estimated 5-10 test cases)
- Two `Ship` structs exist (`src/player/ship.rs` and `src/game_state.rs`) — both need updating; a future refactor to consolidate them is desirable but out of scope here
- The `serde_yaml` crate dependency must be added to `Cargo.toml` if not already present (check if `serde_yaml` or equivalent is already in the dependency tree)

## References
- [Ship Types Game Design ADR #0007](./0007-ship-types-system.md)
- [Movement Mechanics System ADR #0002](./0002-movement-mechanics-system.md)
- [Data Models/Schema ADR #0006](./0006-data-models-schema.md)
- [Tech Stack Selection ADR #0004](./0004-tech-stack-selection.md)
- [Market/Economy System ADR #0005](./0005-market-economy-system.md)
