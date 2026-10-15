import { readFileSync } from 'node:fs';
import { expect, test, type Page } from '@playwright/test';

// The telco-admin practice quizzes as an Education solution: the runner
// on /solutions/telco-admin-quizzes, Field Notes without them, and the
// answer keys only as AES-GCM envelopes opened in the browser by
// /js/answer-keys.js.
//
// The decrypt tests read a real password from the generator's private
// record on primo (scripts/generate-answer-keys.mjs). That file never
// enters the repo, so this spec runs on primo only.
const PASSWORD_FILE = '/home/phaedrus/telco-quiz-passwords-20261015.txt';
const PAGE = '/solutions/telco-admin-quizzes';
const SLUGS = [
  'general-theory-extended',
  'nec-250-grounding-bonding',
  'nec-300-398-wiring-methods',
  'nec-500-516-hazardous-locations',
  'nec-705-770-special-conditions',
  'nec-800-830-communications',
  'nec-100-110-general',
  'nec-mixed-02',
  'pretest-a',
  'pretest-b',
  'rcw-19-28',
  'wa-wac-296-46b',
];
const TITLES = [
  'General Trade Knowledge & Theory — Extended',
  'NEC Article 250 — Grounding & Bonding',
  'NEC Articles 300–398 — Wiring Methods',
  'NEC 2023 Articles 500–516 — Hazardous Locations Practice Test',
  'NEC 2023 Articles 705–770 — Special Conditions',
  'NEC 2023 Chapter 8 — Communications Systems Practice Test',
  'NEC 2023 Articles 90–110 — Introduction, Definitions, and General Requirements',
  'NEC 2023 + General Theory — Mixed Set 02',
  'WA Telecom Admin — Pretest A',
  'WA Telecom Admin — Pretest B',
  'RCW 19.28 — Electricians & Electrical Installations',
  'WAC 296-46B — WA Electrical Safety Standards',
];
// Verbatim explanations from the source sets (content/field-notes/quizzes/).
const PRETEST_A_EXPL =
  'Mandatory rules identify required or prohibited actions and are characterized by the terms shall or shall not.';
const SPECIAL_EXPL =
  'Equipment used to interconnect electric power production sources must be listed for the intended use.';
const WAC_EXPL =
  'Washington adopts the 2023 NEC (NFPA 70-2023) by reference, including Annexes A, B, and C specifically.';
const ERROR_TEXT = "That password doesn't open this key.";

function passwordFor(slug: string): string {
  const line = readFileSync(PASSWORD_FILE, 'utf8')
    .split('\n')
    .find((l) => l.startsWith(`${slug} `));
  const password = line?.slice(slug.length + 1).trim() ?? '';
  expect(password, `password for ${slug} on record`).toMatch(/^[a-z]+-[a-z]+-\d\d$/);
  return password;
}

function panel(page: Page, slug: string) {
  return page.locator(`#key-${slug}`);
}

async function unlock(page: Page, slug: string, password: string): Promise<void> {
  const form = panel(page, slug).locator(`form[data-key-slug="${slug}"]`);
  await expect(form).toBeVisible();
  await form.locator('input[type="password"]').fill(password);
  await form.getByRole('button', { name: 'Unlock' }).click();
}

test('the quizzes page: 200, a picker of 12 sets, 12 locked answer keys', async ({ page, request }) => {
  const res = await request.get(PAGE);
  expect(res.status()).toBe(200);
  await page.goto(PAGE);
  await expect(page.locator('h1')).toHaveText('Telco-Admin Practice Quizzes');
  await expect(page.locator('.fn-quiz-picker button')).toHaveCount(12);
  await expect(page.locator('.fn-quiz-picker .ft-mode-name')).toHaveText(TITLES);
  await expect(page.locator('.ak-panel')).toHaveCount(12);
  const slugs = await page.locator('form[data-key-slug]').evaluateAll((fs) =>
    fs.map((f) => f.getAttribute('data-key-slug')));
  expect(slugs).toEqual(SLUGS);
  await expect(page.locator('.ak-body').first()).toBeEmpty();
});

test('the runner still gives instant feedback with the explanation', async ({ page }) => {
  await page.goto(PAGE);
  await page.locator('.fn-quiz-picker button', { hasText: 'Pretest A' }).click();
  await expect(page.locator('.ft-stage .fn-choice')).toHaveCount(4);
  await page.locator('.ft-stage .fn-choice').first().click();
  await expect(page.locator('.ft-stage .ft-feedback')).toBeVisible();
  await expect(page.locator('.ft-stage .fn-q-expl')).toBeVisible();
});

test('Field Notes carries no quiz sets and no answer keys', async ({ page, request }) => {
  const html = await (await request.get('/field-notes')).text();
  expect(html).not.toContain('Answer keys');
  expect(html).not.toContain('fn-quiz');
  expect(html).not.toContain('data-key-slug');
  await page.goto('/field-notes');
  const text = await page.locator('main, body').first().innerText();
  for (const title of TITLES) expect(text, title).not.toContain(title);
});

