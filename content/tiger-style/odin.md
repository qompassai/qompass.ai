# Tiger Style Odin

## Purpose

Apply the Tiger Style standard to Odin code. Priority order, always:
**Safety > Performance > Developer Experience.** The full guide lives in
[references/TIGER_STYLE_ODIN.md](references/TIGER_STYLE_ODIN.md); this file
is the operational core. When the two disagree, the full guide wins.

## Workflow

1. Pin the exact compiler release and inspect its target list and help
   output before claiming any deployment target. A CPU architecture is
   not a deployment target: record OS, ABI, object format, libc, minimum
   SDK/runtime, native libraries, and packaging separately.
2. Lay the package out top-to-bottom: purpose, public contract (types,
   entry procedures, allocator expectations), then private machinery.
3. Make allocators explicit. Every procedure that allocates answers:
   which allocator, who frees, and what happens on allocator failure.
4. Validate external input with ordinary control flow returning a typed
   error; assert facts a correct implementation already established.
5. Review against the checklist below before calling the code done.

## Operating Rules

- **Contracts first.** For every important procedure, a reviewer must be
  able to identify permitted inputs, maximum work, valid state, resource
  owner, mutation point, failure behavior, and evidence. Avoid implicit
  conventions that require reading the whole program to establish one
  invariant.
- **Odin gives control, not proofs.** There are no Rust-style
  compile-time ownership proofs. Pointers, aliases, foreign data,
  manual lifetime management, and concurrent access require deliberate
  review. Never call code "memory-safe" merely because it compiles or
  keeps bounds checks enabled.
- **Bound everything.** No unbounded loops, retries, queues, or growth.
  Name limits with units (`input_bytes_max`, `retry_count`,
  `queue_capacity`, `deadline`). Prefer simple data and shallow control
  flow; a fixed buffer, explicit loop, or dedicated arena can make an
  invariant clearer, but is not automatically superior — choose by the
  workload's limits, failure policy, and resource lifetime.
- **Allocator discipline.** Default to explicit context allocators;
  never silently switch allocators mid-operation. Temporary arenas have
  documented lifetimes. Allocation failure is an expected error, not an
  assertion — handle it on every path that can allocate.
- **Concurrency.** Document the threading model, shared-state protocol,
  and lock ordering. Never hold a lock across a blocking call without a
  stated reason and bound. Own thread/task handles; cancellation follows
  a documented policy; stale completions cannot publish.
- **Foreign data is untrusted.** C pointers, loaded files, network
  buffers, and plugin inputs are validated before use: lengths, tags,
  versions, and provenance checked with explicit code, not assumed from
  the producer.
- **Deployment honesty.** Linux x86-64 is the baseline native lane;
  every other target needs its own evidence (ARM64 libraries and board
  tests, MSVC/SDK compatibility for Windows, signing for Apple). Do not
  infer Android from Linux ARM64 or iOS from macOS ARM64. Prototype and
  validate the entire pipeline before committing a product to a
  conditional target. A phone can consume an Odin service through the
  same versioned API as a desktop client.
- **Exceptions record the rule, need, affected targets, owner,
  compensating check, and removal condition.** No blanket "performance"
  waiver for bounds checks, allocator errors, authentication, or
  release testing.

## Output Contract

Odin you produce must: compile on the pinned compiler with
diagnostics at full strength, carry doc comments on public procedures
stating contract, allocator expectations, and bounds, return typed
errors on expected failures, assert internal invariants, bound all
work/queues/retries/memory, make every allocation's owner and failure
path explicit, record the deployment target evidence, and pass the
review checklist.

## Review Checklist

- [ ] External input validated before allocation, indexing, and mutation.
- [ ] Every allocation names its allocator, owner, free path, and failure behavior.
- [ ] Pointers, aliases, lifetimes, and foreign data reviewed as untrusted until proven.
- [ ] Work, queues, memory, retries, and time have explicit budgets.
- [ ] Errors preserve failure and cause; assertions enforce established invariants.
- [ ] Concurrency model, lock ordering, and cancellation policy documented.
- [ ] No code called "memory-safe" without the review to back it.
- [ ] Deployment target recorded as OS+CPU+ABI+libc+SDK+packaging with evidence.
- [ ] Logs, argv, and subprocess environments expose no secrets.
- [ ] Toolchain and dependency changes received execution-trust review.
- [ ] Performance claims include measurements and their environment.
