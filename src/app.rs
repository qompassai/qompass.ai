use crate::pages::{about::About, contact::Contact, cv::Cv, home::Home, projects::Projects, research::Research};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta
                    name="description"
                    content="Qompass AI - navigating deep tech's murky waters. Post-quantum systems, agent runtimes, and deep-tech research."
                />
                <Stylesheet id="leptos" href="/pkg/qompass-ai.css" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="header-inner">
                <A href="/" attr:class="brand">
                    <svg class="brand-mark" viewBox="0 0 48 48" aria-hidden="true">
                        <circle cx="24" cy="24" r="21" class="mark-ring" />
                        <circle cx="24" cy="24" r="13.5" class="mark-orbit" />
                        <polygon points="24,4 27,21 24,24 21,21" class="mark-north" />
                        <polygon points="44,24 27,27 24,24 27,21" class="mark-point" />
                        <polygon points="24,44 21,27 24,24 27,27" class="mark-point" />
                        <polygon points="4,24 21,21 24,24 21,27" class="mark-point" />
                        <circle cx="24" cy="24" r="2.4" class="mark-core" />
                    </svg>
                    <span class="brand-word">
                        "QOMPASS " <span class="brand-ai">"AI"</span>
                    </span>
                </A>
                <nav class="site-nav" aria-label="Primary">
                    <A href="/research">"Research"</A>
                    <A href="/projects">"Projects"</A>
                    <A href="/cv">"CV"</A>
                    <A href="/about">"About"</A>
                    <A href="/contact">"Contact"</A>
                </nav>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer-inner">
                <p class="footer-line">"© 2026 Qompass AI. All rights reserved."</p>
                <p class="footer-links">
                    <a href="https://orcid.org/0000-0002-0302-4812" rel="me">
                        "ORCID"
                    </a>
                    <a href="https://github.com/qompassai" rel="me">
                        "GitHub"
                    </a>
                    <a href="https://huggingface.co/qompass" rel="me">
                        "Hugging Face"
                    </a>
                </p>
                <p class="footer-note">
                    "Built with Leptos · Served by vongola · Post-quantum by design"
                </p>
            </div>
        </footer>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="Qompass AI" />
        <Router>
            <Header />
            <main>
                <Routes fallback=|| {
                    view! {
                        <section class="page">
                            <h1>"Off chart"</h1>
                            <p>"That bearing does not exist on this chart."</p>
                        </section>
                    }
                }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <Route path=path!("/contact") view=Contact />
                    <Route path=path!("/cv") view=Cv />
                    <Route path=path!("/projects") view=Projects />
                    <Route path=path!("/research") view=Research />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
