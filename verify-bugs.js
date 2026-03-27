const { chromium } = require('playwright');

(async () => {
  console.log('🔍 Bug Verification: Planet Selection & Next Turn\n');

  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();

  page.on('console', msg => {
    if (msg.type() === 'error') console.log(`[ERROR] ${msg.text()}`);
  });
  page.on('pageerror', err => console.log(`[PAGE ERROR] ${err.message}`));

  await page.goto('http://localhost:8080', { waitUntil: 'networkidle' });
  await page.waitForTimeout(5000);

  console.log('=== BUG 1: Planet Selection ===\n');

  // Check canvas exists
  const canvas = await page.$('.solar-map-canvas');
  console.log(`Canvas found: ${!!canvas}`);

  if (canvas) {
    const box = await canvas.boundingBox();
    console.log(`Canvas size: ${box?.width.toFixed(0)}x${box?.height.toFixed(0)}`);

    // Get pixel data before click - sample around where selection ring should be
    // Venus is at ~370, 392, selection ring is at radius ~17 from center
    const pixelsBefore = await page.evaluate(() => {
      const canvas = document.querySelector('.solar-map-canvas');
      const ctx = canvas.getContext('2d');
      // Sample multiple points around where selection ring should be
      const ringPositions = [
        [370, 392 - 17], // Top of ring
        [370 + 17, 392], // Right of ring
        [370, 392 + 17], // Bottom of ring
        [370 - 17, 392]  // Left of ring
      ];
      const pixels = [];
      for (const [x, y] of ringPositions) {
        pixels.push(ctx.getImageData(x, y, 1, 1).data.slice(0, 4));
      }
      return { ring: pixels };
    });
    console.log('Selection ring pixels before click:', pixelsBefore);

    // Click on Venus (around 382, 404 based on console logs)
    await canvas.click({ position: { x: 382, y: 404 } });
    await page.waitForTimeout(500);

    // Get pixel data after click - should show white selection ring [255, 255, 255]
    const pixelsAfter = await page.evaluate(() => {
      const canvas = document.querySelector('.solar-map-canvas');
      const ctx = canvas.getContext('2d');
      // Sample same positions
      const ringPositions = [
        [370, 392 - 17], // Top of ring
        [370 + 17, 392], // Right of ring
        [370, 392 + 17], // Bottom of ring
        [370 - 17, 392]  // Left of ring
      ];
      const pixels = [];
      for (const [x, y] of ringPositions) {
        pixels.push(ctx.getImageData(x, y, 1, 1).data.slice(0, 4));
      }
      return { ring: pixels };
    });
    console.log('Selection ring pixels after click:', pixelsAfter);

    // Check if any pixel is white (selection ring)
    const hasWhiteRing = pixelsAfter.ring.some(p => p[0] > 250 && p[1] > 250 && p[2] > 250);

    console.log('Selection indicators after click:', { hasWhiteRing });
    if (hasWhiteRing) {
      console.log('✅ Planet selection works! White selection ring detected.');
    } else {
      console.log('❌ BUG CONFIRMED: Canvas click does not trigger planet selection');
    }
  }

  console.log('=== BUG 2: Next Turn Button ===\n');
  
  // Get initial turn
  const turnBefore = await page.evaluate(() => {
    const el = document.querySelector('.stat-value.turn');
    return el ? el.textContent.trim() : 'NOT FOUND';
  });
  console.log(`Turn before: ${turnBefore}`);
  
  // Get canvas pixel data before
  const pixelsBefore = await page.evaluate(() => {
    const canvas = document.querySelector('.solar-map-canvas');
    if (!canvas) return null;
    const ctx = canvas.getContext('2d');
    // Sample pixels from different areas where planets might be
    return {
      center: ctx.getImageData(350, 380, 10, 10).data.slice(0, 4),
      outer: ctx.getImageData(500, 380, 10, 10).data.slice(0, 4)
    };
  });
  console.log('Canvas pixels before turn:', pixelsBefore);
  
  // Click Next Turn
  const nextBtn = await page.$('button:has-text("Next Turn")');
  if (nextBtn) {
    await nextBtn.click();
    await page.waitForTimeout(1000);
  }
  
  // Get turn after
  const turnAfter = await page.evaluate(() => {
    const el = document.querySelector('.stat-value.turn');
    return el ? el.textContent.trim() : 'NOT FOUND';
  });
  console.log(`Turn after: ${turnAfter}`);
  
  // Get canvas pixel data after
  const pixelsAfter = await page.evaluate(() => {
    const canvas = document.querySelector('.solar-map-canvas');
    if (!canvas) return null;
    const ctx = canvas.getContext('2d');
    return {
      center: ctx.getImageData(350, 380, 10, 10).data.slice(0, 4),
      outer: ctx.getImageData(500, 380, 10, 10).data.slice(0, 4)
    };
  });
  console.log('Canvas pixels after turn:', pixelsAfter);
  
  // Check if pixels changed (planets should have moved)
  const pixelsChanged = pixelsBefore && pixelsAfter && (
    pixelsBefore.center[0] !== pixelsAfter.center[0] ||
    pixelsBefore.outer[0] !== pixelsAfter.outer[0]
  );
  
  console.log(`\nPixels changed: ${pixelsChanged}`);
  if (!pixelsChanged) {
    console.log('❌ BUG CONFIRMED: Canvas does not re-render when turn advances');
  } else {
    console.log('✅ Canvas re-renders on turn change');
  }

  await browser.close();
  console.log('\n=== Verification Complete ===\n');
})();
