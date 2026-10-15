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

test('about page renders the Coverage section with press cards', async ({ page }) => {
  await page.goto('/about');
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

test('solutions index links exactly the ready study pages', async ({ page }) => {
  await page.goto('/solutions');
  const links = page.locator('.study-link a');
  await expect(links).toHaveCount(2);
  await expect(links.first()).toHaveAttribute('href', '/solutions/phlow');
  await expect(links.nth(1)).toHaveAttribute('href', '/light-show');
  // Download boxes: four platforms per project. Projects with a real
  // binary URL recorded in content/downloads.toml render as links;
  // the rest stay disabled "Coming soon" spans.
  await expect(page.locator('.download-box')).toHaveCount(32);
  await expect(page.locator('a.download-box')).toHaveCount(12);
  await expect(page.locator('.download-box.is-soon')).toHaveCount(20);
  await expect(page.locator('.download-state', { hasText: 'Coming soon' })).toHaveCount(20);
});

test('phlow study page renders all five sections with KaTeX math', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/solutions/phlow');
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

test('analysis page renders all three tracks and the filters work', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/analysis');
  await expect(page.getByRole('heading', { name: 'Analysis' })).toBeVisible();
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

test('analysis diagrams stay legible at phone width', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/analysis');
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
    document.querySelectorAll<SVGSVGElement>('figure.fig svg, figure.flow-fig svg').forEach((svg) => {
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

test('about page closes with the Phaedrus coda', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (err) => errors.push(err.message));
  page.on('console', (msg) => {
    if (msg.type() === 'error') errors.push(msg.text());
  });
  await page.goto('/about');
  await expect(page).toHaveTitle(/About/);
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
  '/solutions',
  '/solutions/phlow',
  '/analysis',
  '/standards',
  '/standards/rust',
  '/standards/lua',
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
    // The ORCID software list refreshes from pub.orcid.org (connect-src).
    await page.waitForTimeout(2000);
    await expect(page.locator('#orcid-software')).toBeAttached();
  });
  await sweep('/solutions');
  await sweep('/solutions/phlow', async () => {
    const slider = page.locator('.ix-slider');
    await slider.fill('2');
    await expect(page.locator('[data-ix="success"]')).toHaveText('0.774 ± 0.006');
    await slider.fill('0');
    await page.locator('[data-ix="noise"]').check();
  });
  await sweep('/analysis', async () => {
    await page.locator('.tl-filter[data-filter="law"]').click();
    await page.locator('.tl-filter[data-filter="all"]').click();
  });
  await sweep('/standards', async () => {
    await page.locator('#chapter-rust summary').click();
    await expect(page.locator('#chapter-rust .ts-guide')).toBeVisible();
  });
  await sweep('/standards/rust');
  await sweep('/standards/lua');
  await sweep('/about', async () => {
    await page.locator('.cv-coda').scrollIntoViewIfNeeded();
  });
  await sweep('/contact');

  expect(violations).toEqual([]);
  expect(pageErrors).toEqual([]);
});

test('phlow explorer passes data to the DOM only as text', async ({ page }) => {
  await page.goto('/solutions/phlow');
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
  await page.goto('/solutions/phlow');
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

test('standards page renders the standing orders with 17 chapters', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') {
      errors.push(`console: ${message.text()}`);
    }
  });
  await page.goto('/standards');
  await expect(page.getByRole('heading', { name: 'Standards' })).toBeVisible();
  await expect(page.locator('.ts-subhead')).toHaveText('The orders the work is done under.');
  await expect(page.locator('.ts-chapter')).toHaveCount(17);
  // A chapter expands in place (native details, no JS needed).
  const rust = page.locator('#chapter-rust');
  await expect(rust.locator('.ts-guide')).toBeHidden();
  await rust.locator('summary').click();
  await expect(rust.locator('.ts-guide')).toBeVisible();
  await expect(rust.locator('.ts-guide')).toContainText('Bound everything');
  // The full reference is a quiet standalone page linked from the chapter.
  await expect(rust.locator('.ts-ref-link a')).toHaveAttribute('href', '/standards/rust');
  await page.goto('/standards/rust');
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

// The flow: Standards (set) -> Analysis (weigh) -> Field Notes (learn),
// promoted into Research, which returns to revise the Standards.
const FLOW = [
  { route: '/standards', step: 'Step One — Set', prev: '/research', next: '/analysis' },
  { route: '/analysis', step: 'Step Two — Weigh', prev: '/standards', next: '/field-notes' },
  { route: '/field-notes', step: 'Step Three — Learn', prev: '/analysis', next: '/research' },
  { route: '/research', step: null, prev: '/field-notes', next: '/standards' },
];

test('the nav carries the flow consecutively: Standards, Analysis, Field Notes', async ({ page }) => {
  await page.goto('/');
  const hrefs = await page
    .locator('.site-nav a')
    .evaluateAll((links) => links.map((a) => a.getAttribute('href')));
  const at = hrefs.indexOf('/standards');
  expect(at, hrefs.join(' ')).toBeGreaterThanOrEqual(0);
  expect(hrefs.slice(at, at + 3)).toEqual(['/standards', '/analysis', '/field-notes']);
  await expect(page.locator('.site-nav a[href="/analysis"]')).toHaveText('Analysis');
  await expect(page.locator('.site-nav a[href="/on-ai"]')).toHaveCount(0);
});

test('every flow page carries its step line and its prev/next links', async ({ page }) => {
  for (const { route, step, prev, next } of FLOW) {
    await page.goto(route);
    if (step) await expect(page.locator('.page-bearing').first(), route).toHaveText(step);
    const links = page.locator('.fn-flow-nav a');
    await expect(links, route).toHaveCount(2);
    await expect(links.first(), route).toHaveAttribute('href', prev);
    await expect(links.nth(1), route).toHaveAttribute('href', next);
  }
  // The diagram is framing: one on Analysis, one compact on Research,
  // and Analysis keeps exactly its four content figures.
  await page.goto('/analysis');
  await expect(page.locator('figure.flow-fig')).toHaveCount(1);
  await expect(page.locator('figure.fig')).toHaveCount(4);
  await page.goto('/research');
  await expect(page.locator('figure.flow-fig.flow-fig-compact')).toHaveCount(1);
});

test('every flow link resolves to a live page', async ({ page, request }) => {
  const flowLinks = ['/standards', '/analysis', '/field-notes', '/research']
    .map((href) => `.page a[href="${href}"]`)
    .join(', ');
  const seen = new Set<string>();
  for (const { route } of FLOW) {
    await page.goto(route);
    const hrefs = await page
      .locator(flowLinks)
      .evaluateAll((links) => links.map((a) => a.getAttribute('href') ?? ''));
    hrefs.forEach((h) => seen.add(h));
  }
  expect([...seen].sort()).toEqual(['/analysis', '/field-notes', '/research', '/standards']);
  for (const href of seen) {
    const res = await request.get(href);
    expect(res.status(), href).toBe(200);
  }
});

test('/on-ai is a noindex meta-refresh onto /analysis, and nothing says On AI', async ({ page, request }) => {
  const res = await request.get('/on-ai');
  expect(res.status()).toBe(200);
  const body = await res.text();
  expect(body).toContain('<meta http-equiv="refresh" content="0; url=/analysis" />');
  expect(body).toContain('<link rel="canonical" href="https://qompass.ai/analysis" />');
  expect(body).toContain('<meta name="robots" content="noindex" />');
  expect(body).not.toContain('On AI');
  await page.goto('/on-ai');
  await expect(page).toHaveURL(/\/analysis\/?$/);
  await expect(page.getByRole('heading', { name: 'Analysis', exact: true })).toBeVisible();
  for (const route of ['/', '/analysis', '/standards', '/field-notes', '/research']) {
    await page.goto(route);
    expect(await page.title(), route).not.toContain('On AI');
    expect(await page.locator('body').innerText(), route).not.toContain('On AI');
  }
});

test('the flow pulse honors reduced motion; the static diagram stands alone', async ({ browser }) => {
  for (const reducedMotion of ['reduce', 'no-preference'] as const) {
    const context = await browser.newContext({ reducedMotion });
    const page = await context.newPage();
    await page.goto('/analysis');
    const fig = page.locator('figure.flow-fig');
    // The static parts carry the whole meaning: three nodes, the
    // Research peak, two arc legs, two base arrows.
    await expect(fig.locator('.fl-node')).toHaveCount(3);
    await expect(fig.locator('.fl-peak')).toHaveCount(1);
    await expect(fig.locator('.fl-arc')).toHaveCount(2);
    await expect(fig.locator('.fl-base')).toHaveCount(2);
    const pulses = await fig
      .locator('.fl-pulse')
      .evaluateAll((els) => els.map((el) => [getComputedStyle(el).animationName, getComputedStyle(el).opacity]));
    if (reducedMotion === 'reduce') {
      expect(pulses).toEqual([
        ['none', '0'],
        ['none', '0'],
      ]);
    } else {
      expect(pulses.map(([name]) => name)).toEqual(['fl-base', 'fl-arc']);
    }
    await context.close();
  }
});

const HOWTO_TITLES = [
  'Design the learning before the game',
  'Build it on your desk first',
  'Shape it for Android',
  'Sign it, and guard the key',
  'Open the Play Console and claim the package name',
  'Write the store listing',
  'Clear the policy gauntlet',
  'Run the testing tracks',
  'Ship beyond Play',
  'Ship, listen, and iterate',
];

test('light-show how-to: all ten steps read in order with JavaScript off', async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto('/light-show');
  const howto = page.locator('#how-to');
  await expect(howto.getByRole('heading', { name: 'How to build an educational game for Android' })).toBeVisible();
  await expect(howto.locator('.howto-title')).toHaveText(HOWTO_TITLES);
  // Every panel open: practice, worked example, and checklist visible.
  const panels = howto.locator('.howto-panel');
  await expect(panels).toHaveCount(10);
  for (let i = 0; i < 10; i++) {
    await expect(panels.nth(i)).toBeVisible();
    await expect(panels.nth(i).locator('.howto-example')).toBeVisible();
    await expect(panels.nth(i).locator('.howto-checks')).toBeVisible();
  }
  // The enhancement-only controls stay out of the way.
  await expect(howto.locator('.howto-tally')).toBeHidden();
  await expect(howto.locator('.howto-nav').first()).toBeHidden();
  // The how-to sits below the tester call to action, which is untouched.
  const order = await page.evaluate(() => {
    const cta = document.querySelector('a[href="/light-show/testers"]');
    const section = document.getElementById('how-to');
    return cta && section ? cta.compareDocumentPosition(section) & Node.DOCUMENT_POSITION_FOLLOWING : 0;
  });
  expect(order).toBeTruthy();
  await context.close();
});

test('light-show how-to: accordion, prev/next, and the tally work once hydrated', async ({ page }) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(`pageerror: ${error.message}`));
  page.on('console', (message) => {
    if (message.type() === 'error') errors.push(`console: ${message.text()}`);
  });
  await page.goto('/light-show');
  const howto = page.locator('#how-to.is-enhanced');
  await expect(howto).toBeAttached();
  const panels = howto.locator('.howto-panel');
  const toggles = howto.locator('.howto-toggle');
  const tally = howto.locator('.howto-tally');
  await expect(panels.nth(0)).toBeVisible();
  for (let i = 1; i < 10; i++) await expect(panels.nth(i)).toBeHidden();
  await expect(toggles.nth(0)).toHaveAttribute('aria-expanded', 'true');
  await expect(tally).toHaveText('0 of 10 steps complete');

  // Tick every box of step 1: the step completes and the tally moves.
  const boxes = panels.nth(0).locator('input[type="checkbox"]');
  const count = await boxes.count();
  for (let i = 0; i < count; i++) await boxes.nth(i).check();
  await expect(tally).toHaveText('1 of 10 steps complete');
  await expect(howto.locator('.howto-step').nth(0)).toHaveClass(/is-done/);

  // Next moves to step 2 and puts focus on its header.
  await panels.nth(0).getByRole('button', { name: 'Next step →' }).click();
  await expect(panels.nth(1)).toBeVisible();
  await expect(panels.nth(0)).toBeHidden();
  await expect(toggles.nth(1)).toBeFocused();
  await expect(toggles.nth(1)).toHaveAttribute('aria-expanded', 'true');
  await expect(toggles.nth(0)).toHaveAttribute('aria-expanded', 'false');

  // Keyboard: Enter on the step-10 header opens it alone.
  await toggles.nth(9).focus();
  await page.keyboard.press('Enter');
  await expect(panels.nth(9)).toBeVisible();
  await expect(panels.nth(1)).toBeHidden();
  await expect(panels.nth(9).getByRole('button', { name: 'Next step →' })).toHaveCount(0);
  await panels.nth(9).getByRole('button', { name: '← Previous step' }).click();
  await expect(panels.nth(8)).toBeVisible();
  expect(errors).toEqual([]);
});

