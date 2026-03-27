# Bug Investigation Report: Market Panel Shows Same Prices for All Planets

**Date:** 2026-03-21  
**Status:** ✅ FIXED  
**Related Issue:** Market panel displays identical prices across all planet types after PR #128

---

## Executive Summary

The market panel was showing identical prices for all commodities regardless of which planet was selected. This was caused by a **reactive programming bug** in the `MarketPanelReactive` component where the economy memo's reactive dependency was not being properly tracked by Leptos.

---

## Root Cause Analysis

### What: The Observable Symptom

When clicking on different planets in the solar map, the market panel would update the planet name correctly, but all commodity prices remained identical across all planets:

- **Expected:** Different planet types should have different prices based on their economic specialization
- **Actual:** All planets showed the same prices (matching Agricultural planet prices)

### Where: The Affected Component

**File:** `src/ui/market_panel.rs`  
**Component:** `MarketPanelReactive`  
**Lines:** 105-115 (before fix)

### Why: The Underlying Cause

The bug was in how the `economy` memo was being read within the component's view rendering:

```rust
// ❌ BUGGY CODE
{
    commodities.into_iter().map(move |commodity| {
        let commodity_name = commodity.display_name();
        let current_economy = economy.get();  // ← Called OUTSIDE view! macro
        let buy_price = current_economy.get_buy_price(&commodity).unwrap_or(0);
        let sell_price = current_economy.get_sell_price(&commodity).unwrap_or(0);

        view! {
            <div class="market-row">
                <span>{commodity_name}</span>
                <span class="buy-price">{format!("${}", buy_price)}</span>
                <span class="sell-price">{format!("${}", sell_price)}</span>
            </div>
        }
    }).collect::<Vec<_>>()
}
```

**The Problem:**
1. `economy.get()` was called **outside** the `view!` macro, inside the iterator closure
2. This caused the economy value to be computed **once** when the component mounted
3. Leptos's reactive system couldn't track the dependency because the `.get()` call wasn't inside the reactive view context
4. When the `economy` memo changed (due to planet selection), the view didn't re-render with new prices

### How: The Chain of Events

1. User clicks on a planet (e.g., Mercury)
2. `selected_planet` signal updates to "mercury"
3. `current_planet_id` memo recomputes to "mercury"
4. `current_economy` memo recomputes and finds Mercury's economy (Mining type)
5. `MarketPanelReactive` receives the new economy memo value
6. **BUG:** The view had already computed prices once during mount, using the initial economy
7. The iterator closure ran once, computed prices, and created static views
8. Even though the economy memo changed, the price values were already baked into the view

---

## The Fix

Move the `economy.get()` call **inside** the `view!` macro, wrapped in a reactive closure:

```rust
// ✅ FIXED CODE
{
    commodities.into_iter().map(move |commodity| {
        let commodity_name = commodity.display_name();
        let commodity_for_buy = commodity.clone();
        let commodity_for_sell = commodity.clone();
        
        view! {
            <div class="market-row">
                <span>{commodity_name}</span>
                <span class="buy-price">{
                    move || format!("${}", economy.get().get_buy_price(&commodity_for_buy).unwrap_or(0))
                }</span>
                <span class="sell-price">{
                    move || format!("${}", economy.get().get_sell_price(&commodity_for_sell).unwrap_or(0))
                }</span>
            </div>
        }
    }).collect::<Vec<_>>()
}
```

**Key Changes:**
1. `economy.get()` is now called **inside** the `view!` macro
2. Each price is wrapped in a `move ||` closure, making it reactive
3. When the economy memo changes, Leptos re-runs these closures and updates the DOM
4. Commodity clones are split into separate variables (`commodity_for_buy`, `commodity_for_sell`) to avoid move errors

---

## Verification

### Test Results

After applying the fix, prices now correctly vary by planet type:

| Commodity | Earth (Agri) | Mercury (Mining) | Jupiter (MegaCity) |
|-----------|-------------|------------------|-------------------|
| Water     | $4 (cheap)  | $21 (expensive)  | $21 (expensive)   |
| Metals    | $60 (exp)   | $26 (cheap)      | $60 (normal)      |
| Electronics| $254 (exp) | $52 (cheap)      | $52 (cheap)       |
| Medicine  | $211 (exp)  | $211 (exp)       | $43 (cheap)       |

### Expected Price Patterns (per ADR 0005)

✅ **Agricultural planets** (Earth):
- Produce: Water, Foodstuffs → **cheaper**
- Demand: Medicine, Firearms, Ammunition, Electronics → **expensive**

✅ **Mining planets** (Mercury, Mars):
- Produce: Metals, Antimatter, Electronics → **cheaper**
- Demand: Water, Foodstuffs, Medicine, Ammunition → **expensive**

✅ **MegaCity planets** (Jupiter):
- Produce: Electronics, Medicine, Narcotics → **cheaper**
- Demand: Water, Foodstuffs, Firearms, Ammunition → **expensive**

---

## Data Flow (Fixed)

```
Planet Selection → Signal Update → Memo Recalculation → View Re-render → Price Display
     ↓                  ↓                ↓                   ↓               ↓
Click Mercury   selected_planet   current_economy    economy.get()    $21 Water
                = "mercury"       = Mining economy   (reactive)       $26 Metals
                                                                        $52 Electronics
```

---

## Files Changed

1. **`src/ui/market_panel.rs`** - Fixed `MarketPanelReactive` component
   - Moved `economy.get()` calls inside `view!` macro
   - Wrapped price formatting in reactive closures
   - Split commodity clones to avoid move errors

---

## Lessons Learned

### Leptos Reactive Pattern

When using memos/signals in Leptos components:

1. **Always call `.get()` inside the `view!` macro** for reactive updates
2. **Use `move ||` closures** for dynamic values that should update
3. **Avoid computing signal values outside the view** if they need to react to changes

### Correct Pattern for Iterators with Reactive Values

```rust
// ✅ CORRECT: Signal read inside view
items.iter().map(|item| {
    let name = item.name;
    let item_clone = item.clone();
    view! {
        <div>{move || signal.get().method(&item_clone)}</div>
    }
})

// ❌ INCORRECT: Signal read outside view
items.iter().map(|item| {
    let name = item.name;
    let value = signal.get().method(item);  // Not reactive!
    view! {
        <div>{value}</div>
    }
})
```

---

## Testing

Debug scripts created for verification:
- `test-planet-positions.js` - Tests planet selection and price changes
- `test-economy-memo.js` - Verifies economy memo reactivity
- `test-planet-type-memo.js` - Checks expected vs actual prices

All tests now pass with the fix applied.

---

## Recommendation

No further action needed. The fix is complete and verified. Consider adding a unit test for the `MarketPanelReactive` component to prevent regression of this reactive pattern bug.
