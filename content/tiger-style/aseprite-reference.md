# Aseprite CLI reference

All flags verified against <https://www.aseprite.org/docs/cli/> on
2026-09-27 (full page, 1,259 rendered lines). Minimum version is Aseprite
1.3.x unless noted. Items marked **unverified** are not stated in the docs.

## Runtime / general

| Flag | Effect |
| ---- | ------ |
| `--shell` | Start an interactive console/scripting shell |
| `-b`, `--batch` | Batch (headless) mode: no UI. Required for automation |
| `-p`, `--preview` | Preview mode |
| `-v`, `--verbose` | Verbose output |
| `--debug` | Debug output |
| `--noinapp` | Do not run as an in-app process |
| `--help` | Print help |
| `--version` | Print version (use for binary probing) |

## Conversion

| Flag | Effect |
| ---- | ------ |
| `--save-as <file>` | Convert/save the sprite to another file. `--save-as frame001.png` exports animation frames as a numbered PNG sequence |
| `--palette <file>` | Use the given palette file for conversion |
| `--scale <factor>` | Scale the sprite. **Affects sprites opened before the option.** Docs show integer examples only; fractional factors are **unverified** |
| `--color-mode <mode>` | Convert color mode (e.g. `indexed`, `rgb`, `grayscale`) |
| `--dithering-algorithm <algo>` | Dithering algorithm for color conversion |
| `--dithering-matrix <matrix>` | Dithering matrix for color conversion |

## Sprite-sheet / atlas export

| Flag | Effect |
| ---- | ------ |
| `--sheet <png>` | Pack and save the sprite sheet PNG. **Overwrites the output file** |
| `--data <json>` | Sprite-sheet metadata JSON. `--data ""` writes the JSON to stdout |
| `--format json-hash` \| `json-array` | Metadata layout: hash keyed by name, or array |
| `--sheet-type horizontal` \| `vertical` \| `rows` \| `columns` \| `packed` | Sheet layout. `packed` ≡ `--sheet-pack` |
| `--sheet-width <n>`, `--sheet-height <n>` | Fixed sheet dimensions |
| `--sheet-columns <n>`, `--sheet-rows <n>` | Fixed grid layout |
| `--sheet-pack` | Pack frames tightly (same as `--sheet-type packed`) |

## Selection and splitting (order-sensitive)

These options apply to the **next sprite opened on the command line** —
place them **before** the `.aseprite` input file. `--split-layers` and
`--ignore-layer` can require preceding the input.

| Flag | Effect |
| ---- | ------ |
| `--split-layers` | Export each layer as a separate file |
| `--split-tags` | Export each tag as a separate file |
| `--split-slices` | Export each slice as a separate file |
| `--split-grid` | Split by grid (requires v1.3-beta21+) |
| `--layer <name>`, `--import-layer` | Select which layer(s) to export |
| `--all-layers` | Export all layers (incl. hidden) |
| `--ignore-layer <pattern>` | Exclude matching layers |
| `--tag <name>`, `--frame-tag <name>` | Export only the given frame tag |
| `--frame-range <from,to>` | Export a frame range |
| `--slice <name>` | Export only the given slice |

## Export shaping

| Flag | Effect |
| ---- | ------ |
| `--ignore-empty` | Skip empty cels/frames |
| `--merge-duplicates` | Merge duplicate frames |
| `--border-padding <n>` | Padding around the sheet border |
| `--shape-padding <n>` | Padding between packed shapes |
| `--inner-padding <n>` | Padding inside each sprite |
| `--trim` | Trim transparent borders per frame |
| `--trim-sprite` | Trim the whole sprite |
| `--trim-by-grid` | Trim by grid cells |
| `--extrude` | Extrude edge pixels (bleed for tiling) |
| `--crop <x,y,w,h>` | Crop rectangle |
| `--filename-format <fmt>` | Output filename template (placeholders below) |
| `--tagname-format <fmt>` | Tag name template |

`--filename-format` placeholders: `{fullname}`, `{path}`, `{name}`,
`{title}`, `{extension}`, `{layer}`, `{tag}`, `{innertag}`,
`{outertag}`, `{frame}`, `{frame1}`, padded frame variants, `{tagframe}`,
`{duration}`.

## Scripting and introspection

