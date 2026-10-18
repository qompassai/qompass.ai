//! App guide pages, compiled in by `build.rs`.
//!
//! The data comes from `content/apps/*.toml`. Only guides flagged
//! `ready` are emitted: a guide ships by flipping one flag on verified
//! content — never as a stub. A guide carries an app's overview, its
//! how-it-works steps, and its configuration options and controls,
//! every row sourced from the project's own documentation.
include!(concat!(env!("OUT_DIR"), "/app_data.rs"));

/// The ready app guide for `slug`, if one has been published.
pub fn app_guide(slug: &str) -> Option<&'static AppGuide> {
    APP_GUIDES.iter().find(|g| g.slug == slug)
}
