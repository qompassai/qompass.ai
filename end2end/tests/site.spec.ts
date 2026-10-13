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

test('projects index links exactly the ready study pages', async ({ page }) => {
  await page.goto('/projects');
  const links = page.locator('.study-link a');
  await expect(links).toHaveCount(1);
  await expect(links.first()).toHaveAttribute('href', '/projects/phlow');
});

test('phlow study page renders all five sections with KaTeX math', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/projects/phlow');
  for (const label of ['ABSTRACT', 'METHODS', 'RESULTS', 'DISCUSSION', 'CONCLUSION']) {
    await expect(page.locator('.band-label', { hasText: label }).first()).toBeVisible();
  }
  // KaTeX rendered: .katex markup exists inside a math block and no raw
  // display delimiter remains visible in the article text.
  await expect(page.locator('.math-block .katex').first()).toBeAttached();
  const articleText = await page.locator('.project-article').innerText();
  expect(articleText).not.toContain('\\[');
  // Benchmark values match the verified content pack exactly.
  await expect(page.locator('.bench-table').nth(0)).toContainText('0.145 ± 0.025');
  await expect(page.locator('.bench-table').nth(1)).toContainText('0.098 ± 0.005');
  await expect(page.locator('.bench-table').nth(2)).toContainText('2653 passed / 0 failed');
  // The stale-frame explorer mounts and the slider moves the readout to
  // the third measured point.
  await expect(page.locator('[data-interactive="phlow-stale-explorer"]')).toBeAttached();
  const slider = page.locator('.ix-slider');
  await expect(slider).toBeAttached();
  await slider.fill('2');
  await expect(page.locator('[data-ix="success"]')).toHaveText('0.774 ± 0.006');
  await page.waitForLoadState('networkidle');
  expect(errors).toEqual([]);
});
