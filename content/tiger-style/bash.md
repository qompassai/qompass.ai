# Tiger Style Bash

## Purpose

Apply the Tiger Style standard to Bash scripts. Priority order, always:
**Safety > Performance > Developer Experience.** The full guide lives in
[references/TIGER_STYLE_BASH.md](references/TIGER_STYLE_BASH.md); this
file is the operational core. When the two disagree, the full guide wins.

## Workflow

1. Declare the interpreter and version up front: `#!/usr/bin/env bash`
   plus a runtime version guard (`BASH_VERSINFO`). Never write bashisms
   in a `#!/bin/sh` script — pick one and be honest about it.
2. Open every script with the strict-mode header (see below). Lay out
   top-to-bottom: header comment (purpose, usage, exit codes), strict
   mode, constants, helpers, `main()` with `"$@"`, and the
   `main "$@"` invocation guard.
3. Quote every expansion: `"$var"`, `"${arr[@]}"`, `"$(cmd)"`. An
   unquoted expansion is a bug until proven otherwise.
4. Bound everything with named `UPPER_SNAKE_CASE` constants: timeouts,
   retry counts, bytes read, loop iterations.
5. Review against the checklist below before calling the script done.

## Operating Rules

- **Strict mode, honestly.** `set -euo pipefail` is the starting point,
  not a safety proof: `-e` does not fire inside `if` conditions,
  `&&`/`||` lists, or the last command of a pipeline's context in some
  versions. Check the exit status of anything that matters explicitly.
  `set -u` turns unset variables into errors — initialize or default
  (`${VAR:-default}`) deliberately.
- **Contracts first.** Every function answers: permitted inputs,
  rejected inputs, max work, what state changes, exit codes, failure
  behavior. A function communicates status via its return code and data
  via stdout — never mix the two.
- **Quoting.** `"$var"` always. `"${arr[@]}"` for arrays, never
  `$arr` or `${arr[*]}` unquoted. `read -r` always (raw backslashes).
  `IFS=` prefix when reading whole lines. Never `for x in $(cmd)` —
  use `while IFS= read -r` loops or `mapfile`.
- **No `eval`.** No `eval`, no `source` on computed paths, no
  `curl | bash`. If a value must become code, that is a design failure —
  restructure. Verify checksums/signatures on downloaded artifacts
  before executing them.
- **Temp files:** `mktemp` only, never predictable `/tmp` names (symlink
  attacks). Register cleanup with `trap 'rm -rf "$TMPDIR"' EXIT` at
  creation time. One trap per concern; append, don't overwrite.
- **Globbing:** never parse `ls`. Set `nullglob`/`failglob` deliberately
  and document the choice. Quote globs you mean literally.
- **Errors:** a `die()` helper (`printf ... >&2; exit 1`) for fatal
  errors. Distinguish exit codes: 0 success, 1 general error, 2 usage
  error. Log to stderr; data to stdout. Never swallow a failure with
  `|| true` unless you can state what you are ignoring and why.
- **Pipelines:** `pipefail` so a mid-pipe failure is not masked by the
  last command's success. Capture pipeline status explicitly when the
  stages have different failure meanings (`PIPESTATUS`).
- **Bound everything.** `timeout` on network/subprocess calls, `head -c`
  on unbounded reads, `read -t` on interactive input, `--max-count` on
  searches. No unbounded `find /` or recursive globs without depth
  limits.
- **Filesystem:** `mkdir -p` before writing; check with `-e`/`-f`/`-d`
  before acting on paths from outside. Use `--` to end option parsing
  before user-supplied paths (`rm -- "$f"`). Never `rm -rf` an
  unvalidated variable — assert it is non-empty and under an expected
  prefix first.
- **Environment:** treat the environment as untrusted input. `PATH`
  manipulation must be explicit and logged. Never trust `CDPATH`,
  `IFS`, or `LD_PRELOAD` across a privilege boundary.
- **Portability:** if the script must run on macOS/BSD too, avoid GNU-only
  flags or gate them behind `uname` checks. Document the supported
  platforms in the header.
- **Functions:** `local` for every function variable. Functions stay near
  or below 70 lines. One purpose per function; helpers prefixed
  consistently.
- **Formatting:** `shfmt -i 4 -ci` (4-space indent). One formatter, no
  hand-formatting fights. shellcheck must report zero warnings —
  suppressions (`# shellcheck disable=`) require a comment explaining
  why the warning is wrong, not just noisy.
- **Testing:** bats for behavior tests; test the failure paths, not just
  the happy path (half-adversarial). Test with `shellcheck` clean as a
  gate.
- **Comments explain why**, not what. Header carries the contract:
  purpose, usage, arguments, exit codes, side effects, required tools.

## Output Contract

Bash you produce must: open with `#!/usr/bin/env bash` and the
strict-mode header, quote every expansion, use `local` in all functions,
create temp files only via `mktemp` with `trap` cleanup, never `eval` or
pipe curl to bash, bound all reads/loops/retries/timeouts with named
constants, separate data (stdout) from diagnostics (stderr), pass
`shellcheck` with zero warnings and `shfmt` clean, and pass the review
checklist.

## Review Checklist

- [ ] `#!/usr/bin/env bash`, version guard, strict-mode header present.
- [ ] Every expansion quoted; `read -r`; no `for x in $(...)`.
- [ ] No `eval`, no `source` on computed paths, no `curl | bash`.
- [ ] Temp files via `mktemp` with `trap` cleanup registered at
      creation.
- [ ] Exit codes documented and deliberate; `die()` for fatal errors;
      no unexplained `|| true`.
- [ ] `pipefail` set; `PIPESTATUS` checked where stage failures differ.
- [ ] All reads, loops, retries, timeouts bounded by named constants.
- [ ] `rm -rf` targets validated non-empty and under expected prefix;
      `--` before user paths.
- [ ] Functions use `local`; ≤ ~70 lines; single purpose.
- [ ] `shellcheck` zero warnings; `shfmt` clean; bats tests cover
      failure paths.
