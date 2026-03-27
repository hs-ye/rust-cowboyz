# Playtest Scenario: Market Panel Planet-Specific Prices

## Objective
Verify that different planet types display different market prices according to their economic specialization as defined in ADR 0005.

## Background
According to ADR 0005, each planet type should have unique supply/demand patterns:
- **Supplied goods**: Lower prices (local multiplier 0.7)
- **Demanded goods**: Higher prices (local multiplier 1.3)
- **Ignored goods**: Base prices (local multiplier 1.0)

## Preconditions
- Game is running at http://localhost:8080
- Game has loaded to the main screen with solar system map visible
- Market panel is visible on the right side of the screen

## Test Steps

### Step 1: Verify Initial Market (Earth - Agricultural Planet)
1. Observe the default market panel (should show Earth by default)
2. Record the prices for all 10 commodities

**Playwright Code:**
```javascript
const initialPlanet = await page.textContent('.panel-subtitle');
console.log('Initial planet:', initialPlanet);
// Should be "Earth" or player's starting location
expect(initialPlanet).toBe('Earth');

// Capture initial prices
const initialPrices = await page.$$eval('.market-row', rows => {
  return rows.map(row => {
    const spans = row.querySelectorAll('span');
    return {
      name: spans[0].textContent.trim(),
      buy: spans[1].textContent.trim(),
      sell: spans[2].textContent.trim()
    };
  });
});
console.log('Earth prices:', initialPrices);
```

### Step 2: Select Mining Planet (Mercury) and Verify Price Changes
1. Click on Mercury (Mining Planet) in the solar system map
2. Wait for market panel to update
3. Verify the planet name changed to "Mercury"
4. Verify prices are DIFFERENT from Earth's prices

**Expected Price Changes:**
- **Water**: Should be MORE expensive on Mercury (demanded) vs Earth (supplied)
- **Foodstuffs**: Should be MORE expensive on Mercury (demanded) vs Earth (supplied)
- **Metals**: Should be CHEAPER on Mercury (supplied) vs Earth (ignored)
- **Antimatter**: Should be CHEAPER on Mercury (supplied) vs Earth (ignored)
- **Electronics**: Should be CHEAPER on Mercury (supplied) vs Earth (demanded)

**Playwright Code:**
```javascript
// Click on Mercury
await page.click('[data-planet-id="mercury"]');
await page.waitForTimeout(1000);

// Verify planet changed
const mercuryPlanet = await page.textContent('.panel-subtitle');
expect(mercuryPlanet).toBe('Mercury');

// Get Mercury prices
const mercuryPrices = await page.$$eval('.market-row', rows => {
  return rows.map(row => {
    const spans = row.querySelectorAll('span');
    return {
      name: spans[0].textContent.trim(),
      buy: spans[1].textContent.trim(),
      sell: spans[2].textContent.trim()
    };
  });
});

// Verify prices are different
expect(JSON.stringify(mercuryPrices)).not.toEqual(JSON.stringify(initialPrices));

// Verify specific price relationships
const getCommodity = (prices, name) => prices.find(p => p.name === name);
const earthWater = getCommodity(initialPrices, 'Water');
const mercuryWater = getCommodity(mercuryPrices, 'Water');

// Water should be more expensive on Mercury (mining planet demands water)
expect(parseInt(mercuryWater.sell.replace('$', '')))
  .toBeGreaterThan(parseInt(earthWater.sell.replace('$', '')));
```

### Step 3: Select Mega City Planet (Jupiter) and Verify Price Changes
1. Click on Jupiter (Mega City Planet) in the solar system map
2. Wait for market panel to update
3. Verify the planet name changed to "Jupiter"
4. Verify prices are DIFFERENT from both Earth and Mercury

**Expected Price Changes:**
- **Electronics**: Should be CHEAPER on Jupiter (supplied)
- **Medicine**: Should be CHEAPER on Jupiter (supplied)
- **Water**: Should be MORE expensive on Jupiter (demanded)
- **Foodstuffs**: Should be MORE expensive on Jupiter (demanded)

