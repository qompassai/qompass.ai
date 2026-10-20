//! Manuscript study pages under `/research/:slug`, one per Odyssey
//! manuscript flagged `ready` in `content/manuscripts/<slug>.toml`.
//!
//! The pattern follows the Alphabet paper's study pages: the figures
//! are the study's own measurements, drawn as build-time SVG whose
//! bars animate on load (SMIL, no client JavaScript; the static
//! final frame is the figure). Each page carries the why behind the
//! study, its anchor sources — every one verified against the
//! Crossref registry before it was listed — and exactly one link
//! out: the draft and its tracking record in the Odyssey repository.
//!
//! Identity (title, journal, stage, updated) comes from the program
//! rollup snapshot in `crate::manuscript_data`; the page body comes
//! from `crate::manuscript_pages_data`, compiled in by `build.rs`.
//! A slug with no ready page renders the standard off-chart fallback.

use crate::manuscript_data::{Manuscript, MANUSCRIPTS};
use crate::manuscript_pages_data::{manuscript_page, ManuscriptPage};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// The rollup entry for a slug, if the manuscript exists.
fn rollup_entry(slug: &str) -> Option<&'static Manuscript> {
    MANUSCRIPTS.iter().find(|m| m.slug == slug)
}

/// The stage wording the /research tracker uses, kept identical here.
fn stage_label(stage: &str) -> &'static str {
    match stage {
        "planned" => "STAGE — PLANNED",
        "drafting" => "STAGE — DRAFTING",
        "internal-review" => "STAGE — INTERNAL REVIEW",
        "preflight" => "STAGE — PREFLIGHT",
        "packaged" => "STAGE — PACKAGED",
        "submitted" => "STAGE — SUBMITTED",
        "under-review" => "STAGE — UNDER REVIEW",
        "revision" => "STAGE — REVISION",
        "accepted" => "STAGE — ACCEPTED",
        "published" => "STAGE — PUBLISHED",
        _ => "STAGE — WITHDRAWN",
    }
}

/// The one outbound link each study page carries: the manuscript's
/// folder in the Odyssey repository, with its draft, figures and
/// status record.
fn github_url(slug: &str) -> String {
    format!("https://github.com/qompassai/Odyssey/tree/main/manuscripts/{slug}")
}

/// Route entry for `/research/:slug`: a ready study page renders its
/// study; anything else is off chart, as everywhere on the site.
#[component]
pub fn ManuscriptDispatch() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|map| map.get("slug")).unwrap_or_default();
    match manuscript_page(&slug) {
        Some(page) => view! { <ManuscriptStudy page=page /> }.into_any(),
        None => view! {
            <section class="page">
                <h1>"Off chart"</h1>
                <p>"That bearing does not exist on this chart."</p>
            </section>
        }
        .into_any(),
    }
}

