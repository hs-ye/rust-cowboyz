# Playtest Scenario: Planet Orbital Motion

## Objective
Verify that planets orbit the sun in proper 2D circular/elliptical paths when turns are advanced, rather than moving along a 1D line.

## Preconditions
- Game is running and accessible at http://localhost:8080
- Solar system map is displayed with multiple planets
- At least 3 turns can be played

## Test Steps

### Manual Testing
1. Start the game and navigate to the solar system map view
2. Observe the initial positions of all planets
3. Click the "Next Turn" button
4. Observe how each planet moves to its new position
5. Repeat steps 3-4 for at least one full orbit of the fastest planet
6. Watch the orbital path traced by each planet

### Automated Testing (Playwright)
```javascript
const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch({ headless: false });
  const page = await browser.newPage();
  
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(5000); // Wait for WASM load
  
  // Get initial planet positions
  const positions = [];
  for (let turn = 0; turn < 12; turn++) {
    const planetPositions = await page.evaluate(() => {
      // Extract planet positions from canvas
      const canvas = document.querySelector('canvas.solar-map-canvas');
      // ... position extraction logic
    });
    positions.push(planetPositions);
    
    // Click Next Turn button
    await page.click('button:has-text("Next Turn")');
    await page.waitForTimeout(1000);
  }
  
  // Verify positions form a circle, not a line
  // Check that x and y coordinates vary independently
});
```

## Expected Results
- Planets should move in circular or elliptical paths around the sun
- At 0° (start): Planet should be at maximum X offset, center Y (e.g., right side of orbit)
- At 90° (quarter orbit): Planet should be at center X, maximum Y offset (e.g., bottom of orbit)
- At 180° (half orbit): Planet should be at minimum X offset, center Y (e.g., left side of orbit)
- At 270° (three-quarter orbit): Planet should be at center X, minimum Y offset (e.g., top of orbit)
- The orbital path should satisfy the circle equation: (x - centerX)² + (y - centerY)² = radius²

## Actual Results
**BUG FOUND:** Planets move along a diagonal 1D line instead of circular orbits.

Observations:
- At turn 0: Planet at (550, 450)
- At turn 3 (90°): Planet at (400, 300) - should be at (400, 450) for circular orbit
- At turn 6 (180°): Planet at (250, 150) - should be at (250, 300) for circular orbit
- At turn 9 (270°): Planet at (400, 300) - should be at (400, 150) for circular orbit

The planet moves back and forth along the line y = x + (centerY - centerX), oscillating between opposite corners of the screen.

## Pass/Fail Criteria
- [ ] Planets move in circular/elliptical paths around the sun
- [ ] X and Y coordinates vary independently during orbit
- [ ] At 90° orbital position, planet is at top/bottom of orbit (not center)
- [ ] At 180° orbital position, planet is at left/right of orbit (not corner)
- [ ] Orbital radius remains constant (for circular orbits)

## Root Cause Analysis

**Location:** `src/ui/solar_map.rs`, function `calculate_orbital_position()`, lines 73-84

**Bug:** The y-coordinate calculation incorrectly adds π/2 to the angle:
```rust
// BUGGY CODE (line 82-83):
let x = center_x + (orbit_radius as f64 * scale) * angle.cos();
let y = center_y + (orbit_radius as f64 * scale) * (angle + std::f64::consts::FRAC_PI_2).sin();
```

**Why it causes linear motion:**
- `sin(angle + π/2) = cos(angle)` (trigonometric identity)
- Therefore: `y = centerY + radius * cos(angle)`
- Meanwhile: `x = centerX + radius * cos(angle)`
- Both x and y depend on `cos(angle)`, creating linear relationship: `y - centerY = x - centerX`

**Fix:** Remove the `+ FRAC_PI_2` from the y-coordinate calculation:
```rust
// CORRECT CODE:
let x = center_x + (orbit_radius as f64 * scale) * angle.cos();
let y = center_y + (orbit_radius as f64 * scale) * angle.sin();
```

## Related Files
- `src/ui/solar_map.rs` - Contains the buggy `calculate_orbital_position()` function
- `src/simulation/orbits.rs` - Contains orbital mechanics logic (this is correct)
- `test-orbital-motion-bug.js` - Test script demonstrating the bug
