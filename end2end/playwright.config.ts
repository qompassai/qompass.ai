import { defineConfig } from '@playwright/test';

// cargo-leptos serves the freshly built site on its configured
// site-addr (127.0.0.1:3000, see [package.metadata.leptos] in the
// workspace Cargo.toml) for the duration of `cargo leptos end-to-end`.
export default defineConfig({
  testDir: './tests',
  timeout: 30_000,
  workers: 1,
  use: {
    baseURL: 'http://127.0.0.1:3000',
  },
});
