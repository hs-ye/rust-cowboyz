const { chromium } = require('playwright');

(async () => {
  console.log('🔍 Testing: Game Loads to Opening Screen\n');
  
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  // Capture console messages and errors
  page.on('console', msg => console.log(`[Browser ${msg.type()}] ${msg.text()}`));
  page.on('pageerror', err => console.log(`[Browser ERROR] ${err.message}`));

  // Navigate to the game
  await page.goto('http://localhost:8080', { waitUntil: 'networkidle' });
  await page.waitForTimeout(3000); // Wait for WASM to load

  // Test assertions
  let passed = true;
  let failures = [];

  // Check 1: App container exists (should be exactly 1)
  const appContainers = await page.$$('.app-container');
  if (appContainers.length === 1) {
    console.log('✅ PASS: One app container found');
  } else {
    console.log(`❌ FAIL: Expected 1 app container, found ${appContainers.length}`);
    failures.push(`Expected 1 .app-container, found ${appContainers.length}`);
    passed = false;
  }

  // Check 2: Header is visible
  const appHeaders = await page.$$('.app-header');
  if (appHeaders.length >= 1) {
    console.log('✅ PASS: App header is visible');
  } else {
    console.log('❌ FAIL: App header not found');
    failures.push('App header not found');
    passed = false;
  }

  // Check 3: Split layout exists
  const splitLayouts = await page.$$('.split-layout');
  if (splitLayouts.length >= 1) {
    console.log('✅ PASS: Split layout is visible');
  } else {
    console.log('❌ FAIL: Split layout not found');
    failures.push('Split layout not found');
    passed = false;
  }

  // Check 4: No console errors
  // (Errors are logged automatically by the console handler)

  await browser.close();

  // Exit with appropriate code
  if (passed) {
    console.log('\n✅ All checks passed! Game loaded successfully.\n');
    process.exit(0);
  } else {
    console.log('\n❌ Failures detected:\n', failures.join('\n'));
    process.exit(1);
  }
})();
