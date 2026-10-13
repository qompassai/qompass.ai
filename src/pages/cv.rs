use leptos::prelude::*;
use leptos_meta::Title;

include!(concat!(env!("OUT_DIR"), "/cv_data.rs"));

// The CV is data, not markup: everything on this page comes from
// content/cv.toml (compiled in by build.rs). Edit the TOML, rebuild, done.
// The print stylesheet turns this page into the PDF — no separate pipeline.
#[component]
pub fn Cv() -> impl IntoView {
    view! {
        <Title text="CV - Matthew A. Porter - Qompass AI" />
        <section class="page cv">
            <p class="page-bearing">"CHART — SERVICE RECORD"</p>
            <h1>{CV_PROFILE.name}</h1>
            <p class="cv-title">{CV_PROFILE.title}</p>
            <p class="cv-meta">
                {CV_PROFILE.location} " · " <a href="mailto:map@qompass.ai">{CV_PROFILE.email}</a>
            </p>
            <p class="page-lede">{CV_PROFILE.summary}</p>

            <h2 class="cv-head">"Experience"</h2>
            {CV_EXPERIENCE
                .iter()
                .map(|r| {
                    view! {
                        <article class="cv-entry">
                            <div class="cv-row">
                                <h3>{r.role}</h3>
                                <span class="cv-period">{r.period}</span>
                            </div>
                            <p class="cv-org">{r.org}</p>
                            <ul class="cv-bullets">
                                {r
                                    .bullets
                                    .iter()
                                    .map(|b| view! { <li>{*b}</li> })
                                    .collect_view()}
                            </ul>
                        </article>
                    }
                })
                .collect_view()}

            <h2 class="cv-head">"Education"</h2>
            {CV_EDUCATION
                .iter()
                .map(|i| {
                    view! {
                        <article class="cv-entry">
                            <div class="cv-row">
                                <h3>{i.item}</h3>
                                <span class="cv-period">{i.period}</span>
                            </div>
                            <p class="cv-org">{i.org}</p>
                            <p>{i.detail}</p>
                        </article>
                    }
                })
                .collect_view()}

            <h2 class="cv-head">"Certifications & Licenses"</h2>
            {CV_CERTIFICATIONS
                .iter()
                .map(|g| {
                    view! {
                        <p class="cv-skills">
                            <span class="cv-skill-label">{g.label} " — "</span>
                            {g.items.join(", ")}
                        </p>
                    }
                })
                .collect_view()}

            <h2 class="cv-head">"Awards & Decorations"</h2>
            {CV_AWARDS
                .iter()
                .map(|g| {
                    view! {
                        <p class="cv-skills">
                            <span class="cv-skill-label">{g.label} " — "</span>
                            {g.items.join(", ")}
                        </p>
                    }
                })
                .collect_view()}

            <h2 class="cv-head">"Coverage"</h2>
            <div class="press-grid">
                {CV_PRESS
                    .iter()
                    .map(|p| {
                        view! {
                            <article class="press-card">
                                <p class="press-meta">
                                    <span class="press-date">{p.date}</span>
                                    <span class="press-outlet">{p.outlet}</span>
                                </p>
                                <h3 class="press-title">
                                    <a href={p.url} target="_blank" rel="noopener noreferrer">
                                        {p.title}
                                    </a>
                                </h3>
                                <p class="press-note">{p.note}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>

            <h2 class="cv-head">"Service & Review"</h2>
            {CV_SERVICE
                .iter()
                .map(|i| {
                    view! {
                        <article class="cv-entry">
                            <h3>{i.item}</h3>
                            <p>{i.detail}</p>
                        </article>
                    }
                })
                .collect_view()}

            <h2 class="cv-head">"Soundings — Skills"</h2>
            {CV_SKILLS
                .iter()
                .map(|g| {
                    view! {
                        <p class="cv-skills">
                            <span class="cv-skill-label">{g.label} " — "</span>
                            {g.items.join(", ")}
                        </p>
                    }
                })
                .collect_view()}

            <h2 class="cv-head">"Identifiers"</h2>
            <ul class="cv-ids">
                {CV_IDENTIFIERS
                    .iter()
                    .map(|i| {
                        view! {
                            <li>
                                <span class="cv-id-label">{i.label} ": "</span>
                                {if i.url.is_empty() {
                                    view! { <span>{i.value}</span> }.into_any()
                                } else {
                                    view! {
                                        <a href={i.url}>{i.value}</a>
                                    }
                                        .into_any()
                                }}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <h2 class="cv-head">"Off Watch"</h2>
            <p>{CV_PERSONAL_LINE}</p>
            {CV_CODA.as_ref().map(|c| {
                view! {
                    <div class="cv-coda">
                        <p class="page-bearing">{c.eyebrow}</p>
                        <figure class="phaedrus-frame">
                            <span class="phaedrus-ring" aria-hidden="true"></span>
                            <span class="phaedrus-pulse" aria-hidden="true"></span>
                            <img class="phaedrus-img" src={c.image} alt={c.alt} loading="lazy" />
                            <svg
                                class="phaedrus-route"
                                viewBox="0 0 600 24"
                                preserveAspectRatio="none"
                                aria-hidden="true"
                            >
                                <path
                                    pathLength="100"
                                    d="M4 14 C 120 4, 240 22, 360 12 S 560 10, 596 14"
                                ></path>
                            </svg>
                            <figcaption class="phaedrus-caption">
                                <span class="phaedrus-name">{c.caption}</span>
                                <span class="phaedrus-note">{c.note}</span>
                            </figcaption>
                        </figure>
                    </div>
                }
            })}
            <p class="cv-print-note">"Print this page (or save as PDF) for the paper copy — it is styled to travel."</p>
        </section>
    }
}
