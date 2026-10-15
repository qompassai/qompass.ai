//! The site's flow as one diagram — "the long arc over the loop".
//!
//! The base line is the loop in order: Standards (set), Analysis
//! (weigh), Field Notes (learn). Research, the Odyssey, is the arc over
//! it: it rises from Field Notes, where learning is promoted, and comes
//! down into Standards, which it revises. Analysis carries the full
//! figure; Research carries the compact one. One instance per page, so
//! the marker ids stay unique.
//!
//! The static diagram is complete on its own. Under
//! `prefers-reduced-motion: no-preference` a pulse of light runs the
//! base line, then the arc (`.fl-pulse` in `style/main.css`).
use leptos::prelude::*;

#[component]
pub fn FlowArc(#[prop(optional)] compact: bool) -> impl IntoView {
    let class = if compact { "flow-fig flow-fig-compact" } else { "flow-fig" };
    view! {
        <figure class=class>
            <svg viewBox="0 0 360 206" role="img" aria-label="The flow: Standards, then Analysis, then Field Notes along a solid base line; Research, the Odyssey, is a dotted arc over the loop that rises from Field Notes and comes down into Standards">
                <title>"The long arc over the loop"</title>
                <desc>"Three nodes on a solid base line, left to right: 1 Standards (set), 2 Analysis (weigh), 3 Field Notes (learn), with arrows from each to the next. Above them a dotted arc rises from Field Notes, with an arrow into the Research node at its peak (4, promote), and comes down with an arrow into Standards (revises)."</desc>
                <defs>
                    <marker id="fl-arr-base" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
                        <path d="M0 0 L7 3 L0 6 z" class="fl-mk-base"></path>
                    </marker>
                    <marker id="fl-arr-arc" markerWidth="8" markerHeight="8" refX="6" refY="3" orient="auto">
                        <path d="M0 0 L7 3 L0 6 z" class="fl-mk-arc"></path>
                    </marker>
                </defs>
                <path d="M304 140 C300 84 252 46 192 46" class="fl-arc" marker-end="url(#fl-arr-arc)"></path>
                <path d="M168 46 C108 46 60 84 56 138" class="fl-arc" marker-end="url(#fl-arr-arc)"></path>
                <path d="M66 150 H168" class="fl-base" marker-end="url(#fl-arr-base)"></path>
                <path d="M190 150 H292" class="fl-base" marker-end="url(#fl-arr-base)"></path>
                <path d="M56 150 H304" class="fl-pulse fl-pulse-base" pathLength="100"></path>
                <path d="M304 140 C300 84 252 46 192 46 H168 C108 46 60 84 56 138" class="fl-pulse fl-pulse-arc" pathLength="100"></path>
                <circle cx="180" cy="46" r="8" class="fl-peak"></circle>
                <text x="180" y="24" class="fl-name" font-size="14">"Research · Odyssey"</text>
                <text x="180" y="72" class="fl-step fl-arc-step" font-size="12">"4 · PROMOTE"</text>
                <text x="284" y="106" class="fl-note fl-end" font-size="12">"promoted"</text>
                <text x="76" y="106" class="fl-note" font-size="12">"revises"</text>
                <circle cx="56" cy="150" r="9" class="fl-node"></circle>
                <circle cx="180" cy="150" r="9" class="fl-node"></circle>
                <circle cx="304" cy="150" r="9" class="fl-node"></circle>
                <text x="56" y="180" class="fl-name" font-size="14">"Standards"</text>
                <text x="180" y="180" class="fl-name" font-size="14">"Analysis"</text>
                <text x="304" y="180" class="fl-name" font-size="14">"Field Notes"</text>
                <text x="56" y="198" class="fl-step" font-size="12">"1 · SET"</text>
                <text x="180" y="198" class="fl-step" font-size="12">"2 · WEIGH"</text>
                <text x="304" y="198" class="fl-step" font-size="12">"3 · LEARN"</text>
            </svg>
            <figcaption>
                "The loop and the long arc. Standards set the terms, Analysis weighs claims "
                "under them, and Field Notes record the results and what they taught. The "
                "strongest of that learning is promoted into Research — the Odyssey — which "
                "arcs over the loop and comes down to revise the Standards."
            </figcaption>
        </figure>
    }
}
