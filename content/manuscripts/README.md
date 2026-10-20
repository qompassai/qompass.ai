# Manuscript study pages

Each `<slug>.toml` in this directory describes one study page for
`/research/<slug>`, a sub-page of the Research index's manuscript
tracker. `build.rs` compiles these files into
`src/manuscript_pages_data.rs` at build time, in the same style as
`content/projects/*.toml` and `content/tracker/*.toml`. The slug must
exist in the program rollup snapshot (`src/manuscript_data.rs`,
generated from the Odyssey rollup): a content file naming a manuscript
the program does not track fails the build.

## The rollout rule

A page **exists** only when its file sets `ready = true`:

- the router registers `/research/<slug>` only for ready pages,
- the /research tracker renders its study-page link only for
  ready pages,
- `scripts/prerender.sh` prerenders exactly the ready set.

Non-ready files are still parsed at build time, so a malformed TOML
fails the build instead of failing silently at rollout time. The file
stem must equal the declared `slug` (build error otherwise) — the stem
is what `prerender.sh` turns into the route.

## Schema

Identity (required): `slug`. The page's title, journal, stage and
updated date are NOT repeated here — they render from the rollup
snapshot, so the tracker and the study page can never disagree.

| Field | Type | Renders as |
|---|---|---|
| `why` | string | WHY THIS STUDY section (blank-line-separated paragraphs) |
| `graphic` | string id | animated SVG slot under the header; ids are implemented in `src/pages/manuscript.rs` (`ManuscriptGraphic`) — an unknown id renders nothing |
| `sources_note` | string | the line above the source list stating how the sources were verified |
| `references` | list of `{title, url, note}` | SOURCES list; `note` carries journal and year |

Content rule (same as the project study pages): every number and
claim must come from a verified source; gaps are stated as gaps on
the page, never filled in. A reference is listed only after it has
been verified against its registry (for journal articles: the DOI
resolves in Crossref and the registered title matches). The draft
link is not stored here — the page template derives the Odyssey
repository URL from the slug, so each page carries exactly one
outbound link and it cannot drift.
