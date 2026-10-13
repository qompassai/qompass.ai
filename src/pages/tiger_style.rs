//! "Tiger Style" — the standing orders page, `/tiger-style`.
//!
//! One editorial page: manifesto, lineage, principles, then all
//! seventeen language guides as expandable chapters rendered from
//! `crate::tiger_data` (Matt's own guides, verbatim). Full references
//! live on quiet per-language pages linked only from their chapter —
//! the chapter HTML plus seventeen references would push this page
//! far past a sane weight. No chapter needs JavaScript: `<details>`
//! is native, and everything is in the prerendered HTML.
use crate::tiger_data::{tiger_reference, TIGER_GUIDES};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

/// The badge every chapter icon shares.
///
/// A rounded tile, a gradient ring (the `ts-ring-grad` gradient
/// defined once beside the chapters), a soft halo, and a dashed
/// orbit ring the stylesheet slowly turns. Glyphs are hand-drawn
/// in the Qompass palette — stylized motifs, never the projects'
/// own logo assets.
const ICON_BADGE: &str = r#"<rect x="3" y="3" width="42" height="42" rx="11" fill="var(--chart-raise)" stroke="url(#ts-ring-grad)" stroke-width="1.6"/><circle class="ts-icon-halo" cx="24" cy="24" r="11" fill="var(--cherenkov)"/><circle class="ts-icon-ring" cx="24" cy="24" r="17.5" fill="none" stroke="var(--cherenkov)" stroke-opacity=".5" stroke-width="1.3" stroke-dasharray="3.5 5.5"/>"#;

/// The glyph at the centre of one chapter's icon badge, by slug.
fn icon_glyph(slug: &str) -> &'static str {
    match slug {
        "aseprite" => {
            r##"<g fill="#9DBCFF"><rect x="13.5" y="13.5" width="7" height="7" rx="1.5" opacity=".5"/><rect x="22.5" y="13.5" width="7" height="7" rx="1.5" opacity=".85"/><rect x="31.5" y="13.5" width="7" height="7" rx="1.5" opacity=".5"/><rect x="13.5" y="22.5" width="7" height="7" rx="1.5" opacity=".85"/><rect x="31.5" y="22.5" width="7" height="7" rx="1.5" opacity=".85"/><rect x="13.5" y="31.5" width="7" height="7" rx="1.5" opacity=".5"/><rect x="22.5" y="31.5" width="7" height="7" rx="1.5" opacity=".85"/><rect x="31.5" y="31.5" width="7" height="7" rx="1.5" opacity=".5"/></g><rect x="22.5" y="22.5" width="7" height="7" rx="1.5" fill="#C9A227"/>"##
        }
        "bash" => r#"<text class="ts-icon-text" x="24" y="30" text-anchor="middle" font-size="15">&gt;_</text>"#,
        "c" => r#"<text class="ts-icon-text" x="24" y="32" text-anchor="middle" font-size="22">C</text>"#,
        "cpp" => r#"<text class="ts-icon-text" x="24" y="29.5" text-anchor="middle" font-size="13.5">C++</text>"#,
        "go" => r#"<text class="ts-icon-text" x="24" y="30.5" text-anchor="middle" font-size="17">Go</text>"#,
        "kotlin" => {
            r##"<path d="M13 35 L35 13 L35 35 Z" fill="#7C5CFF" opacity=".3"/><text class="ts-icon-text" x="24" y="31" text-anchor="middle" font-size="19">K</text>"##
        }
        "lua" => {
            r##"<path d="M28.5 9.5 A14.5 14.5 0 1 0 28.5 38.5 A17.5 17.5 0 1 1 28.5 9.5 Z" fill="#9DBCFF"/><circle cx="32" cy="15.5" r="1.7" fill="#C9A227"/>"##
        }
        "mojo" => {
            r##"<path d="M24 9 C27.5 14.5 32 18.5 32 26.5 A8 8 0 1 1 16 26.5 C16 18.5 20.5 14.5 24 9 Z" fill="none" stroke="#9DBCFF" stroke-width="1.8"/><path d="M24 21.5 C25.8 23.8 27.3 25.4 27.3 28.3 A3.3 3.3 0 1 1 20.7 28.3 C20.7 25.4 22.2 23.8 24 21.5 Z" fill="#C9A227"/>"##
        }
        "nix" => {
            r##"<g stroke="#9DBCFF" stroke-width="2" stroke-linecap="round"><line x1="24" y1="13" x2="24" y2="35"/><line x1="14.5" y1="18.5" x2="33.5" y2="29.5"/><line x1="33.5" y1="18.5" x2="14.5" y2="29.5"/></g><circle cx="24" cy="24" r="2.3" fill="#C9A227"/>"##
        }
        "odin" => {
            r##"<path d="M15.5 15.5 L12 10.5 M32.5 15.5 L36 10.5" stroke="#7C5CFF" stroke-width="2" stroke-linecap="round" fill="none"/><circle cx="24" cy="24" r="9.5" fill="none" stroke="#9DBCFF" stroke-width="2"/><circle cx="24" cy="24" r="3.4" fill="#C9A227"/>"##
        }
        "php" => {
            r##"<ellipse cx="24" cy="24" rx="13" ry="9.5" fill="none" stroke="#9DBCFF" stroke-width="1.8"/><text class="ts-icon-mono" x="24" y="27.5" text-anchor="middle" font-size="10">php</text>"##
        }
        "python" => {
            r##"<rect x="13" y="13" width="15" height="11" rx="5.5" fill="none" stroke="#5B8CFF" stroke-width="2"/><rect x="20" y="24" width="15" height="11" rx="5.5" fill="none" stroke="#C9A227" stroke-width="2"/><circle cx="19.5" cy="18.5" r="1.7" fill="#9DBCFF"/><circle cx="28.5" cy="29.5" r="1.7" fill="#C9A227"/>"##
        }
        "rust" => {
            r##"<circle cx="24" cy="24" r="11" fill="none" stroke="#9DBCFF" stroke-width="4.5" stroke-dasharray="4.2 4.45"/><circle cx="24" cy="24" r="3.2" fill="#C9A227"/>"##
        }
        "scala" => {
            r##"<polyline points="13,33.5 20,33.5 20,26.5 27,26.5 27,19.5 35,19.5" fill="none" stroke="#9DBCFF" stroke-width="2.4" stroke-linejoin="round" stroke-linecap="round"/><rect x="32.5" y="11.5" width="5" height="5" fill="#C9A227"/>"##
        }
        "typescript" => r#"<text class="ts-icon-text" x="24" y="29.5" text-anchor="middle" font-size="14.5">TS</text>"#,
        "wgsl" => {
            r##"<path d="M12.5 20.5 Q16.5 15.5 20.5 20.5 T28.5 20.5 T36.5 20.5" fill="none" stroke="#5B8CFF" stroke-width="2" stroke-linecap="round"/><path d="M12.5 28 Q16.5 23 20.5 28 T28.5 28 T36.5 28" fill="none" stroke="#7C5CFF" stroke-width="2" stroke-linecap="round"/>"##
        }
        "zig" => r##"<polygon points="27.5,9 15.5,26.5 22.5,26.5 20,39 33,20.5 26,20.5" fill="#C9A227"/>"##,
        _ => r##"<circle cx="24" cy="24" r="4" fill="#9DBCFF"/>"##,
    }
}