test('the served page holds no answer explanation', async ({ request }) => {
  const html = await (await request.get(PAGE)).text();
  for (const expl of [PRETEST_A_EXPL, SPECIAL_EXPL, WAC_EXPL]) expect(html).not.toContain(expl);
});

test('an envelope is the v1 AES-GCM format and not plaintext', async ({ request }) => {
  const res = await request.get('/answer-keys/pretest-a.json');
  expect(res.status()).toBe(200);
  const body = await res.text();
  expect(body).not.toContain(PRETEST_A_EXPL);
  expect(body).not.toContain('Mandatory rules');
  const env = JSON.parse(body);
  expect(Object.keys(env)).toEqual(['v', 'kdf', 'iter', 'salt', 'iv', 'data']);
  expect(env).toMatchObject({ v: 1, kdf: 'PBKDF2-SHA256', iter: 310000 });
  expect(Buffer.from(env.salt, 'base64')).toHaveLength(16);
  expect(Buffer.from(env.iv, 'base64')).toHaveLength(12);
  // Ciphertext plus the 16-byte tag: never shorter than the tag alone.
  expect(Buffer.from(env.data, 'base64').length).toBeGreaterThan(16);
});

test('wrong password shows the error and nothing else; the right one opens the key', async ({ page }) => {
  await page.goto(PAGE);
  const key = panel(page, 'pretest-a');
  await unlock(page, 'pretest-a', 'not-the-password-00');
  await expect(key.locator('.ak-error')).toHaveText(ERROR_TEXT);
  await expect(key.locator('.ak-body')).toBeEmpty();

  await unlock(page, 'pretest-a', passwordFor('pretest-a'));
  await expect(key.locator('.ak-body')).toContainText(PRETEST_A_EXPL);
  await expect(key.locator('.ak-error')).toBeHidden();
  await expect(key.locator('form')).toBeHidden();
  await expect(key.locator('.ak-body .fn-key-q')).toHaveCount(30);
  // Only this panel opened.
  await expect(panel(page, 'pretest-b').locator('.ak-body')).toBeEmpty();

  await key.getByRole('button', { name: 'Hide' }).click();
  await expect(key.locator('.ak-body')).toBeEmpty();
  await expect(key.locator('form')).toBeVisible();
});

test("one set's password does not open another set's key", async ({ page }) => {
  await page.goto(PAGE);
  await unlock(page, 'pretest-b', passwordFor('pretest-a'));
  const key = panel(page, 'pretest-b');
  await expect(key.locator('.ak-error')).toHaveText(ERROR_TEXT);
  await expect(key.locator('.ak-body')).toBeEmpty();
});

test('a tampered or swapped envelope fails closed, even with the right password', async ({ page }) => {
  await page.goto(PAGE);
  const swapped = await (await page.request.get('/answer-keys/pretest-b.json')).text();
  await page.route('**/answer-keys/pretest-a.json', (route) =>
    route.fulfill({ contentType: 'application/json', body: swapped }));
  await unlock(page, 'pretest-a', passwordFor('pretest-a'));
  const key = panel(page, 'pretest-a');
  await expect(key.locator('.ak-error')).toHaveText(ERROR_TEXT);
  await expect(key.locator('.ak-body')).toBeEmpty();

  await page.unroute('**/answer-keys/pretest-a.json');
  const env = JSON.parse(await (await page.request.get('/answer-keys/pretest-a.json')).text());
  const data = Buffer.from(env.data, 'base64');
  data[0] ^= 1;
  const tampered = JSON.stringify({ ...env, data: data.toString('base64') });
  await page.route('**/answer-keys/pretest-a.json', (route) =>
    route.fulfill({ contentType: 'application/json', body: tampered }));
  await unlock(page, 'pretest-a', passwordFor('pretest-a'));
  await expect(key.locator('.ak-error')).toHaveText(ERROR_TEXT);
  await expect(key.locator('.ak-body')).toBeEmpty();
});

test('nothing is stored: no web storage, no cookies after unlocking', async ({ page, context }) => {
  await page.goto(PAGE);
  await unlock(page, 'pretest-a', passwordFor('pretest-a'));
  await expect(panel(page, 'pretest-a').locator('.ak-body')).toContainText(PRETEST_A_EXPL);
  const stored = await page.evaluate(() => [localStorage.length, sessionStorage.length, document.cookie]);
  expect(stored).toEqual([0, 0, '']);
  expect(await context.cookies()).toEqual([]);
});

test('without JavaScript a panel shows only its note', async ({ browser }) => {
  const context = await browser.newContext({ javaScriptEnabled: false });
  const page = await context.newPage();
  await page.goto(PAGE);
  const key = panel(page, 'pretest-a');
  await expect(key.locator('.ak-note')).toBeVisible();
  await expect(key.locator('form')).toBeHidden();
  await expect(key.locator('.ak-body')).toBeEmpty();
  await context.close();
});
