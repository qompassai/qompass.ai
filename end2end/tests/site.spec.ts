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

test('on-ai page renders all three tracks and the filters work', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/on-ai');
  await expect(page.getByRole('heading', { name: 'On AI' })).toBeVisible();
  // All three tracks present in the server-rendered timeline.
  await expect(page.locator('.tl-entry[data-track="protocols"]').first()).toBeVisible();
  await expect(page.locator('.tl-entry[data-track="models"]').first()).toBeVisible();
  await expect(page.locator('.tl-entry[data-track="law"]').first()).toBeVisible();
  // Real SVG diagrams present: four figures, each with a caption and a
  // visibly dimensioned SVG (wide/narrow variants swap by viewport).
  await expect(page.locator('figure.fig')).toHaveCount(4);
  await expect(page.locator('figure.fig figcaption')).toHaveCount(4);
  const visibleSvgs = page.locator('figure.fig svg:visible');
  await expect(visibleSvgs).toHaveCount(4);
  for (const svg of await visibleSvgs.all()) {
    const box = await svg.boundingBox();
    expect(box).not.toBeNull();
    expect(box!.width).toBeGreaterThan(200);
    expect(box!.height).toBeGreaterThan(60);
  }
  // The stack diagram (second figure) carries all five layers as SVG text.
  const stackText = await page.locator('figure.fig').nth(1).locator('svg:visible').textContent();
  for (const layer of ['Models', 'Tools — MCP', 'Agent-to-agent — A2A', 'Payments — AP2', 'Governance']) {
    expect(stackText).toContain(layer);
  }
  // Filter interaction: Law & Policy only.
  await page.locator('.tl-filter[data-filter="law"]').click();
  await expect(page.locator('.tl-entry[data-track="law"]').first()).toBeVisible();
  await expect(page.locator('.tl-entry[data-track="models"]').first()).toBeHidden();
  await expect(page.locator('.tl-entry[data-track="protocols"]').first()).toBeHidden();
  await page.locator('.tl-filter[data-filter="all"]').click();
  await expect(page.locator('.tl-entry[data-track="models"]').first()).toBeVisible();
  await page.waitForLoadState('networkidle');
  expect(errors).toEqual([]);
});

test('on-ai diagrams stay legible at phone width', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/on-ai');
  // Narrow variants swap in: still exactly four visible diagram SVGs.
  await expect(page.locator('figure.fig svg:visible')).toHaveCount(4);
  const overflow = await page.evaluate(
    () => document.documentElement.scrollWidth - document.documentElement.clientWidth,
  );
  expect(overflow).toBeLessThanOrEqual(0);
  // Effective label size: authored font size scaled by the rendered
  // width over viewBox width must stay at reading size on the phone.
  const minEffective = await page.evaluate(() => {
    let min = Number.POSITIVE_INFINITY;
    document.querySelectorAll<SVGSVGElement>('figure.fig svg').forEach((svg) => {
      const rendered = svg.getBoundingClientRect().width;
      if (rendered === 0) return;
      const scale = rendered / svg.viewBox.baseVal.width;
      svg.querySelectorAll('text').forEach((t) => {
        const size = parseFloat(getComputedStyle(t).fontSize);
        min = Math.min(min, size * scale);
      });
    });
    return min;
  });
  expect(minEffective).toBeGreaterThanOrEqual(10.8);
  expect(errors).toEqual([]);
});

test('cv page closes with the Phaedrus coda', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (err) => errors.push(err.message));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });
  await page.goto('/cv');
  await expect(page).toHaveTitle(/CV/);
  const coda = page.locator('.cv-coda');
  await expect(coda).toBeVisible();
  await expect(coda.locator('.page-bearing')).toHaveText('OFF THE CHART');
  const img = coda.locator('.phaedrus-img');
  await img.scrollIntoViewIfNeeded();
  await expect(img).toBeVisible();
  await expect
    .poll(async () => img.evaluate((el: HTMLImageElement) => el.naturalWidth))
    .toBeGreaterThan(0);
  await expect(coda.locator('.phaedrus-name')).toContainText('Phaedrus');
  await expect(coda.locator('.phaedrus-name')).toContainText('Milwaukee-Eight 117');
  await expect(coda.locator('.phaedrus-ring')).toHaveCount(1);
  await expect(coda.locator('.phaedrus-route path')).toHaveCount(1);
  const anim = await coda
    .locator('.phaedrus-ring')
    .evaluate((el) => getComputedStyle(el).animationName);
  expect(anim).toContain('phaedrus-spin');
  expect(errors).toEqual([]);
});

