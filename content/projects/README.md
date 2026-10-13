# Project study pages

Each `<slug>.toml` in this directory describes one project study page
(IMRaD: abstract, methods, results, discussion, conclusion) for
`/projects/<slug>`. `build.rs` compiles these files into
`src/project_data.rs` at build time, in the same style as
`content/cv.toml` and `content/tracker/*.toml`.

## The rollout rule

A page **exists** only when its file sets `ready = true`:

- the router registers `/projects/<slug>` only for ready pages,
- the `/projects` index renders its "Read the study →" link only for
  ready pages,
- `scripts/prerender.sh` prerenders exactly the ready set.

Non-ready files are still parsed at build time, so a malformed TOML
fails the build instead of failing silently at rollout time. The file
stem must equal the declared `slug` (build error otherwise) — the stem
is what `prerender.sh` turns into the route. Shipping a new page is:
write the TOML from verified sources, set `ready = true`, rebuild,
prerender, deploy.

## Schema

Identity (required): `slug`, `name`, `kind_line`.
Optional identity: `repo_url`, `mark` (image path), `status`.

| Field | Type | Renders as |
|---|---|---|
| `abstract` | string | ABSTRACT section (blank-line-separated paragraphs) |
| `methods` | list of `{heading, body, equations}` | METHODS subsections; `equations` are LaTeX strings typeset by KaTeX (vendored, `public/vendor/katex/`) as display math; without JS the raw LaTeX shows in a styled mono span |
| `results.intro` | string | lead-in under RESULTS |
| `results.benchmarks` | list of `{title, columns, rows, note}` | benchmark tables (`rows` = list of string lists) |
| `results.figures_note` | string | note under the tables |
| `discussion` | list of strings | DISCUSSION paragraphs |
| `conclusion` | string | CONCLUSION section |
| `graphic` | string id | animated SVG slot under the header; ids are implemented in `src/pages/project.rs` (`ProjectGraphic`) — an unknown id renders nothing |
| `interactive` | string id | interactive mount point at the foot of the page; ids are implemented in the `INTERACTIVES` registry in `public/js/project.js` — an unknown id leaves the honest fallback text |

Key-ordering rule (TOML semantics): all top-level scalar keys —
including `discussion` and `conclusion` — must appear **before** the
first `[[methods]]` or `[results]` header. A bare key written after an
array-of-tables header silently belongs to that table's last element
instead of the page.

Sections with no content do not render. Content rule: every number,
equation, and claim must come from a verified source pack; gaps are
stated as gaps on the page, never filled in.
