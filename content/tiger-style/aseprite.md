# Tiger Style Aseprite

> **Prerequisite:** `tiger-style-lua` (sibling skill at `../tiger-style-lua/SKILL.md`) — apply it
> first to every Lua script this skill produces or reviews, including Lua
> embedded in generated Aseprite scripts.

## Purpose

Automate Aseprite the way its docs describe it: the CLI for exports and
introspection, Lua scripts for pixel-level work, always in batch mode for
headless pipelines. Safety order, always: **Safety > Performance >
Developer Experience.** Every flag and API member used here was verified
against the official docs on 2026-09-27; anything the docs do not say is
marked unverified. `references/aseprite-cli.md` and
`references/aseprite-scripting-api.md` hold the complete verified tables —
this file is the operational core.

Official sources:
- CLI: <https://www.aseprite.org/docs/cli/>
- Scripting overview: <https://www.aseprite.org/docs/scripting/>
- API index: <https://www.aseprite.org/api/>
- API changes (version guards): <https://aseprite.com/api/Changes>
- LibreSprite scripting API: <https://github.com/libresprite/libresprite/blob/HEAD/SCRIPTING.md>

## Workflow

1. Detect the binary and probe it. Never assume `aseprite` is installed,
   on PATH, or the right version.
2. Prefer direct CLI flags when they cover the job (`--save-as`, `--sheet`,
   `--tag`, `--split-layers`, `--scale`); reach for `--script` only when
   pixel-level logic is required.
3. Run everything in batch mode (`-b`). Guard every script with the batch
   contract below.
4. Write generated Lua to a temp file, invoke with an argv array, enforce a
   bounded timeout, check the exit code, clean up the temp file.
5. Version-guard every API feature younger than the minimum supported
   Aseprite with `app.apiVersion`. Never use a deprecated alias.
6. Treat LibreSprite as a different product: consult the divergence table in
   `references/aseprite-scripting-api.md` before running any Aseprite-1.3
   script or flag against it.

## Operating Rules

### Binary detection and probing

- Search, in order: `ASEPRITE_BIN`, then `NVIM_ASEPRITE_BIN` (diver
  convention), then executable candidates on PATH. Record which one won and
  report it.
- Probe with an argv array, never a shell string:
  `{ bin, '--version' }`. Bound stdout (a few KB) and time (10 s).
- Parse the version from stdout; compare with `Version` semantics. Require
  Aseprite ≥ 1.3 for any feature listed as v1.3+ in the references, and
  LibreSprite ≥ 1.1 only for the LibreSprite-safe subset.
- If no binary is found, stop and say so. Do not download Aseprite silently;
  a download needs explicit approval and a checksum/signature note.

### CLI invocation contract

- Always an argv array (one element per argument), never an interpolated
  shell string. Paths with spaces stay single elements.
- Always `-b` (batch) for automation. GUI launches need explicit approval.
- **Option order matters.** Selection/export options (`--tag`,
  `--frame-range`, `--layer`, `--ignore-layer`, `--all-layers`,
  `--split-layers`, `--split-tags`, `--split-slices`) apply to the next
  sprite opened on the command line: place them **before** the `.aseprite`
  file. `--script-param name=value` goes before `--script script.lua`.
  `--scale` affects sprites opened **before** the option.
- Bound every run: an explicit timeout (default 120 s for exports; longer
  only with a named reason) and bounded stdout/stderr capture.
- Exit-code contract: `0` means success. Any non-zero exit, or a Lua error
  printed to stderr, means the run failed — even if some output files
  exist. Verify expected output files exist after a `0` exit before
  reporting success.
- `--sheet` **overwrites** its output file. `--data ""` prints sheet JSON to
  stdout. `--save-as frame001.png` writes a numbered PNG sequence.
- Temp-file hygiene: generated scripts go to a temp file with a distinctive
  suffix (e.g. `_diver_bgremove.lua`), and are removed after the run whether
  it succeeds or fails.
- Do not pass `--version`-unverified flags. The full verified flag table is
  in `references/aseprite-cli.md`; notably `--split-grid` and
  `--export-tileset` require v1.3-beta21+, and fractional `--scale` factors
  are **unverified** (the docs show integer examples only).

### Lua script contract (batch mode)

Every script run via `-b --script` must satisfy all of these:

1. **No UI, ever.** `app.isUIAvailable` is `false` in batch mode.
   `Dialog()` returns `nil` there (since v1.2.35). Do not call `Dialog`,
   `app.alert`, or `app.tip` — report status with `print()` to stdout and
   failures with `error()` or `os.exit(1)` (non-zero exit is the failure
   signal).
2. **Input through `app.params`.** CLI `--script-param name=value` pairs
   arrive as strings in `app.params`. Validate and coerce every param at
   the top of the script; fail loudly on missing/invalid values.
3. **`app.sprite` can be `nil`.** Check it before touching it. When the CLI
   opens the input file (`-b input.aseprite --script s.lua`), that sprite
   becomes active — but never assume it.
4. **Color mode first.** Pixel math with `app.pixelColor.rgbaR/G/B/A`
   assumes RGBA. Read `sprite.colorMode` and compare against
   `ColorMode.RGB` before interpreting pixels; indexed images hold palette
   indices, not channel bytes. RGB components are **not** premultiplied by
   alpha.
