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
            {TIGER_GUIDES
                .iter()
                .map(|g| {
                    view! {
                        <details class="ts-chapter" id={format!("chapter-{}", g.slug)}>
                            <summary class="ts-summary">
                                <span class="ts-lang">{g.name}</span>
                                <span class="ts-scope">{g.scope}</span>
                            </summary>
                            <div class="ts-guide" inner_html={g.html}></div>
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
