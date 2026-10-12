use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Qompass AI - Navigating Deep Tech's Murky Waters" />
        <section class="hero">
            <canvas id="qompass-hero" aria-hidden="true"></canvas>
            <div class="hero-copy">
                <p class="hero-bearing">"47.66° N · 117.43° W — SPOKANE, FIXED"</p>
                <h1>
                    "Navigating deep tech's " <span class="glow">"murky waters"</span> "."
                </h1>
                <p class="hero-sub">
                    "Qompass AI builds the instruments deep tech runs on: post-quantum "
                    "cryptography benchmarks, agent runtimes, key infrastructure, and the "
                    "developer tools that tie them together."
                </p>
                <p class="hero-ctas">
                    <a class="cta cta-primary" href="/research">
                        "Read the research"
                    </a>
                    <a class="cta cta-quiet" href="/projects">
                        "See the fleet"
                    </a>
                </p>
            </div>
        </section>
        <section class="pillars">
            <article class="pillar">
                <p class="pillar-mark">"01 — SOUNDINGS"</p>
                <h2>"Measured, not marketed"</h2>
                <p>
                    "Every claim we publish is a benchmark we ran: 24 NIST post-quantum KEMs "
                    "timed, attack surfaces exercised, negative results reported as results."
                </p>
            </article>
            <article class="pillar">
                <p class="pillar-mark">"02 — INSTRUMENTS"</p>
                <h2>"Tools that hold a bearing"</h2>
                <p>
                    "A Neovim-native development environment, a bounded agent runtime, a "
                    "post-quantum key server and binary cache — built in the open, in Rust."
                </p>
            </article>
            <article class="pillar">
                <p class="pillar-mark">"03 — FIRST, DO NO HARM"</p>
                <h2>"Safety as the fixed star"</h2>
                <p>
                    "Our published standard for AI systems: explicit contracts, bounded work, "
                    "no hidden behavior — in the papers and in the code alike."
                </p>
            </article>
        </section>
        <section class="chart-band">
            <p class="band-label">"CURRENT FIX"</p>
            <p class="band-copy">
                "This site is compiled from Rust to WebAssembly, generated statically, and "
                "served by vongola — our own post-quantum reverse proxy — alongside volta, "
                "our key server, and bunker, our signed binary cache."
            </p>
        </section>
        <script type="module" src="/js/hero.js"></script>
    }
}
