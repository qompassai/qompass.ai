use crate::pages::identity::IdentitySection;
use leptos::prelude::*;
use leptos_meta::Title;

include!(concat!(env!("OUT_DIR"), "/cv_data.rs"));

// Matt's record folded in from the retired /cv page: the data
// still comes from content/cv.toml (compiled in by build.rs) — this
// page renders every section, the Identifiers through the identity
// section (src/pages/identity.rs, anchor #identity) that absorbed the
// retired /contact page on 2026-10-13.
#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About - Qompass AI" />
        <section class="page">
            <p class="page-bearing">"CHART — THE WORK"</p>
            <h1>"About"</h1>
            <div class="prose">
                <p>
                    "The name is the mission. A compass does not move the ship — it tells the "
                    "truth about where the ship is, in any weather. Under the name Qompass AI, "
                    "I build truthful instruments for the quantum transition: benchmarks that "
                    "measure post-quantum cryptography instead of marketing it, infrastructure "
                    "that refuses deprecated algorithms by design, and AI systems held to the "
                    "oldest safety standard in medicine — first, do no harm."
                </p>
                <p>
                    "My route to deep tech ran through two other professions that "
                    "punish sloppy evidence: military intelligence — a decade on active duty, "
                    "from a tactical cell on the Korean DMZ to brigade intelligence director, "
                    "decorated with the Bronze Star — and medicine, with years of study at "
                    "WSU's Elson S. Floyd College of Medicine, orthopaedic publications, and "
                    "journal peer review continuing alongside the engineering. The red "
                    "teamer's habit runs through all of it: threat-model first, publish the "
                    "negative results, and let the measurements argue. My full record is "
                    "set out below."
                </p>
                <p>
                    "Everything I ship is open source under Apache-2.0, archived with DOIs, "
                    "and — as of this site — served by my own stack: vongola at the edge, "
                    "volta for keys, bunker for packages."
                </p>
            </div>

            <IdentitySection />

            <p class="page-bearing">"CHART — SERVICE RECORD"</p>
            <h2 class="cv-head">{CV_PROFILE.name}</h2>
            <p class="cv-title">{CV_PROFILE.title}</p>
            <p class="cv-meta">{CV_PROFILE.location}</p>
            <p class="page-lede">{CV_PROFILE.summary}</p>
            <p class="cv-download">
                <a href="/files/matt-porter-cv.pdf" rel="external">"Download CV (PDF)"</a>
            </p>

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
        </section>
    }
}