/// One manuscript's study page: its why, its measured figure where
/// the study has measurements, its verified anchor sources, and the
/// link to the draft.
#[component]
fn ManuscriptStudy(page: &'static ManuscriptPage) -> impl IntoView {
    let entry = rollup_entry(page.slug);
    let title = entry.map_or(page.slug, |m| m.title);
    let kind = entry
        .map_or_else(String::new, |m| format!("{} · {} · UPDATED {}", stage_label(m.stage), m.journal_name, m.updated));
    let why_paragraphs: Vec<&'static str> = page.why.split("\n\n").filter(|p| !p.trim().is_empty()).collect();
    view! {
        <Title text=format!("{} - Qompass AI", title) />
        <section class="page project-page">
            <article class="project-article">
                <header class="study-head">
                    <p class="page-bearing">{kind}</p>
                    <h1>{title}</h1>
                </header>
                {(!page.graphic.is_empty())
                    .then(|| view! { <ManuscriptGraphic id=page.graphic /> })}
                <section class="study-section">
                    <p class="band-label">"WHY THIS STUDY"</p>
                    {why_paragraphs
                        .into_iter()
                        .map(|p| view! { <p>{p}</p> })
                        .collect_view()}
                </section>
                {(!page.references.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"SOURCES"</p>
                                <p class="record-meta">{page.sources_note}</p>
                                <ul class="record-list">
                                    {page
                                        .references
                                        .iter()
                                        .map(|r| {
                                            view! {
                                                <li class="record-item">
                                                    <span class="record-title">
                                                        <a href={r.url}>{r.title}</a>
                                                    </span>
                                                    {(!r.note.is_empty())
                                                        .then(|| {
                                                            view! {
                                                                <span class="record-doi">{r.note}</span>
                                                            }
                                                        })}
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </section>
                        }
                    })}
                <section class="study-section">
                    <p class="band-label">"THE DRAFT"</p>
                    <p>
                        "The full draft, its figures and its tracking record live in the "
                        "Odyssey repository, where the manuscript's status.json is the source "
                        "of truth for its stage."
                    </p>
                    <p>
                        <a href={github_url(page.slug)}>
                            "Draft and code on GitHub — Odyssey/manuscripts/" {page.slug}
                        </a>
                    </p>
                </section>
            </article>
            <nav class="fn-flow-nav" aria-label="Site flow">
                <a href="/research">"← Research — the manuscript program"</a>
            </nav>
        </section>
    }
}

/// The measured figures, in the Alphabet pattern: the study's own
/// numbers as bars that draw themselves on load. SMIL only, no
/// client JavaScript; with animation unavailable the bars render at
/// their final widths, which is the static figure.
#[component]
fn ManuscriptGraphic(id: &'static str) -> impl IntoView {
    match id {
        "kintsugi-battery" => view! {
            <figure class="study-graphic">
                <svg
                    class="gfx"
                    viewBox="0 0 760 218"
                    role="img"
                    aria-label="Bar chart: clean digests out of 20 on the qualifying set. The base Qwen2.5-Coder-3B model scored 0 of 20; the same model with a LoRA adapter served scored 17 of 20, with incident recall 1.000."
                >
                    <text class="gfx-head" x="16" y="30">
                        "CLEAN DIGESTS — QUALIFYING SET, OF 20"
                    </text>
                    <text class="gfx-text" x="16" y="82">
                        "Qwen2.5-Coder-3B — base"
                    </text>
                    <rect class="gfx-bar" x="250" y="62" width="0" height="26" />
                    <text class="gfx-head" x="262" y="82">
                        "0 / 20"
                    </text>
                    <text class="gfx-text" x="16" y="138">
                        "Qwen2.5-Coder-3B — LoRA-served"
                    </text>
                    <rect class="gfx-bar gfx-bar-top" x="250" y="118" width="408" height="26">
                        <animate
                            attributeName="width"
                            from="0"
                            to="408"
                            dur="1.4s"
                            fill="freeze"
                        />
                    </rect>
                    <text class="gfx-head" x="670" y="138">
                        "17 / 20"
                    </text>
                    <text class="gfx-text" x="16" y="182">
                        "Incident recall 1.000 · CPU-only · first answers, temperature zero"
                    </text>
                    <text class="gfx-text" x="16" y="204">
                        "The clinical battery itself is pre-registered and pending."
                    </text>
                </svg>
                <figcaption class="study-graphic-caption">
                    "The qualifying measurements behind the roster rule: on the program's "
                    "estate-log digest tasks, adaptation turned a non-producer into the "
                    "digest seat. The clinical battery (MOSAIC) runs the same rule on "
                    "clinical fixtures."
                </figcaption>
            </figure>
        }
        .into_any(),
        "moneyball-roster" => view! {
            <figure class="study-graphic">
                <svg
                    class="gfx"
                    viewBox="0 0 760 232"
                    role="img"
                    aria-label="Bar chart: on-disk size in gigabytes. The three-model roster totals about 4.2 gigabytes; the single 30-billion-parameter model is 18.6 gigabytes. Granite 4.0 1B produced 14.28 production points per gigabyte, the best ratio measured."
                >
                    <text class="gfx-head" x="16" y="30">
                        "ON-DISK SIZE — GIGABYTES"
                    </text>
                    <text class="gfx-text" x="16" y="82">
                        "Three-model roster"
                    </text>
                    <rect class="gfx-bar gfx-bar-top" x="250" y="62" width="108" height="26">
                        <animate
                            attributeName="width"
                            from="0"
                            to="108"
                            dur="1.4s"
                            fill="freeze"
                        />
                    </rect>
                    <text class="gfx-head" x="370" y="82">
                        "≈ 4.2 GB"
                    </text>
                    <text class="gfx-text" x="16" y="138">
                        "Single 30B model"
                    </text>
                    <rect class="gfx-bar" x="250" y="118" width="480" height="26">
                        <animate
                            attributeName="width"
                            from="0"
                            to="480"
                            dur="1.4s"
                            fill="freeze"
                        />
                    </rect>
                    <text class="gfx-head" x="670" y="138">
                        "18.6 GB"
                    </text>
                    <text class="gfx-text" x="16" y="182">
                        "Granite 4.0 1B: 14.28 production points per GB — best ratio measured"
                    </text>
                    <text class="gfx-text" x="16" y="204">
                        "Kintsugi tiny-tier measurements · CPU-only · one workstation"
                    </text>
                </svg>
                <figcaption class="study-graphic-caption">
                    "The production-per-cost evidence this analysis prices: a roster of "
                    "small models, each seated by measured production per unit of cost, "
                    "occupies a quarter of the disk of the single large model it is "
                    "compared against."
                </figcaption>
            </figure>
        }
        .into_any(),
        "viewpoint-boxscore" => view! {
            <figure class="study-graphic">
                <svg
                    class="gfx"
                    viewBox="0 0 760 288"
                    role="img"
                    aria-label="Bar chart: production, measured successes out of 24 across digest, triage and tool tiers. Granite 4.0 1B scored 14 from a 0.98 gigabyte file; Bonsai-8B scored 15 from 1.16 gigabytes; the 30-billion-parameter incumbent scored 20 from 18.56 gigabytes; BitNet b1.58 2B scored 0 as shipped."
                >
                    <text class="gfx-head" x="16" y="30">
                        "PRODUCTION — MEASURED SUCCESSES, OF 24"
                    </text>
                    <text class="gfx-text" x="16" y="76">
                        "Granite 4.0 1B (0.98 GB)"
                    </text>
                    <rect class="gfx-bar gfx-bar-top" x="250" y="56" width="280" height="24">
                        <animate
                            attributeName="width"
                            from="0"
                            to="280"
                            dur="1.4s"
                            fill="freeze"
                        />
                    </rect>
                    <text class="gfx-head" x="542" y="76">
                        "14"
                    </text>
                    <text class="gfx-text" x="16" y="124">
                        "Bonsai-8B, native 1-bit (1.16 GB)"
                    </text>
                    <rect class="gfx-bar" x="250" y="104" width="300" height="24">
                        <animate
                            attributeName="width"
                            from="0"
                            to="300"
                            dur="1.4s"
                            fill="freeze"
                        />
                    </rect>
                    <text class="gfx-head" x="562" y="124">
                        "15"
                    </text>
                    <text class="gfx-text" x="16" y="172">
                        "30B incumbent (18.56 GB)"
                    </text>
                    <rect class="gfx-bar" x="250" y="152" width="400" height="24">
                        <animate
                            attributeName="width"
                            from="0"
                            to="400"
                            dur="1.4s"
                            fill="freeze"
                        />
                    </rect>
                    <text class="gfx-head" x="662" y="172">
                        "20"
                    </text>
                    <text class="gfx-text" x="16" y="220">
                        "BitNet b1.58 2B (as shipped)"
                    </text>
                    <rect class="gfx-bar" x="250" y="200" width="0" height="24" />
                    <text class="gfx-head" x="262" y="220">
                        "0"
                    </text>
                    <text class="gfx-text" x="16" y="262">
                        "Digest, triage and tool tiers · first answers, temperature zero · CPU-only"
                    </text>
                </svg>
                <figcaption class="study-graphic-caption">
                    "The box scores behind the argument: reputation did not predict "
                    "production. The least regarded model in its family was the most "
                    "reliable producer per gigabyte; a famous 1-bit model scored zero "
                    "as shipped, through its own runtime."
                </figcaption>
            </figure>
        }
        .into_any(),
        _ => view! { <span></span> }.into_any(),
    }
}