// — adversarial —

test('light-show how-to: unticking undoes a step; partial ticks never count', async ({ page }) => {
  await page.goto('/light-show');
  const howto = page.locator('#how-to.is-enhanced');
  await expect(howto).toBeAttached();
  const tally = howto.locator('.howto-tally');
  const first = howto.locator('.howto-panel').nth(0).locator('input[type="checkbox"]');
  const count = await first.count();
  for (let i = 0; i < count; i++) await first.nth(i).check();
  await expect(tally).toHaveText('1 of 10 steps complete');
  await first.nth(0).uncheck();
  await expect(tally).toHaveText('0 of 10 steps complete');
  // One box in each of steps 2 and 3: still nothing complete.
  for (const step of [1, 2]) {
    await howto.locator('.howto-toggle').nth(step).click();
    await howto.locator('.howto-panel').nth(step).locator('input[type="checkbox"]').first().check();
  }
  await expect(tally).toHaveText('0 of 10 steps complete');
  // Closing the open step leaves every panel closed, and ticks survive.
  await howto.locator('.howto-toggle').nth(2).click();
  await expect(howto.locator('.howto-panel:visible')).toHaveCount(0);
  await howto.locator('.howto-toggle').nth(1).click();
  await expect(howto.locator('.howto-panel').nth(1).locator('input[type="checkbox"]').first()).toBeChecked();
});