| Flag | Effect |
| ---- | ------ |
| `--script <file>` | Run a Lua script (usually with `-b`) |
| `--script-param name=value` | Pass a parameter to the script; arrives in `app.params`. Place **before** `--script` |
| `--list-layers` | Print layer names |
| `--list-layer-hierarchy` | Print the layer tree |
| `--list-tags` | Print frame tags |
| `--list-slices` | Print slices |
| `--oneframe` | Operate on a single frame |
| `--export-tileset` | Export tileset (requires v1.3-beta21+) |

## Canonical argv shapes

```lua
-- Version probe
{ bin, '--version' }

-- Sprite sheet + metadata (json-array matches diver's importer)
{ bin, '-b', sprite_path, '--sheet', sheet_png, '--data', meta_json,
  '--format', 'json-array', '--sheet-type', 'packed' }

-- PNG frame sequence
{ bin, '-b', sprite_path, '--save-as', 'frame001.png' }

-- Single tag to GIF (selection option precedes the input)
{ bin, '-b', '--tag', 'Walk', sprite_path, '--save-as', 'walk.gif' }

-- Sheet JSON to stdout for verification
{ bin, '-b', sprite_path, '--sheet', sheet_png, '--data', '',
  '--format', 'json-array' }

-- Script with parameters (params precede --script)
{ bin, '-b',
  '--script-param', 'output=' .. out_path,
  '--script-param', 'frames=' .. tostring(n),
  '--script', script_path }

-- Introspection
{ bin, '-b', '--list-layers', sprite_path }
```

## Exit-code and output contract

- `0` = success. Any non-zero exit, or a Lua error on stderr, = failure —
  even if output files exist.
- Verify expected outputs exist after a `0` exit before reporting success.
- `--sheet` overwrites its target; never point it at a file you need to keep
  unless that overwrite is the intent.

## What the docs do not say (unverified)

- Fractional `--scale` factors (docs show integer examples only).
- Exact behavior of `--preview` in automation contexts.
- Whether `--script` output buffering is line-buffered on all platforms —
  treat long silent runs as unknown progress and rely on the timeout.

---

# Aseprite scripting API reference

All members verified against <https://www.aseprite.org/api/> on 2026-09-27.
Source pages: `app`, `image`, `sprite`, `cel`, `tag`, `dialog`, `pixelcolor`,
plus the API changes log at <https://aseprite.com/api/Changes>. Members not
listed here are unverified — check the official page for the installed
version before using them.

Lua runtime: Aseprite embeds Lua 5.3 libraries (base, coroutine, string,
utf-8, table, math, OS facilities, debug). `os.exit` and `os.tmpname` are
**not** available; `os.execute` and `io.open` **may ask the user for
permission** — avoid them in batch scripts.

## `app` namespace

Active objects (all can be `nil` when nothing is active):

- `app.sprite`, `app.image`, `app.cel`, `app.layer`, `app.frame`, `app.tag`
- `app.version` → `Version` object (e.g. `Version("1.3.17")`)
- `app.apiVersion` → integer scripting-API version (`nil` on the very first
  scripting builds; see version table below)
- `app.params` → table of `--script-param name=value` strings
- `app.isUIAvailable` → `false` in `--batch` mode; guard every UI call
- `app.sprites` → array of open sprites
- `app.fgColor` / `app.bgColor` → `Color`; `app.fgTile` / `app.bgTile` (1.3.9.2+)
- `app.brush`, `app.tool`, `app.editor`, `app.window`, `app.theme`,
  `app.clipboard`, `app.preferences`, `app.fs`, `app.os` (platform info,
  1.3.7+), `app.uiScale`, `app.defaultPalette`
- `app.pixelColor` → low-level pixel helpers (below)

Functions:

- `app.open(filename)` → `Sprite` or `nil`
- `app.transaction([label], fn)` — groups mutations into one undo step;
  **rolls back everything if `fn` errors**
- `app.command.<Name>{...}` — run editor commands with named params;
  since 1.3.16-beta1 all commands accept a `ui` param to suppress dialogs
- `app.useTool{...}` — stroke with the active tool (thin editor state in
  batch mode; prefer direct pixel drawing for determinism)
- `app.alert(msg)`, `app.tip(msg)` (1.3.15+) — UI only, never in batch
- `app.refresh()`, `app.undo()`, `app.redo()`
- `json.encode(t)` / `json.decode(s)` — built in since 1.3-rc5

Deprecated aliases — **never use**: `app.activeSprite`, `app.activeLayer`,
`app.activeFrame`, `app.activeCel`, `app.activeImage`, `app.activeTag`,
`app.activeTool`, `app.activeBrush`. Use the short names.

## `Sprite`

