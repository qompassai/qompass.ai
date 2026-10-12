import { expect, test } from '@playwright/test';

test('home page has the Qompass AI title and the hero canvas', async ({ page }) => {
  await page.goto('/');
  await expect(page).toHaveTitle(/Qompass AI/);
  await expect(page.locator('#qompass-hero')).toBeAttached();
});

test('stylesheet is applied: body background is the abyss token', async ({ page }) => {
  // Regression guard: a prerender once shipped pages whose stylesheet
  // link was missing, so the site rendered unstyled while every
  // status code stayed green. The --abyss token (#04070F) reaching
  // the body's computed background proves the stylesheet loaded.
  await page.goto('/');
  const background = await page.evaluate(
    () => getComputedStyle(document.body).backgroundColor,
  );
  expect(background).toBe('rgb(4, 7, 15)');
});

test('cv page renders the Coverage section with press cards', async ({ page }) => {
  await page.goto('/cv');
  await expect(page.getByRole('heading', { name: 'Coverage' })).toBeVisible();
  const cards = page.locator('.press-card');
  await expect(cards.first()).toBeVisible();
  expect(await cards.count()).toBeGreaterThanOrEqual(1);
});

test('home page loads with zero page errors and zero console errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/');
  await page.waitForLoadState('networkidle');
  expect(errors).toEqual([]);
});
