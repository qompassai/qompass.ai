# Tiger Style for Bash

An independent language adaptation of the engineering philosophy in
TigerBeetle's Tiger Style and the supplied **Tiger Style for Lua, version 1.0**.
It is not an official TigerBeetle standard. The hierarchy is a project policy:

1. Preserve correctness, invariants, resource bounds, and authority boundaries.
2. Make resource use predictable and optimize measured costs.
3. Make the implementation easy to read, review, test, and maintain.

A shell script is the most dangerous code in a project: it runs with the
invoker's full authority, its failures are silent by default, and its type
system is "everything is a string, sometimes." This guide treats every script
as a small privileged program and applies the same contract discipline as any
other language.

**MUST** marks a safety requirement of this guide. **SHOULD** marks a default
that may be changed with a documented reason. The 70-line function threshold
is a review aid; never damage `shfmt` output to satisfy a metric.

## Table of Contents

1. [The Tiger Style Contract](#1-the-tiger-style-contract)
2. [Language and Runtime Assumptions](#2-language-and-runtime-assumptions)
3. [The Strict-Mode Header](#3-the-strict-mode-header)
4. [File and Script Layout](#4-file-and-script-layout)
5. [Assertions, Preconditions, and Postconditions](#5-assertions-preconditions-and-postconditions)
6. [Bound Everything](#6-bound-everything)
7. [Control Flow](#7-control-flow)
8. [Quoting and Expansion](#8-quoting-and-expansion)
9. [Error Handling and Exit Codes](#9-error-handling-and-exit-codes)
10. [Pipelines](#10-pipelines)
11. [Functions](#11-functions)
12. [Variables and Scope](#12-variables-and-scope)
13. [Arrays](#13-arrays)
14. [Input Reading](#14-input-reading)
15. [Temp Files and Cleanup](#15-temp-files-and-cleanup)
16. [Filesystem Safety](#16-filesystem-safety)
17. [Processes and Subshells](#17-processes-and-subshells)
18. [Environment Discipline](#18-environment-discipline)
19. [No Eval, No Pipe-to-Shell](#19-no-eval-no-pipe-to-shell)
20. [Globbing and Filename Handling](#20-globbing-and-filename-handling)
21. [Arithmetic](#21-arithmetic)
22. [Networking](#22-networking)
23. [Portability](#23-portability)
24. [Security](#24-security)
25. [Naming](#25-naming)
26. [Formatting](#26-formatting)
27. [Comments and Documentation](#27-comments-and-documentation)
28. [Dependencies](#28-dependencies)
29. [Testing](#29-testing)
30. [Debugging](#30-debugging)
31. [Review Checklist](#31-review-checklist)
32. [Anti-Patterns](#32-anti-patterns)
33. [Reference Script](#33-reference-script)

---

## 1. The Tiger Style Contract

The contract is identical across all Tiger Style languages: at each operation,
record **input → validation → bounded computation → validated change → commit
→ cleanup**. State precisely whether failure leaves state unchanged, partially
updated, or requiring recovery.

For shell, the contract has extra weight because the shell's defaults oppose
it: unquoted expansions split and glob, failed commands are ignored, and
`rm -rf` does exactly what it says. The guide's rules exist to close the gap
between what the script appears to do and what it actually does.

A `shellcheck` clean run is useful evidence, not a replacement for reasoning
about failure modes. [P1]

---

## 2. Language and Runtime Assumptions

Declare the interpreter and minimum version. The reference targets **Bash
4.4+** (associative array improvements, `${var@Q}`); prefer 5.x where
available. Guard at startup:

```bash
if (( BASH_VERSINFO[0] < 4 )) || (( BASH_VERSINFO[0] == 4 && BASH_VERSINFO[1] < 4 )); then
    printf 'error: bash 4.4+ required (have %s)\n' "$BASH_VERSION" >&2
    exit 2
fi
```

Never write bashisms under a `#!/bin/sh` shebang. If the script must be
POSIX `sh`, write POSIX `sh` and lint it with `shellcheck -s sh` plus
`checkbashisms`. Do not assume GNU coreutils on macOS/BSD — gate GNU-only
flags or document Linux-only in the header. [P2]

---

## 3. The Strict-Mode Header

Every script opens with:

```bash
#!/usr/bin/env bash
set -euo pipefail
```

What each flag does and — critically — what it does **not** do:

- `set -e`: exits on a failing command. **Does not fire** inside `if`/`while`
  conditions, `&&`/`||` lists (except the final command), or commands whose
  return status is inverted with `!`. A failing command in a command
  substitution assigned to a variable DOES trigger it in bash 4.4+, but do
  not rely on version-specific corners: check anything that matters
  explicitly.
- `set -u`: unset variable expansion is an error. Use `${VAR:-default}`
  deliberately — a default is a decision, document it.
- `set -o pipefail`: a pipeline's status is the last **nonzero** stage's
  status. Without it, `false | true` succeeds silently.

Strict mode is the starting point. Explicit status checks on meaningful
operations are the actual safety. [P3]

---

## 4. File and Script Layout

Top-to-bottom:

1. Shebang.
2. Header comment: purpose, usage, arguments, exit codes, side effects,
   required tools, supported platforms.
3. Strict-mode header + version guard.
4. `readonly` constants (`UPPER_SNAKE_CASE`).
5. Helper functions (each with a contract comment).
6. `main()` function.
7. Invocation guard at the bottom: `main "$@"`.

Including (sourcing) a library file MUST NOT execute behavior — libraries
contain function definitions and constants only. A script that is both
sourced and executed guards with:

```bash
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
```

---

## 5. Assertions, Preconditions, and Postconditions

Bash has no `assert`. The equivalent is an explicit guard that fails loudly:

```bash
# Invariant: work dir must exist before we write into it.
[[ -d "$WORK_DIR" ]] || die "invariant violated: missing work dir: $WORK_DIR"
```

Use guards for programmer invariants AND for external validation — unlike
compiled languages, there is no optimizer stripping them, so one mechanism
serves both. Never perform the guarded action and the guard in the same
compound command where `-e` semantics get murky; guard first, act second.
[P4]

---

## 6. Bound Everything

A script without bounds is a fork bomb waiting for a large input. Bound
item count, bytes, depth, concurrency, elapsed time, and retries separately.

| Resource | Example policy | Exhaustion behavior |
| --- | --- | --- |
| Network/subprocess time | `timeout 30s` | Kill, report timeout distinctly. |
| Bytes read | `head -c 1M`, `read -n` | Truncate by policy, warn. |
| Interactive input wait | `read -t 10` | Fail closed, do not hang CI. |
| Loop iterations | named `MAX_*` constant | Break with a diagnostic. |
| Find depth | `find -maxdepth 3` | Do not walk the whole tree. |
| Retry attempts | 3 total, one shared deadline | Return the final classified failure. |
| Captured output | `... | head -c 256K` | Stop capture, keep going or abort by policy. |

Retries MUST require an idempotent operation. Use one shared deadline and
bounded backoff with jitter; never stack invisible retry policies across
a script and the tools it calls. [P5]

---

## 7. Control Flow

Early exits for invalid cases; ordinary loops for stateful work. Prefer
`case` over long `if`/`elif` chains for dispatch. Avoid clever
short-circuit expressions as control flow (`[[ x ]] && foo`) for anything
with side effects — write the `if`.

Bound recursive descent or use an explicit stack. `find -exec` with `+`
batches better than `-exec \;` per file; `xargs -0` when filenames may
contain whitespace (they will).

---

## 8. Quoting and Expansion

This section is the heart of safe shell. An unquoted expansion undergoes
word splitting AND pathname expansion. Quote every expansion:

```bash
"$var"            # always
"${arr[@]}"       # all elements, each intact — never $arr or ${arr[*]} unquoted
"$(cmd)"          # command substitution, quoted
"${var:-default}" # deliberate default
"${var:?error}"   # deliberate hard failure on unset/empty
```

`${var:?message}` is the strictest default: it aborts with `message` if
`var` is unset or empty. Use it for variables that must never be empty —
especially before `rm -rf`:

```bash
rm -rf "${TARGET_DIR:?TARGET_DIR must be set}"
```

Brace expansion and tilde expansion happen before variable expansion and
do not occur inside quotes — `"{$a,$b}"` does not expand. Keep the
distinction explicit.

`${var@Q}` (bash 4.4+) produces a quoted form suitable for re-evaluation —
useful for diagnostics that show exactly what a value contains. [P6]

---

## 9. Error Handling and Exit Codes

Define a `die()` helper once per project and use it everywhere:

```bash
die() {
    # Contract: prints message to stderr, exits 1. Never returns.
    printf 'error: %s\n' "$*" >&2
    exit 1
}
```

Exit code contract: `0` success, `1` general error, `2` usage error
(matches `getopts`/`getopt` conventions). Document script-specific codes
in the header.

Rules:

- Data goes to stdout; diagnostics go to stderr. A script whose stdout is
  piped MUST NOT print chatter to stdout.
- Never `|| true` a failure you cannot name. If you ignore a status, say
  what you are ignoring and why in a comment.
- Check the status of `cd` — `cd "$dir" || die "..."` — or a failed `cd`
  silently operates in the wrong directory. (This is how files get deleted
  in the wrong tree.)
- Functions return status via `return`; data via stdout. Never mix the
  two on one stream.

---

## 10. Pipelines

`pipefail` is set, so a mid-pipe failure surfaces. When stages have
different failure meanings, inspect `PIPESTATUS` immediately — it is
overwritten by the next command:

```bash
output=$(generate | transform)
status=( "${PIPESTATUS[@]}" )
(( status[0] == 0 )) || die "generate failed with ${status[0]}"
(( status[1] == 0 )) || die "transform failed with ${status[1]}"
```

Note: `PIPESTATUS` inside a command substitution reflects the
substitution's pipeline. Capture it in the same assignment statement's
wake, before any other command runs.

Avoid useless `cat`; prefer redirection. For process substitution
`<(cmd)` vs pipes, choose based on which side needs the exit status —
process substitution hides it.

---

## 11. Functions

- `local` for EVERY function variable. Without it, variables are global
  and leak across calls — the classic source of "works once, breaks in a
  loop."
- One purpose per function; name it `<verb>_<object>`.
- Functions stay near or below 70 lines.
- Contract comment above each function: inputs (`$1`, `$2`...), stdout
  shape, exit codes, side effects.
- Validate arguments at the top: `(( $# == 2 )) || die "usage: ..."`.

```bash
# Contract: $1 = source dir (must exist), $2 = dest dir (created).
# stdout: none. exit 0 on success, 1 on failure. Side effect: copies tree.
sync_tree() {
    local src="$1" dst="$2"
    (( $# == 2 )) || die "usage: sync_tree SRC DST"
    [[ -d "$src" ]] || die "source dir missing: $src"
    mkdir -p "$dst" || die "cannot create: $dst"
    ...
}
```

---

## 12. Variables and Scope

- `readonly` for constants; declare them once at the top.
- `local` in every function (see §11).
- `export` only what child processes need; never export secrets
  unnecessarily (they appear in `/proc/<pid>/environ`).
- `declare -i` for integer arithmetic intent; `declare -A` for maps
  (bash 4+); `declare -r` as `readonly` alternative.
- Unset what you no longer need in long-running scripts (`unset`).

---

## 13. Arrays

Indexed arrays for ordered data, associative arrays for maps. The rules
that prevent the classic bugs:

```bash
files=()                       # initialize; set -u safe
files+=( "$new" )              # append, quoted
for f in "${files[@]}"; do     # iterate, quoted — safe with spaces/newlines
printf '%s\n' "${files[@]}"    # print, quoted
count="${#files[@]}"           # length
```

Never `${files[*]}` unquoted (joins with first char of IFS, then splits).
`mapfile -t arr < <(cmd)` reads lines into an array without the
subshell-variable-loss problem. Associative arrays: `declare -A m`,
always quote keys: `"${m[$key]}"`.

---

## 14. Input Reading

```bash
while IFS= read -r line; do
    ...
done < "$input_file"
```

- `read -r`: raw, backslashes are literal. Always.
- `IFS=` prefix: preserves leading/trailing whitespace.
- `read -t SECS`: bounded wait; fail closed on timeout.
- `read -n N` / `read -N N`: bounded character counts.
- Never `for line in $(cat file)` — word splitting destroys the input.
- `read` returns nonzero at EOF; with `set -e`, structure loops so the
  final failed `read` does not abort the script (the `while` condition
  context is exempt — but know WHY it is exempt).

---

## 15. Temp Files and Cleanup

```bash
WORK_DIR=$(mktemp -d "${TMPDIR:-/tmp}/mytool.XXXXXX") || die "mktemp failed"
trap 'rm -rf "$WORK_DIR"' EXIT
```

- `mktemp` only. Predictable `/tmp` names invite symlink attacks.
- Register the `trap` on the line after creation — a crash between
  creation and trap registration leaks.
- `trap '...' EXIT` covers normal exit, `die()`, and signals that
  terminate the shell. Add `INT TERM` explicitly for long-running
  scripts that must clean up promptly.
- Append to traps, don't overwrite: capture the existing trap or use a
  single cleanup function that handles all resources.
- One cleanup function per script that releases everything it owns.

---

## 16. Filesystem Safety

- `mkdir -p` before writing into a directory tree you expect.
- Check `-e`/`-f`/`-d`/`-r`/`-w` before acting on paths from outside the
  script — then act. (TOCTOU is real, but checking is still better than
  not checking; for security-critical moves, operate on file descriptors
  or use `O_NOFOLLOW` semantics via dedicated tools.)
- `--` ends option parsing: `rm -- "$f"`, `printf -- "$fmt"`. A filename
  starting with `-` must never be parsed as a flag.
- `rm -rf` rules (non-negotiable):
  1. The variable MUST be validated non-empty: `"${DIR:?}"`.
  2. It SHOULD be checked under an expected prefix.
  3. Never `rm -rf "$VAR/"` where `VAR` could be `/` or empty.
- Prefer `mv`/`cp -a` over delete-then-recreate for atomicity where it
  matters.
- `umask` deliberately at the top of scripts that create sensitive files
  (`umask 077` for secrets).

---

## 17. Processes and Subshells

Every subprocess gets: explicit argv, cwd, env, deadline, output bounds.

```bash
if ! timeout 30s command --flag "$arg" >"$out" 2>"$err"; then
    rc=$?
    (( rc == 124 )) && die "command timed out after 30s"
    die "command failed with $rc; see $err"
fi
```

- `timeout` (coreutils) for deadlines; distinguish 124 (timeout) from
  real failures.
- Pipelines in `$(...)` run in subshells: variable assignments inside do
  not survive. Use `mapfile`, temp files, or restructure.
- `( subshell )` for isolated `cd`/env changes; `{ group; }` when you
  need the current shell.
- Background jobs: track PIDs, `wait` for them, bound their count.
  Unwaited jobs become zombies or orphans.
- `exec` to replace the shell when the script is a launcher wrapper.

---

## 18. Environment Discipline

The environment is untrusted input:

- `PATH`: set it explicitly at the top of security-sensitive scripts
  (`PATH=/usr/bin:/bin`), or validate before use.
- Never trust `CDPATH` (it changes `cd` semantics — unset it),
  `IFS`, `LD_PRELOAD`, or `BASH_ENV` across a trust boundary.
- `env -i` for a clean environment when spawning untrusted children.
- Secrets: prefer file descriptors or process substitution over
  environment variables when the secret's lifetime allows it; env vars
  leak via `/proc`.

---

## 19. No Eval, No Pipe-to-Shell

Banned, no exceptions in Tiger Style:

- `eval "$something"` — if the value must become code, restructure.
- `source`/`\.` on a computed or remote path.
- `curl ... | bash` / `wget -O- ... | sh` — download, verify
  checksum/signature against a pinned value, then execute.

```bash
curl -fsSL -o "$tmp/pkg.tar.gz" "$URL" || die "download failed"
echo "$EXPECTED_SHA256  $tmp/pkg.tar.gz" | sha256sum -c - || die "checksum mismatch"
tar -xzf "$tmp/pkg.tar.gz" -C "$dest"
```

`-f` (fail on HTTP error), `-sS` (silent but show errors), `-L` (follow
redirects deliberately, not blindly), `--proto =https` to pin the
protocol.

---

## 20. Globbing and Filename Handling

- Never parse `ls`. Use globs or `find -print0 | while IFS= read -r -d ''`.
- `nullglob`: non-matching globs expand to nothing (good for loops).
  `failglob`: non-matching globs error (good for required inputs).
  The default (literal passthrough) is the dangerous one — choose
  deliberately and document it.
- `dotglob` if hidden files are in scope; document the choice.
- Filenames can contain newlines, spaces, leading dashes, and glob
  characters. `"${arr[@]}"`, `-print0`, and `--` are the defenses.
- `case` patterns for classification; quote the subject, not the pattern.

---

## 21. Arithmetic

`$(( ... ))` for arithmetic. No `expr`, no `let` (obscure status codes).

```bash
retries=0
(( retries += 1 ))        # status 0 unless result is 0 — beware set -e!
retries=$(( retries + 1 )) # assignment form: always status 0
```

The `(( ... ))` command returns 1 when the result is 0 — under `set -e`
this aborts. Prefer the assignment form `$(( ))` for increments. For
anything beyond integers, call out to a real tool (`awk`, `python3`) —
bash floats do not exist.

---

## 22. Networking

- `curl` flags: `-fsSL --proto =https --max-time 30 --retry 3`.
- Never trust a redirect chain blindly; `--max-redirs` bounded.
- TLS verification is never disabled (`-k`/`--insecure` is banned).
- Secrets in headers: `--header "Authorization: Bearer $TOKEN"` is fine;
  never put secrets in URLs (they land in logs).
- `nc`/`/dev/tcp` for port checks only, never for protocol work you
  cannot bound.

---

## 23. Portability

If the script claims portability, prove it:

- `shellcheck -s sh` for POSIX scripts; test on `dash`.
- Avoid GNU-only flags (`--long-options`, `grep -P`, `sed -z`) or gate
  behind `uname`.
- macOS ships Bash 3.2 as `/bin/bash` — the `#!/usr/bin/env bash` +
  version guard (§2) handles this honestly.
- Line endings: scripts are LF. A stray CRLF breaks the shebang line.
- Document supported platforms in the header; CI on each claimed
  platform.

---

## 24. Security

Shell scripts are high-value targets: they run with user authority and
are easy to get subtly wrong.

- Threat-model every script that touches the network, runs as root, or
  handles untrusted filenames.
- Privilege boundaries: drop privileges early (`setpriv`, `su nobody`);
  never run a network-facing parser as root.
- No secrets in `ps` output: pass secrets via files, fds, or env — never
  as argv.
- `set -o noclobber` to avoid overwriting files with `>`; use `>|` only
  deliberately.
- Audit `sudo` usage: exact command allowlists, never `sudo script.sh`
  with writable script files.
- Log security-relevant events (auth failures, checksum mismatches,
  unexpected input shapes) to stderr/syslog.

---

## 25. Naming

- `UPPER_SNAKE_CASE` for constants and environment-derived config.
- `lower_snake_case` for functions and locals.
- Prefix private helpers consistently (`_helper` or `<tool>_helper`).
- Units in names: `TIMEOUT_SECS`, `MAX_RETRIES`, `CHUNK_SIZE_BYTES`.
- Script files: `lower-kebab-case.sh`; libraries: `lower-kebab-case.lib.sh`
  or `.bash`.

---

## 26. Formatting

One formatter: `shfmt -i 4 -ci -bn` (4-space indent, case-indent,
binary ops on new lines consistently). Run `shfmt -l -d` in CI and fail
on diffs. Never hand-format against the formatter — fix the code, not
the layout.

`shellcheck` MUST report zero warnings. A `# shellcheck disable=SCxxxx`
needs a comment explaining why the warning is a false positive, with a
link or a test proving it. "Noisy" is not a reason.

---

## 27. Comments and Documentation

Comments explain **why**, not what. The header is the contract:

```bash
#!/usr/bin/env bash
# backup-db.sh — nightly PostgreSQL backup with rotation.
#
# Usage: backup-db.sh [--keep N]
# Exit codes: 0 ok, 1 error, 2 usage.
# Side effects: writes to $BACKUP_DIR, rotates old backups.
# Requires: pg_dump, gzip, bash 4.4+.
# Platforms: Linux.
```

Function contracts (§11), non-obvious bounds (§6), and every `|| true`
(§9) get comments. Keep a `CHANGELOG` or git history readable — the
script's evolution is documentation.

---

## 28. Dependencies

- Check required tools at startup with `command -v`:

```bash
for tool in curl jq tar; do
    command -v "$tool" >/dev/null || die "required tool missing: $tool"
done
```

- Pin versions where behavior matters (`jq 1.7+` for `$ARGS`); document
  minimums in the header.
- Prefer tools already on the target system over new dependencies. Each
  dependency is a supply-chain entry — `curl | bash` installers for
  dependencies inherit §19's ban.

---

## 29. Testing

- **bats** (Bash Automated Testing System) for behavior tests. Test the
  failure paths, not just the happy path — half the tests SHOULD be
  adversarial: missing tools, unwritable dirs, timeouts, malformed input,
  empty input, filenames with spaces/newlines/dashes.
- `shellcheck` clean is a CI gate, not a substitute for tests.
- Test with `set -u` semantics in mind: run the test suite under the
  same strict mode the script uses.
- Hermetic tests: use `mktemp -d` per test, `trap` cleanup, never touch
  the real `$HOME` or system paths.
- Example adversarial cases: `TARGET_DIR=""`, `TARGET_DIR=/`,
  filenames with newlines, `curl` returning 404, `timeout` firing.

---

## 30. Debugging

- `bash -x` / `set -x`: trace execution. `PS4='+ ${BASH_SOURCE}:${LINENO}: '`
  makes traces locatable.
- `bashdb` for interactive debugging (DAP via diver's `lua/dap/bash`).
- `bash -n`: syntax check without executing — the `luac -p` of shell.
- Reproduce failures in a container matching the target platform before
  declaring them fixed.

---

## 31. Review Checklist

- [ ] `#!/usr/bin/env bash`, version guard, strict-mode header.
- [ ] Every expansion quoted; `read -r`; no `for x in $(...)`; no
      parsed `ls`.
- [ ] No `eval`, no `source` on computed paths, no `curl | bash`;
      downloads verified by checksum/signature.
- [ ] Temp files via `mktemp` with `trap` cleanup at creation.
- [ ] Exit codes documented; `die()` for fatal errors; data on stdout,
      diagnostics on stderr; no unexplained `|| true`.
- [ ] `pipefail`; `PIPESTATUS` checked where stages differ.
- [ ] All reads, loops, retries, timeouts, depths bounded by named
      constants.
- [ ] `rm -rf` targets validated (`${VAR:?}`, prefix check, `--`).
- [ ] Functions: `local` everywhere, ≤ ~70 lines, contract comments.
- [ ] `shellcheck` zero warnings (suppressions justified in comments);
      `shfmt` clean.
- [ ] bats tests exist; half adversarial; hermetic (temp dirs, traps).
- [ ] Required tools checked with `command -v`; versions documented.

---

## 32. Anti-Patterns

| Pattern | Why it fails | Do this instead |
| --- | --- | --- |
| `rm -rf $DIR` | Unquoted, unset `DIR` → `rm -rf /` | `rm -rf "${DIR:?...}"` + prefix check |
| `curl \| bash` | Executes unverified remote code | Download, verify checksum, then run |
| `for f in $(ls)` | Splits on whitespace | `for f in *` or `find -print0` loop |
| `eval "$cmd"` | Code injection | Restructure; arrays for argv |
| `cd $dir` unchecked | Wrong-dir operations | `cd "$dir" \|\| die` |
| `cat file \| grep x` | Useless cat, hides status | `grep x file`, redirection |
| `if [ $x == "y" ]` | Unquoted, `==` in `[` | `if [[ "$x" == "y" ]]` |
| `export SECRET=...` | Visible in `/proc` | File descriptor or file |
| `chmod 777` | World-writable | Least privilege (`640`, `750`) |
| `sleep` as sync | Races, flakes | Poll with deadline, `wait`, `flock` |

---

## 33. Reference Script

```bash
#!/usr/bin/env bash
# fetch-and-verify.sh — download a release artifact and verify its checksum.
#
# Usage: fetch-and-verify.sh URL EXPECTED_SHA256 DEST_DIR
# Exit codes: 0 ok, 1 error, 2 usage.
# Side effects: writes the artifact into DEST_DIR.
# Requires: curl, sha256sum, bash 4.4+.
# Platforms: Linux.

set -euo pipefail

readonly TIMEOUT_SECS=60
readonly MAX_BYTES=536870912  # 512 MiB

die() {
    printf 'error: %s\n' "$*" >&2
    exit 1
}

# Contract: $1 = URL (https), $2 = sha256 hex, $3 = dest dir (created).
# stdout: path of the verified file. Exit 0/1/2. Side effect: download.
fetch_and_verify() {
    local url="$1" expected="$2" dest_dir="$3"
    (( $# == 3 )) || die "usage: fetch-and-verify.sh URL SHA256 DEST_DIR"
    [[ "$url" == https://* ]] || die "URL must be https: $url"
    [[ "$expected" =~ ^[0-9a-f]{64}$ ]] || die "bad sha256: $expected"
    mkdir -p "$dest_dir" || die "cannot create: $dest_dir"

    local tmp
    tmp=$(mktemp "${TMPDIR:-/tmp}/fetch.XXXXXX") || die "mktemp failed"
    trap 'rm -f "$tmp"' EXIT

    curl -fsSL --proto =https --max-time "$TIMEOUT_SECS" \
        -o "$tmp" "$url" || die "download failed: $url"
    (( $(stat -c%s "$tmp") <= MAX_BYTES )) || die "artifact exceeds size bound"

    printf '%s  %s\n' "$expected" "$tmp" | sha256sum -c - >/dev/null \
        || die "checksum mismatch for $url"

    local dest="$dest_dir/$(basename "$url")"
    mv -- "$tmp" "$dest" || die "cannot move to $dest"
    trap - EXIT
    printf '%s\n' "$dest"
}

main() {
    if (( BASH_VERSINFO[0] < 4 )) || (( BASH_VERSINFO[0] == 4 && BASH_VERSINFO[1] < 4 )); then
        printf 'error: bash 4.4+ required\n' >&2
        exit 2
    fi
    for tool in curl sha256sum stat; do
        command -v "$tool" >/dev/null || die "required tool missing: $tool"
    done
    (( $# == 3 )) || die "usage: fetch-and-verify.sh URL SHA256 DEST_DIR"
    fetch_and_verify "$1" "$2" "$3"
}

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
```
