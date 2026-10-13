# Tiger Style PHP

## Purpose

Apply the Tiger Style standard to PHP code. Priority order, always:
**Safety > Performance > Developer Experience.** The full guide lives in
[references/TIGER_STYLE_PHP.md](references/TIGER_STYLE_PHP.md); this
file is the operational core. When the two disagree, the full guide wins.

## Workflow

1. Declare the SAPI and runtime up front (CLI vs FPM, PHP 8.3+ target,
   opcache/JIT settings). Never assume request isolation holds for a
   long-running worker — put SAPI-specific behavior behind an explicit
   boundary. Type declarations are dev checks, not runtime validation of
   untrusted input.
2. Every file opens with `declare(strict_types=1);`. Lay code out
   top-to-bottom: namespace, imports, class/interface docblock, constants,
   constructor with promoted properties, public methods, private helpers.
   PSR-4 autoloading via Composer; including a file must not start
   sessions, open connections, or emit output.
3. Validate external input by throwing a precise domain exception;
   `assert()` only for programmer invariants — remember
   `zend.assertions=-1` strips assertions in production, so external
   rejection must survive it. Never use `==` on untrusted values; `===`
   always.
4. Bound everything with named `UPPER_SNAKE_CASE` class constants: input
   bytes, item counts, nesting depth, retries, captured output.
5. Review against the checklist below before calling the code done.

## Operating Rules

- **Contracts first.** Every function answers: permitted inputs, rejected
  inputs, max work, what state changes, who owns each resource, failure
  behavior. State whether failure leaves state unchanged or partial.
- **Errors:** use domain exception classes consistently per boundary;
  catch only failures the layer can interpret. Chain with
  `throw new DomainException(..., previous: $e)`. Never use `@` error
  suppression — a silenced failure is a hidden failure. No authorization
  or mutation inside `assert()`.
- **Strict equality.** `==` performs type juggling (`"0" == false`,
  `"1e3" == 1000`); it is banned at trust boundaries. Use `===`/`!==`.
  `empty()` conflates `null`, `false`, `0`, `""`, `"0"`, `[]` — test
  `=== null` explicitly when absence is the question.
- **Bound everything.** No unbounded `file_get_contents` on remote URLs,
  `iterator_to_array`, or retry. Retries need an idempotent operation, a
  shared deadline, bounded backoff, and an injected jitter source.
- **Superglobals are untrusted input.** `$_GET`, `$_POST`, `$_COOKIE`,
  `$_SERVER`, `$_FILES` are attacker-controlled bytes. Read them once at
  the boundary through a validated request object; never index them deep
  in business logic.
- **Serialization:** never `unserialize()` untrusted bytes (object
  injection); prefer JSON with schema validation, or pass
  `['allowed_classes' => false]`. Never `eval()` external text.
- **Database:** PDO with real prepared statements
  (`ATTR_EMULATE_PREPARES => false`); never interpolate values into SQL.
- **Shell:** every `proc_open`/`exec` call gets an explicit argv array,
  cwd, env, deadline, and output bounds; escape with `escapeshellarg()`.
  Distinguish exit status, signal, timeout, and launch failure.
- **Includes:** never `include`/`require` a path built from user input
  (LFI). Autoload via Composer PSR-4; allowlist any dynamic class
  resolution.
- **State:** no mutable globals or static caches holding request data in
  long-running processes (workers, RoadRunner, FrankenPHP); the
  shared-nothing request model does not hold there. Inject dependencies;
  construct per request or prove reuse is safe.
- **Concurrency:** PHP is shared-nothing per request; `Fiber` (8.1+) is
  cooperative, not parallel. Bound fiber counts; never assume ordering
  across fibers without explicit synchronization.
- **Names:** `camelCase` methods/variables, `PascalCase` classes,
  `UPPER_SNAKE_CASE` constants, units in names (`$payloadSizeBytes`).
  Typed properties everywhere; `readonly` where mutation is not part of
  the contract. No `mixed` to silence diagnostics.
- **Formatting:** 4-space indent, 120 columns, one configured formatter —
  php-cs-fixer with PSR-12 (or Laravel Pint). Stable import ordering, no
  competing formatters in sequence. Pin formatter and analyzer versions.
- **Functions:** ordinary methods stay near or below 70 lines. Named
  arguments for policy flags instead of positional booleans. Separate
  decoding, validation, computation, and I/O; return a stable shape for
  similar errors.
- **Resources:** one clear owner per handle, lock, transaction, process.
  `try/finally` for cleanup; register cleanup as acquisition succeeds.
  Never rely on destructors for timely release.
- **Determinism:** sort canonical keys before serializing; never let
  array-order or hash iteration define a wire format. Inject clocks and
  randomness; `random_bytes`/`random_int` for tokens.
- **Comments explain why**, not what. Docblocks carry the public
  contract: units, accepted types, thrown exceptions, side effects.

## Output Contract

PHP you produce must: open with `declare(strict_types=1);`,
type every property/parameter/return, throw precise domain exceptions on
expected failures, assert only internal invariants (valid with
`zend.assertions=-1`), use `===` at trust boundaries, bound all
input/loops/retries/output with named constants, read superglobals once
at the boundary, use real prepared statements, stay within 120 columns
and ~70 lines per method, and pass the review checklist.

## Review Checklist

- [ ] `declare(strict_types=1);` present; all types declared, no `mixed`
      escapes.
- [ ] External input validated by explicit throw; invariants asserted
      (and valid with assertions disabled).
- [ ] `===` at every trust boundary; no `==`, no `empty()` for
      absence checks.
- [ ] All input sizes, loops, retries, output captures bounded by named
      constants.
- [ ] `null` vs `false`/`0`/`""` semantics deliberate.
- [ ] Superglobals read once at the boundary into a validated object.
- [ ] No `unserialize`/`eval` on untrusted data; no user-controlled
      include paths.
- [ ] PDO real prepares; shell calls use argv form with timeout and
      cleanup.
- [ ] Every resource has one owner; cleanup runs on failure paths.
- [ ] Deterministic output where persisted or tested.
- [ ] Methods ≤ ~70 lines, lines ≤ 120 cols, one formatter
      (php-cs-fixer PSR-12 or Pint).
