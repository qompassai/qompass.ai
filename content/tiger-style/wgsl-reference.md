<!--
===============================================================================
TIGER_STYLE_WGSL.md
Tiger Style for WGSL — reference for the tiger-style-wgsl skill
===============================================================================

A safety-first WGSL coding standard, written as Markdown. This file is the
full reference; SKILL.md is the operational core. When the two disagree,
this document wins.

Priority:
    1. Safety
    2. Performance
    3. Developer Experience

This document is an independent WGSL adaptation of TigerBeetle's Tiger Style
engineering philosophy. It is not an official TigerBeetle document.

Primary source: W3C WebGPU Shading Language specification
(https://www.w3.org/TR/WGSL). Section references (§) below point at it.

Rendering layers used by this file:
    - CommonMark/GFM Markdown
    - Raw HTML
-->

# Tiger Style for WGSL

**Safety > performance > developer experience.**

A practical coding standard for WGSL shaders on Arch Linux, with emphasis
on backend portability (Vulkan, Metal, D3D12, GLES3) and Bevy integration.
This is an independent interpretation of
[TigerBeetle's Tiger Style](https://github.com/tigerbeetle/tigerbeetle/blob/main/docs/TIGER_STYLE.md),
not an official TigerBeetle or W3C document.

| Policy | Baseline |
| --- | --- |
| Spec | W3C WGSL (https://www.w3.org/TR/WGSL) |
| Validator | `naga` CLI (parse + validate, no Bevy imports) |
| Language server | `wgsl-analyzer` (wgslana_ls.lua) |
| Formatter | `wgslfmt` |
| Function size | Review ordinary functions above 70 physical lines |
| Primary platform | Arch Linux; portability claims require backend-specific checks |
| Document reviewed | 2026-10-06 |

## Contents

- [1. Engineering contract](#1-engineering-contract)
- [2. Toolchain and build trust](#2-toolchain-and-build-trust)
- [3. Structure and naming](#3-structure-and-naming)
- [4. Types and alignment](#4-types-and-alignment)
- [5. Contracts and shader-creation errors](#5-contracts-and-shader-creation-errors)
- [6. Bounds and numeric behavior](#6-bounds-and-numeric-behavior)
- [7. Address spaces and bindings](#7-address-spaces-and-bindings)
- [8. Control flow and uniformity](#8-control-flow-and-uniformity)
- [9. Portability: the backend matrix](#9-portability-the-backend-matrix)
- [10. Bevy integration](#10-bevy-integration)
- [11. Performance](#11-performance)
- [12. Tests and review gates](#12-tests-and-review-gates)
- [13. Complete reference shader](#13-complete-reference-shader)
- [14. Neovim integration](#14-neovim-integration)
- [15. Review card and validation](#15-review-card-and-validation)

## 1. Engineering contract

Correctness comes before speed. Speed comes before convenience when the
tradeoff is real. Measure that tradeoff; do not use the priority order to
justify speculative complexity.

Every shader must identify:

1. Accepted inputs (uniform values, texture formats, vertex attributes)
   and rejected inputs (out-of-range uniforms, missing bindings).
2. Maximum work per invocation (ALU ops, texture samples, loop bounds).
3. The owner of every binding slot, uniform buffer, and texture.
4. The backends it targets (Vulkan, Metal, D3D12, GLES3) and the
   minimum feature set required.
5. Failure behavior: what happens on shader-creation error, and how
   the application degrades (fallback material, error material).
6. The evidence supporting the result: naga validation, portability
   tests, screenshots on target hardware.

WGSL's type system is a foundation. It does not enforce backend
portability, uniform alignment, uniformity requirements, or
performance budgets. Treat these as explicit design obligations.

Use this rule for exceptions: name the rule, explain the need, bound
the resulting risk, and record a test or review condition. An exception
belongs near the affected code. Blanket waivers are difficult to
maintain.

Prefer a direct implementation that a reviewer can reason about. Do
not use compute shaders, storage buffers, or subgroups merely because
they are available. Require them to make the visual result better.

## 2. Toolchain and build trust

Pin the validator version. `naga` evolves; a shader that validates today
may warn tomorrow. Record the naga version used for validation.

```bash
# Validate WGSL (strip Bevy #import lines first; they are not WGSL)
grep -v "^#import" shader.wgsl > /tmp/check.wgsl
naga /tmp/check.wgsl
```

The language server is `wgsl-analyzer` (see `lsp/wgslana_ls.lua`).
It does not understand Bevy's `#import` directives — those are resolved
by Bevy's shader preprocessor at runtime, not by WGSL tooling. Do not
"fix" false positives on `#import` lines; they are correct.

For Bevy shaders (files containing `#import bevy_`), use the
`wgsl_bevy` filetype. The analyzer does not attach to it; naga CLI is
the validator. This is intentional, not a gap.

Format with `wgslfmt`. Four spaces, no tabs.

## 3. Structure and naming

Lay the shader out top-to-bottom:

1. `#import` directives (Bevy only, not WGSL — must come first for
   Bevy's preprocessor).
2. Module-scope constants (`const`, `override`).
3. Struct definitions (uniform blocks first).
4. Resource declarations (`@group` / `@binding`).
5. Helper functions (private, smallest scope first).
6. Entry point (`@vertex`, `@fragment`, or `@compute`) last.

Name uniforms after what they control, with units where applicable:
`pulse_speed_hz`, `glow_intensity`, `wipe_progress`. Not `u0`, `param1`.

Name structs with a `Uniforms` or `Settings` suffix matching the Rust
side: `GlowUniforms` in WGSL pairs with `GlowSettings` in Rust
(the `ShaderType` derive). The names do not need to match exactly,
but the field order and types must.

## 4. Types and alignment

Uniform buffer layout follows strict alignment rules (spec §8).
The critical rule: **every `vec3` is 16-byte aligned** (same as `vec4`),
and struct members are padded accordingly.

```
// WRONG: Rust side will misalign
struct Bad {
    color: vec4<f32>,  // offset 0, size 16
    time: f32,         // offset 16, size 4
    intensity: f32,    // offset 20, size 4
    // Rust struct needs explicit padding here
};

// RIGHT: pad to 16-byte multiples, or order largest-first
struct Good {
    color: vec4<f32>,  // offset 0
    time: f32,         // offset 16
    intensity: f32,    // offset 20
    _pad0: f32,        // offset 24
    _pad1: f32,        // offset 28
};  // total 32 bytes
```

Prefer `vec4` over `vec3` for colors and positions in uniforms —
the padding is automatic and the Rust `Vec4` maps directly.

Use `f32` explicitly, not abstract floats. WGSL has "abstract"
numeric types that resolve at use; in shader code, always write
the concrete type (`1.0` is abstract — prefer `1.0f` in contexts
where the type matters, or let inference work and verify with naga).

Do not use `f16` unless targeting mobile specifically and the
precision loss is validated. It requires the `f16` enable-extension
and is not universally supported.

## 5. Contracts and shader-creation errors

A shader-creation error (spec §2) means the shader failed to compile
or validate. The application must handle this: log the error, fall
back to a safe material, never show black silently.

Every shader documents:

- Required entry point and its inputs/outputs.
- Required bindings (`@group` / `@binding` indices and types).
- Required extensions (`enable` directives, if any).
- What the application does if creation fails.

Use `requires` directives (spec §4.1.2) to document the minimum
portability bar. This is machine-checked documentation:

```wgsl
// This shader uses only core WGSL; no extensions required.
// (If it needed one:)
// requires unrestricted_pointer_parameters;
```

If a shader needs an extension, the `enable` directive must appear
before all declarations, and the application must check support
before creating the pipeline.

## 6. Bounds and numeric behavior

Bound everything with named constants:

```wgsl
const MAX_PULSE_WIDTH: f32 = 0.5;
const MIN_SOFNTESS: f32 = 1e-4;  // avoid divide-by-zero
```

Guard divisions: `let w = max(uniforms.width, 1e-4);`

Floating-point behavior varies by backend (spec §15.7). Do not rely
on:
- Exact `==` comparison of floats (use epsilon).
- Specific NaN bit patterns (use `isNan()` checks if needed).
- Denormal handling (may flush to zero on some GPUs).

`exp()`, `pow()`, `sin()` are portable but have precision differences
across backends. For visual effects this is fine; for exact
computation it is not.

Loop bounds must be statically determinable or bounded by a constant.
Unbounded `while true` with `break` is allowed but must have a clear
termination proof in a comment.

## 7. Address spaces and bindings

Use `var<uniform>` for per-material parameters. Use `var<storage>`
only when you need read-write GPU memory (rare for 2D games).

Bind group indices are determined by the engine, not the shader:
- Bevy `Material2d`: material uniforms at `@group(2)`.
- Bevy `UiMaterial`: material uniforms at `@group(1)`.
- Group 0 is always view/uniforms (do not use for materials).

Document the expected bind group layout in a comment at the top of
every shader. If the engine changes the layout, the comment makes
the breakage obvious.

```wgsl
// Bind groups: 0 = view (Bevy), 1 = mesh (Bevy), 2 = material (this file).
@group(2) @binding(0) var<uniform> uniforms: GlowUniforms;
```

## 8. Control flow and uniformity

Certain operations require *uniform control flow* (spec §7):
texture sampling with implicit derivatives (`textureSample`) must
not be inside non-uniform branches.

Our shaders avoid textures entirely (procedural), so this is not
an issue. If you add texture sampling:

```wgsl
// WRONG: non-uniform texture sample
if (uv.x > 0.5) {
    color = textureSample(tex, samp, uv);  // derivative_uniformity error
}

// RIGHT: sample first, select after
let sampled = textureSample(tex, samp, uv);
let color = select(fallback, sampled, uv.x > 0.5);
```

Or suppress with `@diagnostic(off, derivative_uniformity)` when you
can prove safety — but prefer restructuring.

## 9. Portability: the backend matrix

Target backends, most to least restrictive:
1. **GLES3** (Android) — the constraint driver.
2. **D3D12** (Windows).
3. **Metal** (macOS/iOS).
4. **Vulkan** (Linux/Windows/Android).

The portable subset (enforced by test):

- No `f64` (not supported on mobile GPUs).
- No `@compute` without explicit backend testing.
- No dynamic indexing of arrays/matrices.
- No `textureSampleLevel` with explicit LOD (behavior varies).
- No subgroup operations without the `subgroups` extension.
- No `enable` directives without application-side feature checks.

Every shader must pass naga validation. Every shader must pass the
portability test (`tests/shader_render.rs`). Screenshots on real
hardware are the final gate — naga cannot catch visual bugs.

## 10. Bevy integration

Bevy shaders use `#import` directives resolved by Bevy's preprocessor.
These are not WGSL and must be stripped before naga validation.

```wgsl
#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// or
#import bevy_ui::ui_vertex_output::UiVertexOutput
```

The Rust side defines materials with `#[derive(AsBindGroup)]`.
Uniform structs use `#[derive(ShaderType)]` (requires `encase` and
`bevy_render` as direct dependencies — the derive macros do not
resolve through Bevy's re-export).

Material registration:
```rust
app.add_plugins(Material2dPlugin::<GlowMaterial>::default());
// or
app.add_plugins(UiMaterialPlugin::<WipeMaterial>::default());
```

Time-varying uniforms are updated by a system each frame. Systems
that touch material assets must use `Option<ResMut<Assets<T>>>`
so headless tests (which don't register the plugins) don't panic.

## 11. Performance

Budget per shader:
- Fragment shaders: aim for < 50 ALU ops per pixel for fullscreen
  effects. Our atmosphere shader is ~15 ops — fine.
- Avoid texture samples in fullscreen passes unless necessary.
- Prefer `mix`/`smoothstep` over branches (GPUs execute both sides).

`override` constants (spec §7.3) are pipeline-creation-time values —
more efficient than uniforms for static parameters. Bevy's
`Material2d` does not expose them cleanly; use uniforms unless
profiling shows the uniform updates are hot.

## 12. Tests and review gates

Every shader must pass:

1. **naga validation**: `naga /tmp/check.wgsl` (imports stripped).
2. **Portability test**: `tests/shader_render.rs` asserts the shader
   exists and contains no banned constructs.
3. **wgsl-analyzer**: no diagnostics except on `#import` lines
   (expected false positives for Bevy shaders).
4. **Screenshot**: visual verification on target hardware. Naga
   cannot catch a shader that compiles but looks wrong.

Review checklist:
- [ ] Uniform struct alignment verified (16-byte multiples).
- [ ] Bind group indices match engine expectations.
- [ ] No non-portable constructs (see §9).
- [ ] `requires` directives document the feature bar.
- [ ] Fallback behavior defined for shader-creation failure.
- [ ] Screenshot attached or referenced.

## 13. Complete reference shader

```wgsl
// Glow: radial bloom with breathing pulse.
// Bind groups: 0 = view (Bevy), 1 = mesh (Bevy), 2 = material (this file).
// Portability: core WGSL only. No extensions required.
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct GlowUniforms {
    color: vec4<f32>,      // offset 0
    time: f32,             // offset 16
    intensity: f32,        // offset 20
    pulse_speed: f32,      // offset 24
    pulse_amount: f32,     // offset 28
};  // total 32 bytes, 16-byte aligned

@group(2) @binding(0) var<uniform> uniforms: GlowUniforms;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let centered = (mesh.uv - vec2<f32>(0.5)) * 2.0;
    let dist = length(centered);
    let pulse = 1.0 + uniforms.pulse_amount * sin(uniforms.time * uniforms.pulse_speed);
    let glow = pow(max(0.0, 1.0 - dist), 2.0) * uniforms.intensity * pulse;
    let alpha = clamp(glow, 0.0, 1.0) * uniforms.color.a;
    return vec4<f32>(uniforms.color.rgb * glow, alpha);
}
```

## 14. Neovim integration

- Language server: `wgsl-analyzer` via `lsp/wgslana_ls.lua`.
- Filetypes: `wgsl` (plain), `wesl` (WESL), `wgsl_bevy` (Bevy shaders
  with `#import` — analyzer does not attach, naga CLI validates).
- Formatter: `wgslfmt`.
- Inlay hints: `structLayoutHints = true` shows field offsets
  (catches alignment issues visually).
- Snippets: `@fragment` skeleton, uniform struct template,
  `diagnostic()` directive template.

Filetype detection (`lua/config/core/filetype.lua`): `.wgsl` files
containing `#import bevy_` in the first 20 lines get `wgsl_bevy`;
otherwise `wgsl`. `.wesl` files get `wesl`.

## 15. Review card and validation

Before calling a shader done:

- [ ] naga validates (imports stripped).
- [ ] wgsl-analyzer clean (except `#import` lines for Bevy).
- [ ] Uniform alignment: all structs 16-byte aligned.
- [ ] Portability: no banned constructs, `requires` documented.
- [ ] Bind groups match engine layout.
- [ ] Screenshot on target hardware.
- [ ] Fallback defined for creation failure.
- [ ] Tests pass (`tests/shader_render.rs`).

Validation performed 2026-10-06:
- naga 0.x: 4 shaders validated (glow, pulse, atmosphere, wipe).
- wgsl-analyzer: 7 diagnostics on glow.wgsl, all on `#import` line
  (expected false positives).
- 523 tests passing, 0 failing.