test('light-show how-to: ticking transmits and stores nothing', async ({ page }) => {
  await page.goto('/light-show');
  const howto = page.locator('#how-to.is-enhanced');
  await expect(howto).toBeAttached();
  await page.waitForLoadState('networkidle');
  const requests: string[] = [];
  page.on('request', (request) => requests.push(request.url()));
  const boxes = howto.locator('.howto-panel').nth(0).locator('input[type="checkbox"]');
  const count = await boxes.count();
  for (let i = 0; i < count; i++) await boxes.nth(i).check();
  await howto.getByRole('button', { name: 'Next step →' }).first().click();
  await expect(howto.locator('.howto-tally')).toHaveText('1 of 10 steps complete');
  await page.waitForTimeout(500);
  expect(requests).toEqual([]);
  const stored = await page.evaluate(() => [localStorage.length, sessionStorage.length, document.cookie]);
  expect(stored).toEqual([0, 0, '']);
  // A reload starts the walk over: nothing persisted.
  await page.reload();
  await expect(page.locator('#how-to.is-enhanced .howto-tally')).toHaveText('0 of 10 steps complete');
});

test('light-show how-to: fits a 390px phone with nothing past the edge', async ({ page }) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/light-show');
  const howto = page.locator('#how-to.is-enhanced');
  await expect(howto).toBeAttached();
  // Open every step in turn; no element or text run in the how-to may
  // reach past the viewport (scrollWidth catches unbroken text too).
  for (let i = 0; i < 10; i++) {
    await howto.locator('.howto-toggle').nth(i).click();
    const offenders = await page.evaluate(() => {
      const width = document.documentElement.clientWidth;
      return Array.from(document.querySelectorAll('#how-to *'))
        .filter((el) => {
          const rect = el.getBoundingClientRect();
          return rect.width > 0 && (rect.right > width + 0.5 || rect.left < -0.5 || (el.clientWidth > 0 && el.scrollWidth > el.clientWidth + 1));
        })
        .map((el) => `${el.tagName.toLowerCase()}.${el.className}`);
    });
    expect(offenders, `step ${i + 1}`).toEqual([]);
  }
  // Move buttons stay thumb-sized.
  const prev = await howto.locator('.howto-panel').nth(9).getByRole('button', { name: '← Previous step' }).boundingBox();
  expect(prev?.height ?? 0).toBeGreaterThanOrEqual(44);
});
