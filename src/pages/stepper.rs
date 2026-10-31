//! Animated step-by-step explainer for solution pages (Fleet 1.0,
//! 2026-10-31).
//!
//! One reusable component, data-driven: the steps for a solution are
//! assembled at render time from that solution's existing site copy
//! only — its compass blurb, its guide summary and `how` steps where
//! a guide exists, its MCP roster detail where one exists, its
//! registered diagram caption where one exists, its cardinal brief,
//! and its recorded source/version. No step claims a capability the
//! page did not already claim; where the copy is thin the explainer
//! simply has fewer steps (always 3–6, pinned by the tests below).
//!
//! The mechanism matches the guide pages' animated diagrams: the
//! server renders the complete explainer (every panel, readable with
//! JavaScript off — the first panel visible, the rest stacked), and
//! the vendored `/js/stepper.js` enhances it in the browser: a range
//! slider scrubs the steps, the step markers and the step list are
//! clickable, and ArrowLeft/ArrowRight move between steps only while
//! the explainer itself has focus (the key handler is bound to the
//! component element, so page keys are never hijacked). Transitions
//! between panels are CSS opacity/transform animations, disabled
//! under `prefers-reduced-motion`.

use crate::app_data::{app_guide, AppGuide};
use crate::pages::compass_services::{Solution, CARDINALS, MCP_SERVERS};
use crate::pages::diagrams::{diagram_caption, page_diagram};
use leptos::prelude::*;

/// One step of an explainer: a stage title and the copy that fills it.
pub struct Step {
    pub title: String,
    pub body: String,
}

fn push(steps: &mut Vec<Step>, title: &str, body: &str) {
    let body = body.trim();
    if !body.is_empty() && steps.len() < 6 {
        steps.push(Step { title: title.to_owned(), body: body.to_owned() });
    }
}

/// The MCP roster detail for a solution, where the solution is one of
/// the rostered servers (the phlow MCP server's roster name is `phlow`).
fn mcp_detail(solution: &Solution) -> Option<&'static str> {
    let key = if solution.slug == "phlow-mcp-server" { "phlow" } else { solution.slug };
    MCP_SERVERS.iter().find(|s| s.name == key).map(|s| s.detail)
}

/// The closing step: where the solution lives, from its recorded
/// href and version — nothing more than the page already links.
fn where_it_lives(solution: &Solution) -> String {
    solution.version.map_or_else(
        || format!("Find it at {} — this page links its source from the header above.", solution.href),
        |v| format!("Published as version {v}. Find it at {} — this page links its source and its release from the header above.", solution.href),
    )
}

/// Steps for one compass solution, assembled from its existing copy.
///
/// Order follows the explainer's arc: what it is → how it works →
/// the pipeline in its own words → where it lives.
pub fn solution_steps(solution: &Solution) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    if let Some(guide) = app_guide(solution.slug) {
        push(&mut steps, "What it is", guide.summary);
        // Reserve one slot for the closing step: at most four of the
        // guide's own how-steps ride the explainer; the full list
        // stays in the HOW IT WORKS flow diagram below it.
        for (i, how) in guide.how.iter().take(4).enumerate() {
            push(&mut steps, &format!("How it works — {}", i + 1), how);
        }
    } else {
        push(&mut steps, "What it is", solution.blurb);
        if let Some(detail) = mcp_detail(solution) {
            push(&mut steps, "How it works", detail);
        }
        if let Some(id) = page_diagram(solution.slug) {
            push(&mut steps, "The pipeline", diagram_caption(id));
        }
        if steps.len() < 3
            && let Some(cardinal) = CARDINALS.get(solution.cardinal)
        {
            push(&mut steps, "Where it sits", cardinal.brief);
        }
        if steps.len() < 3 {
            let refs = crate::pages::compass_services::solution_references(solution.slug);
            if let Some((_, _, note)) = refs.first() {
                push(&mut steps, "The pipeline", note);
            }
        }
    }
    push(&mut steps, "Where it lives", &where_it_lives(solution));
    steps
}

