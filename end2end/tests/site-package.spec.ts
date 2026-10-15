import { expect, test } from '@playwright/test';

// The 2026-10-15 site package: one blended compass readout, solution
// pages on this site, versions in place of status, the cardinal cards
// on /solutions, and the qubit hero. The table below is an independent
// oracle for the 44 solutions — written from the package order, not
// read from src/pages/compass_services.rs. Two carry no version: the
// archived rose.nvim, and the telco-admin quizzes, which live here.
const GH = 'https://github.com/qompassai/';
const LR = 'https://luarocks.org/modules/phaedrusflow/';
const gh = (slug: string, name: string, repo: string) =>
  ({ slug, name, version: '1.00', versionHref: `${GH}${repo}/releases/tag/v1.00` });
const lr = (slug: string, name: string, rock: string, version = '0.1.0-1') =>
  ({ slug, name, version, versionHref: `${LR}${rock}` });

const SOLUTIONS: { slug: string; name: string; version: string | null; versionHref: string | null }[] = [
  gh('qompass-hold', 'Qompass-Hold', 'qompass-hold'),
  gh('qssl', 'Qssl', 'qssl'),
  gh('radar', 'Radar', 'radar'),
  gh('redteam', 'Redteam', 'redteam'),
  gh('tor', 'Tor', 'tor'),
  gh('vongola', 'Vongola', 'vongola'),
  gh('fathom', 'fathom', 'fathom'),
  lr('diver-security', 'diver-security', 'diver-security'),
  gh('blaze-ts-nvim', 'blaze-ts.nvim', 'blaze-ts.nvim'),
  gh('blaze-nvim', 'blaze.nvim', 'blaze.nvim'),
  gh('diver', 'Diver', 'Diver'),
  gh('pathfinders', 'PathFinders', 'PathFinders'),
  gh('qluv', 'Qluv', 'qluv'),
  { slug: 'rose-nvim', name: 'Rose.nvim', version: null, versionHref: null },
  gh('tuxtype', 'Tuxtype', 'tuxtype'),
  gh('unreal', 'Unreal', 'unreal'),
  gh('eigenstyle', 'eigenstyle', 'eigenstyle'),
  lr('blaze-nvim-luarocks', 'blaze.nvim (LuaRocks)', 'blaze.nvim', '1.0-1'),
  lr('diver-research', 'diver-research', 'diver-research'),
  { slug: 'telco-admin-quizzes', name: 'Telco-Admin Practice Quizzes', version: null, versionHref: null },
  gh('azimuth', 'Azimuth', 'Azimuth'),
  gh('current', 'Current', 'current'),
  gh('dotfiles', 'Dotfiles', 'dotfiles'),
  gh('qonfig', 'Qonfig', 'qonfig'),
  gh('qudaz', 'Qudaz', 'qudaz'),
  gh('qxet', 'Qxet', 'qxet'),
  gh('rose', 'Rose', 'rose'),
  gh('lumen', 'lumen', 'lumen'),
  gh('phlow-mcp-server', 'phlow MCP server', 'phlow'),
  lr('diver-ai', 'diver-ai', 'diver-ai'),
  lr('diver-comms', 'diver-comms', 'diver-comms'),
  lr('diver-config', 'diver-config', 'diver-config'),
  lr('diver-dap', 'diver-dap', 'diver-dap'),
  lr('diver-dev', 'diver-dev', 'diver-dev'),
  lr('diver-games', 'diver-games', 'diver-games'),
  lr('diver-lintfmt', 'diver-lintfmt', 'diver-lintfmt'),
  lr('diver-utils', 'diver-utils', 'diver-utils'),
  lr('hyprlua', 'hyprlua', 'hyprlua'),
  gh('beacon', 'beacon', 'beacon'),
  gh('bunker', 'bunker', 'bunker'),
  gh('debian', 'debian', 'debian'),
  gh('network', 'network', 'network'),
  gh('nur', 'NUR', 'NUR'),
  gh('qjp', 'qjp', 'qjp'),
];

