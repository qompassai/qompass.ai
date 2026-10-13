# Tiger Style WGSL

## Purpose

Apply Matt's Tiger Style standard to WGSL shaders. Priority order, always:
**Safety > Performance > Developer Experience.** The full guide lives in
[references/TIGER_STYLE_WGSL.md](references/TIGER_STYLE_WGSL.md); this file
is the operational core. When the two disagree, the full guide wins.

## Workflow

1. Define the shader's contract: inputs (uniforms, textures, vertex
   attributes), backends targeted, maximum per-pixel work, fallback on
   shader-creation failure.
2. Write the WGSL with 16-byte uniform alignment (every `vec3` pads to
   16 bytes; prefer `vec4`). Document bind group layout in a comment.
3. Validate with naga (strip Bevy `#import` lines first — they are not
   WGSL): `grep -v "^#import" shader.wgsl > /tmp/check.wgsl && naga /tmp/check.wgsl`
4. Check wgsl-analyzer diagnostics (ignore `#import` line false positives
   for Bevy shaders; use `wgsl_bevy` filetype).
5. Run the portability test (`tests/shader_render.rs`) and capture a
   screenshot on target hardware. Naga cannot catch visual bugs.
6. Review against the checklist below before calling the shader done.

## Operating Rules

- **Contracts first.** Every shader answers: accepted/rejected inputs,
  trust boundary (uniform values from CPU), max ALU/texture work per
  pixel, owner of every binding slot, failure behavior (fallback
  material), backend coverage.
- **Portability is safety.** GLES3 (Android) is the constraint driver.
  No `f64`, no `@compute` without backend testing, no dynamic indexing,
  no subgroup ops without extensions. The portability test enforces this;
  do not weaken it without a backend-specific validation plan.
- **Alignment discipline.** Uniform structs must be 16-byte aligned.
  Order fields largest-first or add explicit `_pad` fields. Verify the
  Rust `ShaderType` struct matches field-for-field (order and types).
  `structLayoutHints = true` in the editor shows offsets visually.
- **Bind groups are engine contracts.** Bevy `Material2d` = group 2,
  `UiMaterial` = group 1. Document the layout in a shader comment.
  If the engine changes it, the comment makes the breakage obvious.
- **Bevy `#import` is not WGSL.** Strip before naga. Do not "fix"
  wgsl-analyzer errors on `#import` lines. Use `wgsl_bevy` filetype
  for Bevy shaders (analyzer stays quiet, naga CLI validates).
- **Bound everything.** Named constants with units (`MAX_PULSE_WIDTH`,
  `MIN_SOFTNESS`). Guard divisions (`max(w, 1e-4)`). Loop bounds
  statically determinable or constant-bounded.
- **No float equality.** Use epsilon comparisons. Do not rely on NaN
  bit patterns or denormal handling across backends.
- **Uniformity.** Texture samples need uniform control flow. Prefer
  procedural shaders (no textures) for 2D effects. Restructure instead
  of suppressing `derivative_uniformity` unless safety is proven.
- **Errors vs degradation.** Shader-creation failure → log, fall back
  to safe material, never black screen silently. Define the fallback
  before writing the shader.
- **Test gates.** naga validation + portability test + wgsl-analyzer
  + screenshot on hardware. All four before done. Headless tests use
  `Option<ResMut<Assets<T>>>` so missing material plugins don't panic.

## Review Checklist

- [ ] naga validates (imports stripped)
- [ ] Uniform structs 16-byte aligned (Rust side matches)
- [ ] Bind group indices correct for material type
- [ ] No non-portable constructs (`f64`, dynamic indexing, etc.)
- [ ] `requires` directives document the feature bar
- [ ] Fallback behavior defined
- [ ] Screenshot on target hardware
- [ ] `tests/shader_render.rs` passes
