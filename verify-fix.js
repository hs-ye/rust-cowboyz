const { chromium } = require('playwright');

(async () => {
  console.log('🔍 Verifying duplicate UI fix...\n');
  
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  // Capture console messages
  page.on('console', msg => console.log(`[Browser ${msg.type()}] ${msg.text()}`));
  page.on('pageerror', err => console.log(`[Browser ERROR] ${err.message}`));

  // Navigate to the game
  console.log('📍 Navigating to http://localhost:8080...');
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
    
    // Additional verification - check body structure
    const bodyHTML = await page.evaluate(() => {
      const app = document.querySelector('#app');
      return {
        hasAppDiv: app !== null,
        appChildren: app ? app.children.length : 0,
        bodyChildren: document.body.children.length
      };
    });
    
    console.log('📋 Additional checks:');
    console.log(`   #app div exists: ${bodyHTML.hasAppDiv}`);
    console.log(`   #app children: ${bodyHTML.appChildren}`);
    console.log(`   body children: ${bodyHTML.bodyChildren}`);
    
    if (bodyHTML.bodyChildren === 1 && bodyHTML.appChildren === 1) {
      console.log('\n✅ All checks passed! UI structure is correct.\n');
    }
    
    await browser.close();
    process.exit(0);
  } else {
    console.log('\n❌ FAIL: No app container found!');
    console.log('   The app may not have mounted correctly.\n');
    await browser.close();
    process.exit(1);
  }
})();
