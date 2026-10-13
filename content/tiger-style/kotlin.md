# Tiger Style Kotlin

## Purpose

Apply Matt's Tiger Style standard to Kotlin code. Priority order, always:
**Safety > Performance > Developer Experience.** The full guide lives in
[references/TIGER_STYLE_KOTLIN.md](references/TIGER_STYLE_KOTLIN.md); this file
is the operational core. When the two disagree, the full guide wins.

## Workflow

1. Define the target as OS + CPU + runtime/ABI + SDK + dependencies +
   packaging — never just an architecture name. Check the Kotlin and
   Compose support tables for the exact release; neither table proves
   support for every dependency.
2. Lay the module out top-to-bottom: purpose, public contract (types,
   entry points, threading and lifecycle expectations), then private
   machinery.
3. Validate external input before allocating expensive resources or
   publishing state. Use assertions for internal facts, normal error
   paths (sealed result types) for expected external failures.
4. Encode resource budgets as named configuration with units.
5. Review against the checklist below before calling the code done.

## Operating Rules

- **Contracts first.** Each substantial operation identifies its accepted
  input, maximum work, owner, commit point, failure mode, cancellation
  behavior, and evidence.
- **A type-safe language is not a secure application.** Null-safety does
  not cover authorization, deadlines, queue admission, or secret
  handling — those remain application responsibilities, reviewed
  explicitly.
- **Bound everything.** No unbounded loops, retries, queues, collection
  growth, or coroutine fan-out. Name limits with units
  (`input_bytes_max`, `retry_count`, `queue_capacity`, `deadline`).
  Reject invalid input before expensive allocation or state publication.
- **Idioms welcome when their cost and lifetime are clear.** Sealed
  hierarchies, extension functions, collection operations, and
  coroutines are fine — but do not mechanically translate Rust ownership
  syntax or Lua naming into Kotlin. Write Kotlin as Kotlin.
- **Coroutines need structured concurrency.** Every scope has an owner
  and a cancellation policy; cancellation is cooperative, prompt, and
  cannot publish partial effects. No `GlobalScope` for work the app
  owns. Document dispatcher choices and what blocks where.
- **Lifecycle discipline (Android).** No work outlives its owner:
  activities, fragments, and view-models release subscriptions, jobs,
  and listeners. Configuration changes and process death are expected
  events with tested behavior, not edge cases.
- **Multiplatform honesty.** Android, iOS, JVM, and JS are separate
  lanes with separate evidence. Kotlin/Wasm and Compose web are beta —
  say so when used. `linuxArm64` is a Tier 2 target but Linux ARM64 is
  not a supported host for producing final native binaries: build on a
  supported host, run on the board. Apple final binaries require macOS.
  Real-device tests for mobile lanes; simulators alone prove little.
- **Exceptions live next to the affected code:** the rule, reason,
  affected targets, owner, remaining bound, and test or removal
  condition. A blanket exception for a whole application is not an
  engineering argument.

## Output Contract

Kotlin you produce for Matt must: compile on the pinned toolchain with
warnings treated as errors, carry KDoc on public items stating contract
and bounds, use sealed result types (not exceptions) for expected
failures, assert internal invariants, bound all work/queues/retries/
coroutines, keep lifecycle and cancellation explicit, record the target
matrix evidence, and pass the review checklist.

## Review Checklist

- [ ] External input validated before expensive allocation or state publication.
- [ ] Null-safety not mistaken for authorization, deadline, or secret-handling review.
- [ ] Work, queues, collection growth, retries, coroutines, and time have explicit budgets.
- [ ] Coroutine scopes owned; cancellation cooperative, prompt, and side-effect safe.
- [ ] Android lifecycle: no leaked subscriptions, jobs, or listeners; config change and process death tested.
- [ ] Multiplatform target recorded as OS+CPU+runtime/ABI+SDK+packaging with evidence; beta lanes labeled.
- [ ] Errors preserve failure and cause; assertions enforce established invariants.
- [ ] Secrets never in logs, bundles, or checked-in config.
- [ ] Dependency and toolchain changes received execution-trust review.
- [ ] Performance claims include measurements and their environment.
