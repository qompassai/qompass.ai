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

#[cfg(test)]
mod tests {
    use super::{HOSTILE_FIXTURE_HTML, TIGER_GUIDES, TIGER_REFERENCES};

    /// Same active-content rules as the build-time check in build.rs:
    /// no script/iframe tags, no `javascript:` URLs, no `on*=` handler
    /// attributes in tag context.
    fn has_active_content(html: &str) -> bool {
        let lower = html.to_ascii_lowercase();
        if lower.contains("<script") || lower.contains("<iframe") {
            return true;
        }
        for attr in ["href=\"javascript:", "href='javascript:", "src=\"javascript:", "src='javascript:"] {
            if lower.contains(attr) {
                return true;
            }
        }
        lower.split('<').skip(1).map(|segment| segment.split('>').next().unwrap_or_default()).any(tag_has_handler)
    }

    fn tag_has_handler(tag: &str) -> bool {
        let mut cleaned = String::with_capacity(tag.len());
        let mut quote: Option<char> = None;
        for c in tag.chars() {
            match quote {
                Some(q) => {
                    if c == q {
                        quote = None;
                    }
                }
                None => {
                    if c == '"' || c == '\'' {
                        quote = Some(c);
                        cleaned.push(' ');
                    } else {
                        cleaned.push(c);
                    }
                }
            }
        }
        cleaned.split(char::is_whitespace).filter_map(|token| token.split_once('=').map(|(name, _)| name)).any(|name| {
            name.len() > 2 && name.starts_with("on") && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        })
    }

    #[test]
    fn hostile_fixture_renders_inert_and_keeps_legitimate_markup() {
        assert!(!has_active_content(HOSTILE_FIXTURE_HTML), "fixture survived: {HOSTILE_FIXTURE_HTML}");
        for kept in ["<details", "<summary", "<strong", "type=\"checkbox\""] {
            assert!(HOSTILE_FIXTURE_HTML.contains(kept), "sanitizer ate {kept}: {HOSTILE_FIXTURE_HTML}");
        }
    }

    #[test]
    fn emitted_guides_carry_no_active_content() {
        assert!(!TIGER_GUIDES.is_empty());
        for guide in TIGER_GUIDES {
            assert!(!has_active_content(guide.html), "guide {} carries active content", guide.slug);
        }
        for reference in TIGER_REFERENCES {
            assert!(!has_active_content(reference.html), "reference {} carries active content", reference.slug);
        }
    }
}