5. **Transactions for edits.** Wrap document mutations in
   `app.transaction([label], function() ... end)` — it groups undo and
   rolls back the whole change on error. Since v1.3.16, property changes
   outside a transaction do not generate undo steps, so edits that must be
   undoable (or atomic) belong inside one.
6. **No destructive action without confirmation.** `sprite:close()` does
   not prompt; `saveAs()` marks the document saved and retargets it.
   Scripts that must not disturb the open document use `saveCopyAs()`.
   In interactive contexts, confirm before overwriting user files; in batch
   pipelines the explicit output path in the invocation is the
   confirmation — and it must be named, never defaulted to the input path.
7. **Version guards.** Use `app.apiVersion` (number; `nil` on the very first
   scripting builds) for API features, and `app.version` (a `Version`
   object) for app behavior. The per-version feature table is in
   `references/aseprite-scripting-api.md`. Never use a deprecated alias
   (`app.activeSprite`, `app.activeLayer`, `app.activeFrame`,
   `app.activeCel`, `app.activeImage`, `app.activeTag`, `app.activeTool`,
   `app.activeBrush`, `putPixel`/`putImage`/`putSprite`).
8. **Stdlib limits.** Aseprite embeds Lua 5.3 libraries; `os.exit` and
   `os.tmpname` are unavailable, and `os.execute`/`io.open` may ask the
   user for permission — avoid them in batch scripts.
9. **Deterministic pixel work.** For raw pixels prefer `Image:pixels()`
   iteration and the clone→modify→`cel.image = img` pattern (the image
   setter is the documented single-undoable-action replacement) over
   `app.useTool`, which depends on thin batch-mode editor state.

### LibreSprite boundary

LibreSprite 1.1 is **not** API-compatible with Aseprite 1.3 scripts:

- Different `app` surface: `app.activeSprite`/`app.activeLayerNumber`/
  `app.activeImage`/`app.activeFrameNumber` (the names Aseprite deprecated),
  `app.createDialog()` instead of `Dialog()`, `app.version` returns a
  **string**, and there is no documented `app.apiVersion`, `app.params`,
  or `app.isUIAvailable`.
- Different `Image`: only `getPixel`/`putPixel`, `getImageData`/
  `putImageData`, `getPNGData`, `clear(color)` — no `pixels()` iterator,
  no `drawPixel`, `saveAs`, `resize`, `flip`, or `shrinkBounds`.
- Different `Sprite`: `saveAs(fileName, asCopy)`, `commit()`,
  `layer(n)` (0-indexed from bottom), no `saveCopyAs`, no transactions
  named as such.
- No `--script-param` in its scripting docs. CLI batch/script support
  (`-b`, `--script`) is corroborated by third-party source reading, not
  official LibreSprite docs — verify with `libresprite --help` on the real
  binary before relying on it.

Write a compat shim (feature-detect, never version-sniff LibreSprite) when
one pipeline must serve both, or keep two scripts. The full divergence
table is in `references/aseprite-scripting-api.md`.

### Diver wiring (games/aseprite)

- Binary lookup: `ASEPRITE_BIN`, `NVIM_ASEPRITE_BIN`, then PATH candidates
  (`autil.find_binary()`).
- Established export argv shapes: `-b <sprite> --sheet <png> --data
  <json> --format json-array`; `--save-as` for PNG sequences, GIF, and
  palette output; `--sheet-type packed|rows|columns|horizontal|vertical`.
- The AI import pipeline generates a background-keying Lua script run as
  `{ bin, '-b', input, '--script', script_path }`. Known gaps to fix when
  touching it: the generated script calls `app.alert()` on a nil sprite
  (invalid in batch mode — replace with `print` + non-zero exit) and
  assumes RGBA pixels without checking `sprite.colorMode`. Palette
  quantization is manual on the Aseprite path (ImageMagick fallback does
  keying + quantize + point filter, bounded at 64 files / 16 MB / 120 s).
- Default config: 128 px frames, background-key options, palette size,
  auto-open behavior, `scale_via_cli`. Fractional `--scale` is unverified —
  keep integer factors until the docs say otherwise.

## Review checklist

- [ ] Binary detected and version probed; the winning path is reported.
- [ ] Every CLI flag is in `references/aseprite-cli.md` (or verified in the
      docs for the probed version).
- [ ] argv arrays everywhere; bounded timeout and output; exit code
      checked; output files verified after exit 0.
- [ ] Options ordered correctly (selection filters before the input file;
      `--script-param` before `--script`).
- [ ] Script has no UI calls in batch mode; input via `app.params`,
      validated; status via `print`, failure via non-zero exit.
- [ ] `app.sprite` nil-checked; `colorMode` checked before pixel math.
- [ ] Mutations inside `app.transaction`; `saveCopyAs` used where the open
      document must be undisturbed.
- [ ] `app.apiVersion` guards on every post-1.3-beta API feature; no
      deprecated aliases.
- [ ] LibreSprite divergence consulted if the target might be LibreSprite.
- [ ] Temp scripts removed after the run, success or failure.
