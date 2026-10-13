//! Per-project download artifacts, compiled in by `build.rs`.
//!
//! The data comes from `content/downloads.toml`, the single source for
//! every download box on the site (the /solutions grid cards and the
//! study pages render the same component from it). A platform carries
//! a URL only when a real binary for that platform has been published;
//! every platform without one renders as a disabled "Coming soon"
//! box — never a link to a releases page or a source archive.
include!(concat!(env!("OUT_DIR"), "/download_data.rs"));

/// The download artifacts published for `slug`.
///
/// A slug with no entry in `content/downloads.toml` gets the empty
/// record: all four platforms unpublished.
pub fn downloads_for(slug: &str) -> Downloads {
    DOWNLOADS.iter().find(|d| d.slug == slug).copied().unwrap_or(Downloads {
        slug: "",
        android: None,
        linux_aarch64: None,
        linux_x86_64: None,
        windows: None,
    })
}