/// Steps for a guide page whose slug is not a compass solution (the
/// phlow study/guide, Ghost, Light Show, Volta): the guide's own
/// summary, its how-steps, and its repository.
pub fn guide_steps(guide: &'static AppGuide) -> Vec<Step> {
    let mut steps: Vec<Step> = Vec::new();
    push(&mut steps, "What it is", guide.summary);
    for (i, how) in guide.how.iter().take(4).enumerate() {
        push(&mut steps, &format!("How it works — {}", i + 1), how);
    }
    if !guide.repo_url.is_empty() {
        push(
            &mut steps,
            "Where it lives",
            &format!("Source: {} — this page links it from the header above.", guide.repo_url),
        );
    }
    steps
}

/// The explainer for one compass solution.
#[component]
pub fn SolutionStepper(solution: &'static Solution) -> impl IntoView {
    let steps = solution_steps(solution);
    view! { <Stepper name=solution.name steps=steps /> }
}

/// The explainer for one app guide.
#[component]
pub fn GuideStepper(guide: &'static AppGuide) -> impl IntoView {
    let steps = guide_steps(guide);
    view! { <Stepper name=guide.name steps=steps /> }
}

/// The shared explainer markup. `data-stepper` is the hook the
/// vendored script enhances; without JavaScript every panel is
/// present in the page and the first is shown.
#[component]
fn Stepper(name: &'static str, steps: Vec<Step>) -> impl IntoView {
    let total = steps.len();
    view! {
        <section class="stepper" data-stepper tabindex="0" aria-label=format!("{name}: step by step")>
            <div class="stepper-head">
                <h2>"Step by step"</h2>
                <span class="stepper-count" data-stepper-count>
                    {format!("Step 1 of {total}")}
                </span>
            </div>
            <div class="stepper-stage">
                {steps
                    .iter()
                    .enumerate()
                    .map(|(i, step)| {
                        view! {
                            <article
                                class=if i == 0 { "stepper-panel is-active" } else { "stepper-panel" }
                                data-stepper-panel
                                aria-hidden=if i == 0 { "false" } else { "true" }
                            >
                                <h3 class="stepper-title">{step.title.clone()}</h3>
                                <p class="stepper-body">{step.body.clone()}</p>
                            </article>
                        }
                    })
                    .collect_view()}
            </div>
            <ol class="stepper-markers">
                {steps
                    .iter()
                    .enumerate()
                    .map(|(i, step)| {
                        view! {
                            <li>
                                <button
                                    type="button"
                                    class=if i == 0 { "stepper-marker is-active" } else { "stepper-marker" }
                                    data-stepper-marker
                                    aria-label=format!("Step {}: {}", i + 1, step.title)
                                >
                                    {i + 1}
                                </button>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
            <div class="stepper-controls">
                <button type="button" class="stepper-prev" data-stepper-prev>
                    "← Prev"
                </button>
                <input
                    type="range"
                    class="stepper-range"
                    data-stepper-range
                    min="0"
                    max={total.saturating_sub(1)}
                    step="1"
                    value="0"
                    aria-label="Scrub steps"
                />
                <button type="button" class="stepper-next" data-stepper-next>
                    "Next →"
                </button>
            </div>
            <p class="stepper-hint">
                "Scrub the slider, pick a step, or focus here and use ← →."
            </p>
        </section>
    }
}

#[cfg(test)]
mod tests {
    use super::{guide_steps, solution_steps};
    use crate::app_data::APP_GUIDES;
    use crate::pages::compass_services::SOLUTIONS;

    /// Every compass solution carries an explainer of 3–6 steps, and
    /// no step is empty — the Fleet 1.0 contract for this component.
    #[test]
    fn every_solution_has_three_to_six_steps() {
        for s in &SOLUTIONS {
            let steps = solution_steps(s);
            assert!((3..=6).contains(&steps.len()), "{} has {} steps", s.slug, steps.len());
            for step in &steps {
                assert!(!step.title.is_empty(), "{} has an untitled step", s.slug);
                assert!(!step.body.is_empty(), "{} has an empty step", s.slug);
            }
        }
    }

    /// Guide-only pages (phlow, Ghost, Light Show, Volta) get the
    /// same treatment from their guide copy.
    #[test]
    fn every_guide_has_three_to_six_steps() {
        for g in APP_GUIDES {
            let steps = guide_steps(g);
            assert!((3..=6).contains(&steps.len()), "guide {} has {} steps", g.slug, steps.len());
        }
    }
}
