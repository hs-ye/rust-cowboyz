# Playtest Scenario: Market Panel Updates on Planet Selection

## Objective
Verify that the market panel displays correct market data for the selected planet, including commodity prices that reflect the planet's economy type.

## Preconditions
- Game is running at http://localhost:8080
- WASM has loaded (wait ~5 seconds after page load)
- Player starts on Earth with default game state

## Test Steps

### Test 1: Initial Market Display
1. Load the game and wait for it to initialize
2. Observe the market panel on the right side of the screen

**Expected:**
- Market panel should show "Earth" or current planet name in the subtitle
- Market should display commodity prices based on Earth's economy (Agricultural planet type)
- Water and Foodstuffs should be cheaper (produced on Agricultural planets)

**Playwright Verification:**
```javascript
const marketPanel = await page.evaluate(() => {
  const subtitle = document.querySelector('.market-panel .panel-subtitle')?.textContent;
  const rows = document.querySelectorAll('.market-panel .market-row');
  return { subtitle, rowCount: rows.length };
});
// Expected: subtitle matches player location, rowCount > 0
```

### Test 2: Select Different Planet Type (Mining Planet)
1. Click on Mars (Mining planet) in the solar system map
2. Observe the market panel update

**Expected:**
- Market panel subtitle changes to "Mars"
- Metals, Antimatter, and Electronics prices should be lower (produced on Mining planets)
- Water and Foodstuffs prices should be higher (imported to Mining planets)

**Playwright Verification:**
```javascript
await page.click('canvas'); // Click on Mars position
await page.waitForTimeout(1000);
const marsMarket = await page.evaluate(() => {
  const subtitle = document.querySelector('.market-panel .panel-subtitle')?.textContent;
  const metalsRow = document.querySelectorAll('.market-row')[2]; // Metals row
  return { subtitle, metalsPrice: metalsRow?.textContent };
});
// Expected: subtitle === "Mars", metals price reflects Mining planet economy
```

### Test 3: Select MegaCity Planet
1. Click on Jupiter (Mega City planet) in the solar system map
2. Observe the market panel update

**Expected:**
- Market panel subtitle changes to "Jupiter"
- Electronics, Medicine, Narcotics should be cheaper (produced)
- Water, Foodstuffs, Firearms, Ammunition should be more expensive (demanded)

**Playwright Verification:**
```javascript
// Click on Jupiter position
const jupiterMarket = await page.evaluate(() => {
  const subtitle = document.querySelector('.market-panel .panel-subtitle')?.textContent;
  return { subtitle };
});
// Expected: subtitle === "Jupiter"
```

### Test 4: Price Comparison Between Planets
1. Click on Earth, note the price of Electronics
2. Click on Mars (Mining), note the price of Electronics
3. Click on Jupiter (MegaCity), note the price of Electronics

**Expected:**
- Electronics should be cheapest on Mars and Jupiter (both produce it)
- Electronics should be more expensive on Earth (demanded, not produced)
- Price differences should reflect supply/demand mechanics

**Playwright Verification:**
```javascript
const prices = await page.evaluate(() => {
  const getElectronicsPrice = () => {
    const rows = document.querySelectorAll('.market-row');
    for (const row of rows) {
      if (row.textContent.includes('Electronics')) {
        return row.querySelector('.sell-price')?.textContent;
      }
    }
    return null;
  };
  return { earth: getElectronicsPrice() };
});
// Repeat for each planet and compare
```

### Test 5: Return to Player Location
1. Select a different planet
2. Deselect or return to player's current location (Earth)

**Expected:**
- Market panel shows Earth's prices when Earth is selected
- Player location indicator matches market panel subtitle when on current planet

**Playwright Verification:**
```javascript
const locationMatch = await page.evaluate(() => {
  const location = document.querySelector('.stat-value.location')?.textContent;
  const marketSubtitle = document.querySelector('.market-panel .panel-subtitle')?.textContent;
  return location?.toLowerCase() === marketSubtitle?.toLowerCase();
});
// Expected: true
```

## Expected Results

### Market Data by Planet Type

| Planet | Type | Cheap (Produced) | Expensive (Demanded) |
|--------|------|------------------|---------------------|
| Earth | Agricultural | Water, Foodstuffs | Medicine, Firearms, Ammunition, Electronics |
| Mars | Mining | Metals, Antimatter, Electronics | Water, Foodstuffs, Medicine, Ammunition |
| Jupiter | MegaCity | Electronics, Medicine, Narcotics | Water, Foodstuffs, Firearms, Ammunition |
| Venus | Industrial | Electronics, Metals, Ammunition, Antimatter | Water, Foodstuffs, Medicine |
| Saturn | Industrial | Electronics, Metals, Ammunition, Antimatter | Water, Foodstuffs, Medicine |
| Mercury | Mining | Metals, Antimatter, Electronics | Water, Foodstuffs, Medicine, Ammunition |
| Uranus | ResearchOutpost | Electronics, Medicine, AlienArtefacts | Water, Foodstuffs |
| Neptune | FrontierColony | Water, Foodstuffs | Medicine, Firearms, Ammunition, Electronics, Metals, Antimatter, AlienArtefacts |
| Pluto | PirateSpaceStation | Narcotics, Ammunition | Foodstuffs, Firearms, Medicine |

### UI Behavior
- Market panel should update within 1 second of planet selection
- All 10 commodity types should be displayed
- Buy price should be ~5% lower than sell price (market spread)
- Prices should be whole numbers in credits ($)

## Actual Results
*(To be filled during testing)*

## Pass/Fail Criteria

- [ ] Market panel displays planet name in subtitle
- [ ] Market panel updates when different planet is selected
- [ ] Prices reflect planet's economy type (produced goods cheaper)
- [ ] All 10 commodity types are displayed
- [ ] Buy/sell price spread is consistent (~5%)
- [ ] No console errors when selecting planets
- [ ] Market panel shows "Select a planet" when no planet is selected (if applicable)
- [ ] Player location and market subtitle match when on current planet

## Notes

- This test depends on the planet selection bug being fixed (see BUG-INVESTIGATION-001.md)
- If planet selection doesn't work, this test cannot be completed
- The economy system (`src/simulation/economy.rs`) is already implemented and working
- The bug is in the UI layer - market panel is hardcoded with static data

## Related Bugs

- BUG-INVESTIGATION-001.md: Planet Selection and Next Turn bugs
- BUG-INVESTIGATION-002.md: Market Panel Doesn't Update (this bug)

## Test Script

Run automated verification:
```bash
node test-market-panel-bug.js
```
