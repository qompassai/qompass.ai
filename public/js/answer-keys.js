// Answer keys on /solutions/telco-admin-quizzes: each set's key is an
// AES-256-GCM envelope at /answer-keys/<slug>.json, keyed by PBKDF2 of
// that set's password (scripts/generate-answer-keys.mjs writes them).
// The password and the plaintext live only in this page's memory: no
// storage, no cookies. A wrong password fails GCM authentication, and
// any failure renders nothing but the error line.
'use strict';

const ERROR_TEXT = "That password doesn't open this key.";

function bytes(b64) {
  return Uint8Array.from(atob(b64), (c) => c.charCodeAt(0));
}

async function openKey(slug, password) {
  const res = await fetch(`/answer-keys/${slug}.json`, { cache: 'no-store' });
  if (!res.ok) throw new Error(`fetch ${res.status}`);
  const env = await res.json();
  if (env.v !== 1 || env.kdf !== 'PBKDF2-SHA256' || !Number.isSafeInteger(env.iter)) {
    throw new Error('unknown envelope');
  }
  const base = await crypto.subtle.importKey(
    'raw', new TextEncoder().encode(password), 'PBKDF2', false, ['deriveKey'],
  );
  const key = await crypto.subtle.deriveKey(
    { name: 'PBKDF2', salt: bytes(env.salt), iterations: env.iter, hash: 'SHA-256' },
    base,
    { name: 'AES-GCM', length: 256 },
    false,
    ['decrypt'],
  );
  const iv = bytes(env.iv);
  const plain = await crypto.subtle.decrypt({ name: 'AES-GCM', iv }, key, bytes(env.data));
  return new TextDecoder().decode(plain);
}

function wire(form) {
  if (form.dataset.wired) return;
  form.dataset.wired = '1';
  const slug = form.dataset.keySlug;
  const lock = form.closest('.ak-lock');
  const input = form.querySelector('input[type="password"]');
  const button = form.querySelector('button[type="submit"]');
  const error = lock.querySelector('.ak-error');
  const body = lock.querySelector('.ak-body');
  const hide = lock.querySelector('.ak-hide');
  if (!/^[a-z0-9-]+$/.test(slug) || !input || !button || !error || !body || !hide) return;

  form.addEventListener('submit', async (event) => {
    event.preventDefault();
    error.hidden = true;
    body.innerHTML = '';
    button.disabled = true;
    const password = input.value;
    input.value = '';
    try {
      // The fragment is the generator's own escaped HTML; GCM proves it
      // came out of an envelope sealed with this set's password.
      body.innerHTML = await openKey(slug, password);
      form.hidden = true;
      hide.hidden = false;
    } catch {
      body.innerHTML = '';
      error.textContent = ERROR_TEXT;
      error.hidden = false;
    } finally {
      button.disabled = false;
    }
  });

  hide.addEventListener('click', () => {
    body.innerHTML = '';
    hide.hidden = true;
    form.hidden = false;
    input.focus();
  });

  form.hidden = false;
}

// The panels are inner_html, opaque to hydration, so wiring at
// DOMContentLoaded is safe; the second pass catches any panel the
// hydration walk re-rendered (the guard skips ones already wired).
const wireAll = () => document.querySelectorAll('form[data-key-slug]').forEach(wire);
document.addEventListener('DOMContentLoaded', wireAll);
window.addEventListener('qompass:hydrated', wireAll, { once: true });
