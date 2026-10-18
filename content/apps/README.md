# App guide pages

Each `<slug>.toml` in this directory describes one app guide page for
`/solutions/<slug>`: an overview, a how-it-works flow (the guide's
`how` steps, rendered as an animated step diagram), the app's real
configuration options, and its real controls. `build.rs` compiles
these files into `src/app_data.rs` at build time, in the same style
as `content/projects/*.toml`.

## The rollout rule

A guide **exists** only when its file sets `ready = true`:

- the `/solutions/<slug>` dispatch renders the guide when no study
  page claims the slug (a study keeps precedence; the guide's
  configuration and controls sections are then appended to the study),
- `scripts/prerender.sh` prerenders exactly the ready set.

Non-ready files are still parsed at build time, so a malformed TOML
fails the build instead of failing silently at rollout time. The file
stem must equal the declared `slug` (build error otherwise).

## Schema

Identity (required): `slug`, `name`, `kind_line`.
Optional identity: `repo_url`, `mark` (image path; defaults to the
solution's emblem).

| Field | Type | Renders as |
|---|---|---|
| `summary` | string | OVERVIEW section (blank-line-separated paragraphs) |
| `how` | list of strings | HOW IT WORKS flow diagram, one node per step |
| `config` | list of `{name, default, description}` | CONFIGURATION table |
| `controls` | list of `{action, control}` | CONTROLS table |
| `gaps` | string | NOT COVERED section — what the sources did not establish |
| `visuals` | list of `{kind, id, src, caption, alt, poster, latex, define}` | CONCEPTS section, in authored order. `kind` is `diagram` (an `id` in the `src/pages/diagrams` registry), `equation` (`latex` + `define`, the prose defining every quantity), `image`, or `video` (a real artifact at `src`; the caption states its provenance). Every visual carries a `caption`; the build rejects a visual missing what its kind requires |
| `references` | list of `{title, url, note}` | REFERENCES section — the primary sources (specs, RFCs, standards, defining papers) a reader can follow to validate the page. URLs must be https; the build rejects anything else |

Key-ordering rule (TOML semantics): all top-level scalar keys —
including `gaps` — must appear **before** the first `[[config]]` or
`[[controls]]` header.

Content rule: every option, default, command, and step must come
from the project's own documentation or source, verified at writing
time. Anything unsourced is omitted and named in `gaps` — never
filled in.

The same rule governs `visuals` and `references`: a diagram must
encode the app's real architecture, data flow, protocol, or
algorithm — never decoration — and every reference must be a live,
primary source checked at writing time. Where no external source
establishes a claim, the page says the behaviour is the project's
own design instead of borrowing authority.