Constructors: `Sprite(width, height [, colorMode])`, `Sprite(spec)`,
`Sprite(otherSprite)`, `Sprite{ fromFile=filename [, oneFrame] }`.

Key properties: `width`, `height`, `bounds`, `grid`, `pixelRatio`,
`selection`, `filename`, `isValid` (1.3.16+), `isModified`, `colorMode`,
`frames`, `palettes`, `layers`, `cels`, `tags`, `slices`, `tilesets`,
`backgroundLayer`, `transparentColor`, `data`, `properties`, `undoHistory`
(1.3.15+), `tileManagementPlugin`.

Methods:

- `sprite:resize(w, h)`, `sprite:crop(x, y, w, h)`
- `sprite:saveAs(filename)` — marks the document **saved**, retargets it
- `sprite:saveCopyAs(filename)` — exports a copy; saved state untouched
  (use for batch outputs)
- `sprite:close()` — closes **without prompting**
- `sprite:loadPalette(pal)`, `sprite:setPalette(pal)`
- `sprite:assignColorSpace(cs)`, `sprite:convertColorSpace(cs)`
- `sprite:newLayer()`, `sprite:newGroup()`, `sprite:deleteLayer(l)`
- `sprite:newFrame([duration])`, `sprite:newEmptyFrame()`,
  `sprite:deleteFrame(f)`
- `sprite:newCel(layer, frame, image [, pos])`, `sprite:deleteCel(cel)`
- `sprite:newTag(from, to [, name])`, `sprite:deleteTag(tag)`
- `sprite:newSlice([rect [, name]])`, `sprite:deleteSlice(s)`
- `sprite:newTileset([w, h])`, `sprite:deleteTileset(t)`,
  `sprite:newTile(tileset [, index])`, `sprite:deleteTile(...)`
- `sprite:flatten()` — same as `app.command.FlattenLayers()`

Events (`sprite.events:on(name, fn)`): `'change'` (`ev.fromUndo` tells
undo/redo apart, since 1.3-rc1), `'filenamechange'`, and since 1.3.14.3
`'layerblendmode'`, `'layername'`, `'layeropacity'`, `'layervisibility'`.

## `Image`

Constructors: `Image(w, h [, colorMode])`, `Image(spec)`,
`Image(otherImage)`, `Image(otherImage, rect)`, `Image(sprite)`,
`Image{ fromFile=filename }`.

- Standalone `Image` edits generate **no undo info**; images attached to
  cels generate undo for documented operations.
- `Image:pixels([rect])` — iterator yielding pixel objects callable for
  get/set, each with `.x`/`.y`. The documented fast path for bulk work.
- `image:drawPixel(x, y, color)` — generates **no undo by itself**;
  docs recommend clone → modify → assign (`cel.image = img`, which is the
  single-undoable-action replacement).
- `image.bytes` length **must equal** `image.rowStride * image.height`;
  `image.bytesPerPixel` since 1.3-rc7.
- `image:resize(w, h)` methods documented: `bilinear`, `rotsprite`
  (nearest-neighbor not named on the official page — unverified).
- Methods: `clone`, `clear([rect [, color]])` (rect variant since 1.3-rc3),
  `drawPixel`, `getPixel`, `drawImage`, `drawSprite`, `isEqual`, `isEmpty`,
  `isPlain`, `pixels`, `saveAs{ filename, palette }`, `resize`,
  `shrinkBounds` (1.3-rc1+), `flip` (1.3-rc4+, `FlipType`), `context`
  (`GraphicsContext`, 1.3.11-beta1+).
- Deprecated/renamed — **never use**: `putPixel`, `putImage`, `putSprite`.

## `Cel`

A cel is an image at an *xy* position for one layer/frame.

- `cel.sprite`, `cel.layer` (read-only)
- `cel.frame` / `cel.frameNumber` — settable; moves the cel (replacing any
  cel already there). Frame `1` is the first frame, not `0`
- `cel.image` — settable; the preferred replacement (one undoable action)
- `cel.bounds` (read-only `Rectangle`), `cel.position` (settable `Point`)
- `cel.opacity` — `0`–`255` (`0` transparent, `255` opaque)
- `cel.zIndex` — `[-32768, 32767]`, default `0` (1.3-rc3+)
- `cel.color`, `cel.data` (string), `cel.properties`

## `Tag`

- `tag.fromFrame` / `tag.toFrame` → `Frame` objects (very old 1.2.10-beta2
  returned frame numbers instead)
