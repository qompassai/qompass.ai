#!/usr/bin/env bash
# Prerender the Leptos SSR build into dist/ — the static site that gets
# deployed to the VPS. Run after `cargo leptos build --release`.
set -euo pipefail
cd "$(dirname "$0")/.."

BIN="target/release/qompass-ai"
SITE="target/site"
DIST="dist"
ROUTES=("/" "/research" "/projects" "/on-ai" "/tiger-style" "/cv" "/about" "/contact")

# Tiger Style full references: one quiet page per imported reference,
# linked only from its chapter on /tiger-style.
for ref in content/tiger-style/*-reference.md; do
    [ -e "$ref" ] || continue
    slug="$(basename "$ref" -reference.md)"
    ROUTES+=("/tiger-style/$slug")
done

# Project study pages: one route per content/projects/*.toml flagged
# ready = true. build.rs enforces file stem == slug, so the stem IS the
# route's last segment.
for toml in content/projects/*.toml; do
    [ -e "$toml" ] || continue
    if grep -qE '^[[:space:]]*ready[[:space:]]*=[[:space:]]*true' "$toml"; then
        ROUTES+=("/projects/$(basename "$toml" .toml)")
    fi
done

rm -rf "$DIST"
mkdir -p "$DIST"
cp -r "$SITE"/. "$DIST"/

"$BIN" &
SERVER_PID=$!
trap 'kill "$SERVER_PID" 2>/dev/null || true' EXIT

for _ in $(seq 1 50); do
    if curl -sf http://127.0.0.1:3000/ >/dev/null; then
        break
    fi
    sleep 0.2
done

for route in "${ROUTES[@]}"; do
    if [ "$route" = "/" ]; then
        out="$DIST/index.html"
    else
        mkdir -p "$DIST$route"
        out="$DIST$route/index.html"
    fi
    curl -sf "http://127.0.0.1:3000$route" -o "$out"
    echo "prerendered $route -> $out"
done

kill "$SERVER_PID"
trap - EXIT
echo "dist/ ready: $(du -sh "$DIST" | cut -f1)"
