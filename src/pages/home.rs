use leptos::prelude::*;
use leptos_meta::Title;

/// One project in the quick-start picker. Copy is condensed from the
/// /solutions entries; `repo` is the GitHub repository name under
/// github.com/qompassai and drives the clone command.
#[derive(Clone, Copy)]
struct FleetEntry {
    blurb: &'static str,
    kind: &'static str,
    name: &'static str,
    repo: &'static str,
}

const FLEET: [FleetEntry; 8] = [
    FleetEntry {
        blurb: "An Attic-based Nix binary cache with hybrid Ed25519 + ML-DSA-65 \
            narinfo signing — packages signed for the post-quantum era, served \
            from cache.qompass.ai.",
        kind: "NIX BINARY CACHE — RUST",
        name: "bunker",
        repo: "bunker",
    },
    FleetEntry {
        blurb: "A Neovim-native development environment: 138 formatter adapters, \
            deep LSP coverage, in-editor AI — the cockpit the rest of the fleet \
            is built from.",
        kind: "NEOVIM ENVIRONMENT — LUA",
        name: "diver",
        repo: "diver",
    },
    FleetEntry {
        blurb: "A Rust port of the REA MCP reverse-engineering \
            server — depth-sounding for binaries.",
        kind: "REVERSE ENGINEERING — RUST",
        name: "fathom",
        repo: "fathom",
    },
    FleetEntry {
        blurb: "Encrypted mail the way it should feel: OpenPGP compose with a \
            real key picker, Tor transport, and key lookup against volta. \
            Heading to F-Droid.",
        kind: "PGP MAIL CLIENT — ANDROID",
        name: "ghost",
        repo: "ghost",
    },
    FleetEntry {
        blurb: "A fiber-optic cabling puzzle game — route the light, validate \
            the signal. Coming to Google Play, F-Droid, and Windows.",
        kind: "GAME — RUST / BEVY",
        name: "light-show",
        repo: "light-show",
    },
    FleetEntry {
        blurb: "A bounded planner-coder-reviewer agent runtime: one safe core \
            shared by CLI, TUI, and MCP, with operator-approved checks.",
        kind: "AGENT RUNTIME — RUST",
        name: "phlow",
        repo: "phlow",
    },
    FleetEntry {
        blurb: "A Hagrid-derived key server — HKP, VKS, and Web Key Directory — \
            with post-quantum ambitions. Live at keys.qompass.ai.",
        kind: "OPENPGP KEY SERVER — RUST",
        name: "volta",
        repo: "volta",
    },
    FleetEntry {
        blurb: "Post-quantum hybrid TLS reverse proxy on Pingora, with built-in \
            ACME. It serves this site.",
        kind: "REVERSE PROXY — RUST",
        name: "vongola",
        repo: "vongola",
    },
];

