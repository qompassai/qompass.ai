use crate::pages::compass_services::CardinalCards;
use crate::pages::project::DownloadBoxes;
use leptos::prelude::*;
use leptos_meta::Title;

/// Link to a project's study page, rendered only when a ready study page
/// exists for the slug — the rollout mechanism. No stub pages, no dead
/// links: the link appears exactly when the page does.
#[component]
fn StudyLink(slug: &'static str) -> impl IntoView {
    crate::project_data::project_page(slug).map(|_| {
        view! {
            <p class="study-link">
                <a href=format!("/solutions/{slug}")>"Read the study →"</a>
            </p>
        }
    })
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="Solutions - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — THE FLEET"</p>
            <h1>"Solutions"</h1>
            <p class="page-lede">
                "Open source, Apache-2.0, built in the open at "
                <a href="https://github.com/qompassai">"github.com/qompassai"</a>
                ". Every vessel below is in active service — this website runs on three of them."
            </p>
            <div class="entry-list">
                <article class="entry">
                    <img class="project-mark" src="/img/projects/vongola.webp" alt="vongola" />
                    <p class="entry-kind">"REVERSE PROXY — RUST"</p>
                    <h2>
                        <a href="https://github.com/qompassai/vongola">"vongola"</a>
                    </h2>
                    <p>
                        "Post-quantum hybrid TLS reverse proxy on Pingora, with built-in ACME. "
                        "It serves this site."
                    </p>
                    <DownloadBoxes slug="vongola" />
                    <StudyLink slug="vongola" />
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/volta.webp" alt="volta" />
                    <p class="entry-kind">"OPENPGP KEY SERVER — RUST"</p>
                    <h2>
                        <a href="https://github.com/qompassai/volta">"volta"</a>
                    </h2>
                    <p>
                        "A Hagrid-derived key server — HKP, VKS, and Web Key Directory — with "
                        "post-quantum ambitions: ML-KEM/ML-DSA throughout, WebAuthn-class "
                        "authentication, no deprecated algorithms. Live at keys.qompass.ai."
                    </p>
                    <DownloadBoxes slug="volta" />
                    <StudyLink slug="volta" />
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/bunker.webp" alt="bunker" />
                    <p class="entry-kind">"NIX BINARY CACHE — RUST"</p>
                    <h2>
                        <a href="https://github.com/qompassai/bunker">"bunker"</a>
                    </h2>
                    <p>
                        "An Attic-based Nix binary cache with hybrid Ed25519 + ML-DSA-65 "
                        "narinfo signing — packages signed for the post-quantum era, served "
                        "from cache.qompass.ai."
                    </p>
                    <DownloadBoxes slug="bunker" />
                    <StudyLink slug="bunker" />
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/ghost.webp" alt="ghost" />
                    <p class="entry-kind">"PGP MAIL CLIENT — ANDROID"</p>
                    <h2>"ghost"</h2>
                    <p>
                        "Encrypted mail the way it should feel: OpenPGP compose with a real "
                        "key picker, Tor transport, and key lookup against volta. Heading to "
                        "F-Droid."
                    </p>
                    <DownloadBoxes slug="ghost" />
                    <StudyLink slug="ghost" />
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/diver.webp" alt="diver" />
                    <p class="entry-kind">"NEOVIM ENVIRONMENT — LUA"</p>
                    <h2>
                        <a href="https://github.com/qompassai/diver">"diver"</a>
                    </h2>
                    <p>
                        "A Neovim-native development environment: 138 formatter adapters, deep "
                        "LSP coverage, in-editor AI — the cockpit the rest of the fleet is "
                        "built from."
                    </p>
                    <DownloadBoxes slug="diver" />
                    <StudyLink slug="diver" />
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/phlow.webp" alt="phlow" />
                    <p class="entry-kind">"AGENT RUNTIME — RUST"</p>
                    <h2>
                        <a href="https://github.com/qompassai/phlow">"phlow"</a>
                    </h2>
                    <p>
                        "A bounded planner-coder-reviewer agent runtime: one safe core shared "
                        "by CLI, TUI, and MCP, with operator-approved checks and an "
                        "ATLAS-mapped adversarial test program."
                    </p>
                    <DownloadBoxes slug="phlow" />
                    <StudyLink slug="phlow" />
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/light-show.webp" alt="light-show" />
                    <p class="entry-kind">"GAME — RUST / BEVY"</p>
                    <h2>
                        <a href="https://github.com/qompassai/light-show">"light-show"</a>
                    </h2>
                    <p>
                        "A fiber-optic cabling puzzle game — route the light, validate the "
                        "signal. Coming to Google Play, F-Droid, and Windows."
                    </p>
                    <DownloadBoxes slug="light-show" />
                    <StudyLink slug="light-show" />
                    <p class="study-link">
                        <a href="/light-show">"Visit the game →"</a>
                    </p>
                </article>
                <article class="entry">
                    <img class="project-mark" src="/img/projects/fathom.webp" alt="fathom" />
                    <p class="entry-kind">"REVERSE ENGINEERING — RUST"</p>
                    <h2>
                        <a href="https://github.com/qompassai/fathom">"fathom"</a>
                    </h2>
                    <p>
                        "A Rust port of the REA MCP reverse-engineering server — "
                        "depth-sounding for binaries."
                    </p>
                    <DownloadBoxes slug="fathom" />
                    <StudyLink slug="fathom" />
                </article>
            </div>
            <CardinalCards />
        </section>
    }
}
