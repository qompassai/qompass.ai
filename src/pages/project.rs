//! Project study pages — `/solutions/<slug>` — and the generated page
//! of every other solution on the compass.
//!
//! A study page is an `IMRaD` write-up (abstract, methods, results,
//! discussion, conclusion) rendered from the compiled-in content model
//! in `crate::project_data`. The compiled-in page list holds only pages
//! whose content file is flagged ready, so a slug renders a study only
//! when its page is published; any other slug gets the site's standard
//! "Off chart" fallback. A section with no content behind it does not
//! render.
//!
//! Equations are authored as LaTeX in the content file and emitted as
//! `\[...\]` / inline `\(...\)` source spans in `.project-article`;
//! `/js/project.js` typesets them in the browser with the self-hosted
//! `KaTeX` under `/vendor/katex` (loaded only on these pages). With no
//! JavaScript the raw LaTeX remains visible in a styled mono span —
//! degraded, never broken.
use crate::app_data::{app_guide, AppGuide, GuideControl, GuideEntry, GuideVisual};
use crate::pages::compass_services::{solution_page, version_link, Solution, CARDINALS};
use crate::pages::telco_quizzes::{self, TelcoQuizzes};
use crate::project_data::{project_page, Benchmark};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// Split a content body into paragraphs on blank lines.
fn paragraphs(text: &str) -> Vec<&str> {
    text.split("\n\n").map(str::trim).filter(|p| !p.is_empty()).collect()
}

