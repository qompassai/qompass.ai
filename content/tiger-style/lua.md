# Tiger Style Lua

## Purpose

Apply Matt's Tiger Style standard to Lua code. Priority order, always:
**Safety > Performance > Developer Experience.** The full guide lives in
[references/TIGER_STYLE_LUA.md](references/TIGER_STYLE_LUA.md); this file is
the operational core. When the two disagree, the full guide wins.

## Workflow

1. Determine the runtime (Lua 5.4+, LuaJIT, Neovim, embedded). Put
   runtime-specific behavior behind a small explicit boundary; never assume
   one silently.
2. Lay the module out top-to-bottom: header/purpose, host API aliases,
   `local M = {}`, constants, LuaCATS types, validation helpers, private
   leaf helpers, orchestration functions, `setup()` if needed, `return M`.
3. Validate external input with `return nil, "descriptive error"`; assert
   internal invariants with `assert()` (one invariant per assert).
4. Bound everything with named `*_MAX` constants: loops, retries, queues,
   caches, histories, output sizes.
5. Review against the checklist below before calling the code done.

## Operating Rules

- **Contracts first.** Every function answers: permitted inputs, rejected
  inputs, max work, what state changes, who owns each resource, failure
  behavior.
- **Assertions are for programmer errors and invariants**, never for
  external input. Never write bare `pcall(do_work)` — capture and
  propagate the error.
- **Bound everything.** No unexplained `while true`, no unbounded retry,
  no unbounded growth. Retry budgets are named constants
  (e.g. `RETRY_COUNT_MAX = 4`).
- **Control flow:** positive conditions, branch-light leaf loops, no
  `ready and start()` statement-shorthand, no recursion over
  attacker-controlled depth.
- **Nil/false discipline:** never `local enabled = options.enabled or true`
  when `false` is valid — check `== nil` explicitly. Give `nil` one
  documented meaning per context.
- **Names are `snake_case` with units last:** `timeout_ms`,
  `payload_size_bytes`, `retry_count`, `generation_id`. No `n`, `sz`,
  `tmp2`, `stuff`.
- **Formatting:** 4-space indent, max 100 columns, one statement per line,
  trailing commas in multiline tables. stylua config:
  `column_width = 100`, `indent_type = "Spaces"`, `indent_width = 4`.
- **Functions:** ordinary functions stay near or below 70 lines. Prefer
  named option tables over positional booleans:
  `copy_range({ source = ..., overwrite = true })`.
- **State and mutation:** validate input, do pure/bounded leaf work, commit
  at a single obvious point. Copy caller-owned config if later mutation is
  possible. Async callbacks carry a generation token and reject stale
  results.
- **Resources:** one clear owner per file, socket, timer, `vim.uv` handle,
  subprocess. Teardown is idempotent and nils the handle.
- **Security:** never interpolate untrusted values into shell strings —
  use `vim.system({ "git", "status", "--", user_path })` argv form.
  Treat `load`, `dofile`, `package.loadlib`, FFI as privileged.
- **Determinism:** sort map keys before serializing; never rely on
  `pairs()` order; make locale/timezone assumptions explicit.
- **Comments explain why**, not what. Use LuaCATS (`---@param`,
  `---@return`, `---@class`) where it aids review.
- **Neovim:** prefer `vim.api`, `vim.fs`, `vim.system`, `vim.uv`; keep
  `setup()` idempotent; `require()` loads definitions, not global side
  effects.

## Output Contract

Lua you produce for Matt must: compile cleanly, carry LuaCATS types on
exported functions, assert its invariants, return `(nil, err)` on expected
failures, bound all loops/retries/queues, stay within 100 columns and ~70
lines per function, and pass the review checklist.

## Review Checklist

- [ ] External input validated before privileged use; internal invariants asserted.
- [ ] All loops/retries/caches/queues bounded by named constants.
- [ ] `nil` vs `false` semantics deliberate; no `or` default clobbers `false`.
- [ ] Index/count/size/unit names explicit and unit-suffixed.
- [ ] Every resource has one owner; teardown exists on failure paths.
- [ ] No shell string interpolation of untrusted input; argv form used.
- [ ] Async callbacks reject stale generations; handles revalidated after delay.
- [ ] Output deterministic where persisted or tested; no `pairs()` order reliance.
- [ ] Functions ≤ ~70 lines, lines ≤ 100 cols, comments explain intent.
