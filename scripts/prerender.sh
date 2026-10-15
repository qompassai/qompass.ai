#!/usr/bin/env bash
# Prerender the Leptos SSR build into dist/ — the static site that gets
# deployed to the VPS. Run after `cargo leptos build --release`.
set -euo pipefail
cd "$(dirname "$0")/.."

BIN="target/release/qompass-ai"
SITE="target/site"
DIST="dist"
ROUTES=("/" "/research" "/solutions" "/analysis" "/standards" "/about" "/privacy" "/legal" "/ontrack/privacy" "/light-show" "/light-show/testers" "/field-notes" "/field-notes/trainer")

# Tiger Style full references: one quiet page per imported reference,
# linked only from its chapter on /standards.
for ref in content/tiger-style/*-reference.md; do
    [ -e "$ref" ] || continue
    slug="$(basename "$ref" -reference.md)"
    ROUTES+=("/standards/$slug")
done

# Project study pages: one route per content/projects/*.toml flagged
# ready = true. build.rs enforces file stem == slug, so the stem IS the
# route's last segment.
for toml in content/projects/*.toml; do
    [ -e "$toml" ] || continue
    if grep -qE '^[[:space:]]*ready[[:space:]]*=[[:space:]]*true' "$toml"; then
        ROUTES+=("/solutions/$(basename "$toml" .toml)")
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

# Field notes: one page per note the /field-notes index links, so the
# NOTES list in src/pages/field_service.rs is the one place to add a note.
mapfile -t NOTE_ROUTES < <(grep -o 'href="/field-notes/notes/[a-z0-9-]*"' "$DIST/field-notes/index.html" \
    | sed -e 's/^href="//' -e 's/"$//' | sort -u)
if [ "${#NOTE_ROUTES[@]}" -eq 0 ]; then
    echo "prerender: /field-notes links no field notes" >&2
    exit 1
fi
for route in "${NOTE_ROUTES[@]}"; do
    mkdir -p "$DIST$route"
    curl -sf "http://127.0.0.1:3000$route" -o "$DIST$route/index.html"
    echo "prerendered $route -> $DIST$route/index.html"
done

# Solution pages: one page per solution the /solutions cardinal cards
# link, so SOLUTIONS in src/pages/compass_services.rs is the one place
# to add a solution. Study pages above are skipped, already rendered.
mapfile -t SOLUTION_ROUTES < <(grep -o 'href="/solutions/[a-z0-9-]*"' "$DIST/solutions/index.html" \
    | sed -e 's/^href="//' -e 's/"$//' | sort -u)
if [ "${#SOLUTION_ROUTES[@]}" -eq 0 ]; then
    echo "prerender: /solutions links no solution pages" >&2
    exit 1
fi
for route in "${SOLUTION_ROUTES[@]}"; do
    [ -e "$DIST$route/index.html" ] && continue
    mkdir -p "$DIST$route"
    curl -sf "http://127.0.0.1:3000$route" -o "$DIST$route/index.html"
    echo "prerendered $route -> $DIST$route/index.html"
done

kill "$SERVER_PID"
trap - EXIT

# Redirect pages for the 2026-10-13 rename: the standing-orders page
# moved from /tiger-style to /standards so the standards collection
# can grow past the one lineage. The deployed site is static files
# with no server-side redirect rule for these paths, so each old
# address keeps a small page that meta-refreshes to the new one and
# names it canonical.
write_redirect() {
    local from="$1" to="$2"
    local title="${3:-Standards - Qompass AI}" label="${4:-Standards}"
    mkdir -p "$DIST$from"
    cat > "$DIST$from/index.html" <<EOF
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<title>$title</title>
<link rel="canonical" href="https://qompass.ai$to" />
<meta http-equiv="refresh" content="0; url=$to" />
<meta name="robots" content="noindex" />
</head>
<body>
<p>This page moved to <a href="$to">$label</a>.</p>
</body>
</html>
EOF
    echo "redirect $from -> $to"
}

write_redirect "/tiger-style" "/standards"
for ref in content/tiger-style/*-reference.md; do
    [ -e "$ref" ] || continue
    slug="$(basename "$ref" -reference.md)"
    write_redirect "/tiger-style/$slug" "/standards/$slug"
done

# Same treatment for the CV, folded into /about (record and, since
# the /contact fold, identifiers) on 2026-10-13: the standalone page is retired, the old
# address lands on the page that now carries the record.
write_redirect "/cv" "/about" "About - Qompass AI" "About"

# And for /contact, folded the same day into the identity section of
# /about: the mailbox, the key, and every identifier now live at
# /about#identity, and the old address lands there.
write_redirect "/contact" "/about#identity" "Contact - Qompass AI" "Contact on the About page"

# Same treatment for the fleet index, renamed from Projects to
# Solutions on 2026-10-13: the grid and every ready study page moved
# under /solutions, and the old addresses land on their new homes.
write_redirect "/projects" "/solutions" "Solutions - Qompass AI" "Solutions"
for toml in content/projects/*.toml; do
    [ -e "$toml" ] || continue
    if grep -qE '^[[:space:]]*ready[[:space:]]*=[[:space:]]*true' "$toml"; then
        slug="$(basename "$toml" .toml)"
        write_redirect "/projects/$slug" "/solutions/$slug" "Solutions - Qompass AI" "Solutions"
    fi
done

# And for the Field Service strand, renamed Field Notes on 2026-10-13:
# the landing, the trainer, and every note moved under /field-notes.
write_redirect "/field-service" "/field-notes" "Field Notes - Qompass AI" "Field Notes"
write_redirect "/field-service/trainer" "/field-notes/trainer" "Field Notes - Qompass AI" "the TIA-598 trainer"
for route in "${NOTE_ROUTES[@]}"; do
    write_redirect "/field-service${route#/field-notes}" "$route" "Field Notes - Qompass AI" "Field Notes"
done

# And for On AI, renamed Analysis on 2026-10-13 as the middle step of
# the flow (Standards -> Analysis -> Field Notes).
write_redirect "/on-ai" "/analysis" "Analysis - Qompass AI" "Analysis"

# The web server reads dist/ as an unprivileged user, so every deployed
# file must be world-readable and every directory world-traversable.
# Source assets have arrived with owner-only modes before now (600/640
# on the project marks, the CV image, and the page JS — agent-copied
# files), and cp/rsync preserve those modes faithfully: on 2026-10-12
# Caddy answered 403 with an empty body for every project mark while
# the pages themselves served fine. Normalize here, in the one step
# every deploy runs, and refuse to ship a dist the server cannot read.
chmod -R a+rX "$DIST"
unreadable="$(find "$DIST" -type f ! -perm -o=r -print)"
if [ -n "$unreadable" ]; then
    echo "prerender: dist contains files the web server cannot read:" >&2
    printf '%s\n' "$unreadable" >&2
    exit 1
fi

echo "dist/ ready: $(du -sh "$DIST" | cut -f1)"
