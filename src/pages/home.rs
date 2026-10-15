use crate::pages::compass_services::CompassServices;
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Qompass AI - Navigating Deep Tech's Murky Waters" />
        <section class="hero">
            <canvas id="qompass-hero" aria-hidden="true"></canvas>
            <div class="hero-copy">
                <h1>
                    "Navigating deep tech's " <span class="glow">"murky waters"</span> "."
                </h1>
                <p class="hero-sub">
                    "Qompass AI builds the instruments deep tech runs on — security, "
                    "research, and deployment tooling, measured, not marketed."
                </p>
                <p class="hero-ctas">
                    <a class="cta cta-brass" href="/standards">
                        "Start with the Standards"
                    </a>
                    <a class="cta cta-quiet" href="/solutions">
                        "See the solutions"
                    </a>
                </p>
            </div>
        </section>
        <CompassServices />
        <section class="proof-band">
            <p class="band-label">"SUPPORTED BY"</p>
            <div class="proof-grid">
                <article class="proof-card">
                    <h2 class="proof-name">"Zimmer Biomet Foundation"</h2>
                    <p class="proof-fact">"$30,000 grant (Mar 2024)"</p>
                </article>
                <article class="proof-card">
                    <h2 class="proof-name">"Greater Spokane Inc. PathFinders"</h2>
                    <p class="proof-fact">"$2,000 (Oct 2024)"</p>
                </article>
            </div>
        </section>
        <section class="invitation">
            <p class="band-label">"OPEN CHANNEL"</p>
            <h2 class="band-title">"Got a hard problem?"</h2>
            <p class="band-copy">
                "Security reviews, research tooling, and deployment work — measured, not marketed."
            </p>
            <p class="hero-ctas">
                <a class="cta cta-brass" href="/about#identity">
                    "Start a conversation"
                </a>
            </p>
        </section>
        <script type="module" src="/js/hero.js"></script>
    }
}
