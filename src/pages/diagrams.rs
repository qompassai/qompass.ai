//! Concept diagram registry for solution pages.
//!
//! Each diagram is an inline SVG drawn in the design tokens — the same
//! idiom as `phlow-frame` in `project.rs`: `<svg class="gfx"
//! viewBox="0 0 760 H" role="img" aria-label="Diagram: …">` built only
//! from the `gfx-*` classes (gfx-panel, gfx-head, gfx-text, gfx-note,
//! gfx-bar, gfx-arrow, gfx-pulse, gfx-pulse-y, gfx-badge,
//! gfx-badge-text, gfx-dash, gfx-hot, gfx-chip, gfx-fill-*). Text is
//! set at 10.5px or larger at viewBox scale so figures stay legible at
//! phone width. Motion comes only from the classes the stylesheet
//! animates inside its `prefers-reduced-motion: no-preference` block
//! (gfx-pulse travels +32px in x, gfx-pulse-y +32px in y, gfx-bar
//! grows, gfx-decide breathes); every diagram must still read
//! perfectly as a static figure.
//!
//! Diagrams live in per-batch modules (`b0` = the pilot pages, `b1`…
//! `b5` = the rollout batches, `b6` = the generated pages' shared
//! diagrams, built with the pilots) so authors never edit shared
//! files:
//! a batch module owns its ids end to end — the `IDS` list below and
//! the `diagram` match. The registry delegates in batch order. An
//! unknown id renders nothing: a page ships a diagram only when one
//! is registered here, the rule `ProjectGraphic` set.
//!
//! A diagram here must encode the app's actual architecture, data
//! flow, protocol sequence, state machine, or algorithm — traced from
//! the project's own documentation and source, never decoration and
//! never a generic template with the names swapped.

use leptos::prelude::*;

pub mod b0;
pub mod b1;
pub mod b2;
pub mod b3;
pub mod b4;
pub mod b5;
pub mod b6;

/// The bare `<svg>` for a diagram id (the caller wraps it in the
/// figure + caption), or `None` when no batch module owns the id.
pub fn diagram(id: &'static str) -> Option<AnyView> {
    b0::diagram(id)
        .or_else(|| b1::diagram(id))
        .or_else(|| b2::diagram(id))
        .or_else(|| b3::diagram(id))
        .or_else(|| b4::diagram(id))
        .or_else(|| b5::diagram(id))
        .or_else(|| b6::diagram(id))
}

/// Whether any batch module owns this diagram id. The registry's
/// integrity tests resolve every id a content file names through
/// this, without constructing views.
pub fn known(id: &str) -> bool {
    [b0::IDS, b1::IDS, b2::IDS, b3::IDS, b4::IDS, b5::IDS, b6::IDS].iter().any(|ids| ids.contains(&id))
}

/// The diagram a generated solution page (one with no guide TOML)
///
/// carries, by slug: the four MCP servers share the protocol that is
/// genuinely their common concept, Fathom has its own pipeline, and
/// the ten Diver module rocks share the map of the environment they
/// compose. Any other slug gets no diagram — a diagram there would be
///
/// decoration.
pub fn page_diagram(slug: &str) -> Option<&'static str> {
    match slug {
        "eigenstyle" | "lumen" | "phlow-mcp-server" => Some("mcp-flow"),
        "fathom" => Some("fathom-pipeline"),
        s if s.starts_with("diver-") => Some("diver-modules"),
        _ => None,
    }
}

/// The caption for a registry-owned diagram on a generated page
/// (guide pages carry their captions in their content files).
pub fn diagram_caption(id: &str) -> &'static str {
    match id {
        "mcp-flow" => {
            "The Model Context Protocol session every MCP server on this page speaks: \
             initialize, tools/list, tools/call — JSON-RPC 2.0 envelopes over stdio."
        }
        "fathom-pipeline" => {
            "Fathom's reverse-engineering pipeline: a target binary flows through its \
             provider engines into evidence envelopes, served over MCP and the CLI."
        }
        "diver-modules" => {
            "The Diver module rocks compose one Neovim environment: each rock is one \
             subsystem of the configuration the Diver guide page documents."
        }
        "telco-answer-keys" => {
            "How a set's answer key stays sealed: encrypted per set when the site is \
             built, shipped as ciphertext only, and opened in the candidate's browser — \
             the password is never stored or sent."
        }
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::{known, page_diagram};
    use crate::app_data::APP_GUIDES;
    use crate::project_data::PROJECT_PAGES;

    /// Every diagram a ready guide names must exist in the registry —
    /// a typo'd id would otherwise silently render no figure at all.
    #[test]
    fn guide_diagram_ids_resolve() {
        for guide in APP_GUIDES {
            for visual in guide.visuals {
                if visual.kind == "diagram" {
                    assert!(known(visual.id), "guide {} names unregistered diagram {:?}", guide.slug, visual.id);
                }
            }
        }
    }

    /// Every visual must be a kind the renderer understands, and every
    ///
    /// equation must carry the prose that defines its quantities
    /// (build.rs already enforces this at compile time; the test pins
    /// the contract on the compiled-in data itself).
    #[test]
    fn visuals_are_well_formed() {
        for guide in APP_GUIDES {
            for visual in guide.visuals {
                assert!(
                    ["diagram", "equation", "image", "video"].contains(&visual.kind),
                    "guide {} has visual of unknown kind {:?}",
                    guide.slug,
                    visual.kind
                );
                assert!(!visual.caption.is_empty(), "guide {} has a visual with no caption", guide.slug);
                if visual.kind == "equation" {
                    assert!(
                        !visual.latex.is_empty() && !visual.define.is_empty(),
                        "guide {} has an equation without definitions",
                        guide.slug
                    );
                }
            }
        }
    }

    /// References are the reader's way to check the page: each must be
    /// an https URL with a title, on guides and study pages alike.
    #[test]
    fn references_are_https_with_titles() {
        for guide in APP_GUIDES {
            for r in guide.references {
                assert!(r.url.starts_with("https://"), "guide {} reference {:?} is not https", guide.slug, r.url);
                assert!(!r.title.is_empty(), "guide {} has an untitled reference", guide.slug);
            }
        }
        for page in PROJECT_PAGES {
            for r in page.references {
                assert!(r.url.starts_with("https://"), "study {} reference {:?} is not https", page.slug, r.url);
                assert!(!r.title.is_empty(), "study {} has an untitled reference", page.slug);
            }
        }
    }

    /// The diagrams wired to generated pages must be registered.
    #[test]
    fn solution_media_well_formed() {
        for s in crate::pages::compass_services::SOLUTIONS {
            for (src, alt, caption) in crate::pages::compass_services::solution_media(s.slug) {
                assert!(src.starts_with("/img/"), "{}: media src {src}", s.slug);
                assert!(!alt.is_empty(), "{}: media alt empty", s.slug);
                assert!(!caption.is_empty(), "{}: media caption empty", s.slug);
            }
        }
    }

    #[test]
    fn generated_page_diagrams_resolve() {
        for slug in ["eigenstyle", "fathom", "lumen", "phlow-mcp-server", "diver-ai", "diver-utils"] {
            let id = page_diagram(slug).expect("a diagram is wired for this slug");
            assert!(known(id), "{slug} is wired to unregistered diagram {id:?}");
        }
    }
}
