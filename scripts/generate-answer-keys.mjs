#!/usr/bin/env node
// Encrypt the telco-admin practice answer keys for
// /solutions/telco-admin-quizzes.
//
// Reads the set order from content/field-notes/quizzes.toml and each
// set's JSON from content/field-notes/quizzes/, builds the answer-key
// HTML fragment per set (the content the old static key on Field Notes
// showed: description, then per question its number, prompt, the
// choices with the correct one marked, the answer, the explanation,
// code sections, index lookup paths), and encrypts it with a fresh
// password per set.
//
// Crypto, matching public/js/answer-keys.js (WebCrypto):
//   key  = PBKDF2(password UTF-8, salt 16 B, 310000 iterations, SHA-256, 32 B)
//   data = AES-256-GCM(key, iv 12 B, fragment UTF-8) ciphertext || 16 B tag
// Output: public/answer-keys/<slug>.json =
//   {"v":1,"kdf":"PBKDF2-SHA256","iter":310000,"salt":b64,"iv":b64,"data":b64}
//
// Passwords go ONLY to PASSWORD_FILE (outside the repo, mode 0600); stdout
// lists the slugs, never a password. Re-running replaces every password
// and every envelope together, so the file and public/answer-keys/ always
// agree — relay the new passwords after a re-run.
//
// Run from anywhere, standard library only:
//   node scripts/generate-answer-keys.mjs
import { createCipheriv, pbkdf2Sync, randomBytes, randomInt } from 'node:crypto';
import { chmodSync, mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');
const CONTENT = join(ROOT, 'content', 'field-notes');
const OUT_DIR = join(ROOT, 'public', 'answer-keys');
const PASSWORD_FILE = '/home/phaedrus/telco-quiz-passwords-20261015.txt';

const ITERATIONS = 310000;
const SALT_BYTES = 16;
const IV_BYTES = 12;
const KEY_BYTES = 32;
const TAG_BYTES = 16;
const SETS_MAX = 64;
const QUESTIONS_MAX = 500;

// Ordinary concrete nouns; two per password, drawn with crypto randomness.
const WORDS = [
  'anchor', 'apple', 'badger', 'barrel', 'basket', 'beacon', 'bucket', 'cabin',
  'candle', 'canoe', 'carrot', 'castle', 'cedar', 'chisel', 'cliff', 'copper',
  'falcon', 'feather', 'ferry', 'garden', 'glove', 'granite', 'hammer', 'harbor',
  'helmet', 'kettle', 'ladder', 'lantern', 'maple', 'meadow', 'mitten', 'orchard',
  'otter', 'paddle', 'pebble', 'pepper', 'pillow', 'pine', 'pocket', 'quarry',
  'rabbit', 'ribbon', 'saddle', 'shovel', 'spruce', 'thimble', 'tractor', 'trumpet',
  'tunnel', 'walnut', 'wagon', 'willow',
];

function assert(condition, message) {
  if (!condition) throw new Error(`generate-answer-keys: ${message}`);
}

/** HTML-escape source text: & < > " '. */
function esc(text) {
  assert(typeof text === 'string', `expected a string, got ${typeof text}`);
  return text
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

/** The `order = [...]` list from quizzes.toml (the one key it carries). */
function readOrder() {
  const toml = readFileSync(join(CONTENT, 'quizzes.toml'), 'utf8');
  const block = toml.match(/^order\s*=\s*\[([^\]]*)\]/m);
  assert(block !== null, 'quizzes.toml has no order list');
  const names = [...block[1].matchAll(/"([^"]+)"/g)].map((m) => m[1]);
  assert(names.length > 0 && names.length <= SETS_MAX, `order lists ${names.length} sets`);
  for (const name of names) assert(/^[a-z0-9-]+\.json$/.test(name), `bad set file ${name}`);
  return names;
}

/** Explanation text, sections, and index paths, as build.rs reads them. */
function explanationOf(q) {
  if (typeof q.explanation === 'string') return { text: q.explanation, sections: [], index: [] };
  const correct = q.explanation?.correct;
  assert(correct && typeof correct.text === 'string', `question ${q.id}: bad explanation`);
  return { text: correct.text, sections: correct.sections ?? [], index: correct.indexKeywords ?? [] };
}

const LETTERS = ['A', 'B', 'C', 'D'];

function questionHtml(q) {
  assert(Number.isSafeInteger(q.id) && q.id > 0, `bad question id ${q.id}`);
  assert(Array.isArray(q.choices) && q.choices.length <= LETTERS.length, `question ${q.id}: choices`);
  const matches = q.choices.flatMap((c, i) => (c === q.answer ? [i] : []));
  assert(matches.length === 1, `question ${q.id}: answer matches ${matches.length} choices`);
  const answer = matches[0];
  const choices = q.choices
    .map((c, i) => (i === answer ? `<li class="ak-correct"><strong>${esc(c)}</strong> ✓</li>` : `<li>${esc(c)}</li>`))
    .join('');
  const { text, sections, index } = explanationOf(q);
  return [
    `<li class="fn-key-q" value="${q.id}">`,
    `<p class="fn-key-prompt">${esc(q.question)}</p>`,
    `<ol class="fn-key-choices" type="A">${choices}</ol>`,
    `<p class="ak-answer"><strong>Answer: ${LETTERS[answer]}. ${esc(q.choices[answer])}</strong></p>`,
    text === '' ? '' : `<p class="ak-expl">${esc(text)}</p>`,
    sections.length === 0 ? '' : `<p class="fn-q-ref">Sections: ${esc(sections.join(', '))}</p>`,
    index.length === 0 ? '' : `<p class="fn-q-ref">Index lookup: ${esc(index.join(' · '))}</p>`,
    '</li>',
  ].join('');
}

function setHtml(set) {
  assert(Array.isArray(set.questions), `${set.id}: no questions`);
  assert(set.questions.length > 0 && set.questions.length <= QUESTIONS_MAX, `${set.id}: question count`);
  const desc = set.description ? `<p class="fn-key-desc">${esc(set.description)}</p>` : '';
  return `${desc}<ol class="fn-key-list">${set.questions.map(questionHtml).join('')}</ol>`;
}

function password() {
  const word = () => WORDS[randomInt(WORDS.length)];
  return `${word()}-${word()}-${String(randomInt(100)).padStart(2, '0')}`;
}

function encrypt(plaintext, pass) {
  const salt = randomBytes(SALT_BYTES);
  const iv = randomBytes(IV_BYTES);
  const key = pbkdf2Sync(Buffer.from(pass, 'utf8'), salt, ITERATIONS, KEY_BYTES, 'sha256');
  const cipher = createCipheriv('aes-256-gcm', key, iv);
  const body = Buffer.concat([cipher.update(plaintext, 'utf8'), cipher.final()]);
  const tag = cipher.getAuthTag();
  assert(tag.length === TAG_BYTES, 'GCM tag is not 16 bytes');
  return {
    v: 1,
    kdf: 'PBKDF2-SHA256',
    iter: ITERATIONS,
    salt: salt.toString('base64'),
    iv: iv.toString('base64'),
    data: Buffer.concat([body, tag]).toString('base64'),
  };
}

function main() {
  assert(new Set(WORDS).size === WORDS.length && WORDS.length >= 48, 'wordlist');
  const sets = readOrder().map((name) => JSON.parse(readFileSync(join(CONTENT, 'quizzes', name), 'utf8')));
  const slugs = sets.map((s) => s.id);
  for (const slug of slugs) assert(/^[a-z0-9-]+$/.test(slug), `bad slug ${slug}`);
  assert(new Set(slugs).size === slugs.length, 'duplicate slug');

  const passwords = slugs.map(() => password());
  // Passwords first, atomically and private: an envelope never exists
  // without its password on record.
  const lines = slugs.map((slug, i) => `${slug} ${passwords[i]}`).join('\n');
  const tmp = `${PASSWORD_FILE}.tmp`;
  writeFileSync(tmp, `# slug password — telco-admin answer keys\n${lines}\n`, { mode: 0o600 });
  chmodSync(tmp, 0o600);
  renameSync(tmp, PASSWORD_FILE);
  chmodSync(PASSWORD_FILE, 0o600);

  mkdirSync(OUT_DIR, { recursive: true });
  sets.forEach((set, i) => {
    const envelope = encrypt(setHtml(set), passwords[i]);
    writeFileSync(join(OUT_DIR, `${set.id}.json`), `${JSON.stringify(envelope)}\n`);
    console.log(set.id);
  });
}

main();
