//! Tiger Style guides, compiled in by `build.rs`.
//!
//! Sources live in `content/tiger-style/` (see that directory's
//! `index.toml`): Matt's own guides, copied verbatim from
//! `~/workspace/skills/tiger-style-*/` (frontmatter stripped, repo-
//! relative links adapted); `build.rs` renders them to HTML with
//! pulldown-cmark at build time.

include!(concat!(env!("OUT_DIR"), "/tiger_style_data.rs"));

/// The guide chapter for a language slug, if one was imported.
pub fn tiger_guide(slug: &str) -> Option<&'static TigerGuide> {
    TIGER_GUIDES.iter().find(|g| g.slug == slug)
}

/// The full reference for a language slug, if one was imported.
pub fn tiger_reference(slug: &str) -> Option<&'static TigerReference> {
    TIGER_REFERENCES.iter().find(|r| r.slug == slug)
}