test('(a) home has no static cardinal cards and no In Progress', async ({ page }) => {
  await page.goto('/');
  await expect(page.locator('.cs-directory')).toHaveCount(0);
  await expect(page.locator('.cs-group')).toHaveCount(0);
  await expect(page.locator('.cs-status')).toHaveCount(0);
  expect(await page.locator('body').innerText()).not.toContain('In Progress');
});

test('(a) the readout is one list: no cardinal header, no percentile', async ({ page }) => {
  await page.goto('/');
  const readout = page.locator('.compass-services-readout');
  await expect(readout.locator('ul')).toHaveCount(1);
  await expect(readout.locator('.pillar-mark, .cs-r-head, .cs-r-share, .cs-r-bar')).toHaveCount(0);
  const text = await readout.innerText();
  expect(text).not.toMatch(/\d\s*%/);
  expect(text).not.toMatch(/NORTH|EAST|SOUTH|WEST/);
  expect(text).not.toContain('In Progress');
});

test('(a) the blend still moves: at NE the readout joins two cardinals, still one list', async ({ page }) => {
  await page.goto('/');
  const readout = page.locator('.compass-services-readout');
  // Near 000: Security alone, its 8 solutions.
  await expect(readout.locator('a.cs-solution')).toHaveCount(8);
  // Steer to 045 with the NE tick. The tick turns with the idle sweep,
  // so it is never "stable" for a click: focus (which pauses the sweep)
  // and press Enter, which also stops the sweep for good.
  const ne = page.getByRole('button', { name: 'Set bearing to 045 degrees — NE' });
  await ne.focus();
  await ne.press('Enter');
  await expect(readout.locator('.cs-r-bearing')).toHaveText(/^045°/);
  // Security (8) then Education (12), one list, no percent anywhere.
  await expect(readout.locator('a.cs-solution')).toHaveCount(20);
  await expect(readout.locator('ul')).toHaveCount(1);
  expect(await readout.innerText()).not.toMatch(/\d\s*%/);
  const hrefs = await readout.locator('a.cs-solution').evaluateAll((as) => as.map((a) => a.getAttribute('href')));
  expect(hrefs[0]).toBe('/solutions/qompass-hold');
  expect(hrefs[8]).toBe('/solutions/blaze-ts-nvim');
  for (const href of hrefs) expect(href).toMatch(/^\/solutions\/[a-z0-9-]+$/);
});

test('(b) all 44 solution links point to /solutions/<slug>', async ({ page }) => {
  await page.goto('/solutions');
  const links = page.locator('.cs-directory a.cs-solution');
  await expect(links).toHaveCount(44);
  const got = await links.evaluateAll((as) => as.map((a) => [a.getAttribute('href'), a.textContent?.trim()]));
  expect(got).toEqual(SOLUTIONS.map((s) => [`/solutions/${s.slug}`, s.name]));
  // No solution link escapes to GitHub or LuaRocks.
  const outward = await page.locator('.cs-directory a.cs-solution[href^="http"]').count();
  expect(outward).toBe(0);
});

test('(b) versions on /solutions link to the release; rose.nvim and the quizzes carry none', async ({ page }) => {
  await page.goto('/solutions');
  for (const s of SOLUTIONS) {
    const item = page.locator('.cs-directory li', { has: page.locator(`a.cs-solution[href="/solutions/${s.slug}"]`) });
    const version = item.locator('a.cs-version');
    if (s.version === null) {
      await expect(version).toHaveCount(0);
    } else {
      await expect(version).toHaveText(s.version);
      await expect(version).toHaveAttribute('href', s.versionHref!);
    }
  }
  // No fabricated 1.00 on a LuaRocks entry.
  const rockOnes = await page.locator(`.cs-directory a.cs-version[href^="${LR}"]`, { hasText: '1.00' }).count();
  expect(rockOnes).toBe(0);
});