#[component]
fn Quickstart() -> impl IntoView {
    let selected = RwSignal::new("vongola");
    let copied = RwSignal::new(false);
    let current = move || FLEET.iter().find(|entry| entry.repo == selected.get()).copied();
    let copy = move |_| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                let command = format!("git clone https://github.com/qompassai/{}.git", selected.get());
                let promise = window.navigator().clipboard().write_text(&command);
                // The label only flips on a settled write, and a denied write
                // (unfocused document, permission policy) is swallowed here
                // instead of surfacing as an unhandled rejection. The promise
                // reactions fire after this handler returns, so forget() hands
                // each closure to the JS side, which frees it when called.
                let mark_copied = wasm_bindgen::closure::Closure::once(move |_| copied.set(true));
                let swallow = wasm_bindgen::closure::Closure::once(|_| ());
                // The catch hangs off the derived promise: a rejection skips
                // the fulfillment arm and propagates into it, so it must be
                // consumed there, not on the original promise.
                let _ = promise.then(&mark_copied).catch(&swallow);
                mark_copied.forget();
                swallow.forget();
            }
        }
    };
    view! {
        <section class="quickstart">
            <p class="band-label">"THE FLEET"</p>
            <h2 class="band-title">"Quick start"</h2>
            <div class="qs-picker" role="group" aria-label="Choose a project">
                {FLEET
                    .iter()
                    .map(|entry| {
                        let repo = entry.repo;
                        view! {
                            <button
                                type="button"
                                class=move || {
                                    if selected.get() == repo { "qs-chip qs-chip-active" } else { "qs-chip" }
                                }
                                aria-pressed=move || (selected.get() == repo).to_string()
                                on:click=move |_| {
                                    selected.set(repo);
                                    copied.set(false);
                                }
                            >
                                {entry.name}
                            </button>
                        }
                    })
                    .collect_view()}
            </div>
            <div aria-live="polite">
                {move || {
                    current()
                        .map(|entry| {
                            view! {
                                <article class="qs-detail">
                                    <p class="entry-kind">{entry.kind}</p>
                                    <h3 class="qs-name">{entry.name}</h3>
                                    <p class="qs-blurb">{entry.blurb}</p>
                                    <div class="qs-command">
                                        <pre class="qs-code">
                                            <code>
                                                {format!("git clone https://github.com/qompassai/{}.git", entry.repo)}
                                            </code>
                                        </pre>
                                        <button type="button" class="qs-copy" on:click=copy>
                                            {move || if copied.get() { "Copied" } else { "Copy" }}
                                        </button>
                                    </div>
                                    <p class="qs-link">
                                        <a href=format!("https://github.com/qompassai/{}", entry.repo)>
                                            "View on GitHub"
                                        </a>
                                    </p>
                                </article>
                            }
                        })
                }}
            </div>
        </section>
    }
}

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
                    "Qompass AI builds the instruments deep tech runs on — security, "
                    "research, and deployment tooling, measured, not marketed."
                </p>
                <p class="hero-ctas">
                    <a class="cta cta-brass" href="/solutions">
                        "See the work"
                    </a>
                    <a class="cta cta-quiet" href="/research">
                        "Read the research"
                    </a>
                </p>
            </div>
        </section>
        <section class="services-band">
            <p class="band-label">"FOUR CARDINAL SERVICES"</p>
            <div class="services">
                <article class="service-card">
                    <p class="pillar-mark">"N — NORTH"</p>
                    <h2>"Security"</h2>
                    <p>
                        "NIST-compliant security solutions — post-quantum cryptography, "
                        "red-team tooling, and privacy infrastructure, built in the open."
                    </p>
                    <ul class="service-links">
                        <li>
                            <a href="https://github.com/qompassai/qompass-hold">"Qompass-Hold"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/qssl">"Qssl"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/radar">"Radar"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/redteam">"Redteam"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/tor">"Tor"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/vongola">"Vongola"</a>
                        </li>
                    </ul>
                </article>
                <article class="service-card">
                    <p class="pillar-mark">"E — EAST"</p>
                    <h2>"Education"</h2>
                    <p>
                        "Empowering education solutions — learning tools that live in the "
                        "editor, and PathFinders, our internship program for new builders."
                    </p>
                    <ul class="service-links">
                        <li>
                            <a href="https://github.com/qompassai/blaze-ts.nvim">"blaze-ts.nvim"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/blaze.nvim">"blaze.nvim"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/Diver">"Diver"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/PathFinders">"PathFinders"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/qluv">"Qluv"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/rose.nvim">"Rose.nvim"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/tuxtype">"Tuxtype"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/unreal">"Unreal"</a>
                        </li>
                    </ul>
                </article>
                <article class="service-card">
                    <p class="pillar-mark">"S — SOUTH"</p>
                    <h2>"Deployment"</h2>
                    <p>
                        "Quality deployment solutions — dotfiles, configuration, and "
                        "release tooling that carry a project from first checkout to "
                        "production."
                    </p>
                    <ul class="service-links">
                        <li>
                            <a href="https://github.com/qompassai/Azimuth">"Azimuth"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/current">"Current"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/dotfiles">"Dotfiles"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/qonfig">"Qonfig"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/qudaz">"Qudaz"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/qxet">"Qxet"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/rose">"Rose"</a>
                        </li>
                    </ul>
                </article>
                <article class="service-card">
                    <p class="pillar-mark">"W — WEST"</p>
                    <h2>"Servers"</h2>
                    <p>
                        "Efficient server solutions — Nix packaging, a signed binary "
                        "cache, and the network plumbing the rest of the fleet runs on."
                    </p>
                    <ul class="service-links">
                        <li>
                            <a href="https://github.com/qompassai/beacon">"beacon"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/bunker">"bunker"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/debian">"debian"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/network">"network"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/NUR">"NUR"</a>
                        </li>
                        <li>
                            <a href="https://github.com/qompassai/qjp">"qjp"</a>
                        </li>
                    </ul>
                </article>
            </div>
        </section>
        <Quickstart />
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