/// One benchmark table from the content model, in the design system.
#[component]
fn BenchTable(bench: &'static Benchmark) -> impl IntoView {
    view! {
        <figure class="bench">
            <figcaption class="bench-title">{bench.title}</figcaption>
            <div class="bench-scroll">
                <table class="bench-table">
                    {(!bench.columns.is_empty())
                        .then(|| {
                            view! {
                                <thead>
                                    <tr>
                                        {bench
                                            .columns
                                            .iter()
                                            .map(|c| view! { <th>{*c}</th> })
                                            .collect_view()}
                                    </tr>
                                </thead>
                            }
                        })}
                    <tbody>
                        {bench
                            .rows
                            .iter()
                            .map(|row| {
                                view! {
                                    <tr>
                                        {row
                                            .iter()
                                            .map(|cell| view! { <td>{*cell}</td> })
                                            .collect_view()}
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
            {(!bench.note.is_empty())
                .then(|| view! { <figcaption class="bench-note">{bench.note}</figcaption> })}
        </figure>
    }
}

/// The four platform download boxes for one project, in the house
/// order: Linux x86_64, Linux aarch64, Windows, Android.
///
/// The boxes render from `crate::download_data` — the single source,
/// `content/downloads.toml`. A platform with a recorded URL is a real
/// download anchor; a platform without one is a disabled span with
/// `aria-disabled` and a "Coming soon" annotation, never a link. The
/// grid cards on /solutions and the study pages render this same
/// component, so the two surfaces cannot disagree.
#[component]
pub fn DownloadBoxes(slug: &'static str) -> impl IntoView {
    let downloads = crate::download_data::downloads_for(slug);
    let platforms = [
        ("Linux x86_64", downloads.linux_x86_64),
        ("Linux aarch64", downloads.linux_aarch64),
        ("Windows", downloads.windows),
        ("Android", downloads.android),
    ];
    view! {
        <div class="download-boxes">
            {platforms
                .into_iter()
                .map(|(label, url)| {
                    url.map_or_else(
                        || {
                            view! {
                                <span class="download-box is-soon" aria-disabled="true">
                                    <span class="download-platform">{label}</span>
                                    <span class="download-state">"Coming soon"</span>
                                </span>
                            }
                            .into_any()
                        },
                        |url| {
                            view! {
                                <a class="download-box" href=url download>
                                    <span class="download-platform">{label}</span>
                                    <span class="download-state">"Download"</span>
                                </a>
                            }
                            .into_any()
                        },
                    )
                })
                .collect_view()}
        </div>
    }
}

/// Animated explainer registry, keyed by the page's `graphic` id.
///
/// Graphics are inline SVG drawn in the design tokens; motion is
/// transform/opacity only and lives in the `prefers-reduced-motion:
/// no-preference` block of the stylesheet, so reduced-motion visitors
/// get the static diagram. An unknown id renders nothing — a page
/// ships a graphic only when one is registered here.
#[component]
fn ProjectGraphic(id: &'static str) -> impl IntoView {
    match id {
        "phlow-frame" => view! {
            <figure class="study-graphic">
                <svg
                    class="gfx"
                    viewBox="0 0 760 300"
                    role="img"
                    aria-label="Diagram: shared inputs feed an identical saliency snapshot in every agent; its SHA-256 hash is the shared frame; matching frames align and serialize on a shared top target."
                >
                    <rect class="gfx-panel" x="8" y="70" width="160" height="160" rx="6" />
                    <text class="gfx-head" x="88" y="94" text-anchor="middle">
                        "SHARED INPUTS"
                    </text>
                    <text class="gfx-text gfx-row" x="24" y="124">
                        "task naming"
                    </text>
                    <text class="gfx-text gfx-row" x="24" y="148">
                        "diagnostics"
                    </text>
                    <text class="gfx-text gfx-row" x="24" y="172">
                        "SCIP centrality"
                    </text>
                    <text class="gfx-text gfx-row" x="24" y="196">
                        "recency"
                    </text>
                    <text class="gfx-text gfx-row" x="24" y="220">
                        "editor state"
                    </text>
                    <line class="gfx-arrow" x1="176" y1="150" x2="216" y2="150" />
                    <circle class="gfx-pulse" cx="180" cy="150" r="3.5" />
                    <rect class="gfx-panel" x="224" y="52" width="276" height="196" rx="6" />
                    <text class="gfx-head" x="362" y="76" text-anchor="middle">
                        "SALIENCY SNAPSHOT"
                    </text>
                    <text class="gfx-text" x="240" y="106">
                        "a.rs#parse"
                    </text>
                    <rect
                        class="gfx-bar gfx-bar-top"
                        x="330"
                        y="96"
                        width="137"
                        height="10"
                        style="animation-delay:0.1s"
                    />
                    <text class="gfx-text" x="240" y="136">
                        "b.rs#check"
                    </text>
                    <rect
                        class="gfx-bar"
                        x="330"
                        y="126"
                        width="99"
                        height="10"
                        style="animation-delay:0.35s"
                    />
                    <text class="gfx-text" x="240" y="166">
                        "c.rs"
                    </text>
                    <rect
                        class="gfx-bar"
                        x="330"
                        y="156"
                        width="56"
                        height="10"
                        style="animation-delay:0.6s"
                    />
                    <text class="gfx-text" x="240" y="196">
                        "d.rs#run"
                    </text>
                    <rect
                        class="gfx-bar"
                        x="330"
                        y="186"
                        width="19"
                        height="10"
                        style="animation-delay:0.85s"
                    />
                    <text class="gfx-note" x="240" y="228">
                        "computed identically by every agent"
                    </text>
                    <line class="gfx-arrow" x1="508" y1="150" x2="548" y2="150" />
                    <circle
                        class="gfx-pulse"
                        cx="512"
                        cy="150"
                        r="3.5"
                        style="animation-delay:1.2s"
                    />
                    <rect class="gfx-panel" x="556" y="52" width="196" height="64" rx="6" />
                    <text class="gfx-head" x="654" y="80" text-anchor="middle">
                        "SHA-256"
                    </text>
                    <text class="gfx-note" x="654" y="100" text-anchor="middle">
                        "the hash is the frame"
                    </text>
                    <rect class="gfx-panel" x="556" y="132" width="196" height="52" rx="6" />
                    <text class="gfx-text" x="654" y="154" text-anchor="middle">
                        "focus a — top: a.rs#parse"
                    </text>
                    <text class="gfx-text" x="654" y="172" text-anchor="middle">
                        "focus b — top: a.rs#parse"
                    </text>
                    <g class="gfx-decide">
                        <rect class="gfx-badge" x="556" y="200" width="196" height="48" rx="6" />
                        <text class="gfx-badge-text" x="654" y="222" text-anchor="middle">
                            "SAME TOP → SERIALIZE"
                        </text>
                        <text class="gfx-note" x="654" y="240" text-anchor="middle">
                            "holder: smaller agent id"
                        </text>
                    </g>
                </svg>
                <figcaption class="study-graphic-caption">
                    "One coordination round. Alignment is defined only inside a single "
                    "frame: if the hashes differ, alignment is an error and every "
                    "consumer fails closed."
                </figcaption>
            </figure>
        }
        .into_any(),
        _ => ().into_any(),
    }
}

/// The how-it-works diagram for an app guide: the app's own sourced
/// steps as a vertical flow of numbered nodes joined by a line a pulse
/// travels down (motion in the stylesheet's no-preference block, so
/// reduced-motion visitors get the static diagram). One figure.
#[component]
fn FlowDiagram(steps: &'static [&'static str]) -> impl IntoView {
    view! {
        <figure class="flow-diagram">
            <ol class="flow-steps">
                {steps
                    .iter()
                    .enumerate()
                    .map(|(i, step)| {
                        view! {
                            <li class="flow-step">
                                <span class="flow-node">{i + 1}</span>
                                <span class="flow-text">{*step}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
            <figcaption class="study-graphic-caption">
                "How it works, step by step — each step as the project's own documentation states it."
            </figcaption>
        </figure>
    }
}

/// One configuration table from an app guide, in the design system.
#[component]
fn GuideConfigTable(entries: &'static [GuideEntry]) -> impl IntoView {
    view! {
        <figure class="bench">
            <div class="bench-scroll">
                <table class="bench-table guide-table">
                    <thead>
                        <tr>
                            <th>"Option"</th>
                            <th>"Default"</th>
                            <th>"What it does"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {entries
                            .iter()
                            .map(|e| {
                                view! {
                                    <tr>
                                        <td>
                                            <code>{e.name}</code>
                                        </td>
                                        <td>{if e.default.is_empty() { "—" } else { e.default }}</td>
                                        <td>{e.description}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </figure>
    }
}

/// One controls table from an app guide, in the design system.
#[component]
fn GuideControlsTable(entries: &'static [GuideControl]) -> impl IntoView {
    view! {
        <figure class="bench">
            <div class="bench-scroll">
                <table class="bench-table guide-table">
                    <thead>
                        <tr>
                            <th>"Action"</th>
                            <th>"Control"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {entries
                            .iter()
                            .map(|e| {
                                view! {
                                    <tr>
                                        <td>{e.action}</td>
                                        <td>
                                            <code>{e.control}</code>
                                        </td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </div>
        </figure>
    }
}

/// The CONFIGURATION and CONTROLS sections of an app guide, shared by
/// the guide page and by a study page whose project also carries a
/// guide. A section with no sourced rows does not render.
#[component]
fn GuideSections(guide: &'static AppGuide) -> impl IntoView {
    view! {
        {(!guide.config.is_empty())
            .then(|| {
                view! {
                    <section class="study-section">
                        <p class="band-label">"CONFIGURATION"</p>
                        <GuideConfigTable entries=guide.config />
                    </section>
                }
            })}
        {(!guide.controls.is_empty())
            .then(|| {
                view! {
                    <section class="study-section">
                        <p class="band-label">"CONTROLS"</p>
                        <GuideControlsTable entries=guide.controls />
                    </section>
                }
            })}
    }
}

/// The REFERENCES section shared by guide pages, study pages, and
/// generated solution pages: the primary sources a reader can follow
/// to validate the specs, protocols, and equations a page relies on.
/// Each entry is `(title, url, note)`; the note says what the source
/// establishes. An empty list renders nothing.
#[component]
pub fn ReferencesSection(refs: Vec<(&'static str, &'static str, &'static str)>) -> impl IntoView {
    view! {
        <section class="study-section">
            <p class="band-label">"REFERENCES"</p>
            <ol class="ref-list">
                {refs
                    .into_iter()
                    .map(|(title, url, note)| {
                        view! {
                            <li>
                                <a href={url}>{title}</a>
                                {(!note.is_empty())
                                    .then(|| view! { <span class="ref-note">{note}</span> })}
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </section>
    }
}

/// One visual from a guide's `visuals` list: a registry diagram, an
/// equation with the prose that defines its quantities, or a real
/// media artifact (image or video) with its provenance caption. An
/// unknown diagram id renders nothing, so a page ships a diagram only
/// when one is registered — the same rule as `ProjectGraphic`.
#[component]
fn GuideVisualView(visual: &'static GuideVisual) -> impl IntoView {
    match visual.kind {
        "diagram" => crate::pages::diagrams::diagram(visual.id).map_or_else(
            || ().into_any(),
            |svg| {
                view! {
                    <figure class="study-graphic">
                        {svg}
                        <figcaption class="study-graphic-caption">{visual.caption}</figcaption>
                    </figure>
                }
                .into_any()
            },
        ),
        "equation" => view! {
            <figure class="math-figure">
                <div class="math-block">
                    // inner_html, not a text child: KaTeX replaces the
                    // span's content in the browser, and hydration must
                    // not walk nodes a page script owns (as in METHODS).
                    <span
                        class="math-source"
                        inner_html=format!("\\[{}\\]", visual.latex)
                    ></span>
                </div>
                <p class="eq-define">{visual.define}</p>
                <figcaption class="study-graphic-caption">{visual.caption}</figcaption>
            </figure>
        }
        .into_any(),
        "image" => view! {
            <figure class="study-graphic visual-media">
                <img src={visual.src} alt={visual.alt} loading="lazy" />
                <figcaption class="study-graphic-caption">{visual.caption}</figcaption>
            </figure>
        }
        .into_any(),
        "video" => view! {
            <figure class="study-graphic visual-media">
                <video
                    src={visual.src}
                    poster={visual.poster}
                    controls
                    preload="none"
                    playsinline
                ></video>
                <figcaption class="study-graphic-caption">{visual.caption}</figcaption>
            </figure>
        }
        .into_any(),
        _ => ().into_any(),
    }
}

/// The CONCEPTS section of an app guide: its diagrams, equations, and
/// real media in the order the content file authors them. Shared by
/// the guide page and by a study page whose project also carries a
/// guide, so the two surfaces cannot drift apart.
#[component]
fn GuideVisuals(guide: &'static AppGuide) -> impl IntoView {
    (!guide.visuals.is_empty()).then(|| {
        view! {
            <section class="study-section">
                <p class="band-label">"CONCEPTS"</p>
                {guide
                    .visuals
                    .iter()
                    .map(|v| view! { <GuideVisualView visual={v} /> })
                    .collect_view()}
            </section>
        }
    })
}

/// The app guide page for one application: what it is, how it works
/// (the flow diagram), its real configuration options and controls,
/// and — stated plainly — what the sources did not cover.
#[component]
fn AppGuidePage(guide: &'static AppGuide) -> impl IntoView {
    let solution = solution_page(guide.slug);
    let mark = if guide.mark.is_empty() { solution.and_then(Solution::mark) } else { Some(guide.mark) };
    view! {
        <Title text=format!("{} - Qompass AI", guide.name) />
        <section class="page project-page">
            <article class="project-article">
                <header class="study-head">
                    <p class="page-bearing">{guide.kind_line}</p>
                    {mark
                        .map(|m| {
                            view! { <img class="project-mark" src=m alt=guide.name /> }
                        })}
                    <h1>{guide.name}</h1>
                    <p class="study-meta">
                        {solution
                            .and_then(|s| {
                                s.version.map(|_| {
                                    view! { <span>"Version " {version_link(s)}</span> }
                                })
                            })}
                        {(!guide.repo_url.is_empty())
                            .then(|| {
                                view! { <a href=guide.repo_url>"Source on GitHub"</a> }
                            })}
                    </p>
                </header>
                {(!guide.summary.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"OVERVIEW"</p>
                                {paragraphs(guide.summary)
                                    .into_iter()
                                    .map(|p| view! { <p>{p}</p> })
                                    .collect_view()}
                            </section>
                        }
                    })}
                {(!guide.how.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"HOW IT WORKS"</p>
                                <FlowDiagram steps=guide.how />
                            </section>
                        }
                    })}
                <GuideVisuals guide=guide />
                <GuideSections guide=guide />
                {(!guide.gaps.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"NOT COVERED"</p>
                                {paragraphs(guide.gaps)
                                    .into_iter()
                                    .map(|p| view! { <p>{p}</p> })
                                    .collect_view()}
                            </section>
                        }
                    })}
                {(!guide.references.is_empty())
                    .then(|| {
                        view! {
                            <ReferencesSection refs={
                                guide
                                    .references
                                    .iter()
                                    .map(|r| (r.title, r.url, r.note))
                                    .collect::<Vec<_>>()
                            } />
                        }
                    })}
            </article>
            {guide
                .visuals
                .iter()
                .any(|v| v.kind == "equation")
                .then(|| {
                    view! {
                        <link rel="stylesheet" href="/vendor/katex/katex.min.css" />
                        <script src="/vendor/katex/katex.min.js"></script>
                        <script src="/vendor/katex/contrib/auto-render.min.js"></script>
                        <script src="/js/project.js"></script>
                    }
                })}
        </section>
    }
}

/// Route entry for `/solutions/:slug`. The telco-admin practice quizzes
/// have their own page; every other slug is dispatched against
/// the compiled-in (ready-only) page list first: a match renders the
/// study. Otherwise a solution on the compass renders its generated
/// page, and anything else renders the same "Off chart" fallback as any
/// unknown bearing on the site.
#[component]
pub fn ProjectDispatch() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|map| map.get("slug")).unwrap_or_default();
    if slug == telco_quizzes::SLUG {
        return view! { <TelcoQuizzes /> }.into_any();
    }
    if let Some(page) = project_page(&slug) {
        return view! { <ProjectStudy slug=page.slug /> }.into_any();
    }
    if let Some(guide) = app_guide(&slug) {
        return view! { <AppGuidePage guide=guide /> }.into_any();
    }
    solution_page(&slug).map_or_else(
        || {
            view! {
                <section class="page">
                    <h1>"Off chart"</h1>
                    <p>"That bearing does not exist on this chart."</p>
                </section>
            }
            .into_any()
        },
        |solution| view! { <SolutionPage solution=solution /> }.into_any(),
    )
}

/// The generated page for one solution on the compass: its mark where
/// it has one, its cardinal and that service's brief, its version linked
/// to the release, and the way out to where it lives.
#[component]
fn SolutionPage(solution: &'static Solution) -> impl IntoView {
    let cardinal = CARDINALS.get(solution.cardinal);
    let source =
        if solution.href.starts_with("https://luarocks.org/") { "Module on LuaRocks" } else { "Source on GitHub" };
    view! {
        <Title text=format!("{} - Qompass AI", solution.name) />
        <section class="page project-page">
            <article class="project-article">
                <header class="study-head">
                    <p class="page-bearing">{cardinal.map(|c| c.mark)}</p>
                    {solution
                        .mark()
                        .map(|mark| view! { <img class="project-mark" src=mark alt=solution.name /> })}
                    <h1>{solution.name}</h1>
                    <p class="study-meta">
                        {solution
                            .version
                            .is_some()
                            .then(|| view! { <span>"Version " {version_link(solution)}</span> })}
                        <a href=solution.href>{source}</a>
                    </p>
                </header>
                {(solution.slug == "rose-nvim")
                    .then(|| {
                        view! {
                            <p>
                                "rose.nvim now lives inside " <a href="/solutions/diver">"Diver"</a>
                                ". This repository is archived and carries no release."
                            </p>
                        }
                    })}
                {cardinal
                    .map(|c| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">{c.name.to_uppercase()}</p>
                                <p>{c.brief}</p>
                            </section>
                        }
                    })}
                {crate::pages::diagrams::page_diagram(solution.slug)
                    .and_then(|id| {
                        crate::pages::diagrams::diagram(id).map(|svg| (id, svg))
                    })
                    .map(|(id, svg)| {
                        view! {
                            <figure class="study-graphic">
                                {svg}
                                <figcaption class="study-graphic-caption">
                                    {crate::pages::diagrams::diagram_caption(id)}
                                </figcaption>
                            </figure>
                        }
                    })}
                {crate::pages::compass_services::solution_media(solution.slug)
                    .into_iter()
                    .map(|(src, alt, caption)| {
                        view! {
                            <figure class="visual-media">
                                <img src=src alt=alt loading="lazy" />
                                <figcaption>{caption}</figcaption>
                            </figure>
                        }
                    })
                    .collect_view()}
                {(!crate::pages::compass_services::solution_references(solution.slug)
                    .is_empty())
                    .then(|| {
                        view! {
                            <ReferencesSection refs={
                                crate::pages::compass_services::solution_references(solution.slug)
                                    .to_vec()
                            } />
                        }
                    })}
            </article>
        </section>
    }
}

/// The study page for one ready project.
#[component]
pub fn ProjectStudy(slug: &'static str) -> impl IntoView {
    let Some(page) = project_page(slug) else {
        return view! {
            <section class="page">
                <h1>"Off chart"</h1>
                <p>"That bearing does not exist on this chart."</p>
            </section>
        }
        .into_any();
    };
    let has_results = !page.results_intro.is_empty() || !page.benchmarks.is_empty() || !page.figures_note.is_empty();
    view! {
        <Title text=format!("{} - Qompass AI", page.name) />
        <section class="page project-page">
            <article class="project-article">
                <header class="study-head">
                    <p class="page-bearing">{page.kind_line}</p>
                    {(!page.mark.is_empty())
                        .then(|| {
                            view! {
                                <img class="project-mark" src={page.mark} alt={page.name} />
                            }
                        })}
                    <h1>{page.name}</h1>
                    <p class="study-meta">
                        {(!page.status.is_empty())
                            .then(|| view! { <span>{page.status}</span> })}
                        {(!page.repo_url.is_empty())
                            .then(|| {
                                view! {
                                    <a href={page.repo_url}>"Source on GitHub"</a>
                                }
                            })}
                    </p>
                </header>
                <section class="study-section">
                    <p class="band-label">"DOWNLOADS"</p>
                    <DownloadBoxes slug={page.slug} />
                </section>
                {(!page.graphic.is_empty()).then(|| view! { <ProjectGraphic id={page.graphic} /> })}
                {app_guide(page.slug).map(|guide| view! { <GuideVisuals guide=guide /> })}
                {(!page.abstract_text.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"ABSTRACT"</p>
                                {paragraphs(page.abstract_text)
                                    .into_iter()
                                    .map(|p| view! { <p>{p}</p> })
                                    .collect_view()}
                            </section>
                        }
                    })}
                {(!page.methods.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"METHODS"</p>
                                {page
                                    .methods
                                    .iter()
                                    .map(|m| {
                                        view! {
                                            <h3 class="study-subhead">{m.heading}</h3>
                                            {paragraphs(m.body)
                                                .into_iter()
                                                .map(|p| view! { <p>{p}</p> })
                                                .collect_view()}
                                            {m
                                                .equations
                                                .iter()
                                                .map(|eq| {
                                                    view! {
                                                        <div class="math-block">
                                                            // inner_html, not a text child: KaTeX
                                                            // replaces the span's content in the
                                                            // browser, and hydration must not walk
                                                            // (and mismatch) nodes a page script owns.
                                                            <span
                                                                class="math-source"
                                                                inner_html=format!("\\[{eq}\\]")
                                                            ></span>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()}
                                        }
                                    })
                                    .collect_view()}
                            </section>
                        }
                    })}
                {has_results
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"RESULTS"</p>
                                {paragraphs(page.results_intro)
                                    .into_iter()
                                    .map(|p| view! { <p>{p}</p> })
                                    .collect_view()}
                                {page
                                    .benchmarks
                                    .iter()
                                    .map(|b| view! { <BenchTable bench={b} /> })
                                    .collect_view()}
                                {(!page.figures_note.is_empty())
                                    .then(|| {
                                        view! {
                                            <p class="figures-note">{page.figures_note}</p>
                                        }
                                    })}
                            </section>
                        }
                    })}
                {app_guide(page.slug)
                    .map(|guide| view! { <GuideSections guide=guide /> })}
                {(!page.discussion.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"DISCUSSION"</p>
                                {page
                                    .discussion
                                    .iter()
                                    .map(|p| {
                                        view! {
                                            <p>{*p}</p>
                                        }
                                    })
                                    .collect_view()}
                            </section>
                        }
                    })}
                {(!page.conclusion.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"CONCLUSION"</p>
                                {paragraphs(page.conclusion)
                                    .into_iter()
                                    .map(|p| view! { <p>{p}</p> })
                                    .collect_view()}
                            </section>
                        }
                    })}
                {(!page.interactive.is_empty())
                    .then(|| {
                        view! {
                            <section class="study-section">
                                <p class="band-label">"TRY IT"</p>
                                // Fallback via inner_html, not child nodes: the
                                // page script replaces the slot's content when it
                                // mounts the interactive, and hydration must not
                                // walk (and mismatch) nodes a page script owns.
                                <div
                                    class="interactive-slot"
                                    id="project-interactive"
                                    data-interactive={page.interactive}
                                    inner_html="<p class=\"interactive-fallback\">This element is interactive in the browser; the measurements behind it are in the tables above.</p>"
                                ></div>
                            </section>
                        }
                    })}
                {(!page.references.is_empty())
                    .then(|| {
                        view! {
                            <ReferencesSection refs={
                                page
                                    .references
                                    .iter()
                                    .map(|r| (r.title, r.url, r.note))
                                    .collect::<Vec<_>>()
                            } />
                        }
                    })}
            </article>
            <link rel="stylesheet" href="/vendor/katex/katex.min.css" />
            <script src="/vendor/katex/katex.min.js"></script>
            <script src="/vendor/katex/contrib/auto-render.min.js"></script>
            <script src="/js/project.js"></script>
        </section>
    }
    .into_any()
}