const ALL_ROUTES = [
  '/',
  '/research',
  '/projects',
  '/projects/phlow',
  '/on-ai',
  '/tiger-style',
  '/tiger-style/rust',
  '/tiger-style/lua',
  '/cv',
  '/about',
  '/contact',
  '/privacy',
];

test('security headers are set on every route', async ({ request }) => {
  for (const route of ALL_ROUTES) {
    const res = await request.get(route);
    expect(res.status(), route).toBe(200);
    const h = res.headers();
    expect(h['strict-transport-security'], route).toContain('max-age=31536000');
    expect(h['x-content-type-options'], route).toBe('nosniff');
    expect(h['x-frame-options'], route).toBe('DENY');
    expect(h['referrer-policy'], route).toBe('strict-origin-when-cross-origin');
    expect(h['cross-origin-opener-policy'], route).toBe('same-origin');
    expect(h['cross-origin-resource-policy'], route).toBe('same-origin');
    expect(h['permissions-policy'], route).toContain('camera=()');
    expect(h['permissions-policy'], route).toContain('interest-cohort=()');
    const csp = h['content-security-policy'] ?? '';
    for (const directive of [
      "default-src 'self'",
      "object-src 'none'",
      "base-uri 'self'",
      "form-action 'self'",
      "frame-ancestors 'none'",
      "img-src 'self' data:",
      "font-src 'self'",
      "connect-src 'self' https://pub.orcid.org",
    ]) {
      expect(csp, `${route}: ${directive}`).toContain(directive);
    }
    // Script injection stays impossible: the script-src directive itself
    // carries no unsafe-* escape hatches (style attributes are the one
    // scoped exception, and they live in style-src-attr, not here).
    const scriptSrc = csp
      .split(';')
      .map((s) => s.trim())
      .find((s) => s.startsWith('script-src'));
    expect(scriptSrc, route).toBeDefined();
    // Quoted tokens: 'wasm-unsafe-eval' is present on evidence and must
    // not trip these — the bare 'unsafe-eval'/'unsafe-inline' sources are
    // what is forbidden.
    expect(scriptSrc, route).not.toContain("'unsafe-inline'");
    expect(scriptSrc, route).not.toContain("'unsafe-eval'");
  }
});

test('no CSP violations on any route, including interactions', async ({ page }) => {
  const violations: string[] = [];
  const pageErrors: string[] = [];
  let where = '';
  page.on('console', (message) => {
    if (message.type() === 'error' && /content security policy/i.test(message.text())) {
      violations.push(`${where}: ${message.text()}`);
    }
  });
  page.on('pageerror', (error) => pageErrors.push(`${where}: ${error.message}`));
  await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

  const sweep = async (route: string, interact?: () => Promise<void>) => {
    where = route;
    await page.goto(route);
    await page.waitForLoadState('networkidle');
    if (interact) await interact();
    await page.waitForLoadState('networkidle');
  };

  await sweep('/', async () => {
    // Quickstart is hydration-driven: the active chip moving proves the
    // WASM bundle instantiated and hydrated under the CSP.
    const chips = page.locator('.qs-chip');
    await chips.nth(1).click();
    await expect(chips.nth(1)).toHaveClass(/qs-chip-active/);
    await page.locator('.qs-copy').click();
    await expect(page.locator('#qompass-hero')).toBeAttached();
  });
  await sweep('/research', async () => {
    // The ORCID island refreshes from pub.orcid.org (connect-src).
    await page.waitForTimeout(2000);
    await expect(page.locator('#orcid-works')).toBeAttached();
  });
  await sweep('/projects');
  await sweep('/projects/phlow', async () => {
    const slider = page.locator('.ix-slider');
    await slider.fill('2');
    await expect(page.locator('[data-ix="success"]')).toHaveText('0.774 ± 0.006');
    await slider.fill('0');
    await page.locator('[data-ix="noise"]').check();
  });
  await sweep('/on-ai', async () => {
    await page.locator('.tl-filter[data-filter="law"]').click();
    await page.locator('.tl-filter[data-filter="all"]').click();
  });
  await sweep('/tiger-style', async () => {
    await page.locator('#chapter-rust summary').click();
    await expect(page.locator('#chapter-rust .ts-guide')).toBeVisible();
  });
  await sweep('/tiger-style/rust');
  await sweep('/tiger-style/lua');
  await sweep('/cv', async () => {
    await page.locator('.cv-coda').scrollIntoViewIfNeeded();
  });
  await sweep('/about');
  await sweep('/contact');

  expect(violations).toEqual([]);
  expect(pageErrors).toEqual([]);
});

