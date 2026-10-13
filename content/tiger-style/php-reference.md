# Tiger Style for PHP

An independent language adaptation of the engineering philosophy in
TigerBeetle's Tiger Style and the supplied **Tiger Style for Lua, version 1.0**.
It is not an official TigerBeetle standard. The hierarchy is a project policy:

1. Preserve correctness, invariants, resource bounds, and authority boundaries.
2. Make resource use predictable and optimize measured costs.
3. Make the implementation easy to read, review, test, and maintain.

PHP's history is a long list of footguns that became features: loose
comparison, silent type coercion, `unserialize` object injection,
superglobals as ambient authority. Modern PHP (8.3+) has the tools to write
safe code — strict types, readonly properties, enums, fibers, real static
analysis. This guide assumes you use all of them, because the language's
defaults will not save you.

**MUST** marks a safety requirement of this guide. **SHOULD** marks a default
that may be changed with a documented reason. The 70-line method threshold
and 120-column target are review aids; never damage formatter output to
satisfy a metric.

## Table of Contents

1. [The Tiger Style Contract](#1-the-tiger-style-contract)
2. [Language and Runtime Assumptions](#2-language-and-runtime-assumptions)
3. [Strict Types, Every File](#3-strict-types-every-file)
4. [File and Project Layout](#4-file-and-project-layout)
5. [Assertions, Preconditions, and Postconditions](#5-assertions-preconditions-and-postconditions)
6. [Bound Everything](#6-bound-everything)
7. [Control Flow](#7-control-flow)
8. [Strict Comparison](#8-strict-comparison)
9. [Error Handling](#9-error-handling)
10. [Absence, False, and Defaults](#10-absence-false-and-defaults)
11. [Types and Declarations](#11-types-and-declarations)
12. [Superglobals Are Untrusted Input](#12-superglobals-are-untrusted-input)
13. [State and Mutation](#13-state-and-mutation)
14. [Resource Lifetime](#14-resource-lifetime)
15. [Database Access](#15-database-access)
16. [Serialization](#16-serialization)
17. [Filesystem Safety](#17-filesystem-safety)
18. [Processes and Shells](#18-processes-and-shells)
19. [Networking and HTTP](#19-networking-and-https)
20. [Concurrency: Fibers](#20-concurrency-fibers)
21. [Determinism](#21-determinism)
22. [Security](#22-security)
23. [Naming](#23-naming)
24. [Formatting](#24-formatting)
25. [Comments and Documentation](#25-comments-and-documentation)
26. [Dependencies](#26-dependencies)
27. [Language-Specific Engineering](#27-language-specific-engineering)
28. [Testing](#28-testing)
29. [Debugging](#29-debugging)
30. [Review Checklist](#30-review-checklist)
31. [Anti-Patterns](#31-anti-patterns)
32. [Reference Class](#32-reference-class)

---

## 1. The Tiger Style Contract

At each public operation, record: **input → validation → bounded computation
→ validated change → commit → cleanup**. State precisely whether failure
leaves state unchanged, partially updated, or requiring recovery.

PHP blurs validation and coercion: without `strict_types`, passing `"123abc"`
to an `int` parameter silently becomes `123`. The contract is meaningless if
the runtime rewrites inputs before validation sees them — hence §3 is
non-negotiable.

A PHPStan level-max (or Psalm strict) clean run is useful evidence, not a
replacement for runtime validation of untrusted input. [P1]

---

## 2. Language and Runtime Assumptions

Declare the PHP version, SAPI, and platform. The reference targets **PHP
8.3+** (typed class constants, `json_validate`, dynamic class constant
fetch); 8.4's property hooks and asymmetric visibility are allowed where
they clarify the contract. Minimum minor version goes in `composer.json`:

```json
"require": { "php": ">=8.3" }
```

Never assume the SAPI: CLI, FPM, RoadRunner/FrankenPHP workers, and embedded
SAPIs have different lifetime and concurrency semantics. The shared-nothing
request model holds for CLI-one-shot and FPM, and does **not** hold for
long-running workers — put SAPI-specific behavior behind an explicit
boundary (§13). Do not assume opcache, JIT, or a particular `php.ini`
(`zend.assertions`, `memory_limit`, `max_execution_time`) — assert what
matters at startup or document the requirement. [P2]

---

## 3. Strict Types, Every File

Every PHP file opens with:

```php
<?php

declare(strict_types=1);
```

No exceptions. Without it, scalar type declarations coerce: `"1e3"` becomes
`1000`, `"abc"` becomes `0` with a warning, `true` becomes `1`. That is not
a type system; it is a suggestion box.

`strict_types` is per-file, not per-project — a single missing declaration
reopens the hole. Enforce with a CI check (grep the opening lines) or a
PHP-CS-Fixer rule. Note it affects only *calls made from that file*:
parameters are checked strictly when the *calling* file declares strict
types. Library code cannot protect itself from a sloppy caller — validate
at the boundary anyway (§5). [P3]

---

## 4. File and Project Layout

PSR-4 autoloading via Composer. Layout:

```
src/
  Domain/
    Exception/
    ValueObject/
  Infrastructure/
bin/            # CLI entry points (thin: parse args, call application)
tests/
composer.json
phpstan.neon / psalm.xml
.php-cs-fixer.php
```

Including a file MUST NOT execute behavior: no top-level `session_start()`,
no connection opening, no output. Entry points (`bin/`, `public/index.php`)
are thin — parse input, build the application, run, format output.

One class per file, filename matches class name. Keep `composer.json`
scripts as the project's command vocabulary (`composer test`,
`composer analyse`, `composer fix`). [P4]

---

## 5. Assertions, Preconditions, and Postconditions

`assert()` is controlled by `zend.assertions`: with `-1` (production
recommendation) assertions are **not even compiled**. Never use `assert()`
for external validation or authorization — it may not exist at runtime.

```php
// Invariant: programmer error if violated. May vanish in production.
assert($count >= 0);

// External input: always an explicit throw. Survives zend.assertions=-1.
if ($count < 0) {
    throw new InvalidInputException("count must be >= 0, got {$count}");
}
```

Prefer small validators with precise postconditions over annotations plus
unchecked casts. A validator returns a validated value object or throws —
callers never re-check. [P5]

---

## 6. Bound Everything

A count bound alone is not a complete resource budget. Bound item count,
individual item size, nesting depth, cumulative bytes, elapsed time, and
retry amplification separately.

| Resource | Example policy | Exhaustion behavior |
| --- | --- | --- |
| Input bytes | 1 MiB before decoding | Reject before allocation grows. |
| JSON depth | 32 (`json_decode` `$depth`) | Reject the structure. |
| Collection items | 1,024 | Return a capacity error. |
| DB rows fetched | `LIMIT` + cursor | Page, never load-all. |
| Attempts | 3 total, one shared deadline | Return the final classified failure. |
| Captured output | 256 KiB per stream | Stop capture, terminate or drain. |
| Upload size | `upload_max_filesize` + app check | Reject at the boundary. |

PHP-specific: `memory_limit` is a backstop, not a budget — enforce
application-level bounds first. `max_input_vars` (default 1000) silently
truncates oversized forms; treat truncation as a validation signal, not a
silent default. Streaming (`fopen` + chunked reads) does not automatically
bound total consumption — bound the total.

Retries MUST require an idempotent operation or an idempotency mechanism.
One shared deadline, bounded backoff, injected jitter source. [P6]

---

## 7. Control Flow

Early returns/throws for invalid cases; ordinary loops for stateful work.
`match` over long `if`/`elseif` chains for dispatch — it is exhaustive and
strict by default.

Avoid truthiness shortcuts when `0`, `""`, `"0"`, or `false` are meaningful
domain values (§8, §10). Bound recursion or use an explicit stack.
Generators (`yield`) for bounded streaming over large datasets — but a
generator does not bound the *consumer*; bound both ends.

---

## 8. Strict Comparison

PHP's `==` performs type juggling. Memorize the horrors, then never use it
at a trust boundary:

```php
"0"   == false   // true
"1e3" == 1000    // true
"abc" == 0       // true (PHP < 8; still true for "" == 0 patterns)
0     == "foo"   // true (PHP < 8)
```

**Rule: `===` and `!==` everywhere a decision depends on the value.**
`==` is banned at trust boundaries and SHOULD be banned project-wide
(enforce with a fixer rule or code review).

`switch` uses loose comparison — prefer `match` (strict). `in_array()`
defaults to loose: always pass `true` as the third argument.
`array_search()` likewise. Sorting with `SORT_REGULAR` juggles types;
choose the sort flag deliberately. [P7]

---

## 9. Error Handling

Use domain exception classes consistently per boundary. Catch only failures
the layer can interpret. Chain context:

```php
try {
    $repo->save($order);
} catch (PDOException $e) {
    throw new OrderPersistenceException("save failed for order {$id}", 0, $e);
}
```

Rules:

- Never use `@` error suppression. A silenced failure is a hidden
  failure; convert errors to exceptions with a global handler instead.
- Do not catch `Throwable` casually: `Error` (type errors, assertion
  failures) usually indicates a programmer bug — let it propagate unless
  the layer is a deliberate bulkhead.
- A broad `catch` at a service boundary still needs logging, cleanup, and
  a defined failure response. Never leak secrets or stack internals in
  user-facing messages.
- Return a stable shape for similar errors (a result object or a
  documented exception hierarchy), not ad-hoc `false`/`null` mixes.
- `set_error_handler` + `set_exception_handler` at entry points: no
  silent warnings, no HTML error pages leaking paths.

---

## 10. Absence, False, and Defaults

PHP has four falsy-adjacent values that are NOT interchangeable: `null`,
`false`, `0`/`0.0`, `""`/`"0"`, `[]`. `empty()` conflates all of them —
**never use `empty()` to test for absence.**

```php
// Wrong: rejects 0, "0", false — all potentially valid.
if (empty($input)) { throw ...; }

// Right: absence is null, decide deliberately about the rest.
if ($input === null) { throw ...; }
```

- `??` (null coalescing) is the deliberate default operator:
  `$limit = $input['limit'] ?? self::DEFAULT_LIMIT;`
- `?:` (Elvis) conflates falsy — avoid unless falsy-collapse is the
  documented intent.
- Nullable types `?T` mean "T or null", not "argument may be omitted" —
  use a sentinel or a separate overload for omitted-vs-null.
- `isset()` returns false for null values; `array_key_exists()` is the
  precise presence test.
- Never use `null` to mean two different things in one API. [P8]

---

## 11. Types and Declarations

Type everything: properties, parameters, returns. Prefer:

- `readonly` properties (8.1+) where mutation is not part of the
  contract; `readonly class` (8.2+) for pure value objects.
- Backed enums over string constants: `enum Status: string`.
- `final` classes by default; inheritance is opt-in with a documented
  extension contract.
- Intersection types and `never`/`noreturn` where they express the
  contract (`never` for `die`-equivalents).
- No `mixed` to silence the analyzer. If the type is genuinely unknown,
  validate and narrow at the boundary, then carry a precise type.

Docblocks carry what the type system cannot: units, array shapes
(`array{status: string, count: int}`), thrown exceptions, ownership.

Variance: respect LSP — a subclass accepting *less* or returning *more*
breaks callers. The analyzer checks this; listen to it. [P9]

---

## 12. Superglobals Are Untrusted Input

`$_GET`, `$_POST`, `$_COOKIE`, `$_FILES`, `$_SERVER`, `$_ENV` are
attacker-controlled bytes. Rules:

1. Read them **once**, at the outermost boundary, into a validated
   request object. Business logic never touches a superglobal.
2. `filter_input()` / `filter_var()` for the mechanical layer; domain
   validators for the semantic layer.
3. `$_FILES` entries need: error code check (`UPLOAD_ERR_OK`),
   `is_uploaded_file()`, size bound, MIME sniffing (not the client-sent
   type), and a server-chosen destination name — never the client
   filename.
4. `$_SERVER['HTTP_*']` headers are user input. `REQUEST_URI`,
   `PATH_INFO`, `PHP_SELF` need parsing, not trust.
5. `register_globals` is long dead; `extract($_GET)` resurrects its
   ghost — banned.

```php
// Boundary: once, validated, then never touch $_GET again.
$page = filter_input(INPUT_GET, 'page', FILTER_VALIDATE_INT, [
    'options' => ['default' => 1, 'min_range' => 1, 'max_range' => 10_000],
]);
```

---

## 13. State and Mutation

- No mutable globals. No static properties holding request data.
- **Long-running workers break shared-nothing**: RoadRunner,
  FrankenPHP worker mode, and ReactPHP/Swoole keep the process alive
  across requests. Static caches, singletons with request state, and
  open handles MUST be reset per request or proven request-scoped.
  Construct per request; share only immutable services.
- Dependency injection over service location. Constructor injection;
  promoted properties for brevity.
- Prefer immutable value objects (`readonly class`) for domain data;
  mutation happens through explicit methods that return new instances
  or through a clearly-owned aggregate.
- Sessions: regenerate ID on privilege change; set `cookie_httponly`,
  `cookie_secure`, `cookie_samesite`; never store more than a
  reference in the session.

---

## 14. Resource Lifetime

One clear owner per handle, lock, transaction, process. `try/finally`
for cleanup; acquire late, release early:

```php
$handle = fopen($path, 'rb') ?: throw new IoException("open failed: {$path}");
try {
    // bounded work
} finally {
    fclose($handle);
}
```

- DB transactions: begin late, commit/rollback explicitly, never leave
  a transaction open across unbounded work.
- Locks (`flock`): always with a timeout strategy; never hold across
  network calls.
- Never rely on destructors (`__destruct`) for timely release —
  circular references and shutdown ordering make them unreliable.
- Generators holding resources: the consumer MUST close them
  (`$gen->close()` in `finally`) or resources leak until GC.

---

## 15. Database Access

PDO with **real** prepared statements:

```php
$pdo->setAttribute(PDO::ATTR_EMULATE_PREPARES, false);
$pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
$pdo->setAttribute(PDO::ATTR_DEFAULT_FETCH_MODE, PDO::FETCH_ASSOC);
```

- Never interpolate values into SQL. Ever. Not "just this once."
- Identifiers (table/column names) cannot be parameterized — allowlist
  them against a fixed set.
- `LIMIT`/`OFFSET` values are integers: cast and bound them.
- Fetch with cursors for large result sets; never `fetchAll()` on an
  unbounded query.
- Transactions: explicit begin/commit/rollback; retry only on
  serialization failures with idempotent bodies.
- Error mode MUST be exceptions — silent `false` returns are the
  `@`-suppression of databases.

---

## 16. Serialization

- **Never `unserialize()` untrusted bytes.** Object injection leads to
  RCE via gadget chains. Prefer JSON with schema validation.
- If `unserialize` is unavoidable (legacy session data): pass
  `['allowed_classes' => false]` or an explicit allowlist, and treat
  the result as untrusted.
- `json_decode`: always check `json_last_error()` or use
  `JSON_THROW_ON_ERROR`; bound `$depth`; decode to arrays or validated
  DTOs, not `stdClass` graphs you navigate blindly.
- `serialize()` output is PHP-specific — never a wire format.
- Signing: `hash_hmac('sha256', $data, $key)` with `hash_equals()` for
  comparison (never `===` on secrets — timing).

---

## 17. Filesystem Safety

- `mkdir` recursive before writing; check readability/writability.
- Never build include paths from user input (LFI). Autoload via
  Composer; allowlist any dynamic class resolution.
- Uploads: server-chosen names, outside the web root or with execution
  disabled, MIME-sniffed.
- `realpath()` + prefix check before operating on user-influenced paths
  (directory traversal).
- Temporary files: `tempnam()` + immediate `unlink`-after-open or
  `sys_get_temp_dir()` with random names; never predictable names.
- File locks for read-modify-write cycles; atomic writes via
  temp-file-then-`rename()`.

---

## 18. Processes and Shells

Every subprocess gets explicit argv, cwd, env, deadline, output bounds.
Prefer `proc_open` with an argv array over `exec()`/`shell_exec()` with
a string — the string form invokes a shell.

```php
$descriptors = [0 => ['pipe', 'r'], 1 => ['pipe', 'w'], 2 => ['pipe', 'w']];
$process = proc_open($argv, $descriptors, $pipes, $cwd, $env);
```

- If a shell string is unavoidable: `escapeshellarg()` every value,
  never the whole command.
- Timeouts: `stream_select()` with a deadline; kill (`proc_terminate`)
  on expiry; `proc_close` to reap. Distinguish exit status, signal,
  timeout, launch failure.
- Bound captured output (read in chunks with a total cap).
- Never pass user input as the command name.

---

## 19. Networking and HTTP

- cURL: `CURLOPT_TIMEOUT`/`CURLOPT_CONNECTTIMEOUT` always;
  `CURLOPT_PROTOCOLS` pinned to HTTPS; `CURLOPT_SSL_VERIFYPEER` never
  disabled; `CURLOPT_FOLLOWLOCATION` with `CURLOPT_MAXREDIRS` bounded.
- Verify response status codes explicitly; a 200 with an error body is
  still an error — parse the body contract.
- Bound response bodies (`CURLOPT_MAXFILESIZE` or streaming reads).
- Retries: idempotent methods only (or idempotency keys), bounded,
  with backoff and jitter.
- Secrets in `Authorization` headers, never in URLs.
- SSRF: validate and resolve URLs against an allowlist; never fetch a
  user-supplied URL without constraining the destination (no
  `169.254.169.254`, no intranet).

---

## 20. Concurrency: Fibers

Fibers (8.1+) are **cooperative**, not parallel: only one fiber runs at a
time, switching at explicit `Fiber::suspend()` points.

- Never assume ordering across fibers without explicit coordination.
- Bound fiber counts; each fiber has its own stack (memory cost).
- Shared state between fibers needs the same discipline as threads for
  the interleaving points — re-check state after every suspension.
- For true parallelism: processes (`proc_open`, `pcntl_fork` in CLI)
  or a queue/worker architecture. `pcntl` is unavailable in FPM —
  another SAPI boundary (§2).
- Async I/O libraries (Revolt, Amphp) are event loops, not threads;
  blocking calls stall the loop — use their non-blocking APIs.

---

## 21. Determinism

- Sort canonical keys before serializing/hashing (`ksort` recursively
  for signature payloads).
- Never let array insertion order define a wire format unless the
  format specifies ordering.
- Inject clocks (`ClockInterface`) and randomness; `random_bytes()` /
  `random_int()` for security-sensitive values (never `rand()`/`mt_rand()`
  for tokens).
- `DateTimeImmutable` over `DateTime`; explicit timezones, never the
  default; UTC for storage.

---

## 22. Security

- Passwords: `password_hash()` (bcrypt/argon2 via `PASSWORD_DEFAULT`),
  `password_verify()`, `password_needs_rehash()`. Nothing else.
- XSS: `htmlspecialchars($v, ENT_QUOTES | ENT_HTML5, 'UTF-8')` on every
  untrusted value entering HTML. Context-aware escaping for JS/URL/CSS
  contexts — one escaper does not fit all.
- CSRF: per-session tokens on state-changing requests; `SameSite=Lax`
  minimum on session cookies.
- SQLi: §15. Command injection: §18. LFI: §17. Object injection: §16.
  SSRF: §19. Header injection: never put untrusted values in headers
  without stripping CR/LF.
- `open_basedir` is not a security boundary. Neither is a WAF. Validate
  in the application.
- Log security events (auth failures, validation rejections at trust
  boundaries, checksum mismatches) without logging secrets.
- Keep `display_errors=Off` in production; errors go to logs, never to
  responses.

---

## 23. Naming

- `camelCase` methods and variables, `PascalCase` classes/interfaces/
  enums/traits, `UPPER_SNAKE_CASE` constants.
- Units in names: `$timeoutSecs`, `$maxBytes`, `$retryCount`.
- Booleans read as predicates: `$isActive`, `$hasPermission`.
- Interfaces: no `Interface` suffix unless the codebase convention
  demands it — prefer capability names (`LoggerInterface` is fine where
  PSR sets the precedent).
- Avoid abbreviations except industry-standard (`$url`, `$id`).

---

## 24. Formatting

One formatter: **php-cs-fixer** with PSR-12 (or Laravel Pint, which is a
php-cs-fixer preset runner — pick one per project, never both in
sequence). 4-space indent, 120 columns. Stable import ordering
(`ordered_imports`). Pin the version in `composer.json`
(`--dev`) and run it in CI; fail on diffs.

---

## 25. Comments and Documentation

Comments explain **why**, not what. Docblocks carry the public contract:

```php
/**
 * Parse and validate a page cursor.
 *
 * @param array{page?: mixed} $input Raw query parameters.
 * @return int Validated page number, 1-based.
 * @throws InvalidInputException If page is missing, non-integer, or out of range.
 */
function parsePage(array $input): int
```

Array shapes (`array{...}`), units, ownership, and thrown exceptions belong
in the docblock. Keep `@throws` accurate — the analyzer and the reader
both rely on it.

---

## 26. Dependencies

- `composer.json`: exact PHP requirement, sorted packages, no
  abandoned packages (`composer outdated --direct` in CI).
- Audit: `composer audit` in CI. A known CVE in a dependency is a
  build failure, not a backlog item.
- Lock file committed; `composer install --no-dev --optimize-autoloader`
  in production; `--no-scripts` unless scripts are reviewed.
- Each dependency is supply-chain surface: prefer well-maintained,
  scoped packages; avoid `*` version constraints.
- Platform check: `"platform-check": true` (default) so the wrong PHP
  fails fast instead of failing weirdly.

---

## 27. Language-Specific Engineering

- **OPcache**: enable in production (`opcache.enable=1`,
  `validate_timestamps=0` with a deploy-time restart); preloading for
  large codebases. Never develop against a stale cache — know which
  mode you are in.
- **JIT** (`opcache.jit`): measure before enabling; it helps CPU-bound
  code and does nothing for I/O-bound request handlers.
- **Attributes** over docblock annotations for machine-read metadata
  (routes, validation rules) — they are real syntax, refactorable and
  analyzable.
- **Enums** for every closed set; backed enums at trust boundaries
  (`Status::tryFrom($input)` validates in one call).
- **Fibers** for cooperative concurrency (§20); **parallel** extension
  only where the deployment guarantees it.
- **FFI**: powerful and dangerous — treat like `eval` for review
  purposes; bound and sandbox.

---

## 28. Testing

- **PHPUnit** (or Pest, which runs on PHPUnit) for unit/integration
  tests. Half the tests SHOULD be adversarial: malformed input,
  boundary values, empty input, oversized input, wrong types under
  `strict_types`, concurrent double-submit, expired tokens.
- Test the contract, not the implementation: public behavior, thrown
  exceptions, boundary values.
- Static analysis (PHPStan max / Psalm strictest) and the fixer run in
  CI before tests — a dirty analysis gate fails the build.
- Hermetic tests: in-memory SQLite or testcontainers, never the real
  database; clock injection for time-dependent tests; no network.
- Mutation testing (Infection) on critical paths where the budget
  allows — it catches tests that pass without asserting.

---

## 29. Debugging

- Xdebug: step debugging via the php-debug adapter (diver's
  `lua/dap/php`); `xdebug_break()` for hard breakpoints.
- `var_dump` is a last resort; structured logging
  (PSR-3, Monolog) with context arrays is the durable form.
- Reproduce in a container matching production (same PHP minor,
  same `php.ini`, same opcache mode) before declaring fixed.
- Profile with Xdebug or Blackfire — never optimize on intuition.

---

## 30. Review Checklist

- [ ] `declare(strict_types=1);` in every file; all properties,
      parameters, returns typed; no `mixed` escapes.
- [ ] External input validated by explicit throw; `assert()` only for
      invariants (valid with `zend.assertions=-1`).
- [ ] `===` at trust boundaries; no `==`, no `empty()` for absence;
      `in_array`/`array_search` strict; `match` over `switch`.
- [ ] `null` vs `false`/`0`/`""` semantics deliberate; `??` for
      defaults.
- [ ] Superglobals read once at the boundary into a validated object.
- [ ] All input sizes, loops, retries, output captures bounded by named
      constants.
- [ ] PDO real prepares; identifiers allowlisted; no SQL interpolation.
- [ ] No `unserialize`/`eval` on untrusted data; `allowed_classes`
      where unavoidable.
- [ ] Subprocesses use argv form with timeout, output bounds, cleanup.
- [ ] Every resource has one owner; `try/finally` cleanup on failure
      paths.
- [ ] Passwords via `password_hash`; output escaped contextually;
      CSRF tokens on mutations.
- [ ] Deterministic output where persisted, signed, or tested.
- [ ] Methods ≤ ~70 lines, lines ≤ 120 cols, one formatter
      (php-cs-fixer PSR-12 or Pint); PHPStan max / Psalm strict clean.

---

## 31. Anti-Patterns

| Pattern | Why it fails | Do this instead |
| --- | --- | --- |
| Missing `strict_types` | Silent coercion rewrites inputs | Declare it in every file |
| `==` at a boundary | Type juggling (`"0" == false`) | `===` / `!==` |
| `empty($x)` for absence | Conflates null/false/0/"" | `$x === null` |
| `@$unsafe` | Silences failures | Exception handler |
| `unserialize($user)` | Object injection → RCE | JSON + schema validation |
| `"SELECT ... $id"` | SQL injection | PDO real prepared statements |
| `extract($_GET)` | Variable poisoning | Validated request object |
| `include $page . '.php'` | LFI | Allowlisted routing |
| `shell_exec("cmd $in")` | Command injection | argv array + `escapeshellarg` |
| `md5($pw)` / `sha1($pw)` | Broken/fast hashes | `password_hash()` |
| `rand()` for tokens | Predictable | `random_bytes()` |
| Mutable static cache in worker | Cross-request leaks | Per-request construction |

---

## 32. Reference Class

```php
<?php

declare(strict_types=1);

namespace Qompass\Example;

use InvalidArgumentException;

/**
 * Bounded, validated page cursor for paginated listings.
 */
final readonly class PageCursor
{
    private const int MIN_PAGE = 1;
    private const int MAX_PAGE = 10_000;
    private const int DEFAULT_PAGE = 1;

    public function __construct(
        public int $page,
    ) {
        if ($page < self::MIN_PAGE || $page > self::MAX_PAGE) {
            throw new InvalidArgumentException(
                sprintf('page must be %d..%d, got %d', self::MIN_PAGE, self::MAX_PAGE, $page),
            );
        }
    }

    /**
     * Build from raw query parameters. The single boundary for this value.
     *
     * @param array{page?: mixed} $input
     * @throws InvalidArgumentException If page is present but not a valid integer.
     */
    public static function fromInput(array $input): self
    {
        $raw = $input['page'] ?? self::DEFAULT_PAGE;
        if (!is_int($raw)) {
            throw new InvalidArgumentException('page must be an integer');
        }

        return new self($raw);
    }

    public function offset(int $perPage): int
    {
        if ($perPage < 1 || $perPage > 100) {
            throw new InvalidArgumentException('perPage must be 1..100');
        }

        return ($this->page - 1) * $perPage;
    }
}
```