/// The one-sentence header summary for a chapter, where one is used.
///
/// Three chapters' imported scope lines are long, multi-sentence
/// skill descriptions whose substance the guide body already
/// carries; their headers keep one sentence summarizing the
/// approach instead, and nothing is repeated inside. Every other
/// chapter folds its scope line into the chapter as the lead and
/// its header is name-only.
fn chapter_headline(slug: &str) -> Option<&'static str> {
    match slug {
        "c" => Some("Safety-first C — explicit contracts, checked bounds, and disciplined manual ownership, hardened against overflow and use-after-free."),
        "nix" => Some("Safety-first Nix — pure evaluation, explicit inputs, and pinned sources, validated by a pinned toolchain."),
        "typescript" => Some("Safety-first TypeScript under the strict profile — explicit contracts, strict null handling, and bounded async work."),
        _ => None,
    }
}

/// Reference links for one chapter's body, by slug.
///
/// The official language or tool documentation, the standard or
/// toolchain reference the guide names, and the lineage link where
/// it fits. Every URL verified live when this list was written.
fn chapter_references(slug: &str) -> &'static [(&'static str, &'static str)] {
    match slug {
        "aseprite" => &[
            ("Aseprite", "https://www.aseprite.org/"),
            ("Aseprite CLI docs", "https://www.aseprite.org/docs/cli/"),
            ("Aseprite scripting docs", "https://www.aseprite.org/docs/scripting/"),
        ],
        "bash" => &[
            ("Bash manual", "https://www.gnu.org/software/bash/manual/html_node/index.html"),
            ("ShellCheck", "https://www.shellcheck.net/"),
        ],
        "c" => {
            &[("C reference (cppreference)", "https://en.cppreference.com/w/c"), ("clangd", "https://clangd.llvm.org/")]
        }
        "cpp" => &[
            ("C++ reference (cppreference)", "https://en.cppreference.com/w/cpp"),
            ("C++ Core Guidelines", "https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines"),
        ],
        "go" => &[("Go documentation", "https://go.dev/doc/"), ("Effective Go", "https://go.dev/doc/effective_go")],
        "kotlin" => &[
            ("Kotlin documentation", "https://kotlinlang.org/docs/home.html"),
            ("Kotlin standard library", "https://kotlinlang.org/api/latest/jvm/stdlib/"),
        ],
        "lua" => &[("Lua 5.4 reference manual", "https://www.lua.org/manual/5.4/"), ("LuaJIT", "https://luajit.org/")],
        "mojo" => &[
            ("Mojo documentation", "https://docs.modular.com/mojo/"),
            ("Mojo manual", "https://docs.modular.com/mojo/manual/"),
        ],
        "nix" => &[
            ("Nix manual", "https://nixos.org/manual/nix/stable/"),
            ("Nixpkgs manual", "https://nixos.org/manual/nixpkgs/stable/"),
            ("nix.dev", "https://nix.dev/"),
        ],
        "odin" => {
            &[("Odin", "https://odin-lang.org/"), ("Odin language overview", "https://odin-lang.org/docs/overview/")]
        }
        "php" => &[
            ("PHP manual", "https://www.php.net/manual/en/"),
            ("PSR-12 coding style", "https://www.php-fig.org/psr/psr-12/"),
        ],
        "python" => {
            &[("Python documentation", "https://docs.python.org/3/"), ("PEP 8", "https://peps.python.org/pep-0008/")]
        }
        "rust" => &[
            ("The Rust Book", "https://doc.rust-lang.org/book/"),
            ("Clippy documentation", "https://doc.rust-lang.org/clippy/"),
            ("Rust API Guidelines", "https://rust-lang.github.io/api-guidelines/"),
            ("TigerStyle", "https://tigerstyle.dev/"),
        ],
        "scala" => &[
            ("Scala documentation", "https://docs.scala-lang.org/"),
            ("Scala 3 Book", "https://docs.scala-lang.org/scala3/book/introduction.html"),
        ],
        "typescript" => {
            &[("TypeScript documentation", "https://www.typescriptlang.org/docs/"), ("Biome", "https://biomejs.dev/")]
        }
        "wgsl" => &[("WGSL specification", "https://www.w3.org/TR/WGSL/"), ("Bevy", "https://bevyengine.org/")],
        "zig" => &[
            ("Zig learning resources", "https://ziglang.org/learn/"),
            ("Zig language reference", "https://ziglang.org/documentation/master/"),
            ("TigerStyle", "https://tigerstyle.dev/"),
        ],
        _ => &[],
    }
}