**Playwright Code:**
```javascript
// Click on Jupiter
await page.click('[data-planet-id="jupiter"]');
await page.waitForTimeout(1000);

// Verify planet changed
const jupiterPlanet = await page.textContent('.panel-subtitle');
expect(jupiterPlanet).toBe('Jupiter');

// Get Jupiter prices
const jupiterPrices = await page.$$eval('.market-row', rows => {
  return rows.map(row => {
    const spans = row.querySelectorAll('span');
    return {
      name: spans[0].textContent.trim(),
      buy: spans[1].textContent.trim(),
      sell: spans[2].textContent.trim()
    };
  });
});

// Verify prices are different from both Earth and Mercury
expect(JSON.stringify(jupiterPrices)).not.toEqual(JSON.stringify(initialPrices));
expect(JSON.stringify(jupiterPrices)).not.toEqual(JSON.stringify(mercuryPrices));
```

### Step 4: Test All Planet Types
1. Click on each remaining planet type
2. Verify each shows unique prices
3. Verify at least 3 distinct price sets exist (not all identical)

**Planets to Test:**
- Venus (Industrial)
- Mars (Mining) - should be similar to Mercury
- Saturn (Industrial) - should be similar to Venus
- Uranus (Research Outpost)
- Neptune (Frontier Colony)
- Pluto (Pirate Space Station)

**Playwright Code:**
```javascript
const planetIds = ['venus', 'mars', 'saturn', 'uranus', 'neptune', 'pluto'];
const allPrices = { earth: initialPrices, mercury: mercuryPrices, jupiter: jupiterPrices };

for (const planetId of planetIds) {
  await page.click(`[data-planet-id="${planetId}"]`);
  await page.waitForTimeout(1000);
  
  const planetName = await page.textContent('.panel-subtitle');
  const prices = await page.$$eval('.market-row', rows => {
    return rows.map(row => {
      const spans = row.querySelectorAll('span');
      return {
        name: spans[0].textContent.trim(),
        buy: spans[1].textContent.trim(),
        sell: spans[2].textContent.trim()
      };
    });
  });
  
  allPrices[planetId] = prices;
  console.log(`${planetName} prices:`, prices);
}

// Verify not all prices are identical
const uniquePriceSets = new Set(Object.values(allPrices).map(p => JSON.stringify(p)));
console.log(`Unique price sets: ${uniquePriceSets.size} out of ${Object.keys(allPrices).length} planets`);
```

## Expected Results

### Player Scenario
- Each planet type displays visibly different market prices
- Supplied goods are noticeably cheaper (approximately 30% lower)
- Demanded goods are noticeably more expensive (approximately 30% higher)
- Price differences make strategic trade routes apparent

### Playwright Script Output
- No JavaScript errors in console
- All planet selections successfully update the market panel
- At least 5-7 unique price sets across all planets (some planet types share similar patterns)
- Specific price relationships match ADR 0005 specifications:
  - Agricultural planets have cheapest Water/Food
  - Mining planets have cheapest Metals/Antimatter/Electronics
  - Mega City has cheapest Electronics/Medicine/Narcotics
  - Pirate Station has cheapest Narcotics/Ammunition

## Actual Results
- (To be filled during testing)

## Pass/Fail Criteria
- [ ] Market panel updates when selecting different planets
- [ ] At least 3 distinct price sets exist across all planets
- [ ] Agricultural planet (Earth) has lowest Water prices
- [ ] Mining planet (Mercury/Mars) has lowest Metals prices
- [ ] Mega City (Jupiter) has lowest Electronics prices
- [ ] No JavaScript errors during planet selection
- [ ] Price differences are significant enough for strategic gameplay (>20% variation)

## Notes
- **Known Bug**: As of 2026-03-21, all planets show identical prices (see BUG-INVESTIGATION-003.md)
- This scenario should fail until the bug is fixed
- After fix, this scenario becomes a regression test
