# Playtest Scenario: Planet Selection and Next Turn Bugs

## Objective
Test two critical bugs in the game UI:
1. **Planet Selection Bug**: Clicking on planets in the solar map doesn't change the selected planet
2. **Next Turn Bug**: Clicking the 'Next Turn' button doesn't cause planets to rotate/Canvas doesn't re-render

## Preconditions
- Game server running at http://localhost:8080
- WASM fully loaded (wait 5 seconds after page load)

## Test Steps

### Test 1: Planet Selection via Canvas Click

1. Navigate to http://localhost:8080
2. Wait for game to load (app-container visible)
3. Locate the solar system map canvas (`.solar-map-canvas`)
4. Click on the canvas at position (70% width, 50% height) where outer planets should be
5. Observe if any visual selection indicator appears

**Playwright Code:**
```javascript
const canvas = await page.$('.solar-map-canvas');
const box = await canvas.boundingBox();
await canvas.click({ position: { x: box.width * 0.7, y: box.height * 0.5 } });
await page.waitForTimeout(500);

// Check for selection indicators
const selectionState = await page.evaluate(() => {
  const selectedElements = document.querySelectorAll('[class*="selected"]');
  return selectedElements.length;
});
```

### Test 2: Next Turn Button and Canvas Re-render

1. Record the current turn number from `.stat-value.turn`
2. Record canvas pixel data at specific coordinates
3. Click the "Next Turn" button
4. Wait 1 second
5. Record the new turn number
6. Record canvas pixel data at the same coordinates
7. Compare pixel data to verify canvas re-rendered

**Playwright Code:**
```javascript
const turnBefore = await page.evaluate(() => 
  document.querySelector('.stat-value.turn').textContent.trim()
);

const pixelsBefore = await page.evaluate(() => {
  const canvas = document.querySelector('.solar-map-canvas');
  const ctx = canvas.getContext('2d');
  return ctx.getImageData(350, 380, 10, 10).data.slice(0, 4);
});

await page.click('button:has-text("Next Turn")');
await page.waitForTimeout(1000);

const turnAfter = await page.evaluate(() => 
  document.querySelector('.stat-value.turn').textContent.trim()
);

const pixelsAfter = await page.evaluate(() => {
  const canvas = document.querySelector('.solar-map-canvas');
  const ctx = canvas.getContext('2d');
  return ctx.getImageData(350, 380, 10, 10).data.slice(0, 4);
});
```

## Expected Results

### Planet Selection
- Clicking on a planet should select it
- Visual feedback should appear (white ring around selected planet)
- The `selected_planet` signal should update
- Planet details should appear in the UI

### Next Turn
- Turn counter should increment (e.g., 1 → 2)
- Canvas should re-render with updated planet positions
- Planet positions should change based on orbital mechanics
- Pixel data at sampled locations should change

## Actual Results

### Planet Selection
- ❌ Clicking canvas produces no selection
- ❌ No visual feedback appears
- ❌ `selectedCount: 0, highlightedCount: 0, whiteRingsCount: 0`

### Next Turn
- ✅ Turn counter increments (1 → 2)
- ❌ Canvas does NOT re-render
- ❌ Planet positions remain static
- ❌ Pixel data unchanged: `{ center: [255,152,0,255], outer: [10,10,26,255] }`

## Pass/Fail Criteria

- [ ] ❌ Planet selection works when clicking on canvas
- [ ] ❌ Canvas re-renders when turn advances
- [ ] ❌ Planet positions update based on orbital mechanics

## Root Cause Summary

See detailed investigation in `BUG-INVESTIGATION-001.md`

**Key Issues:**
1. `SolarMap` component receives `current_turn` as a `u32` value, not a reactive signal
2. The `Effect` in `SolarMap` cannot track changes to primitive values
3. Planet positions are calculated at render time but the component doesn't re-render on turn change
4. Click handler may not be properly wired to the selection callback

## Files to Fix

1. `src/ui/web.rs` - Lines 112-117: Pass signals instead of values to `SolarMap`
2. `src/ui/solar_map.rs` - Component signature needs to accept signals or callbacks for reactivity