#[component]
pub fn TigerStyle() -> impl IntoView {
    view! {
        <Title text="Tiger Style - Qompass AI" />
        <section class="page tiger-page">
            <p class="page-bearing">"STANDING ORDERS"</p>
            <h1>"Tiger Style"</h1>
            <p class="ts-subhead">"The orders the work is done under."</p>
            <p class="page-lede">
                "Tiger Style is Qompass AI's standing orders for code. Safety first, "
                "everything bounded, nothing hidden — in every language we ship."
            </p>

            <h2 class="band-label">"RESPECT WHERE DUE"</h2>
            <p>
                "This standard descends from "
                <a href="https://tigerstyle.dev/" rel="external">"TigerStyle"</a>
                ", the coding standard written for TigerBeetle — a database that "
                "cannot afford to be wrong — and, behind it, NASA JPL's "
                <a href="https://spinroot.com/gerard/pdf/P10.pdf" rel="external">
                    "Power of Ten"
                </a>
                ", Gerard J. Holzmann's rules for developing safety-critical code. "
                "It is adapted here with respect, and the debt is stated plainly."
            </p>
            <p>
                "What Qompass takes from both: determinism over cleverness — "
                "explicit limits, bounded work, deterministic interfaces. A program "
                "should verify itself while it runs, and shut down rather than run "
                "wrong. And doing it right the first time, because production "
                "rarely grants a second draft."
            </p>

            <h2 class="band-label">"THE ORDERS THEMSELVES"</h2>

            <div class="ts-principle">
                <h3 class="study-subhead">"Safety, then performance, then convenience"</h3>
                <p>
                    "The priority order, always. When the three compete — and they "
                    "compete daily — safety wins outright, performance is argued "
                    "with measurements, and developer experience takes what is left."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 108" role="img" aria-label="Priority bars: safety full width, performance narrower, developer experience narrowest">
                    <rect class="gfx-panel" x="0" y="0" width="470" height="26" />
                    <rect class="gfx-bar-top" x="0" y="0" width="470" height="26" />
                    <rect class="gfx-panel" x="0" y="41" width="340" height="26" />
                    <rect class="gfx-bar" x="0" y="41" width="340" height="26" />
                    <rect class="gfx-panel" x="0" y="82" width="220" height="26" />
                    <rect x="0" y="82" width="220" height="26" fill="var(--violet)" />
                    <text class="gfx-text" x="482" y="18">"SAFETY"</text>
                    <text class="gfx-text" x="352" y="59">"PERFORMANCE"</text>
                    <text class="gfx-text" x="232" y="100">"DEVELOPER EXPERIENCE"</text>
                </svg>
                <blockquote class="ts-quote">
                    <p>"Priority order, always: Safety > Performance > Developer Experience."</p>
                    <cite>"— the Rust guide, below"</cite>
                </blockquote>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Contracts first"</h3>
                <p>
                    "Every substantial operation answers, before it is written: "
                    "accepted and rejected inputs, the trust boundary, the maximum "
                    "work, memory, output, and time, the owner of every allocation "
                    "and handle, and what failure and cancellation do. If the "
                    "contract cannot be stated, the operation is not understood "
                    "yet — and is not written yet."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 92" role="img" aria-label="A contract box with four labeled ports: inputs, bounds, owner, failure">
                    <rect class="gfx-panel" x="180" y="16" width="200" height="60" />
                    <text class="gfx-text" x="280" y="51" text-anchor="middle">"OPERATION"</text>
                    <text class="gfx-note" x="60" y="51">"INPUTS →"</text>
                    <text class="gfx-note" x="436" y="30">"→ BOUNDS"</text>
                    <text class="gfx-note" x="436" y="51">"→ OWNER"</text>
                    <text class="gfx-note" x="436" y="72">"→ FAILURE"</text>
                </svg>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Bound everything"</h3>
                <p>
                    "Limits are named constants carrying units — output_bytes_max, "
                    "retry_count, queue_capacity, deadline — and index ranges are "
                    "validated with checked arithmetic before an end is computed. "
                    "A bound that lives only in someone's head is not a bound."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 76" role="img" aria-label="A capacity bar stopped by a brass MAX marker before the end">
                    <rect class="gfx-panel" x="0" y="28" width="470" height="20" />
                    <rect class="gfx-bar" x="0" y="28" width="330" height="20" />
                    <line x1="446" y1="14" x2="446" y2="62" stroke="var(--brass)" stroke-width="2" />
                    <text class="gfx-text" x="482" y="44">"MAX"</text>
                    <text class="gfx-note" x="0" y="70">"work grows to the limit — and stops there, by construction"</text>
                </svg>
                <blockquote class="ts-quote">
                    <p>"No unbounded loops, retries, queues, or growth."</p>
                    <cite>"— the Rust guide, below"</cite>
                </blockquote>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Errors are data; assertions are law"</h3>
                <p>
                    "Invalid input, configuration, or protocol data gets a typed "
                    "error with bounded context; the caller owns recovery. A "
                    "corrupt internal invariant gets an assertion — the program "
                    "verifies itself while it runs and stops rather than run "
                    "wrong. Nothing unwraps external data, and no required "
                    "mutation hides inside an assertion."
                </p>
                <svg class="ts-diagram" viewBox="0 0 560 96" role="img" aria-label="A fork: bad input returns as a typed error; a broken invariant halts the program">
                    <text class="gfx-note" x="0" y="30">"bad input ───→ typed error ───→ caller recovers"</text>
                    <text class="gfx-note" x="0" y="66">"broken invariant ───→ assertion ───→ halt, loudly"</text>
                    <line x1="0" y1="80" x2="470" y2="80" stroke="var(--chart-line)" />
                </svg>
            </div>

            <div class="ts-principle">
                <h3 class="study-subhead">"Paired tools"</h3>
                <p>
                    "No guide ships alone. Each names its toolchain — language "
                    "server, formatter, linter, debugger — so the standard is "
                    "enforced by machines, not taste. The pairing is in every "
                    "chapter below: rust-analyzer, clippy, and rustfmt for Rust; "
                    "zls and zig fmt for Zig; shellcheck and shfmt for Bash."
                </p>
                <div class="ts-tools">
                    <span class="ts-tool">"LSP — see the contract"</span>
                    <span class="ts-tool">"FORMAT — one shape"</span>
                    <span class="ts-tool">"LINT — zero warnings"</span>
                    <span class="ts-tool">"DEBUG — inspect, don't guess"</span>
                </div>
            </div>

            <h2 class="band-label">"THE ORDERS, BY LANGUAGE"</h2>
            <p class="on-ai-note">
                "Seventeen chapters — the operational core of each language's "
                "guide, verbatim. Open one and read it here; the full reference "
                "behind each chapter is one link further, on its own page."
            </p>
            <svg class="ts-defs" aria-hidden="true">
                <defs>
                    <linearGradient
                        id="ts-ring-grad"
                        x1="0"
                        y1="0"
                        x2="48"
                        y2="48"
                        gradientUnits="userSpaceOnUse"
                    >
                        <stop offset="0" stop-color="#5B8CFF"></stop>
                        <stop offset=".55" stop-color="#7C5CFF"></stop>
                        <stop offset="1" stop-color="#C9A227"></stop>
                    </linearGradient>
                </defs>
            </svg>
            {TIGER_GUIDES
                .iter()
                .map(|g| {
                    let headline = chapter_headline(g.slug);
                    let lead = headline.is_none().then_some(g.scope);
                    let icon = format!("{ICON_BADGE}{}", icon_glyph(g.slug));
                    let refs = chapter_references(g.slug)
                        .iter()
                        .enumerate()
                        .map(|(i, &(label, url))| {
                            view! {
                                {(i > 0)
                                    .then(|| {
                                        view! { <span class="ts-ref-sep">" · "</span> }
                                    })}
                                <a href={url} rel="external">{label}</a>
                            }
                        })
                        .collect_view();
                    view! {
                        <details class="ts-chapter" id={format!("chapter-{}", g.slug)}>
                            <summary class="ts-summary">
                                <svg
                                    class="ts-icon"
                                    viewBox="0 0 48 48"
                                    aria-hidden="true"
                                    inner_html={icon}
                                ></svg>
                                <span class="ts-lang">{g.name}</span>
                                {headline
                                    .map(|h| {
                                        view! { <span class="ts-headline">{h}</span> }
                                    })}
                            </summary>
                            <div class="ts-guide">
                                {lead
                                    .map(|scope| {
                                        view! { <p class="ts-lead">{scope}</p> }
                                    })}
                                <div inner_html={g.html}></div>
                                <p class="ts-refs">
                                    <span class="ts-refs-label">"References"</span>
                                    {refs}
                                </p>
                            </div>
                            {(g.has_reference)
                                .then(|| {
                                    view! {
                                        <p class="ts-ref-link">
                                            <a href={format!("/tiger-style/{}", g.slug)}>
                                                {format!("Read the full {} reference →", g.name)}
                                            </a>
                                        </p>
                                    }
                                })}
                        </details>
                    }
                })
                .collect_view()}

            <p class="ts-closing">
                "These orders are standing: they do not expire between releases, "
                "and they are not suspended for deadlines. Every project on the "
                <a href="/projects">"projects page"</a>
                " was built under them — that is what the phrase means when it "
                "appears in a repository: the work was done under these orders."
            </p>
        </section>
    }
}

/// Quiet standalone page for one language's full reference, linked
/// only from its chapter on `/tiger-style`. Unknown slugs get the
/// site's standard fallback.
#[component]
pub fn TigerStyleDispatch() -> impl IntoView {
    let params = use_params_map();
    let reference = params.with(|map| map.get("slug").and_then(|slug| tiger_reference(slug.as_str())));
    reference.map_or_else(
        || {
            view! {
                <section class="page">
                    <h1>"Off chart"</h1>
                    <p>"That bearing does not exist on this chart."</p>
                </section>
            }
            .into_any()
        },
        |reference| {
            view! {
                <Title text=format!("{} Tiger Style reference - Qompass AI", reference.name) />
                <section class="page tiger-page">
                    <p class="page-bearing">"STANDING ORDERS — FULL REFERENCE"</p>
                    <h1>{format!("{} — the full reference", reference.name)}</h1>
                    <p class="ts-back">
                        <a href="/tiger-style">"← The standing orders"</a>
                    </p>
                    <div class="ts-guide" inner_html={reference.html}></div>
                    <p class="ts-back">
                        <a href="/tiger-style">"← The standing orders"</a>
                    </p>
                </section>
            }
            .into_any()
        },
    )
}