- `tag.frames` → count; `tag.name` (settable string)
- `tag.aniDir` → `AniDir.FORWARD` / `REVERSE` / `PING_PONG` /
  `PING_PONG_REVERSE` (last since 1.3-rc1)
- `tag.repeats` → `0` = unspecified (infinite in UI, once on export),
  `n` = plays *n* times (1.3-rc1+)
- `tag.color`, `tag.data` (string), `tag.properties`

## `Dialog` (interactive only)

`Dialog()` / `Dialog(title)` / `Dialog{ title, autofit, notitlebar,
resizeable (1.3.15+), parent, onclose }`. **Returns `nil` when
`app.isUIAvailable` is `false`** (since v1.2.35) — never construct one in
batch scripts.

Widgets: `entry`, `number`, `slider`, `check`, `radio`, `button`,
`combobox`, `color`, `shades{ mode="pick"|"sort" }`, `label`, `separator`
(1.3.14+), `tab`/`endtabs` (1.3-rc7+), `canvas` (1.3-rc1+), `newrow{
always=true }`, `modify{...}`, `show{ wait=false, bounds, autoscrollbars
(1.3-rc7+), hand (1.3.10+) }`. Results land in `dlg.data` keyed by widget
`id`. Method chaining is supported.

## `app.pixelColor` (low-level, fast)

Pixels are unsigned ints: RGBA = 32-bit (8 bits/channel), grayscale =
16-bit (alpha + gray), indexed = 8-bit palette index.

- `app.pixelColor.rgba(r, g, b [, alpha])` — alpha defaults to 255 (opaque)
- `app.pixelColor.rgbaR/G/B/A(v)` — channel extractors. **Components are
  NOT premultiplied by alpha.**
- `app.pixelColor.graya(g [, alpha])`, `grayaV(v)`, `grayaA(v)`

Check `sprite.colorMode == ColorMode.RGB` before using the `rgba*`
extractors; on indexed images the int is a palette index.

## Batch-mode script skeleton

```lua
-- params: input=path, output=path, key=#rrggbb, fuzz=0-255
local params = app.params
local input = params.input
assert(type(input) == 'string' and input ~= '', 'missing param: input')
-- ... validate the rest the same way ...

local spr = app.open(input)
assert(spr ~= nil, 'cannot open: ' .. input)
assert(spr.colorMode == ColorMode.RGB, 'need RGB, got ' .. tostring(spr.colorMode))

app.transaction('tiger-style-aseprite op', function()
  -- mutations here; any error rolls everything back
end)

spr:saveCopyAs(params.output)  -- never saveAs: keep the input's saved state
print('OK ' .. params.output)
```

Rules encoded here: input via `app.params` with validation; nil-checked
`app.open`; color-mode gate; mutations in `app.transaction`; `saveCopyAs`
not `saveAs`; status on stdout, failure as a Lua error (non-zero exit).
No `Dialog`, no `app.alert`, no `os.execute`/`io.open`.

## Version-guard table (`app.apiVersion`)

From <https://aseprite.com/api/Changes>. Guard every feature at or below
your minimum supported Aseprite.

| apiVersion | Aseprite | Notable additions |
| ---------- | -------- | ----------------- |
| nil / 1 | 1.2.10-beta1…beta3 | first scripting builds |
| 18 | 1.2.35 | `Dialog()` returns nil in batch mode |
| 20 | 1.2.40 / 1.3-beta21 | `ExportSpriteSheet` gains `splitGrid`, `fromTilesets` |
| 21 | 1.3-rc1 | `Image.bounds`, `Image:shrinkBounds()`, `AniDir.PING_PONG_REVERSE`, `Tag.repeats`, `Sprite.isModified`, `GraphicsContext`, `Sprite:newTile()`, `Dialog:canvas()`, short names (`app.sprite` etc.) replace `app.activeSprite` |
| 22 | 1.3-rc2 | `GraphicsContext.opacity`/`blendMode`, `app.uiScale` |
| 23 | 1.3-rc3 | `require()`, `app.editor`, `Cel.zIndex`, `Image:clear(rect, color)` |
| 24 | 1.3-rc4 | `Image:flip()`, `FlipType` |
| 25 | 1.3-rc5 | `app.window`, `json.encode`/`json.decode` |
| 26 | 1.3-rc7 | `Dialog:tab()`/`endtabs()`, `Image.bytesPerPixel` |
| 28 | 1.3.7 | `app.os` |
| 29 | 1.3.9.2 | `app.fgTile`/`app.bgTile` |
| 30 | 1.3.10 / 1.3.11-beta1 | `app.editor.zoom`/`scroll`; `Image.context` |
| 32 | 1.3.13 | `app.clipboard`, `beforesitechange` event |
| 33 | 1.3.14 | layer `uuid`, `os.rename()`/`os.remove()` |
| 34 | 1.3.14.3 | sprite layer-property events |
| 35 | 1.3.15 | `sprite.undoHistory`, Dialog `resizeable`, `app.tip`, Dialog `autofit`/`sizeHint`, `FlipType.DIAGONAL` |
| 36 | 1.3.15.3 / 1.3.16-beta1 | `ui` param on all commands |
| 38 | 1.3.16 | `Sprite.isValid`; **behavior change:** property changes outside a transaction no longer generate undo steps |
| 38 | 1.3.17-beta1 | Dialog `onresize` |
| 39 | 1.3.17 | security fix: a script can no longer gain file access granted to another script |

