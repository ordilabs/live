const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const context = await browser.newContext();
  const page = await context.newPage();

  await page.goto('http://live-ol.local:3000');

  // Click first inscription
  await page.click('ul > li:first-child > a');

  // Check that an image is displayed
  const element = await page.$('img');
  if (!element) {
    console.error('Expected element not found');
  } else {
    console.log('Test passed');
  }

  await browser.close();
})();
