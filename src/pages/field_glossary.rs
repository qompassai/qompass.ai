//! The Field Notes glossary: terms and equations adapted from Matt's
//! study repository (see `content/field-notes/` and
//! `scripts/generate-field-notes-equations.cjs`).
//!
//! Each equation is a build-time SVG (no math runtime, nothing for the
//! CSP to allow), followed by a one-line plain-language reading and a
//! legend that defines every symbol in it — the generator refuses to
//! write an equation with an undefined symbol. The glyph paths the
//! equations `<use>` live once, in [`EQUATION_DEFS`].
//!
//! Every term is server-rendered; the search box is an enhancement that
//! appears after hydration and only toggles `hidden` on the entries.
use crate::field_notes_data::{equation_count, GlossaryTerm, EQUATION_DEFS, GLOSSARY};
use leptos::prelude::*;

/// Does `term` match the lowercase `query` (by name or definition)?
fn matches(term: &GlossaryTerm, query: &str) -> bool {
    query.is_empty() || term.term.to_lowercase().contains(query) || term.definition.to_lowercase().contains(query)
}

fn term_entry(term: &'static GlossaryTerm, query: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="fn-term" id=term.slug hidden=move || query.with(|q| !matches(term, q))>
            <dt>
                <a class="fn-term-name" href=format!("#{}", term.slug)>
                    {term.term}
                </a>
            </dt>
            <dd>
                <p class="fn-term-def">{term.definition}</p>
                {term
                    .equations
                    .iter()
                    .map(|e| {
                        view! {
                            <figure class="fn-eq" id=e.id>
                                <div class="fn-eq-math" inner_html=e.svg></div>
                                <figcaption>
                                    <p class="fn-eq-plain">{e.plain}</p>
                                    {(!e.legend.is_empty())
                                        .then(|| {
                                            view! {
                                                <dl class="fn-eq-legend">
                                                    {e
                                                        .legend
                                                        .iter()
                                                        .map(|&(label, meaning)| {
                                                            view! {
                                                                <div>
                                                                    <dt inner_html=label></dt>
                                                                    <dd>{meaning}</dd>
                                                                </div>
                                                            }
                                                        })
                                                        .collect_view()}
                                                </dl>
                                            }
                                        })}
                                </figcaption>
                            </figure>
                        }
                    })
                    .collect_view()}
            </dd>
        </div>
    }
}

#[component]
pub fn FieldGlossary() -> impl IntoView {
    let query = RwSignal::new(String::new());
    let enhanced = RwSignal::new(false);
    Effect::new(move |_| enhanced.set(true));
    let shown = move || query.with(|q| GLOSSARY.iter().filter(|t| matches(t, q)).count());
    view! {
        <section class="study-section fn-glossary" id="glossary">
            <h2 class="band-label">"GLOSSARY"</h2>
            <p>
                {format!(
                    concat!(
                        "{} terms and {} equations from my own study notes — fiber and PON, ",
                        "electrical theory for the telecom administrator exam, the NEC articles that ",
                        "govern communications wiring, and the quantum physics I keep next to them. ",
                        "Every equation has a legend: each symbol, and its unit where it has one.",
                    ),
                    GLOSSARY.len(),
                    equation_count(),
                )}
            </p>
            <div class="fn-gloss-search" hidden=move || !enhanced.get()>
                <label class="band-label" for="fn-gloss-q">
                    "Search the glossary"
                </label>
                <input
                    id="fn-gloss-q"
                    class="fn-input"
                    type="search"
                    autocomplete="off"
                    placeholder="dB, splitter, NEC 770, qubit…"
                    prop:value=move || query.get()
                    on:input=move |ev| query.set(event_target_value(&ev).trim().to_lowercase())
                />
                <p class="fn-gloss-count" aria-live="polite">
                    {move || format!("Showing {} of {} terms", shown(), GLOSSARY.len())}
                </p>
            </div>
            <div class="fn-eq-defs" aria-hidden="true" inner_html=EQUATION_DEFS></div>
            <dl class="fn-gloss">{GLOSSARY.iter().map(|t| term_entry(t, query)).collect_view()}</dl>
        </section>
    }
}