test('phlow explorer passes data to the DOM only as text', async ({ page }) => {
  await page.goto('/projects/phlow');
  const slot = page.locator('[data-interactive="phlow-stale-explorer"]');
  await expect(slot.locator('.ix-slider')).toBeAttached();
  // The mount builds no active elements, and the plot labels (set via
  // textContent after mount) carry the measured points.
  expect(await slot.locator('script, img, iframe, object, embed, form').count()).toBe(0);
  await expect(slot.locator('[data-ix-point-label="1"]')).toHaveText('p = 0.10');
  const slider = slot.locator('.ix-slider');
  for (const value of ['0', '1', '2']) {
    await slider.fill(value);
    for (const key of ['success', 'rounds', 'note', 'plabel']) {
      const kids = await slot.locator(`[data-ix="${key}"]`).evaluate((el) => el.childElementCount);
      expect(kids, `${key} at slider ${value}`).toBe(0);
    }
  }
  await slider.fill('0');
  await slot.locator('[data-ix="noise"]').check();
  await expect(slot.locator('[data-ix="note"]')).toContainText('Perception noise');
  const noteKids = await slot.locator('[data-ix="note"]').evaluate((el) => el.childElementCount);
  expect(noteKids).toBe(0);
});

test('KaTeX renders hostile equation source inertly (trust: false)', async ({ page }) => {
  await page.goto('/projects/phlow');
  await expect(page.locator('.math-block .katex').first()).toBeAttached();
  const verdict = await page.evaluate(() => {
    const article = document.querySelector('.project-article');
    if (!article) return 'no article element';
    const span = document.createElement('span');
    span.textContent = String.raw`\(\htmlClass{pwned}{x}\) \[\href{javascript:alert(1)}{y}\]`;
    article.appendChild(span);
    const render = (window as unknown as { renderMath?: () => boolean }).renderMath;
    if (typeof render !== 'function') return 'renderMath not exposed';
    render();
    if (article.querySelector('.pwned')) return 'htmlClass reached the DOM';
    if (article.querySelector('a[href^="javascript:"]')) return 'javascript: link reached the DOM';
    return 'inert';
  });
  expect(verdict).toBe('inert');
});

test('tiger style page renders the standing orders with 17 chapters', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/tiger-style');
  await expect(page.getByRole('heading', { name: 'Tiger Style' })).toBeVisible();
  await expect(page.locator('.ts-subhead')).toHaveText('The orders the work is done under.');
  await expect(page.locator('.ts-chapter')).toHaveCount(17);
  // A chapter expands in place (native details, no JS needed).
  const rust = page.locator('#chapter-rust');
  await expect(rust.locator('.ts-guide')).toBeHidden();
  await rust.locator('summary').click();
  await expect(rust.locator('.ts-guide')).toBeVisible();
  await expect(rust.locator('.ts-guide')).toContainText('Bound everything');
  // The full reference is a quiet standalone page linked from the chapter.
  await expect(rust.locator('.ts-ref-link a')).toHaveAttribute('href', '/tiger-style/rust');
  await page.goto('/tiger-style/rust');
  await expect(page.locator('.ts-guide')).toContainText('Tiger Style for Rust');
  await page.waitForLoadState('networkidle');
  expect(errors).toEqual([]);
});

test('privacy page is linked from the footer and states the policy', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (err) => errors.push(`pageerror: ${err.message}`));
  await page.goto('/');
  const link = page.locator('.site-footer a[href="/privacy"]');
  await expect(link).toHaveCount(1);
  await link.click();
  await expect(page).toHaveURL(/\/privacy$/);
  await expect(page.getByRole('heading', { name: 'Privacy Policy' })).toBeVisible();
  await expect(page.locator('.prose')).toContainText('collects no personal data');
  await expect(page.locator('.prose')).toContainText('ai.qompass.lightshow');
  await expect(page.locator('.prose a[href="mailto:map@qompass.ai"]')).toHaveCount(1);
  expect(errors).toEqual([]);
});
