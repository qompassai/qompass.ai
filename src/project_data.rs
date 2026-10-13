//! Project study pages, compiled in by `build.rs`.
//!
//! The data comes from `content/projects/*.toml`. Only pages flagged
//! `ready` are emitted: the router registers a route for exactly these
//! slugs and the /solutions index links exactly these pages, so a study
//! ships by flipping one flag on verified content — never as a stub.
include!(concat!(env!("OUT_DIR"), "/project_data.rs"));

/// The ready study page for `slug`, if one has been published.
pub fn project_page(slug: &str) -> Option<&'static ProjectPage> {
    PROJECT_PAGES.iter().find(|p| p.slug == slug)
}
