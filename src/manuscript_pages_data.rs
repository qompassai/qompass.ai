//! Manuscript study pages, compiled in by `build.rs`.
//!
//! The data comes from `content/manuscripts/*.toml`. Only pages
//! flagged `ready` are emitted: the router registers a route for
//! exactly these slugs and the /research tracker links exactly these
//! pages, so a study page ships by flipping one flag on verified
//! content — never as a stub. Identity (title, journal, stage) stays
//! with the program rollup snapshot in `crate::manuscript_data`;
//! this module carries the page body only.
include!(concat!(env!("OUT_DIR"), "/manuscript_pages_data.rs"));

/// The ready study page for `slug`, if one has been published.
pub fn manuscript_page(slug: &str) -> Option<&'static ManuscriptPage> {
    MANUSCRIPT_PAGES.iter().find(|p| p.slug == slug)
}
