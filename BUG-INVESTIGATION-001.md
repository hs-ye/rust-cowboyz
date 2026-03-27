# Bug Investigation: Planet Selection and Next Turn Not Working

**Date:** 2026-03-21  
**Investigator:** Qwen Playtester  
**Status:** Root Cause Identified  

---

## Issue Summary

Two critical bugs prevent core gameplay functionality:

1. **Planet Selection Bug**: Clicking on planets in the solar system map does not select them
2. **Next Turn Bug**: Clicking "Next Turn" increments the turn counter but planets don't rotate/orbit

---

## Steps to Reproduce

### Bug 1: Planet Selection

1. Start the game at http://localhost:8080
2. Wait for WASM to load (5 seconds)
3. Click anywhere on the solar system map canvas
4. Observe: No planet is selected, no visual feedback

### Bug 2: Next Turn

1. Start the game at http://localhost:8080
2. Note the current turn number (e.g., "1")
3. Click the "Next Turn" button
4. Observe: Turn number increments (1 → 2) BUT planets don't move on the canvas

---

## Expected Behavior

### Planet Selection
- Clicking a planet should select it
- A white selection ring should appear around the selected planet
- Planet details should be shown in the UI

### Next Turn
- Turn counter should increment
- Canvas should re-render with updated planet positions
- Planets should appear to orbit based on their orbital periods

---

## Actual Behavior

### Planet Selection
```
Selection indicators after click: { 
  selectedCount: 0, 
  highlightedCount: 0, 
  whiteRingsCount: 0 
}
```
- No selection state changes
- No visual feedback

### Next Turn
```
Turn before: 1
Canvas pixels before: { center: [255,152,0,255], outer: [10,10,26,255] }
Turn after: 2
Canvas pixels after: { center: [255,152,0,255], outer: [10,10,26,255] }
Pixels changed: false
```
- Turn counter increments correctly
- Canvas does NOT re-render
- Planet positions remain static

---

## Root Cause Analysis

### Bug 1: Planet Selection - Signal Reactivity Issue

**Location:** `src/ui/web.rs` lines 112-117

**Code:**
```rust
<SolarMap
    planets={planets.clone()}
    current_turn={turn.get()}           // ❌ Passes value, not signal
    player_location={location.get()}     // ❌ Passes value, not signal
    selected_planet={selected_planet.get()} // ❌ Passes value, not signal
    on_planet_select={Some(Box::new(move |id| {
        set_selected_planet.set(Some(id));
    }))}
/>
```

**Problem:**
The `SolarMap` component receives `selected_planet` as `Option<String>` (a value), not as a reactive signal. When `set_selected_planet.set()` is called, the `SolarMap` component doesn't know to re-render because it only received a snapshot of the value at component creation time.

**Signal Flow:**
```
User clicks canvas 
  → on_canvas_click handler fires
  → on_planet_select callback executes
  → set_selected_planet.set(Some(id)) updates signal
  → ❌ SolarMap doesn't re-render (received value, not signal)
  → ❌ No visual feedback appears
```

**Console Warnings:**
```
[warning] At src/ui/web.rs:112:48, you access a signal or memo 
defined at src/ui/web.rs:14:28 outside a reactive tracking context.
```

These warnings confirm that signals are being accessed (`.get()`) outside reactive contexts, meaning changes won't trigger re-renders.

---

### Bug 2: Next Turn - Component Not Receiving Signal Updates

**Location:** `src/ui/web.rs` lines 112-114 AND `src/ui/solar_map.rs` lines 91-100

**Code in web.rs:**
```rust
<SolarMap
    current_turn={turn.get()}  // ❌ Passes u32 value, not ReadSignal
    ...
/>
```

**Code in solar_map.rs:**
```rust
pub fn SolarMap(
    planets: Vec<MapPlanet>,
    current_turn: u32,  // ❌ Should be ReadSignal<u32> or Signal<u32>
    player_location: String,
    selected_planet: Option<String>,
    on_planet_select: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    // ...
    
    // This Effect tries to track current_turn but can't because it's a u32 value
    Effect::new(move |_| {
        let _ = current_turn;  // ❌ This does nothing - current_turn is not a signal
        let _ = selected_planet.clone();
        let _ = player_location.clone();
        render_canvas_for_effect();
    });
}
```

**Problem:**
The `Effect::new()` in `SolarMap` is supposed to re-run when `current_turn` changes, but `current_turn` is a `u32` primitive value, not a `ReadSignal`. Leptos's reactive system can only track signals, not primitive values.

**Signal Flow:**
```
User clicks "Next Turn"
  → set_turn.update(|t| *t += 1) increments turn signal
  → Turn display updates (uses move || turn.get())
  → ❌ SolarMap.current_turn is a u32 value, not a signal
  → ❌ Effect doesn't re-run
  → ❌ Canvas doesn't re-render
  → ❌ Planets don't appear to move
```

**Orbital Calculation (works correctly but never re-executed):**
```rust
// In solar_map.rs render_canvas closure
let position = crate::simulation::orbits::calculate_orbit_position(
    planet.orbit_period,
    current_turn,  // This value is stale - captured at component creation
);
```