## LibreSprite 1.1 divergence table

Primary source: <https://github.com/libresprite/libresprite/blob/HEAD/SCRIPTING.md>
and the [releases page](https://github.com/LibreSprite/LibreSprite/releases)
(v1.0, Nov 2023: "Support for Javascript (V8) and Lua scripting"; later
pre-release: "More scripting API functions"). LibreSprite's scripting API
tracks the old GPL-era Aseprite API, **not** Aseprite 1.3.

| Area | Aseprite 1.3 | LibreSprite 1.1 |
| ---- | ------------ | --------------- |
| Active objects | `app.sprite`, `app.layer`, `app.frame`, `app.cel`, `app.image`, `app.tag` | `app.activeSprite`, `app.activeDocument`, `app.activeLayerNumber`, `app.activeImage`, `app.activeFrameNumber` (the names Aseprite deprecated) |
| Version | `app.version` → `Version` object | `app.version` → **string** |
| API guard | `app.apiVersion` → integer | not documented |
| Script params | `--script-param` → `app.params` | not documented |
| UI check | `app.isUIAvailable` | not documented |
| Dialogs | `Dialog()` constructor | `app.createDialog()` method |
| Image pixels | `Image:pixels()` iterator, `drawPixel`, `saveAs`, `resize`, `flip`, `shrinkBounds` | only `getPixel`/`putPixel`, `getImageData`/`putImageData`, `getPNGData` (base64 PNG), `clear(color)` |
| Sprite save | `saveCopyAs()` (saved state untouched) | `saveAs(fileName, asCopy)` |
| Sprite layers | `sprite.layers` array | `sprite:layer(n)`, 0-indexed from bottom |
| Transactions | `app.transaction(label, fn)` | `sprite:commit()` commits the current transaction |
| Commands | `app.command.Name{ params }` | parameter-less GUI commands (`SaveFile()`, `NewFrame()`, …) |
| pixelColor | `rgba(r,g,b[,a])` (alpha optional) | `rgba(r,g,b,a)` (alpha required per their docs); same `rgbaR/G/B/A`, `graya`, `grayaV`, `grayaA` |
| Extensions | `package.json` plugin manifests | scripts dropped into the user `scripts` folder (third-party-reported; verify locally) |
| CLI | full flag set (see `aseprite-cli.md`) | `-b`/`--batch` and `--script` corroborated by third-party source reading of their `app_options.cpp`; `--data`, `--format`, `--sheet`, `--list-layers`, `--list-tags` also reported. **Not in official LibreSprite docs — verify with `libresprite --help` on the real binary.** |

Practical rule: an Aseprite 1.3 script will not run on LibreSprite 1.1
without porting (active-object names, image iteration, save/transaction
calls all differ). For a dual-target pipeline, feature-detect
(`if app.apiVersion ~= nil then ... else ... end`) rather than sniffing
for LibreSprite, and keep the LibreSprite path to the small documented
subset above.

## Unverified / not covered

- `Palette`, `Layer` (beyond basics), `Frame`, `Slice`, `Tileset`, `Tile`,
  `GraphicsContext`, `Selection`, `Range`, `Editor`, `Plugin`, `WebSocket`,
  `Timer`, `Color`, `ColorSpace`, `Rectangle`/`Point`/`Size` — consult the
  official page for the installed version before use.
- Nearest-neighbor as a named `Image:resize` method (docs name only
  `bilinear` and `rotsprite`).
- `app.command.*` individual command parameter lists — check
  <https://www.aseprite.org/api/> per command.
- LibreSprite CLI surface beyond what their docs state; their docs never
  mention batch mode or `--script-param`.
