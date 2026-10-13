# Qompass AI — website

The qompass.ai website: a Leptos (Rust/WebAssembly) static site with islands,
served by vongola. Design: a compass for the quantum deep — abyssal charts,
Cherenkov blue, brass instruments.
<a href="./LICENSE"><img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License: Apache 2.0"></a>

## Layout

- `src/` — Leptos app (pages in `src/pages/`)
- `content/cv.toml` — the founder's record as data; edit it and rebuild to update /about (the record) and /contact (the identifiers)
- `scripts/fetch-orcid.py` — regenerates `src/orcid_data.rs` from the live ORCID record
- `style/main.css` — the design system
- `public/` — vendored assets (fonts, Three.js, island scripts); no CDNs at runtime

## Build

```sh
python3 scripts/fetch-orcid.py   # refresh the ORCID snapshot
cargo leptos build --release     # needs cargo-leptos + the wasm32 target
```

## License

This project is licensed under the [Apache License, Version 2.0](./LICENSE).