The orbital calculation function `calculate_orbit_position()` works correctly, but it's called with a stale `current_turn` value that never updates.

---

## Code Locations

### Primary Issues

| File | Lines | Issue |
|------|-------|-------|
| `src/ui/web.rs` | 112-117 | Passing signal values instead of signals to SolarMap |
| `src/ui/solar_map.rs` | 91-100 | SolarMap component signature uses primitive types instead of signals |
| `src/ui/solar_map.rs` | 256-263 | Effect cannot track primitive value changes |

### Related Code

| File | Lines | Description |
|------|-------|-------------|
| `src/ui/web.rs` | 14-18 | Signal definitions (turn, location, selected_planet) |
| `src/ui/web.rs` | 143-145 | Turn display uses correct reactive pattern: `{move || turn.get()}` |
| `src/simulation/orbits.rs` | 32-38 | `calculate_orbit_position()` - works correctly |

---

## Recommended Fixes

### Fix 1: Pass Signals to SolarMap Component

**In `src/ui/web.rs`:**
```rust
// Change from:
<SolarMap
    planets={planets.clone()}
    current_turn={turn.get()}
    player_location={location.get()}
    selected_planet={selected_planet.get()}
    on_planet_select={Some(Box::new(move |id| {
        set_selected_planet.set(Some(id));
    }))}
/>

// To (using callbacks that return current values):
<SolarMap
    planets={planets.clone()}
    current_turn={move || turn.get()}
    player_location={move || location.get()}
    selected_planet={move || selected_planet.get()}
    on_planet_select={Some(Box::new(move |id| {
        set_selected_planet.set(Some(id));
    }))}
/>
```

### Fix 2: Update SolarMap Component Signature

**In `src/ui/solar_map.rs`:**
```rust
// Change component to accept callbacks instead of values
#[cfg(feature = "web")]
#[component]
pub fn SolarMap(
    planets: Vec<MapPlanet>,
    current_turn: Box<dyn Fn() -> u32>,  // Callback instead of u32
    player_location: Box<dyn Fn() -> String>,
    selected_planet: Box<dyn Fn() -> Option<String>>,
    on_planet_select: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    // In render function, call the callbacks:
    let turn = current_turn();
    let location = player_location();
    let selection = selected_planet();
    
    // Effect now tracks the callbacks which internally track signals
    Effect::new(move |_| {
        let _ = current_turn();  // Now this tracks the signal!
        let _ = selected_planet();
        let _ = player_location();
        render_canvas_for_effect();
    });
}
```

### Alternative Fix: Use Leptos Signals

Another approach is to use `ReadSignal` or `Signal` types:

```rust
use leptos::ReadSignal;

#[cfg(feature = "web")]
#[component]
pub fn SolarMap(
    planets: Vec<MapPlanet>,
    current_turn: ReadSignal<u32>,
    player_location: ReadSignal<String>,
    selected_planet: ReadSignal<Option<String>>,
    on_planet_select: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    Effect::new(move |_| {
        let _ = current_turn.get();  // Now this tracks!
        let _ = selected_planet.get();
        let _ = player_location.get();
        render_canvas_for_effect();
    });
}
```

---

## Verification Steps

After applying fixes:

1. Run `cargo build --features web`
2. Run `trunk serve --port 8080`
3. Execute `node verify-bugs.js`
4. Expected results:
   - Planet selection should show visual feedback
   - Canvas pixels should change after "Next Turn"

---

## Additional Findings

### Console Warnings

The browser console shows these warnings confirming the root cause:

```
[warning] At src/ui/web.rs:112:48, you access a signal or memo 
(defined at src/ui/web.rs:14:28) outside a reactive tracking context.
This might mean your app is not responding to changes in signal values 
in the way you expect.

Here's how to fix it:
1. If this is inside a `view!` macro, make sure you are passing a function, not a value.
  ❌ NO  <p>{x.get() * 2}</p>
  ✅ YES <p>{move || x.get() * 2}</p>
```

This is exactly Leptos telling us that we're accessing signals incorrectly.

### Working Pattern in Same File

The turn display in `src/ui/web.rs` line 143 uses the CORRECT pattern:

```rust
<span class="stat-value turn">{move || turn.get()}</span>
```

This is why the turn counter updates correctly - it uses a closure that Leptos can track reactively.

---

## Impact

- **Severity:** Critical
- **User Impact:** Game is unplayable - cannot select destinations or advance time
- **Scope:** Affects all gameplay mechanics that depend on turn progression

---

## Files Modified for Testing

- `debug-bugs.js` - Initial debug script
- `debug-planet-selection.js` - Detailed selection debug
- `verify-bugs.js` - Bug verification script
- `playtest/03-planet-selection-next-turn-bugs.md` - Playtest scenario

---

## Conclusion

Both bugs stem from the same root cause: **signals are being accessed as values instead of being passed as reactive callbacks to the `SolarMap` component**. The fix requires changing how the `App` component passes data to `SolarMap` and updating `SolarMap`'s component signature to accept reactive callbacks or `ReadSignal` types.
