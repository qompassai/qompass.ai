use crate::pages::{
    about::About,
    fiber_trainer::FiberTrainer,
    field_service::{FieldNoteDispatch, FieldService},
    home::Home,
    legal::Legal,
    light_show::{LightShow, LightShowTesters},
    on_ai::OnAi,
    ontrack_privacy::OnTrackPrivacy,
    privacy::Privacy,
    project::ProjectDispatch,
    projects::Projects,
    research::Research,
    tiger_style::{TigerStyle, TigerStyleDispatch},
};
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
                    <A href="/solutions">"Solutions"</A>
                    <A href="/on-ai">"On AI"</A>
                    <A href="/standards">"Standards"</A>
                    <A href="/field-service">"Field Service"</A>
                    <A href="/about">"About"</A>
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
                <p class="footer-social">
                    <a
                        class="social-link"
                        href="https://orcid.org/0000-0002-0302-4812"
                        rel="me"
                        aria-label="ORCID"
                    >
                        <svg
                            class="social-icon icon-orcid"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path d="M12 0C5.372 0 0 5.372 0 12s5.372 12 12 12 12-5.372 12-12S18.628 0 12 0zM7.369 4.378c.525 0 .947.431.947.947s-.422.947-.947.947a.95.95 0 0 1-.947-.947c0-.525.422-.947.947-.947zm-.722 3.038h1.444v10.041H6.647V7.416zm3.562 0h3.9c3.712 0 5.344 2.653 5.344 5.025 0 2.578-2.016 5.025-5.325 5.025h-3.919V7.416zm1.444 1.303v7.444h2.297c3.272 0 4.022-2.484 4.022-3.722 0-2.016-1.284-3.722-4.097-3.722h-2.222z" />
                        </svg>
                    </a>
                    <a
                        class="social-link"
                        href="https://github.com/qompassai"
                        rel="me"
                        aria-label="GitHub"
                    >
                        <svg
                            class="social-icon icon-github"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" />
                        </svg>
                    </a>
                    <a
                        class="social-link"
                        href="https://huggingface.co/qompass"
                        rel="me"
                        aria-label="Hugging Face"
                    >
                        <svg
                            class="social-icon icon-huggingface"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path d="M12.025 1.13c-5.77 0-10.449 4.647-10.449 10.378 0 1.112.178 2.181.503 3.185.064-.222.203-.444.416-.577a.96.96 0 0 1 .524-.15c.293 0 .584.124.84.284.278.173.48.408.71.694.226.282.458.611.684.951v-.014c.017-.324.106-.622.264-.874s.403-.487.762-.543c.3-.047.596.06.787.203s.31.313.4.467c.15.257.212.468.233.542.01.026.653 1.552 1.657 2.54.616.605 1.01 1.223 1.082 1.912.055.537-.096 1.059-.38 1.572.637.121 1.294.187 1.967.187.657 0 1.298-.063 1.921-.178-.287-.517-.44-1.041-.384-1.581.07-.69.465-1.307 1.081-1.913 1.004-.987 1.647-2.513 1.657-2.539.021-.074.083-.285.233-.542.09-.154.208-.323.4-.467a1.08 1.08 0 0 1 .787-.203c.359.056.604.29.762.543s.247.55.265.874v.015c.225-.34.457-.67.683-.952.23-.286.432-.52.71-.694.257-.16.547-.284.84-.285a.97.97 0 0 1 .524.151c.228.143.373.388.43.625l.006.04a10.3 10.3 0 0 0 .534-3.273c0-5.731-4.678-10.378-10.449-10.378M8.327 6.583a1.5 1.5 0 0 1 .713.174 1.487 1.487 0 0 1 .617 2.013c-.183.343-.762-.214-1.102-.094-.38.134-.532.914-.917.71a1.487 1.487 0 0 1 .69-2.803m7.486 0a1.487 1.487 0 0 1 .689 2.803c-.385.204-.536-.576-.916-.71-.34-.12-.92.437-1.103.094a1.487 1.487 0 0 1 .617-2.013 1.5 1.5 0 0 1 .713-.174m-10.68 1.55a.96.96 0 1 1 0 1.921.96.96 0 0 1 0-1.92m13.838 0a.96.96 0 1 1 0 1.92.96.96 0 0 1 0-1.92M8.489 11.458c.588.01 1.965 1.157 3.572 1.164 1.607-.007 2.984-1.155 3.572-1.164.196-.003.305.12.305.454 0 .886-.424 2.328-1.563 3.202-.22-.756-1.396-1.366-1.63-1.32q-.011.001-.02.006l-.044.026-.01.008-.03.024q-.018.017-.035.036l-.032.04a1 1 0 0 0-.058.09l-.014.025q-.049.088-.11.19a1 1 0 0 1-.083.116 1.2 1.2 0 0 1-.173.18q-.035.029-.075.058a1.3 1.3 0 0 1-.251-.243 1 1 0 0 1-.076-.107c-.124-.193-.177-.363-.337-.444-.034-.016-.104-.008-.2.022q-.094.03-.216.087-.06.028-.125.063l-.13.074q-.067.04-.136.086a3 3 0 0 0-.135.096 3 3 0 0 0-.26.219 2 2 0 0 0-.12.121 2 2 0 0 0-.106.128l-.002.002a2 2 0 0 0-.09.132l-.001.001a1.2 1.2 0 0 0-.105.212q-.013.036-.024.073c-1.139-.875-1.563-2.317-1.563-3.203 0-.334.109-.457.305-.454m.836 10.354c.824-1.19.766-2.082-.365-3.194-1.13-1.112-1.789-2.738-1.789-2.738s-.246-.945-.806-.858-.97 1.499.202 2.362c1.173.864-.233 1.45-.685.64-.45-.812-1.683-2.896-2.322-3.295s-1.089-.175-.938.647 2.822 2.813 2.562 3.244-1.176-.506-1.176-.506-2.866-2.567-3.49-1.898.473 1.23 2.037 2.16c1.564.932 1.686 1.178 1.464 1.53s-3.675-2.511-4-1.297c-.323 1.214 3.524 1.567 3.287 2.405-.238.839-2.71-1.587-3.216-.642-.506.946 3.49 2.056 3.522 2.064 1.29.33 4.568 1.028 5.713-.624m5.349 0c-.824-1.19-.766-2.082.365-3.194 1.13-1.112 1.789-2.738 1.789-2.738s.246-.945.806-.858.97 1.499-.202 2.362c-1.173.864.233 1.45.685.64.451-.812 1.683-2.896 2.322-3.295s1.089-.175.938.647-2.822 2.813-2.562 3.244 1.176-.506 1.176-.506 2.866-2.567 3.49-1.898-.473 1.23-2.037 2.16c-1.564.932-1.686 1.178-1.464 1.53s3.675-2.511 4-1.297c.323 1.214-3.524 1.567-3.287 2.405.238.839 2.71-1.587 3.216-.642.506.946-3.49 2.056-3.522 2.064-1.29.33-4.568 1.028-5.713-.624" />
                        </svg>
                    </a>
                    <a
                        class="social-link"
                        href="https://www.youtube.com/@QompassAI"
                        rel="me"
                        aria-label="YouTube"
                    >
                        <svg
                            class="social-icon icon-youtube"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path d="M23.498 6.186a3.016 3.016 0 0 0-2.122-2.136C19.505 3.545 12 3.545 12 3.545s-7.505 0-9.377.505A3.017 3.017 0 0 0 .502 6.186C0 8.07 0 12 0 12s0 3.93.502 5.814a3.016 3.016 0 0 0 2.122 2.136c1.871.505 9.376.505 9.376.505s7.505 0 9.377-.505a3.015 3.015 0 0 0 2.122-2.136C24 15.93 24 12 24 12s0-3.93-.502-5.814zM9.545 15.568V8.432L15.818 12l-6.273 3.568z" />
                        </svg>
                    </a>
                    <a
                        class="social-link"
                        href="https://opencollective.com/qompassai"
                        rel="me"
                        aria-label="Open Collective"
                    >
                        <svg
                            class="social-icon icon-opencollective"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            aria-hidden="true"
                        >
                            <path d="M12 0C5.373 0 0 5.373 0 12s5.373 12 12 12c2.54 0 4.894-.79 6.834-2.135l-3.107-3.109a7.715 7.715 0 1 1 0-13.512l3.107-3.109A11.943 11.943 0 0 0 12 0zm9.865 5.166l-3.109 3.107A7.67 7.67 0 0 1 19.715 12a7.682 7.682 0 0 1-.959 3.727l3.109 3.107A11.943 11.943 0 0 0 24 12c0-2.54-.79-4.894-2.135-6.834z" />
                        </svg>
                    </a>
                </p>
                <p class="footer-note">
                    <A href="/about#identity">"Contact"</A>
                    " · "
                    <A href="/privacy">"Privacy"</A>
                    " · "
                    <A href="/legal">"Legal"</A>
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
                    <Route path=path!("/privacy") view=Privacy />
                    <Route path=path!("/ontrack/privacy") view=OnTrackPrivacy />
                    <Route path=path!("/legal") view=Legal />
                    <Route path=path!("/solutions") view=Projects />
                    <Route path=path!("/light-show") view=LightShow />
                    <Route path=path!("/light-show/testers") view=LightShowTesters />
                    <Route path=path!("/field-service") view=FieldService />
                    <Route path=path!("/field-service/trainer") view=FiberTrainer />
                    <Route path=path!("/field-service/notes/:slug") view=FieldNoteDispatch />
                    <Route path=path!("/on-ai") view=OnAi />
                    <Route path=path!("/standards") view=TigerStyle />
                    <Route path=path!("/standards/:slug") view=TigerStyleDispatch />
                    <Route path=path!("/solutions/:slug") view=ProjectDispatch />
                    <Route path=path!("/research") view=Research />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