test('(b) every /solutions/<slug> returns 200 with its name and version', async ({ page, request }) => {
  test.setTimeout(120_000);
  for (const s of SOLUTIONS) {
    const res = await request.get(`/solutions/${s.slug}`);
    expect(res.status(), s.slug).toBe(200);
    await page.goto(`/solutions/${s.slug}`);
    await expect(page.locator('h1'), s.slug).toHaveText(s.name);
    const version = page.locator('.study-meta a.cs-version');
    if (s.version === null) {
      await expect(version, s.slug).toHaveCount(0);
      // The whole article, not just .study-meta: the quizzes page has none.
      await expect(page.locator('.project-article'), s.slug).not.toContainText('Version');
      if (s.slug === 'rose-nvim') {
        await expect(page.locator('.project-article'), s.slug).toContainText('now lives inside Diver');
      }
    } else {
      await expect(version, s.slug).toHaveText(s.version);
      await expect(version, s.slug).toHaveAttribute('href', s.versionHref!);
    }
  }
});

test('(b) an unknown or malformed slug is off chart, never a solution page', async ({ page }) => {
  // (A dot-segment like %2e%2e is normalized away by the browser before
  // it reaches the route, so it is not a slug and is not tried here.)
  for (const slug of ['not-a-solution', 'Diver', 'blaze.nvim', 'rose-nvim-', '..%2fdiver', 'diver%00']) {
    await page.goto(`/solutions/${slug}`);
    await expect(page.locator('.cs-version'), slug).toHaveCount(0);
    await expect(page.locator('.study-meta'), slug).toHaveCount(0);
    const h1Texts = await page.locator('h1').allInnerTexts();
    expect(h1Texts.join(' '), slug).not.toMatch(/^(Diver|blaze\.nvim|Rose\.nvim)$/);
  }
});

test('(c) /solutions shows the four cardinal cards below the eight fleet entries', async ({ page }) => {
  await page.goto('/solutions');
  await expect(page.locator('.entry-list > .entry')).toHaveCount(8);
  const cards = page.locator('.cs-directory > .cs-group');
  await expect(cards).toHaveCount(4);
  await expect(cards.locator('h2')).toHaveText(['Security', 'Education', 'Deployment', 'Infrastructure']);
  const below = await page.evaluate(() => {
    const list = document.querySelector('.entry-list');
    const dir = document.querySelector('.cs-directory');
    return !!list && !!dir && (list.compareDocumentPosition(dir) & Node.DOCUMENT_POSITION_FOLLOWING) !== 0;
  });
  expect(below).toBe(true);
  await expect(page.locator('.cs-status')).toHaveCount(0);
  expect(await page.locator('body').innerText()).not.toContain('In Progress');
});

test('(d) the hero canvas is present and the qubit runs without errors', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (m) => { if (m.type() === 'error') errors.push(`console: ${m.text()}`); });
  await page.goto('/');
  const hero = page.locator('canvas#qompass-hero');
  await expect(hero).toBeAttached();
  const box = await hero.boundingBox();
  expect(box && box.width > 0 && box.height > 0).toBe(true);
  // Long enough for a hold, a burst, and a reconverge onto the next mark.
  await page.waitForTimeout(6_000);
  expect(errors).toEqual([]);
});

test('(d) reduced motion: hero present, static, without errors', async ({ browser }) => {
  const context = await browser.newContext({ reducedMotion: 'reduce' });
  const page = await context.newPage();
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (m) => { if (m.type() === 'error') errors.push(`console: ${m.text()}`); });
  // Only the first mark is fetched under reduced motion.
  const marks: string[] = [];
  page.on('request', (r) => { if (r.url().includes('/img/projects/')) marks.push(new URL(r.url()).pathname); });
  await page.goto('/');
  await expect(page.locator('canvas#qompass-hero')).toBeAttached();
  await page.waitForTimeout(2_000);
  expect(errors).toEqual([]);
  expect(marks).toEqual(['/img/projects/bunker.webp']);
  await context.close();
});
