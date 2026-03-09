# Playtest Scenario: Duplicate UI Rendering

## Objective
Verify that the web UI renders correctly without duplicate interfaces.

## Preconditions
- Development server running: `trunk serve --port 8080`
- Playwright installed: `npm install playwright`
- Chromium browser: `npx playwright install chromium`

## Automated Test Script
```bash
node verify-fix.js
```

## Test Script
The test script `verify-fix.js` has been created:

```javascript
const { chromium } = require('playwright');

(async () => {
  console.log('🔍 Verifying duplicate UI fix...\n');
  
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  // Capture console messages and errors
  page.on('console', msg => console.log(`[Browser ${msg.type()}] ${msg.text()}`));
  page.on('pageerror', err => console.log(`[Browser ERROR] ${err.message}`));

  // Navigate to the game
  await page.goto('http://localhost:8080', { waitUntil: 'networkidle' });
  await page.waitForTimeout(3000); // Wait for WASM to load

  // Check for duplicate app containers
  const appContainers = await page.$$('.app-container');
  const appHeaders = await page.$$('.app-header');
  const splitLayouts = await page.$$('.split-layout');

  console.log('\n📊 Results:');
  console.log(`   .app-container count: ${appContainers.length}`);
  console.log(`   .app-header count: ${appHeaders.length}`);
  console.log(`   .split-layout count: ${splitLayouts.length}`);

  // Check if there are duplicates
  if (appContainers.length > 1) {
    console.log('\n❌ FAIL: Multiple app containers found!');
    console.log('   The duplicate UI bug is still present.\n');
    await browser.close();
    process.exit(1);
  } else if (appContainers.length === 1) {
    console.log('\n✅ PASS: Only one app container found!');
    console.log('   The duplicate UI bug has been fixed.\n');
    await browser.close();
    process.exit(0);
  } else {
    console.log('\n❌ FAIL: No app container found!');
    console.log('   The app may not have mounted correctly.\n');
    await browser.close();
    process.exit(1);
  }
})();
```

## Manual Test Steps (for reference)
1. Start the dev server with `trunk serve --port 8080`
2. Open http://localhost:8080 in a browser
3. Observe the initial page load
4. Scroll down the page to check for duplicate content

## Expected Results
- Single instance of "Space Cowboys" header
- Single set of UI panels (Map, Player Status, Ship Status, Inventory, Market)
- No duplicate content when scrolling
- Only one `.app-container` element in the DOM

## Pass/Fail Criteria
- [ ] Exactly 1 `.app-container` element exists
- [ ] Exactly 1 `.app-header` element exists
- [ ] Exactly 1 `.split-layout` element exists
- [ ] No duplicate content visible when scrolling

## Test Execution Log

### Test Run 1: Initial Bug Discovery
- **Date**: 2026-03-09
- **Result**: ❌ FAIL
- **Evidence**: Found 2 `.app-container` elements
- **Bug Reported**: Issue #101

### Test Run 2: After Fix Attempt 1
- **Date**: 2026-03-09
- **Result**: ❌ FAIL (still duplicating)
- **Notes**: First fix attempt (removing main from main_web.rs) was insufficient

### Test Run 3: After Final Fix
- **Date**: 2026-03-09
- **Result**: ✅ PASS
- **Evidence**: 
  ```
  📊 Results:
     .app-container count: 1
     .app-header count: 1
     .split-layout count: 1
  
  ✅ PASS: Only one app container found!
     The duplicate UI bug has been fixed.
  ```
- **Fix Applied**: Removed JavaScript event listener from index.html (PR #103)

## Root Cause Analysis (Historical)

### What
The web application rendered two identical copies of the App component.

### Where
Two entry points both calling `mount_to_body()`:
1. `src/lib.rs` - `start()` with `#[wasm_bindgen(start)]` (auto-runs)
2. `index.html` - JavaScript calling `window.wasmBindings.start()` (manual call)

### Why
The `#[wasm_bindgen(start)]` attribute already makes `start()` auto-execute when WASM loads. The additional JavaScript event listener in `index.html` caused a second call to `start()`, mounting the App twice.

### Fix Applied
Removed the JavaScript event listener from `index.html`:
```html
<!-- Removed this -->
<script>
    window.addEventListener("TrunkApplicationStarted", function() {
        if (window.wasmBindings && window.wasmBindings.start) {
            window.wasmBindings.start();
        }
    });
</script>
```

## Bug Ticket
- **Issue**: https://github.com/hs-ye/rust-cowboyz/issues/101
- **Title**: [Bug] Duplicate UI interface - two copies of App rendered
- **Label**: bug
- **Status**: Fixed (PR #103)

## Tester Notes
This bug was caught and verified using Playwright automated testing. The test script `verify-fix.js` should be kept as a regression test to ensure this issue doesn't reappear in future updates.

Date Tested: 2026-03-09
Tester: Playtester Agent
